/**
 * Defines the available rendering modes for the Voronoi diagram.
 */
export const DisplayMode = {
  CELLS: 0,
  GRAYSCALE: 1,
  HEATMAP: 2,
  CONTOURS: 3,
} as const;

const PASTEL_PALETTE_UINT8: number[][] = [
  [0xD0, 0xA0, 0xA5], [0xC8, 0x7E, 0x6F], [0xB0, 0x6B, 0x5C], [0x9B, 0x5B, 0x5F],
  [0xC5, 0x8C, 0x79], [0xDD, 0xAA, 0x55], [0xC7, 0xA2, 0x50], [0xD1, 0x84, 0x60],
  [0xEA, 0xA2, 0x43], [0xC8, 0x8A, 0x3B], [0x7A, 0x8D, 0x7B], [0x6A, 0x70, 0x49],
  [0x83, 0x8A, 0x73], [0x5A, 0x63, 0x49], [0x4A, 0x7C, 0x82], [0x6A, 0x82, 0x9A],
  [0x6C, 0x85, 0xA1], [0x5E, 0x74, 0x85], [0x7E, 0x88, 0xB0], [0x8D, 0x9B, 0xA6],
  [0x9A, 0x8B, 0x9E], [0x7D, 0x5A, 0x7D], [0xA3, 0x86, 0x9C], [0x78, 0x66, 0x84],
  [0x82, 0x6D, 0x8C], [0xA8, 0x98, 0x8A], [0x8C, 0x8C, 0x8C], [0x6E, 0x5A, 0x4F],
  [0x5A, 0x5A, 0x5A], [0xB4, 0xAF, 0xAF],
];

export class GpuVoronoi {
  // Core WebGPU properties
  private readonly canvas: HTMLCanvasElement;
  private adapter!: GPUAdapter;
  private device!: GPUDevice;
  private context!: GPUCanvasContext;
  private presentationFormat!: GPUTextureFormat;

  // JFA resources
  private jfaStep: number;
  private jfaPass: number = 0;
  private jfaCompleted: boolean = false;
  private jfaTextures!: [GPUTexture, GPUTexture];
  private jfaBindGroups!: [GPUBindGroup, GPUBindGroup];
  private jfaPipeline!: GPUComputePipeline;

  // Data buffers
  private seedsBuffer!: GPUBuffer;
  private colorsBuffer!: GPUBuffer;
  private uniformsBuffer!: GPUBuffer;
  
  // Rendering resources
  private renderPipeline!: GPURenderPipeline;
  private renderBindGroup!: GPUBindGroup;

  // Data properties
  private readonly seed_count: number = PASTEL_PALETTE_UINT8.length;
  private seeds: { x: number; y: number }[] = [];
  private currentDisplayMode: number = DisplayMode.CELLS;

  /**
   * Creates an instance of the GpuVoronoi renderer.
   * @param canvas The HTML canvas element to render to.
   */
  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.jfaStep = Math.pow(2, Math.floor(Math.log2(Math.max(canvas.width, canvas.height))));
  }

  /**
   * Initializes WebGPU, creates resources, and prepares for rendering.
   * Must be called before `draw()`.
   * @returns A promise that resolves to `true` on success, or `false` on failure.
   */
  public async init(): Promise<boolean> {
    if (!navigator.gpu) return false;
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) return false;
    this.adapter = adapter;
    this.device = await this.adapter.requestDevice();
    
    const context = this.canvas.getContext('webgpu');
    if (!context) return false;
    this.context = context;

    this.presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    this.context.configure({
      device: this.device,
      format: this.presentationFormat,
      alphaMode: 'opaque',
    });

    this.generateSeeds();
    this.createBuffers();
    this.jfaTextures = [this.createJfaTexture(), this.createJfaTexture()];
    
    this.createJfaPipeline();
    this.createRenderPipeline();
    
    this.jfaBindGroups = [
      this.createJfaBindGroup(this.jfaTextures[0], this.jfaTextures[1]),
      this.createJfaBindGroup(this.jfaTextures[1], this.jfaTextures[0]),
    ];
    
    this.renderBindGroup = this.createRenderBindGroup(this.jfaTextures[0]);

    this.runSeedInitialization();

    return true;
  }
  
  /**
   * Sets the current display mode for the visualization.
   * @param mode A value from the `DisplayMode` enum.
   */
  public setDisplayMode(mode: number): void {
    this.currentDisplayMode = mode;
    if (this.jfaCompleted) {
      requestAnimationFrame(() => this.draw());
    }
  }

  private generateSeeds(): void {
    const seedSet = new Set<string>();
    for (let i = 0; i < this.seed_count; i++) {
      let x: number, y: number, key: string;
      do {
        x = Math.floor(Math.random() * this.canvas.width);
        y = Math.floor(Math.random() * this.canvas.height);
        key = `${x},${y}`;
      } while (seedSet.has(key));
      seedSet.add(key);
      this.seeds.push({ x, y });
    }
  }

  private createBuffers(): void {
    const seedData = new Float32Array(this.seed_count * 2);
    this.seeds.forEach((s, i) => {
        seedData[i * 2] = s.x;
        seedData[i * 2 + 1] = s.y;
    });
    this.seedsBuffer = this.device.createBuffer({
      size: seedData.byteLength,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      mappedAtCreation: true,
    });
    new Float32Array(this.seedsBuffer.getMappedRange()).set(seedData);
    this.seedsBuffer.unmap();
    
    const colorData = new Float32Array(this.seed_count * 4);
    PASTEL_PALETTE_UINT8.forEach((c, i) => {
        colorData[i * 4 + 0] = c[0] / 255.0;
        colorData[i * 4 + 1] = c[1] / 255.0;
        colorData[i * 4 + 2] = c[2] / 255.0;
        colorData[i * 4 + 3] = 1.0;
    });
    this.colorsBuffer = this.device.createBuffer({
        size: colorData.byteLength,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
        mappedAtCreation: true,
    });
    new Float32Array(this.colorsBuffer.getMappedRange()).set(colorData);
    this.colorsBuffer.unmap();

    this.uniformsBuffer = this.device.createBuffer({
        size: 5 * 4, // 5 * 32-bit values (4 floats, 1 uint)
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
  }

  private createJfaTexture(): GPUTexture {
    return this.device.createTexture({
      size: [this.canvas.width, this.canvas.height],
      format: 'rg32float',
      usage:
        GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.STORAGE_BINDING |
        GPUTextureUsage.COPY_DST |
        GPUTextureUsage.COPY_SRC,
    });
  }
  
  private createJfaPipeline(): void {
    const jfaShaderModule = this.device.createShaderModule({
      code: `
        struct Uniforms {
            step: f32,
            width: f32,
            height: f32,
            maxDistance: f32,
            mode: u32,
        };
        struct Seeds { data: array<vec2<f32>>, };

        @group(0) @binding(0) var<uniform> uniforms: Uniforms;
        @group(0) @binding(1) var readTexture: texture_2d<f32>;
        @group(0) @binding(2) var writeTexture: texture_storage_2d<rg32float, write>;
        @group(0) @binding(3) var<storage, read> seeds: Seeds;

        fn distanceSq(p1: vec2<f32>, p2: vec2<f32>) -> f32 {
            let diff = p1 - p2;
            return dot(diff, diff);
        }

        @compute @workgroup_size(8, 8)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            let pixelCoord = vec2<f32>(f32(global_id.x), f32(global_id.y));
            if (pixelCoord.x >= uniforms.width || pixelCoord.y >= uniforms.height) { return; }

            let step = i32(uniforms.step);
            let readCoord = vec2<i32>(global_id.xy);

            var bestPixelData = textureLoad(readTexture, readCoord, 0).xy;
            var bestSeedIndex = i32(bestPixelData.x);
            var minDisSq: f32;

            // If the pixel is empty, initialize with a huge distance.
            if (bestSeedIndex < 0) {
                minDisSq = 1.0e9;
            } else {
                minDisSq = distanceSq(pixelCoord, seeds.data[bestSeedIndex]);
            }

            for (var j = -1; j <= 1; j = j + 1) {
                for (var i = -1; i <= 1; i = i + 1) {
                    let sampleCoord = readCoord + vec2<i32>(i * step, j * step);
                    let clampedCoord = clamp(sampleCoord, vec2<i32>(0), vec2<i32>(i32(uniforms.width) - 1, i32(uniforms.height) - 1));
                    let candidateIndex = i32(textureLoad(readTexture, clampedCoord, 0).x);
                    if (candidateIndex >= 0) {
                        let candidatePos = seeds.data[candidateIndex];
                        let disSq = distanceSq(pixelCoord, candidatePos);
                        if (disSq < minDisSq) {
                            minDisSq = disSq;
                            bestSeedIndex = candidateIndex;
                        }
                    }
                }
            }
            textureStore(writeTexture, readCoord, vec4<f32>(f32(bestSeedIndex), sqrt(minDisSq), 0.0, 0.0));
        }
      `,
    });
    this.jfaPipeline = this.device.createComputePipeline({
      layout: 'auto',
      compute: { module: jfaShaderModule, entryPoint: 'main' },
    });
  }

  private createRenderPipeline(): void {
    const renderShaderModule = this.device.createShaderModule({
      code: `
        struct Uniforms {
            step: f32,
            width: f32,
            height: f32,
            maxDistance: f32,
            mode: u32,
        };
        struct Colors { data: array<vec4<f32>>, };

        @group(0) @binding(0) var jfaTexture: texture_2d<f32>;
        @group(0) @binding(1) var<uniform> uniforms: Uniforms;
        @group(0) @binding(2) var<storage, read> colors: Colors;

        @vertex
        fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
            let pos = array(
                vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
                vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
            );
            return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
        }

        @fragment
        fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
            let pixelCoord = vec2<i32>(frag_coord.xy);
            let jfaData = textureLoad(jfaTexture, pixelCoord, 0).xy;
            let seedIndex = i32(jfaData.x);
            let distance = jfaData.y;
            let line_width = fwidth(distance) * 1.5;

            if (seedIndex < 0) { return vec4<f32>(0.0, 0.0, 0.0, 1.0); }

            if (uniforms.mode == 0u) { // CELLS
                return colors.data[seedIndex];
            }

            let normalizedDist = distance / uniforms.maxDistance;

            if (uniforms.mode == 1u) { // GRAYSCALE
                let gray = 1.0 - normalizedDist * 2.0;
                return vec4<f32>(gray, gray, gray, 1.0);
            }

            if (uniforms.mode == 2u) { // HEATMAP
                let c1 = vec4(0.0, 0.0, 0.0, 1.0);
                let c2 = vec4(0.2, 0.2, 1.0, 1.0);
                let c3 = vec4(1.0, 1.0, 0.0, 1.0);
                let mix1 = mix(c1, c2, smoothstep(0.0, 0.25, normalizedDist));
                return mix(mix1, c3, smoothstep(0.25, 0.7, normalizedDist));
            }

            if (uniforms.mode == 3u) { // CONTOURS
                let line_color = vec4(1.0, 1.0, 1.0, 1.0);
                let bg_color = colors.data[seedIndex] * 0.5;
                let spacing = 25.0;
                let pattern = abs(fract(distance / spacing - 0.5) * 2.0 - 1.0);
                let line = smoothstep(1.0 - line_width, 1.0, pattern);
                return mix(bg_color, line_color, line);
            }

            return vec4<f32>(1.0, 0.0, 1.0, 1.0); // Error color
        }
      `,
    });

    this.renderPipeline = this.device.createRenderPipeline({
        layout: 'auto',
        vertex: { module: renderShaderModule, entryPoint: 'vs_main' },
        fragment: {
            module: renderShaderModule,
            entryPoint: 'fs_main',
            targets: [{ format: this.presentationFormat }],
        },
        primitive: { topology: 'triangle-list' },
    });
  }

  private createJfaBindGroup(readTexture: GPUTexture, writeTexture: GPUTexture): GPUBindGroup {
    return this.device.createBindGroup({
      layout: this.jfaPipeline.getBindGroupLayout(0),
      entries: [
        { binding: 0, resource: { buffer: this.uniformsBuffer } },
        { binding: 1, resource: readTexture.createView() },
        { binding: 2, resource: writeTexture.createView() },
        { binding: 3, resource: { buffer: this.seedsBuffer } },
      ],
    });
  }
  
  private createRenderBindGroup(finalJfaTexture: GPUTexture): GPUBindGroup {
    return this.device.createBindGroup({
        layout: this.renderPipeline.getBindGroupLayout(0),
        entries: [
            { binding: 0, resource: finalJfaTexture.createView() },
            { binding: 1, resource: { buffer: this.uniformsBuffer } },
            { binding: 2, resource: { buffer: this.colorsBuffer } },
        ],
    });
  }

  private runSeedInitialization(): void {
    const initialData = new Float32Array(this.canvas.width * this.canvas.height * 2);
    for (let i = 0; i < initialData.length; i += 2) {
        initialData[i] = -1.0;
        initialData[i + 1] = 1.0e9;
    }

    this.seeds.forEach((seed, i) => {
      const index = (Math.floor(seed.y) * this.canvas.width + Math.floor(seed.x)) * 2;
      initialData[index] = i;
      initialData[index + 1] = 0.0;
    });

    this.device.queue.writeTexture(
      { texture: this.jfaTextures[0] },
      initialData,
      { bytesPerRow: this.canvas.width * 2 * 4 },
      { width: this.canvas.width, height: this.canvas.height }
    );
  }

  /**
   * Executes one pass of the JFA algorithm (if incomplete) and renders the result to the canvas.
   * This method should be called in a loop (e.g., `requestAnimationFrame`) to see the JFA process animate.
   */
  public draw(): void {
    const maxDistance = Math.sqrt(this.canvas.width**2 + this.canvas.height**2);
    const uniformData = new Float32Array(5);
    uniformData[0] = this.jfaStep;
    uniformData[1] = this.canvas.width;
    uniformData[2] = this.canvas.height;
    uniformData[3] = maxDistance;
    const uniformDataU32 = new Uint32Array(uniformData.buffer);
    uniformDataU32[4] = this.currentDisplayMode;
    
    this.device.queue.writeBuffer(this.uniformsBuffer, 0, uniformData.buffer);

    const commandEncoder = this.device.createCommandEncoder();
    
    if (!this.jfaCompleted) {
      const readIndex = this.jfaPass % 2;
      const passEncoder = commandEncoder.beginComputePass();
      passEncoder.setPipeline(this.jfaPipeline);
      passEncoder.setBindGroup(0, this.jfaBindGroups[readIndex]);
      passEncoder.dispatchWorkgroups(
        Math.ceil(this.canvas.width / 8),
        Math.ceil(this.canvas.height / 8)
      );
      passEncoder.end();

      this.jfaStep = Math.floor(this.jfaStep / 2);
      this.jfaPass++;

      if (this.jfaStep < 1) {
        this.jfaCompleted = true;
        const finalTextureIndex = this.jfaPass % 2;
        if (finalTextureIndex === 1) {
            commandEncoder.copyTextureToTexture(
                { texture: this.jfaTextures[1] },
                { texture: this.jfaTextures[0] },
                [this.canvas.width, this.canvas.height]
            );
        }
      }
    } 
    
    const passEncoder = commandEncoder.beginRenderPass({
      colorAttachments: [{
        view: this.context.getCurrentTexture().createView(),
        loadOp: 'clear',
        clearValue: { r: 0.1, g: 0.1, b: 0.15, a: 1.0 },
        storeOp: 'store',
      }],
    });
    
    passEncoder.setPipeline(this.renderPipeline);
    passEncoder.setBindGroup(0, this.renderBindGroup);
    passEncoder.draw(6);
    passEncoder.end();
    
    this.device.queue.submit([commandEncoder.finish()]);

    if (!this.jfaCompleted) {
      requestAnimationFrame(() => this.draw());
    } else if (this.jfaPass > 0) {
        console.log("GPU JFA Done.");
        this.jfaPass = -1; // Prevent this log from firing again
    }
  }
}
