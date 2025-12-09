import { CodeEditor } from '../../js/monaco_integration';
import type { EditorError } from '../../js/monaco_integration';

const STORAGE_KEY = 'wgpu_palette_code';

const RESULT = `
const a = vec3<f32>(0.5, 0.5, 0.5);
const b = vec3<f32>(0.5, 0.5, 0.5);
const c = vec3<f32>(1.0, 1.0, 1.0);
const d = vec3<f32>(0.00, 0.33, 0.67);
`

const INITIAL_SHADER_CODE = `
struct Uniforms {
  // Each vec3 is 12 bytes, but due to alignment rules, it's padded to 16 bytes.
  // So the total size is 4 * 16 = 64 bytes.
  resolution: vec2<f32>,
  param1: vec3<f32>,
  param2: vec3<f32>,
  param3: vec3<f32>,
  param4: vec3<f32>,
};

fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32>
{
    return a + b*cos(6.283185*(c*t+d));
}

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
  let normalizedX = fragCoord.x / u.resolution.x;

  // Use the normalized value and one of the new params to set the color.
  // The color will be black on the left, and the color of param1 on the right.
  let color = u.param1 * normalizedX;

  return vec4<f32>(palette(normalizedX, u.param1, u.param2, u.param3, u.param4), 1.0);
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

  private frameCount: number = 0;
  private lastFpsTime: number = 0;
  private fpsElement: HTMLElement | null = null;

  private shaderCode: string;

  private param1 = [0.5, 0.5, 0.5];
  private param2 = [0.5, 0.5, 0.5];
  private param3 = [1.0, 1.0, 1.0];
  private param4 = [0.00, 0.33, 0.67];

  constructor(canvas: HTMLCanvasElement, shaderCode: string) {
    this.canvas = canvas;
    this.shaderCode = shaderCode;
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
      requestAnimationFrame((t) => this.render(t));
    });
    observer.observe(this.canvas);

    this.lastTime = performance.now();
    this.lastFpsTime = this.lastTime;
    this.render(this.lastTime);

    return true;
  }

  private createUniformBuffer(): void {
    this.uniformBuffer = this.device.createBuffer({
      size: 80,
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

  public draw() {
    requestAnimationFrame((t) => this.render(t));
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
      0.0, 0.0,

      this.param1[0],
      this.param1[1],
      this.param1[2],
      0.0,

      this.param2[0],
      this.param2[1],
      this.param2[2],
      0.0,

      this.param3[0],
      this.param3[1],
      this.param3[2],
      0.0,

      this.param4[0],
      this.param4[1],
      this.param4[2],
      0.0,
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
      return errors;
    }

    try {
      this.createPipeline(shaderModule);
      this.createBindGroup();
    } catch (e) {
      console.warn("Pipeline creation failed:", e);
    }
    requestAnimationFrame((t) => this.render(t));
    return [];
  }

  public updateParam(paramIndex: 1 | 2 | 3 | 4, componentIndex: 0 | 1 | 2, value: number) {
    switch (paramIndex) {
      case 1: this.param1[componentIndex] = value; break;
      case 2: this.param2[componentIndex] = value; break;
      case 3: this.param3[componentIndex] = value; break;
      case 4: this.param4[componentIndex] = value; break;
    }
    requestAnimationFrame((t) => this.render(t));
  }
  /**
    * Reads the current parameter values and formats them into a WGSL code string.
    * @returns A string containing the WGSL constants for the current palette.
    */
  public getPaletteAsWGSL(): string {
    // Helper function to format a number array into a vec3 string,
    // ensuring each number has two decimal places.
    const formatVec3 = (p: number[]) => {
      const x = p[0].toFixed(2);
      const y = p[1].toFixed(2);
      const z = p[2].toFixed(2);
      return `vec3<f32>(${x}, ${y}, ${z})`;
    };

    // Use a template literal to build the final string, using the helper
    // for each parameter array.
    const RESULT = `
const a = ${formatVec3(this.param1)};
const b = ${formatVec3(this.param2)};
const c = ${formatVec3(this.param3)};
const d = ${formatVec3(this.param4)};
`;
    return RESULT;
  }
}

function setupInputHandlers(renderer: WebGpuRenderer) {
  for (let i = 1; i <= 4; i++) {
    for (let j = 1; j <= 3; j++) {
      const numberInput = document.getElementById(`p${i}_num${j}`) as HTMLInputElement;
      const sliderInput = document.getElementById(`p${i}_slider${j}`) as HTMLInputElement;

      if (numberInput && sliderInput) {
        const updateValue = (newValue: string) => {
          const valueAsNumber = parseFloat(newValue);
          numberInput.value = newValue;
          sliderInput.value = newValue;
          renderer.updateParam(i as 1 | 2 | 3 | 4, (j - 1) as 0 | 1 | 2, valueAsNumber);

          //update the result code
          const codeOutput = document.querySelector('#result-code') as HTMLElement;
          const wgslCode = renderer.getPaletteAsWGSL();
          codeOutput.textContent = wgslCode;
        };
        numberInput.addEventListener('input', (event) => {
          updateValue((event.target as HTMLInputElement).value);
        });
        sliderInput.addEventListener('input', (event) => {
          updateValue((event.target as HTMLInputElement).value);
        });
      }
    }
  }
}

// --- Execution ---

const canvas = document.querySelector('#gpu_shader') as HTMLCanvasElement;
const wrapper = document.querySelector('#gpu_wrapper') as HTMLElement;
const resetBtn = document.querySelector('#gpu_btn-reset') as HTMLButtonElement;
const savedCode = localStorage.getItem(STORAGE_KEY);
const startCode = savedCode ? savedCode : INITIAL_SHADER_CODE;
const renderer = new WebGpuRenderer(canvas, startCode);

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

renderer.init().then(() => {
  setupInputHandlers(renderer);
  renderer.draw();
});

// Monaco
try {
  const myEditor = new CodeEditor('monaco-container', startCode, 'c');
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
