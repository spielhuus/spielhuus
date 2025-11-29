import { CodeEditor } from '../../js/monaco_integration';
import type { EditorError } from '../../js/monaco_integration';

const STORAGE_KEY = 'gol_shader_code';

const INITIAL_SHADER_CODE = `
struct Uniforms {
  resolution: vec2<f32>,
  grid_size: vec2<f32>,
  time: f32,
  birth_rule: u32,
  survive_rule: u32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2f,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> cellStateIn: array<u32>;
@group(0) @binding(2) var<storage, read_write> cellStateOut: array<u32>;


fn cellIndex(cell: vec2u) -> u32 {
   let clamped_cell = min(cell, vec2u(uniforms.grid_size) - vec2u(1,1));
   return clamped_cell.y * u32(uniforms.grid_size.x) + clamped_cell.x;
}

fn cellActive(x: u32, y: u32) -> u32 {
   let wrapped_x = (x + u32(uniforms.grid_size.x)) % u32(uniforms.grid_size.x);
   let wrapped_y = (y + u32(uniforms.grid_size.y)) % u32(uniforms.grid_size.y);
   
   if (cellStateIn[cellIndex(vec2(wrapped_x, wrapped_y))] > 0u) {
     return 1u;
   }
   return 0u;
}

fn is_in_rule(neighbors: u32, rule_mask: u32) -> bool {
    // e.g., if neighbors is 3 and mask is for B3 (..001000),
    // (mask >> 3) is (..000001).
    // (..000001) & 1u is 1u, so it returns true.
    return ((rule_mask >> neighbors) & 1u) == 1u;
}

@compute @workgroup_size(8, 8)
fn compute_main(@builtin(global_invocation_id) cell: vec3u) {
   // Determine how many active neighbors this cell has.
   let activeNeighbors = cellActive(cell.x+1, cell.y+1) +
                         cellActive(cell.x+1, cell.y) +
                         cellActive(cell.x+1, cell.y-1) +
                         cellActive(cell.x, cell.y-1) +
                         cellActive(cell.x-1, cell.y-1) +
                         cellActive(cell.x-1, cell.y) +
                         cellActive(cell.x-1, cell.y+1) +
                         cellActive(cell.x, cell.y+1);

   let i = cellIndex(cell.xy);

  let state = cellStateIn[i];
   if (state > 0u) { // If the cell is ALIVE
     if (is_in_rule(activeNeighbors, uniforms.survive_rule)) {
       cellStateOut[i] = state + 1u;
     } else {
       cellStateOut[i] = 0u;
     }
   } else { // If the cell is DEAD
     if (is_in_rule(activeNeighbors, uniforms.birth_rule)) {
       cellStateOut[i] = 1u;
     } else {
       cellStateOut[i] = 0u;
     }
   }
}

@vertex
fn vs_main(
    vert: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.uv = vert.position.xy * 0.5 + 0.5;
    output.position = vec4f(vert.position, 1.0);
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
   // Calculate which cell this pixel belongs to.
    let cell = vec2u(floor(in.uv * uniforms.grid_size));
 
    // Find the index of that cell and get its state.
    let state = f32(cellStateIn[cellIndex(cell)]);

    // The rest of the coloring logic is the same as before.
    if (state < 1.0) {
        // Discarding is an option, but returning transparent black is fine.
        return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Return black for dead cells
    }

    let max_age = 20.0;
    let t = clamp(state / max_age, 0.0, 1.0);

    let color_young = vec3<f32>(1.0, 0.0, 0.0); // Red
    let color_old = vec3<f32>(0.4, 0.0, 0.8);   // Dark Purple

    let final_color = mix(color_young, color_old, t);
    return vec4<f32>(final_color, 1.0);
}
`;

export class WebGpuRenderer {
  private canvas: HTMLCanvasElement;
  private device!: GPUDevice;
  private context!: GPUCanvasContext;
  private format!: GPUTextureFormat;

  private renderPipeline!: GPURenderPipeline;
  private computePipeline!: GPUComputePipeline;
  private bindGroupLayout!: GPUBindGroupLayout;
  private uniformBuffer!: GPUBuffer;
  private cellStateBuffers!: [GPUBuffer, GPUBuffer];
  private vertexBuffer!: GPUBuffer;
  private bindGroups!: [GPUBindGroup, GPUBindGroup];
  private simulationStep = 0;
  private WORKGROUP_SIZE = 8;

  private lastTime: number = 0;
  private totalTime: number = 0;
  private animationId: number = 0;
  private isPlaying: boolean = true;

  private frameCount: number = 0;
  private lastFpsTime: number = 0;
  private fpsElement: HTMLElement | null = null;

  private shaderCode: string;

  private birth: number;
  private survive: number;
  private cellSize: number;

  constructor(canvas: HTMLCanvasElement, shaderCode: string) {
    this.canvas = canvas;
    this.shaderCode = shaderCode;
    this.birth = 1 << 3;
    this.survive = (1 << 2) | (1 << 3);
    this.cellSize = 4;
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

  private ruleBitmask(rule_num: number): number {
    let mask: number = 0;
    let n: number = rule_num;

    if (n === 0) {
      return 1 << 0;
    }

    while (n > 0) {
      const digit = n % 10;
      if (digit <= 8) {
        mask |= 1 << digit;
      }
      n = Math.floor(n / 10);
    }
    return mask;
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

    this.createVertexBuffer();
    this.createUniformBuffer();
    this.createStorageBuffers();

    try {
      const shaderModule = this.device.createShaderModule({ code: this.shaderCode });
      await this.createPipelinesAndBindGroups(shaderModule);
    } catch (e) {
      console.error("Failed to initialize pipelines:", e);
      return false;
    }

    const observer = new ResizeObserver(entries => {
      for (const entry of entries) {
        const width = entry.devicePixelContentBoxSize?.[0].inlineSize ||
          entry.contentBoxSize[0].inlineSize * devicePixelRatio;
        const height = entry.devicePixelContentBoxSize?.[0].blockSize ||
          entry.contentBoxSize[0].blockSize * devicePixelRatio;

        // Prevent resize loops and zero-sized canvases
        if (width !== this.canvas.width || height !== this.canvas.height) {
          this.handleResize(width, height);
        }
      }
    });
    observer.observe(this.canvas);

    this.lastTime = performance.now();
    this.lastFpsTime = this.lastTime;
    this.render(this.lastTime);

    return true;
  }

  private resetSimulation(): void {
    if (!this.device) return;
    const wasPlaying = this.isPlaying;
    this.isPlaying = false;
    cancelAnimationFrame(this.animationId);
    this.simulationStep = 0;
    this.createStorageBuffers();
    this.createBindGroups();
    if (wasPlaying) {
      this.isPlaying = true;
      this.animationId = requestAnimationFrame((t) => this.render(t));
    }
  }

  private handleResize(width: number, height: number) {
    if (!this.device) return;

    this.canvas.width = Math.max(1, width);
    this.canvas.height = Math.max(1, height);

    this.context.configure({
      device: this.device,
      format: this.format,
      alphaMode: 'opaque',
    });
    this.resetSimulation();
  }


  private createVertexBuffer(): void {
    const vertices = new Float32Array([
      // Triangle 1
      -1.0, -1.0, 0.0,
      1.0, -1.0, 0.0,
      1.0, 1.0, 0.0,
      // Triangle 2
      -1.0, -1.0, 0.0,
      1.0, 1.0, 0.0,
      -1.0, 1.0, 0.0,
    ]);

    this.vertexBuffer = this.device.createBuffer({
      label: 'Vertex Buffer',
      size: vertices.byteLength,
      usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    });
    this.device.queue.writeBuffer(this.vertexBuffer, 0, vertices);
  }


  private createUniformBuffer(): void {
    this.uniformBuffer = this.device.createBuffer({
      size: 32,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
  }

  private createStorageBuffers(): void {
    const gridSizeX = Math.ceil(this.canvas.width / this.cellSize);
    const gridSizeY = Math.ceil(this.canvas.height / this.cellSize);
    const cellCount = gridSizeX * gridSizeY;

    if (this.cellStateBuffers) {
      this.cellStateBuffers[0].destroy();
      this.cellStateBuffers[1].destroy();
    }

    this.cellStateBuffers = [
      this.device.createBuffer({
        label: 'Cell State 0',
        size: cellCount * 4, // u32 is 4 bytes
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      }),
      this.device.createBuffer({
        label: 'Cell State 1',
        size: cellCount * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      })
    ];

    const initialCellState = new Uint32Array(cellCount);
    for (let i = 0; i < cellCount; ++i) {
      initialCellState[i] = Math.random() > 0.6 ? 1 : 0;
    }
    this.device.queue.writeBuffer(this.cellStateBuffers[0], 0, initialCellState);
  }

  private async createPipelinesAndBindGroups(shaderModule: GPUShaderModule): Promise<void> {
    this.bindGroupLayout = this.device.createBindGroupLayout({
      entries: [
        { binding: 0, visibility: GPUShaderStage.COMPUTE | GPUShaderStage.FRAGMENT | GPUShaderStage.VERTEX, buffer: { type: 'uniform' } },
        { binding: 1, visibility: GPUShaderStage.COMPUTE | GPUShaderStage.FRAGMENT, buffer: { type: 'read-only-storage' } },
        { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } },
      ]
    });

    const pipelineLayout = this.device.createPipelineLayout({
      bindGroupLayouts: [this.bindGroupLayout]
    });

    // Create Compute Pipeline
    this.computePipeline = this.device.createComputePipeline({
      layout: pipelineLayout,
      compute: {
        module: shaderModule,
        entryPoint: 'compute_main',
      },
    });

    // Create Render Pipeline
    this.renderPipeline = this.device.createRenderPipeline({
      layout: pipelineLayout,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
        buffers: [{
          arrayStride: 3 * 4,
          attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x3' }],
        }],
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
    this.createBindGroups();

  }

  private createBindGroups(): void {
    this.bindGroups = [
      this.device.createBindGroup({
        layout: this.bindGroupLayout, // Use the stored layout
        entries: [
          { binding: 0, resource: { buffer: this.uniformBuffer } },
          { binding: 1, resource: { buffer: this.cellStateBuffers[0] } }, // In
          { binding: 2, resource: { buffer: this.cellStateBuffers[1] } }, // Out
        ],
      }),
      this.device.createBindGroup({
        layout: this.bindGroupLayout, // Use the stored layout
        entries: [
          { binding: 0, resource: { buffer: this.uniformBuffer } },
          { binding: 1, resource: { buffer: this.cellStateBuffers[1] } }, // In
          { binding: 2, resource: { buffer: this.cellStateBuffers[0] } }, // Out
        ],
      }),
    ];
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

    const gridSizeX = Math.ceil(this.canvas.width / this.cellSize);
    const gridSizeY = Math.ceil(this.canvas.height / this.cellSize);

    const uniformArrayBuffer = new ArrayBuffer(32);
    const uniformFloatView = new Float32Array(uniformArrayBuffer);
    const uniformUintView = new Uint32Array(uniformArrayBuffer);

    uniformFloatView[0] = this.canvas.width;  // resolution.x
    uniformFloatView[1] = this.canvas.height; // resolution.y
    uniformFloatView[2] = gridSizeX;          // grid_size.x
    uniformFloatView[3] = gridSizeY;          // grid_size.y
    uniformFloatView[4] = this.totalTime;     // time
    uniformUintView[5] = this.birth;           // birth_rule
    uniformUintView[6] = this.survive;         // survive_rule

    this.device.queue.writeBuffer(this.uniformBuffer, 0, uniformArrayBuffer);

    const commandEncoder = this.device.createCommandEncoder();

    const computePass = commandEncoder.beginComputePass();
    computePass.setPipeline(this.computePipeline);
    computePass.setBindGroup(0, this.bindGroups[this.simulationStep % 2]);

    const workgroupCountX = Math.ceil(gridSizeX / this.WORKGROUP_SIZE);
    const workgroupCountY = Math.ceil(gridSizeY / this.WORKGROUP_SIZE);

    computePass.dispatchWorkgroups(workgroupCountX, workgroupCountY);
    computePass.end();

    const textureView = this.context.getCurrentTexture().createView();
    const renderPass = commandEncoder.beginRenderPass({
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0, g: 0, b: 0, a: 0 },
        loadOp: 'clear',
        storeOp: 'store',
      }],
    });

    renderPass.setPipeline(this.renderPipeline);
    renderPass.setBindGroup(0, this.bindGroups[this.simulationStep % 2]);
    renderPass.setVertexBuffer(0, this.vertexBuffer);
    renderPass.draw(6);
    renderPass.end();

    this.device.queue.submit([commandEncoder.finish()]);
    this.simulationStep++;

    if (this.isPlaying) {
      this.animationId = requestAnimationFrame((t) => this.render(t));
    }
  }

  public async updateShader(newShaderCode: string): Promise<EditorError[]> {
    if (!this.device) return [];

    try {
      const shaderModule = this.device.createShaderModule({ code: newShaderCode });
      const compilationInfo = await shaderModule.getCompilationInfo();
      const errorMessages = compilationInfo.messages.filter(msg => msg.type === 'error');

      if (errorMessages.length > 0) {
        return errorMessages.map(msg => ({
          line: msg.lineNum, column: msg.linePos, length: msg.length, message: msg.message
        }));
      }

      await this.createPipelinesAndBindGroups(shaderModule);

    } catch (e: any) {
      console.error("Failed to update shader and pipelines:", e);
      return [{ line: 1, column: 1, length: 1, message: e.message }];
    }
    return [];
  }

  public setBirth(birth: number) {
    this.birth = this.ruleBitmask(birth);
    this.resetSimulation();
  }
  public setSurvive(survive: number) {
    this.survive = this.ruleBitmask(survive);
    this.resetSimulation();
  }
  public setCellSize(cellSize: number) {
    this.cellSize = cellSize;
    this.resetSimulation();
  }
}

// --- Execution ---

const birthInput = document.querySelector('#birth') as HTMLInputElement;
const surviveInput = document.querySelector('#survive') as HTMLInputElement;
const sizeInput = document.querySelector('#size') as HTMLInputElement;

if (birthInput) {
  birthInput.addEventListener('change', () => {
    const val = parseInt(birthInput.value);
    if (!isNaN(val)) {
      renderer.setBirth(val);
    }
  });
}

if (surviveInput) {
  surviveInput.addEventListener('change', () => {
    const val = parseInt(surviveInput.value);
    if (!isNaN(val)) {
      renderer.setSurvive(val);
    }
  });
}

if (sizeInput) {
  sizeInput.addEventListener('change', () => {
    const val = parseInt(sizeInput.value);
    if (!isNaN(val)) {
      renderer.setCellSize(val);
    }
  });
}

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
