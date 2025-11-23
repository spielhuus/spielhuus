import { CodeEditor, EditorError } from '../../js/monaco_integration';

const STORAGE_KEY = 'wgpu_shader_code';

const INITIAL_SHADER_CODE = `
struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  deltaTime: f32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> @builtin(position) vec4<f32> {
  var pos = array<vec2<f32>, 3>(
    vec2(-1.0, -1.0),
    vec2( 3.0, -1.0),
    vec2(-1.0,  3.0)
  );
  return vec4<f32>(pos[vertexIndex], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) fragCoord: vec4<f32>) -> @location(0) vec4<f32> {
  let uv = fragCoord.xy / u.resolution;
  let color = vec3<f32>(
    uv.x, 
    uv.y, 
    0.5 + 0.5 * sin(u.time)
  );
  return vec4<f32>(color, 1.0);
}
`;

export class WebGpuRenderer {
  private canvas: HTMLCanvasElement;
  private device!: GPUDevice;
  private context!: GPUCanvasContext;
  private format!: GPUTextureFormat;
  private pipeline!: GPURenderPipeline;
  private uniformBuffer!: GPUBuffer;
  private bindGroup!: GPUBindGroup;

  private lastTime: number = 0;
  private totalTime: number = 0;
  private animationId: number = 0;
  private isPlaying: boolean = true;

  private frameCount: number = 0;
  private lastFpsTime: number = 0;
  private fpsElement: HTMLElement | null = null;

  private shaderCode: string;

  constructor(canvas: HTMLCanvasElement, shaderCode: string) {
    this.canvas = canvas;
    this.shaderCode = shaderCode;
  }

  public setFpsElement(element: HTMLElement | null) {
    this.fpsElement = element;
  }

  public togglePlay() {
    this.isPlaying = !this.isPlaying;
    if (this.isPlaying) {
      this.lastTime = performance.now();
      this.render(this.lastTime);
    } else {
      cancelAnimationFrame(this.animationId);
    }
    return this.isPlaying;
  }

  public async init(): Promise<boolean> {
    if (!navigator.gpu) return false;
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) return false;

    this.device = await adapter.requestDevice();
    this.context = this.canvas.getContext('webgpu') as GPUCanvasContext;
    this.format = navigator.gpu.getPreferredCanvasFormat();

    this.context.configure({
      device: this.device,
      format: this.format,
      alphaMode: 'opaque',
    });

    this.createUniformBuffer();
    this.createPipeline(this.shaderCode);
    this.createBindGroup();

    const observer = new ResizeObserver(entries => {
      for (const entry of entries) {
        const width = entry.devicePixelContentBoxSize?.[0].inlineSize ||
          entry.contentBoxSize[0].inlineSize * devicePixelRatio;
        const height = entry.devicePixelContentBoxSize?.[0].blockSize ||
          entry.contentBoxSize[0].blockSize * devicePixelRatio;

        this.canvas.width = Math.max(1, width);
        this.canvas.height = Math.max(1, height);

        this.context.configure({
          device: this.device,
          format: this.format,
          alphaMode: 'opaque',
        });
      }
    });
    observer.observe(this.canvas);

    this.lastTime = performance.now();
    this.lastFpsTime = this.lastTime;
    this.render(this.lastTime);

    return true;
  }

  private createUniformBuffer(): void {
    this.uniformBuffer = this.device.createBuffer({
      size: 4 * 4,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
  }

  private createPipeline(shaderModuleOrCode: string | GPUShaderModule): void {
    let shaderModule: GPUShaderModule;
    if (typeof shaderModuleOrCode === 'string') {
      shaderModule = this.device.createShaderModule({ code: shaderModuleOrCode });
    } else {
      shaderModule = shaderModuleOrCode;
    }

    this.pipeline = this.device.createRenderPipeline({
      layout: 'auto',
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_main',
        targets: [{ format: this.format }],
      },
      primitive: {
        topology: 'triangle-list',
      },
    });
  }

  private createBindGroup(): void {
    this.bindGroup = this.device.createBindGroup({
      layout: this.pipeline.getBindGroupLayout(0),
      entries: [{ binding: 0, resource: { buffer: this.uniformBuffer } }],
    });
  }

  private render(timestamp: number): void {
    const deltaTime = (timestamp - this.lastTime) / 1000;
    this.lastTime = timestamp;
    this.totalTime += deltaTime;

    this.frameCount++;
    if (timestamp - this.lastFpsTime >= 1000) {
      const fps = Math.round((this.frameCount * 1000) / (timestamp - this.lastFpsTime));
      if (this.fpsElement) this.fpsElement.innerText = `${fps} FPS`;
      this.frameCount = 0;
      this.lastFpsTime = timestamp;
    }

    const uniformData = new Float32Array([
      this.canvas.width,
      this.canvas.height,
      this.totalTime,
      deltaTime
    ]);
    this.device.queue.writeBuffer(this.uniformBuffer, 0, uniformData);

    const commandEncoder = this.device.createCommandEncoder();
    const textureView = this.context.getCurrentTexture().createView();

    const passEncoder = commandEncoder.beginRenderPass({
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0, g: 0, b: 0, a: 1 },
        loadOp: 'clear',
        storeOp: 'store',
      }],
    });

    passEncoder.setPipeline(this.pipeline);
    passEncoder.setBindGroup(0, this.bindGroup);
    passEncoder.draw(3);
    passEncoder.end();

    this.device.queue.submit([commandEncoder.finish()]);

    if (this.isPlaying) {
      this.animationId = requestAnimationFrame((t) => this.render(t));
    }
  }

  public async updateShader(newShaderCode: string): Promise<EditorError[]> {
    if (!this.device) return [];

    const shaderModule = this.device.createShaderModule({ code: newShaderCode });
    const compilationInfo = await shaderModule.getCompilationInfo();
    const errorMessages = compilationInfo.messages.filter(msg => msg.type === 'error');

    if (errorMessages.length > 0) {
      const errors: EditorError[] = errorMessages.map(msg => ({
        line: msg.lineNum,
        column: msg.linePos,
        length: msg.length,
        message: msg.message
      }));

      // console.error("Shader Errors:", errors);
      return errors;
    }

    try {
      this.createPipeline(shaderModule);
      this.createBindGroup();
    } catch (e) {
      console.warn("Pipeline creation failed:", e);
    }
    return [];
  }
}

// --- Execution ---

const canvas = document.querySelector('#gpu_shader') as HTMLCanvasElement;
const wrapper = document.querySelector('#gpu_wrapper') as HTMLElement;
const playBtn = document.querySelector('#gpu_btn-play-pause') as HTMLButtonElement;
const fpsLabel = document.querySelector('#gpu_fps-counter') as HTMLElement;
const fullBtn = document.querySelector('#gpu_btn-fullscreen') as HTMLButtonElement;
const resetBtn = document.querySelector('#gpu_btn-reset') as HTMLButtonElement;
const savedCode = localStorage.getItem(STORAGE_KEY);
const startCode = savedCode ? savedCode : INITIAL_SHADER_CODE;
const renderer = new WebGpuRenderer(canvas, startCode);
renderer.setFpsElement(fpsLabel);

if (wrapper) {
  let inactivityTimer: number;
  const showControls = () => {
    wrapper.classList.add('gpu_user-active');
    clearTimeout(inactivityTimer);
    inactivityTimer = setTimeout(() => {
      wrapper.classList.remove('gpu_user-active');
    }, 2000);
  };

  wrapper.addEventListener('mousemove', showControls);
  wrapper.addEventListener('click', showControls);
  wrapper.addEventListener('mouseleave', () => {
    clearTimeout(inactivityTimer);
    wrapper.classList.remove('gpu_user-active');
  });
}

// Play/Pause
if (playBtn) {
  playBtn.addEventListener('click', () => {
    const isNowPlaying = renderer.togglePlay();
    playBtn.innerText = isNowPlaying ? '⏸' : '▶';
  });
}

// Fullscreen
if (fullBtn && wrapper) {
  fullBtn.addEventListener('click', () => {
    if (!document.fullscreenElement) {
      wrapper.requestFullscreen().catch(err => {
        console.error(`Error attempting to enable fullscreen: ${err.message}`);
      });
    } else {
      document.exitFullscreen();
    }
  });

  document.addEventListener('fullscreenchange', () => {
    if (document.fullscreenElement) {
      fullBtn.innerText = '✕';
      fullBtn.title = "Exit Fullscreen";
    } else {
      fullBtn.innerText = '⛶';
      fullBtn.title = "Enter Fullscreen";
    }
  });
}

renderer.init();

// Monaco
try {
  const myEditor = new CodeEditor('monaco-container', startCode, 'rust');
  myEditor.editor.onDidChangeModelContent(async () => {
    const newCode = myEditor.getValue();
    localStorage.setItem(STORAGE_KEY, newCode);

    const errors = await renderer.updateShader(newCode);
    myEditor.setMarkers(errors);
  });

  if (resetBtn) {
    resetBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      if (confirm("Reset code to default? This will lose your changes.")) {
        localStorage.removeItem(STORAGE_KEY);
        myEditor.editor.setValue(INITIAL_SHADER_CODE);
      }
    });
  }

} catch (e) {
  console.error("Failed to load Monaco Editor", e);
}
