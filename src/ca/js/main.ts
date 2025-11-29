import { CodeEditor } from '../../js/monaco_integration';
import type { EditorError } from '../../js/monaco_integration';

const STORAGE_KEY = 'ca_shader_code';

const INITIAL_SHADER_CODE = `
@group(0) @binding(0) var<uniform> u_params: Params;
@group(0) @binding(1) var source_texture: texture_storage_2d<r32uint, read>;
@group(0) @binding(2) var dest_texture: texture_storage_2d<r32uint, write>;
@group(0) @binding(1) var display_texture: texture_2d<u32>;

struct Params {
    // The ECA rule, from 0 to 255.
    rule: u32,
    width: u32,
    height: u32,
    size: u32,
    current_generation: u32, 
};

// This function takes the 3-cell neighborhood (left, center, right)
// and looks up the new state based on the rule.
fn get_rule_output(left: u32, center: u32, right: u32, rule: u32) -> u32 {
    // Combine the 3 cells into a 3-bit number (0-7).
    let index = (left << 2) | (center << 1) | right;
    // Use the index to check the corresponding bit in the 8-bit rule.
    return (rule >> index) & 1u;
}

// -------------------- COMPUTE SHADER --------------------

// Our workgroup size. Can be tuned for performance. 64 is a good start.
@compute @workgroup_size(64, 1, 1)
fn compute_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let x = global_id.x;
    let y = u_params.current_generation;

    // Stop if we're past the texture dimensions
    if (x >= u_params.width || y >= u_params.height) {
        return;
    }

    // This is the generation we are *writing to*.
    let current_gen = y;
    // We read from the generation *before* it.
    let prev_gen = y - 1u;

    // Handle the first generation (y=0) separately, as it has no parent.
    // We let it be initialized by the CPU and do no work here.
    if (current_gen == 0) {
        return;
    }

    // Get the texture dimensions for wrapping logic
    let dims = textureDimensions(source_texture);

    // Read the 3-cell neighborhood from the previous generation (prev_gen).
    // We use modulo arithmetic for wrapping (toroidal) boundary conditions.
    let left_x = (x + u_params.width - 1u) % u_params.width;
    let center_x = x;
    let right_x = (x + 1u) % u_params.width;

    // textureLoad wants an ivec2, so we cast.
    let left_val = textureLoad(source_texture, vec2<i32>(i32(left_x), i32(prev_gen))).r;
    let center_val = textureLoad(source_texture, vec2<i32>(i32(center_x), i32(prev_gen))).r;
    let right_val = textureLoad(source_texture, vec2<i32>(i32(right_x), i32(prev_gen))).r;
    
    // Calculate the new cell state.
    let new_state = get_rule_output(left_val, center_val, right_val, u_params.rule);

    // Write the new state to the destination texture.
    textureStore(dest_texture, vec2<i32>(i32(x), i32(current_gen)), vec4<u32>(new_state, 0, 0, 0));
}


// -------------------- RENDER SHADER --------------------

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(in_vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(in_vertex_index % 2u) * 4.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(
        (out.clip_position.x + 1.0) / 2.0,
        1.0 - (out.clip_position.y + 1.0) / 2.0
    );
    return out;
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let texel_coords = vec2<i32>(frag_coord.xy / f32(u_params.size));
    let int_color = textureLoad(display_texture, texel_coords, 0).r;
    let color = f32(int_color);
    return vec4<f32>(0.9 * color, 0.0, 0.0, color);
}
`

// const INITIAL_SHADER_CODE = `
// struct Uniforms {
//   rule: u32,
//   width: u32,
//   height: u32,
//   size: u32,
//   current_generation: u32,
// };
//
// @group(0) @binding(0) var<uniform> u: Uniforms;
// // Binding 1: Read-only current grid (for rendering) or source (for compute)
// @group(0) @binding(1) var grid_texture: texture_2d<u32>;
// // Binding 2: Write-only next row (for compute)
// @group(0) @binding(2) var intermediate_texture: texture_storage_2d<r32uint, write>;
//
// // --- Vertex Shader ---
// struct VertexOutput {
//   @builtin(position) position: vec4<f32>,
//   @location(0) uv: vec2<f32>,
// };
//
// @vertex
// fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
//   var pos = array<vec2<f32>, 3>(
//     vec2(-1.0, -1.0),
//     vec2( 3.0, -1.0),
//     vec2(-1.0,  3.0)
//   );
//   var output: VertexOutput;
//   output.position = vec4<f32>(pos[vertexIndex], 0.0, 1.0);
//
//   // Convert position to UV (0.0 to 1.0)
//   output.uv = output.position.xy * 0.5 + 0.5;
//   // Flip Y for texture sampling if needed, though wgpu standard is usually consistent
//   output.uv.y = 1.0 - output.uv.y; 
//   return output;
// }
//
// // --- Fragment Shader ---
// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//   let x = u32(in.uv.x * f32(u.width));
//   let y = u32(in.uv.y * f32(u.height));
//
//   // Sample the texture. Note: textures are 0-indexed.
//   // textureLoad returns vec4<u32>, we only care about the red channel (r)
//   let cell = textureLoad(grid_texture, vec2<u32>(x, y), 0).r;
//
//   if (cell == 1u) {
//      return vec4<f32>(1.0, 1.0, 1.0, 1.0); // White
//   } else {
//      return vec4<f32>(0.0, 0.0, 0.0, 0.0); // transparent
//   }
// }
//
// // --- Compute Shader ---
// @compute @workgroup_size(64)
// fn compute_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
//     let x = global_id.x;
//     if (x >= u.width) {
//         return;
//     }
//
//     // Determine neighbors from the PREVIOUS row (current_generation - 1)
//     let prev_row = u.current_generation - 1u;
//
//     // Handle wrapping (Left and Right edges)
//     let left_idx = select(x - 1u, u.width - 1u, x == 0u);
//     let right_idx = select(x + 1u, 0u, x == u.width - 1u);
//
//     let left = textureLoad(grid_texture, vec2<u32>(left_idx, prev_row), 0).r;
//     let center = textureLoad(grid_texture, vec2<u32>(x, prev_row), 0).r;
//     let right = textureLoad(grid_texture, vec2<u32>(right_idx, prev_row), 0).r;
//
//     // Calculate index for the rule (binary 111 = 7, 000 = 0)
//     let index = (left << 2u) | (center << 1u) | right;
//
//     // Check if the bit at 'index' is set in 'rule'
//     let state = (u.rule >> index) & 1u;
//
//     // Write to the intermediate texture at the current row
//     textureStore(intermediate_texture, vec2<u32>(x, u.current_generation), vec4<u32>(state, 0u, 0u, 0u));
// }
// `;

enum StartRow {
  Middle = 1,
  Left = 2,
  Right = 3,
  Random = 4,
}

export class WebGpuRenderer {
  private canvas: HTMLCanvasElement;
  private device!: GPUDevice;
  private context!: GPUCanvasContext;
  private format!: GPUTextureFormat;

  private computePipeline!: GPUComputePipeline;
  private renderPipeline!: GPURenderPipeline;
  private uniformBuffer!: GPUBuffer;
  private computeBindGroup!: GPUBindGroup;
  private renderBindGroup!: GPUBindGroup;

  private gridTexture!: GPUTexture;
  private intermediateTexture!: GPUTexture;

  private lastTime: number = 0;
  private animationId: number = 0;
  private isPlaying: boolean = true;

  private frameCount: number = 0;
  private lastFpsTime: number = 0;
  private fpsElement: HTMLElement | null = null;

  private shaderCode: string;

  public rule: number = 90;
  public cellSize: number = 4;
  public startRow: StartRow = StartRow.Middle;

  private gridWidth: number = 0;
  private gridHeight: number = 0;
  private currentGeneration: number = 1;

  constructor(canvas: HTMLCanvasElement, shaderCode: string) {
    this.canvas = canvas;
    this.shaderCode = shaderCode;
  }

  public setFpsElement(element: HTMLElement | null) {
    this.fpsElement = element;
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
      alphaMode: 'premultiplied',
    });

    this.createUniformBuffer();

    await this.resizeGrid();
    const observer = new ResizeObserver(entries => {
      for (const entry of entries) {
        const width = entry.devicePixelContentBoxSize?.[0].inlineSize ||
          entry.contentBoxSize[0].inlineSize * devicePixelRatio;
        const height = entry.devicePixelContentBoxSize?.[0].blockSize ||
          entry.contentBoxSize[0].blockSize * devicePixelRatio;

        const needsResize = this.canvas.width !== width || this.canvas.height !== height;

        this.canvas.width = Math.max(1, width);
        this.canvas.height = Math.max(1, height);

        this.context.configure({
          device: this.device,
          format: this.format,
          alphaMode: 'premultiplied',
        });

        if (needsResize && this.device) {
          this.resizeGrid();
        }
      }
    });
    observer.observe(this.canvas);

    this.lastTime = performance.now();
    this.lastFpsTime = this.lastTime;
    this.render(this.lastTime);

    return true;
  }

  public async resizeGrid() {
    if (!this.device) return;
    this.isPlaying = false;
    if (this.animationId) {
      cancelAnimationFrame(this.animationId);
      this.animationId = 0;
    }
    this.gridWidth = Math.floor(this.canvas.width / this.cellSize);
    this.gridHeight = Math.floor(this.canvas.height / this.cellSize);

    if (this.gridWidth === 0 || this.gridHeight === 0) return;

    // Create Textures
    const textureDesc: GPUTextureDescriptor = {
      size: [this.gridWidth, this.gridHeight, 1],
      format: 'r32uint',
      usage: GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.STORAGE_BINDING |
        GPUTextureUsage.COPY_DST |
        GPUTextureUsage.COPY_SRC,
    };

    if (this.gridTexture) this.gridTexture.destroy();
    if (this.intermediateTexture) this.intermediateTexture.destroy();

    this.gridTexture = this.device.createTexture(textureDesc);
    this.intermediateTexture = this.device.createTexture({
      ...textureDesc,
      label: 'Intermediate Grid Texture'
    });

    // Recreate Pipelines and BindGroups with new shader/textures
    await this.createPipelinesAndGroups(this.shaderCode);

    this.reset();
  }

  public reset() {
    if (!this.device || !this.gridTexture) return;

    this.isPlaying = true;
    this.currentGeneration = 1;

    const size = this.gridWidth * this.gridHeight;
    const initialData = new Uint32Array(size); // Default 0s

    // Set initial state
    if (this.startRow === StartRow.Middle) {
      initialData[Math.floor(this.gridWidth / 2)] = 1;
    } else if (this.startRow === StartRow.Left) {
      initialData[0] = 1;
    } else if (this.startRow === StartRow.Right) {
      initialData[this.gridWidth - 1] = 1;
    } else if (this.startRow === StartRow.Random) {
      for (let i = 0; i < this.gridWidth; i++) {
        if (Math.random() > 0.5) {
          initialData[i] = 1;
        }
      }
    }

    // Upload to Grid Texture
    this.device.queue.writeTexture(
      { texture: this.gridTexture },
      initialData,
      { bytesPerRow: this.gridWidth * 4, rowsPerImage: this.gridHeight },
      [this.gridWidth, this.gridHeight, 1]
    );

    // Clear Intermediate Texture
    const clearData = new Uint32Array(size);
    this.device.queue.writeTexture(
      { texture: this.intermediateTexture },
      clearData,
      { bytesPerRow: this.gridWidth * 4, rowsPerImage: this.gridHeight },
      [this.gridWidth, this.gridHeight, 1]
    );
    if (!this.animationId) {
      this.lastTime = performance.now();
      this.render(this.lastTime);
    }
  }

  private createUniformBuffer(): void {
    this.uniformBuffer = this.device.createBuffer({
      size: 32,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
  }

  private async createPipelinesAndGroups(code: string): Promise<void> {
    const shaderModule = this.device.createShaderModule({ code });

    // Check for compilation errors
    const info = await shaderModule.getCompilationInfo();
    if (info.messages.some(m => m.type === 'error')) {
      throw new Error("Shader compilation error");
    }

    // --- Compute Pipeline ---
    // Layout: 0: Uniform, 1: Grid(Read), 2: Intermediate(Write)
    // Note: In WebGPU, we usually define layout explicitly or use 'auto'. 
    // Since we re-create bindgroups often, let's use auto for simplicity here 
    // matching the Rust logic which binds these specific slots.

    this.computePipeline = this.device.createComputePipeline({
      layout: 'auto',
      compute: {
        module: shaderModule,
        entryPoint: 'compute_main',
      },
    });

    this.computeBindGroup = this.device.createBindGroup({
      layout: this.computePipeline.getBindGroupLayout(0),
      entries: [
        { binding: 0, resource: { buffer: this.uniformBuffer } },
        { binding: 1, resource: this.gridTexture.createView() },
        { binding: 2, resource: this.intermediateTexture.createView() },
      ],
    });

    // --- Render Pipeline ---
    this.renderPipeline = this.device.createRenderPipeline({
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

    this.renderBindGroup = this.device.createBindGroup({
      layout: this.renderPipeline.getBindGroupLayout(0),
      entries: [
        { binding: 0, resource: { buffer: this.uniformBuffer } },
        { binding: 1, resource: this.gridTexture.createView() },
      ],
    });
  }

  private updateSim() {
    if (this.currentGeneration < this.gridHeight) {
      // Update Uniforms
      // rule(u32), width(u32), height(u32), size(u32), current_gen(u32)
      const uniformData = new Uint32Array([
        this.rule,
        this.gridWidth,
        this.gridHeight,
        this.cellSize,
        this.currentGeneration
      ]);
      this.device.queue.writeBuffer(this.uniformBuffer, 0, uniformData);

      const encoder = this.device.createCommandEncoder({ label: 'Compute Encoder' });

      // Compute Pass
      const computePass = encoder.beginComputePass();
      computePass.setPipeline(this.computePipeline);
      computePass.setBindGroup(0, this.computeBindGroup);
      const workgroupCountX = Math.ceil(this.gridWidth / 64);
      computePass.dispatchWorkgroups(workgroupCountX);
      computePass.end();

      // Copy newly computed row from Intermediate -> Grid
      // Rust: copy_texture_to_texture
      encoder.copyTextureToTexture(
        {
          texture: this.intermediateTexture,
          mipLevel: 0,
          origin: { x: 0, y: this.currentGeneration, z: 0 }
        },
        {
          texture: this.gridTexture,
          mipLevel: 0,
          origin: { x: 0, y: this.currentGeneration, z: 0 }
        },
        {
          width: this.gridWidth,
          height: 1,
          depthOrArrayLayers: 1
        }
      );

      this.device.queue.submit([encoder.finish()]);
      this.currentGeneration++;
    } else {
      this.isPlaying = false;
    }
  }

  private render(timestamp: number): void {
    if (!this.device || !this.renderPipeline) return;
    if (!this.isPlaying) {
      this.animationId = 0;
      return;
    }

    // Calculate time deltas
    this.lastTime = timestamp;

    this.frameCount++;
    if (timestamp - this.lastFpsTime >= 1000) {
      const fps = Math.round((this.frameCount * 1000) / (timestamp - this.lastFpsTime));
      if (this.fpsElement) this.fpsElement.innerText = `${fps} FPS`;
      this.frameCount = 0;
      this.lastFpsTime = timestamp;
    }

    // Run Simulation Step
    this.updateSim();

    // Render Step
    const commandEncoder = this.device.createCommandEncoder();
    const textureView = this.context.getCurrentTexture().createView();

    const passEncoder = commandEncoder.beginRenderPass({
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }, // Background color
        loadOp: 'clear',
        storeOp: 'store',
      }],
    });

    passEncoder.setPipeline(this.renderPipeline);
    passEncoder.setBindGroup(0, this.renderBindGroup);
    passEncoder.draw(3);
    passEncoder.end();

    this.device.queue.submit([commandEncoder.finish()]);

    if (this.isPlaying) {
      this.animationId = requestAnimationFrame((t) => this.render(t));
    } else {
      this.animationId = 0;
    }
  }

  public async updateShader(newShaderCode: string): Promise<EditorError[]> {
    if (!this.device) return [];

    try {
      this.shaderCode = newShaderCode;
      // We need to re-create grid-dependent bind groups if we recompile, 
      // but the grid textures themselves persist.
      await this.createPipelinesAndGroups(newShaderCode);
      if (!this.isPlaying) {
        this.isPlaying = true;
        this.render(performance.now());
      }
    } catch (e) {
      // Basic error handling for now, real implementation would parse CompilationInfo
      console.warn("Pipeline creation failed:", e);
    }

    // Returning empty array as simplified error handling for this snippet
    return [];
  }
}

// --- Execution ---

const canvas = document.querySelector('#gpu_shader') as HTMLCanvasElement;
const wrapper = document.querySelector('#gpu_wrapper') as HTMLElement;
const fpsLabel = document.querySelector('#gpu_fps-counter') as HTMLElement;
const fullBtn = document.querySelector('#gpu_btn-fullscreen') as HTMLButtonElement;
const resetBtn = document.querySelector('#gpu_btn-reset') as HTMLButtonElement;

const ruleInput = document.querySelector('#rule') as HTMLInputElement;
const sizeInput = document.querySelector('#size') as HTMLInputElement;
const initialInput = document.querySelector('#initial') as HTMLSelectElement;

const savedCode = localStorage.getItem(STORAGE_KEY);
const startCode = savedCode ? savedCode : INITIAL_SHADER_CODE;
const renderer = new WebGpuRenderer(canvas, startCode);
renderer.setFpsElement(fpsLabel);

if (ruleInput) {
  ruleInput.addEventListener('input', () => {
    const val = parseInt(ruleInput.value);
    if (!isNaN(val)) {
      renderer.rule = val;
      renderer.reset();
    }
  });
}

if (sizeInput) {
  sizeInput.addEventListener('change', () => {
    const val = parseInt(sizeInput.value);
    if (!isNaN(val)) {
      renderer.cellSize = val;
      renderer.resizeGrid(); // Size change requires grid resize
    }
  });
}

if (initialInput) {
  initialInput.addEventListener('change', () => {
    const val = parseInt(initialInput.value);
    if (!isNaN(val)) {
      renderer.startRow = val as StartRow;
      renderer.reset();
    }
  });
}

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
      setTimeout(() => renderer.resizeGrid(), 100);
    } else {
      fullBtn.innerText = '⛶';
      fullBtn.title = "Enter Fullscreen";
      setTimeout(() => renderer.resizeGrid(), 100);
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
        renderer.updateShader(INITIAL_SHADER_CODE);
        renderer.reset();
      }
    });
  }

} catch (e) {
  console.error("Failed to load Monaco Editor", e);
}
