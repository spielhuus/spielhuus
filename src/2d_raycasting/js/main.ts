import { Vector2 } from '../../js/vector';

const WALLS_COUNT = 5;
const RAYS_COUNT = 360;
const TRANSPARENT = "#00000000";
const WALL_THICKNESS = 8.0;
const WIDTH = 1280;
const HEIGHT = 860;
const CIRCLE_DISTANCE = 80;
const CIRCLE_RADIUS = 30;

class WebGPURaycaster {
  walls: { start: Vector2, end: Vector2, color: string }[];
  position: Vector2;
  acceleration = new Vector2(0, 0);
  velocity: Vector2;
  theta = 0;
  maxSpeed = 2;
  circleCenter = new Vector2(0, 0);
  target = new Vector2(0, 0);
  black: string;
  orange: string;
  yellow: string;
  darkorange: string;

  private device!: GPUDevice;
  private context!: GPUCanvasContext;
  private presentationFormat!: GPUTextureFormat;
  private computePipeline!: GPUComputePipeline;
  private renderPipeline!: GPURenderPipeline;
  private rayRenderPipeline!: GPURenderPipeline;
  private computeBindGroup!: GPUBindGroup;
  private renderBindGroup!: GPUBindGroup;
  private wallsBuffer!: GPUBuffer;
  private paramsBuffer!: GPUBuffer;
  private raysStorageBuffer!: GPUBuffer;
  private raysVertexBuffer!: GPUBuffer;
  private wallsVertexBuffer!: GPUBuffer;
  private colorUniformBuffer!: GPUBuffer;
  private wallCtx: CanvasRenderingContext2D;
  private moverCtx: CanvasRenderingContext2D;

  constructor(private gpuCanvas: HTMLCanvasElement, private moverCanvas: HTMLCanvasElement, private wallCanvas: HTMLCanvasElement) {
    this.wallCtx = wallCanvas.getContext('2d')!;
    this.moverCtx = moverCanvas.getContext('2d')!;
    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.darkorange = rootStyles.getPropertyValue('--darkorange').trim();
    this.yellow = rootStyles.getPropertyValue('--yellow').trim();
    this.position = new Vector2(WIDTH / 2, HEIGHT / 2);
    this.velocity = new Vector2(Math.random() * 2 - 1, Math.random() * 2 - 1);

    this.walls = [
      // canvas border walls
      { start: new Vector2(1, 1), end: new Vector2(WIDTH - 1, 1), color: TRANSPARENT },
      { start: new Vector2(WIDTH - 1, 1), end: new Vector2(WIDTH - 1, HEIGHT - 1), color: TRANSPARENT },
      { start: new Vector2(WIDTH - 1, HEIGHT - 1), end: new Vector2(1, HEIGHT - 1), color: TRANSPARENT },
      { start: new Vector2(1, HEIGHT - 1), end: new Vector2(1, 1), color: TRANSPARENT },
    ];

    // random inner walls
    for (let i = 0; i < WALLS_COUNT; i++) {
      this.walls.push({
        start: new Vector2(Math.random() * WIDTH, Math.random() * HEIGHT),
        end: new Vector2(Math.random() * WIDTH, Math.random() * HEIGHT),
        color: this.black
      });
    }
  }

  async init() {
    if (!navigator.gpu) {
      throw new Error("WebGPU not supported on this browser.");
    }
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
      throw new Error("No appropriate GPUAdapter found.");
    }
    this.device = await adapter.requestDevice();

    this.context = this.gpuCanvas.getContext('webgpu')!;
    this.presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    this.context.configure({
      device: this.device,
      format: this.presentationFormat,
      alphaMode: 'premultiplied',
    });

    this.createBuffers();
    this.createPipelinesAndBindGroups();
    this.animate();
  }

  private createBuffers() {
    // wall bertex buffer (for rendering)
    const wallVertices = new Float32Array(this.walls.length * 6 * 2); // 6 vertices per wall, 2 floats per vertex
    let offset = 0;

    this.walls.forEach(wall => {
      if (wall.color != TRANSPARENT) {
        const p1 = wall.start;
        const p2 = wall.end;
        const direction = p2.sub(p1).norm();
        const normal = new Vector2(-direction.y, direction.x);
        const thicknessVector = normal.mul_scalar(WALL_THICKNESS / 2.0);

        const v1 = p1.add(thicknessVector); // Top-left
        const v2 = p2.add(thicknessVector); // Top-right
        const v3 = p1.sub(thicknessVector); // Bottom-left
        const v4 = p2.sub(thicknessVector); // Bottom-right

        wallVertices[offset++] = v3.x;
        wallVertices[offset++] = v3.y;
        wallVertices[offset++] = v2.x;
        wallVertices[offset++] = v2.y;
        wallVertices[offset++] = v1.x;
        wallVertices[offset++] = v1.y;

        wallVertices[offset++] = v3.x;
        wallVertices[offset++] = v3.y;
        wallVertices[offset++] = v4.x;
        wallVertices[offset++] = v4.y;
        wallVertices[offset++] = v2.x;
        wallVertices[offset++] = v2.y;
      }
    });

    this.wallsVertexBuffer = this.device.createBuffer({
      size: wallVertices.byteLength,
      usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
      mappedAtCreation: true,
    });

    new Float32Array(this.wallsVertexBuffer.getMappedRange()).set(wallVertices);
    this.wallsVertexBuffer.unmap();

    // wall data buffer
    const wallData = new Float32Array(this.walls.length * 4);
    this.walls.forEach((wall, i) => {
      wallData[i * 4 + 0] = wall.start.x;
      wallData[i * 4 + 1] = wall.start.y;
      wallData[i * 4 + 2] = wall.end.x;
      wallData[i * 4 + 3] = wall.end.y;
    });

    this.wallsBuffer = this.device.createBuffer({
      size: wallData.byteLength,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
      mappedAtCreation: true,
    });
    new Float32Array(this.wallsBuffer.getMappedRange()).set(wallData);
    this.wallsBuffer.unmap();

    // parameters buffer
    this.paramsBuffer = this.device.createBuffer({
      size: 16, // 2 floats for pos (8 bytes) + 1 u32 (4 bytes) + 1 u32 (4 bytes) = 16 bytes
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });

    const raysBufferSize = RAYS_COUNT * 2 * 2 * 4; // RAYS_COUNT lines, 2 vertices/line, 2 floats/vertex, 4 bytes/float

    this.raysStorageBuffer = this.device.createBuffer({
      size: raysBufferSize,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
    });

    this.raysVertexBuffer = this.device.createBuffer({
      size: raysBufferSize,
      usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
    });

    const colorData = this.getColors();
    this.colorUniformBuffer = this.device.createBuffer({
      size: colorData.byteLength,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
      mappedAtCreation: true,
    });
    new Float32Array(this.colorUniformBuffer.getMappedRange()).set(colorData);
    this.colorUniformBuffer.unmap();
  }

  private createPipelinesAndBindGroups() {
    const shaderModule = this.device.createShaderModule({
      code: `
struct Wall {
    p1: vec2<f32>,
    p2: vec2<f32>,
};

struct Params {
    moverPos: vec2<f32>,
    rayCount: u32,
    wallCount: u32,
};

struct ColorUniforms {
   wallColor: vec4<f32>,
   rayColor: vec4<f32>,
};               

@group(0) @binding(0) var<storage, read> walls: array<Wall>;
@group(0) @binding(1) var<uniform> params: Params;
@group(0) @binding(2) var<storage, read_write> rayVertices: array<vec2<f32>>;
@group(0) @binding(3) var<uniform> colorUniforms: ColorUniforms;

const PI = 3.14159265359;

// The main compute shader function
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let ray_index = global_id.x;
    if (ray_index >= params.rayCount) {
        return;
    }

    let ray_angle = (f32(ray_index) / f32(params.rayCount)) * 2.0 * PI;
    let ray_dir = vec2<f32>(cos(ray_angle), sin(ray_angle));

    var closest_t = 100000.0;

    // The intersection logic from your original code
    for (var i: u32 = 0u; i < params.wallCount; i = i + 1u) {
        let wall = walls[i];
        let x1 = wall.p1.x;
        let y1 = wall.p1.y;
        let x2 = wall.p2.x;
        let y2 = wall.p2.y;

        let x3 = params.moverPos.x;
        let y3 = params.moverPos.y;
        let x4 = params.moverPos.x + ray_dir.x;
        let y4 = params.moverPos.y + ray_dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if (den == 0.0) {
            continue;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if (t > 0.0 && t < 1.0 && u > 0.0) {
            if (u < closest_t) {
                closest_t = u;
            }
        }
    }

    // Calculate the intersection point and write it to the vertex buffer
    let intersection_point = params.moverPos + ray_dir * closest_t;

    // Each ray needs two vertices: the start (mover) and the end (intersection)
    rayVertices[ray_index * 2u] = params.moverPos;
    rayVertices[ray_index * 2u + 1u] = intersection_point;
}

@vertex
fn vs_main(@location(0) in_pos: vec2<f32>) -> @builtin(position) vec4<f32> {
    // Convert pixel coordinates to clip space coordinates
    let zero_to_two = in_pos / vec2<f32>(${WIDTH}.0, ${HEIGHT}.0) * 2.0;
    let clip_space = zero_to_two - vec2<f32>(1.0, 1.0);
    // Flip Y-axis because WebGPU's clip space Y is up
    return vec4<f32>(clip_space.x, -clip_space.y, 0.0, 1.0);
}

@fragment
fn fs_walls_main() -> @location(0) vec4<f32> {
    return colorUniforms.wallColor;
}

@fragment
fn fs_rays_main() -> @location(0) vec4<f32> {
    return colorUniforms.rayColor;
}
            `,
    });

    this.computePipeline = this.device.createComputePipeline({
      layout: 'auto',
      compute: { module: shaderModule, entryPoint: 'main' },
    });

    const renderBindGroupLayout = this.device.createBindGroupLayout({
      entries: [
        { binding: 3, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } }
      ]
    });
    const renderPipelineLayout = this.device.createPipelineLayout({
      bindGroupLayouts: [renderBindGroupLayout]
    });
    this.renderPipeline = this.device.createRenderPipeline({
      layout: renderPipelineLayout,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
        buffers: [{
          arrayStride: 2 * 4,
          attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }],
        }]
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_walls_main',
        targets: [{ format: this.presentationFormat }]
      },
      primitive: { topology: 'triangle-list' },
    });

    this.rayRenderPipeline = this.device.createRenderPipeline({
      layout: renderPipelineLayout,
      vertex: {
        module: shaderModule,
        entryPoint: 'vs_main',
        buffers: [{
          arrayStride: 2 * 4,
          attributes: [{ shaderLocation: 0, offset: 0, format: 'float32x2' }],
        }]
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fs_rays_main',
        targets: [{
          format: this.presentationFormat,
          blend: {
            color: {
              operation: 'add',
              srcFactor: 'src-alpha',
              dstFactor: 'one-minus-src-alpha',
            },
            alpha: {
              operation: 'add',
              srcFactor: 'one',
              dstFactor: 'one-minus-src-alpha',
            }
          }
        }]
      },
      primitive: { topology: 'line-list' },
    });

    this.computeBindGroup = this.device.createBindGroup({
      layout: this.computePipeline.getBindGroupLayout(0),
      entries: [
        { binding: 0, resource: { buffer: this.wallsBuffer } },
        { binding: 1, resource: { buffer: this.paramsBuffer } },
        { binding: 2, resource: { buffer: this.raysStorageBuffer } },
      ],
    });

    this.renderBindGroup = this.device.createBindGroup({
      layout: this.renderPipeline.getBindGroupLayout(0),
      entries: [
        { binding: 3, resource: { buffer: this.colorUniformBuffer } },
      ],
    });
  }

  private calculateForce() {

    this.circleCenter = this.velocity.length() > 0
      ? this.velocity.norm().mul_scalar(CIRCLE_DISTANCE)
      : new Vector2(CIRCLE_DISTANCE, 0);
    this.theta += (Math.random() * 2 - 1) * 0.3;
    this.target = new Vector2(
      Math.cos(this.theta) * CIRCLE_RADIUS, Math.sin(this.theta) * CIRCLE_RADIUS
    );
  }

  private move() {
    const wanderForce = this.circleCenter.add(this.target);
    this.acceleration = this.acceleration.add(wanderForce);
    this.velocity = this.velocity.add(this.acceleration);
    this.velocity = this.velocity.limit(this.maxSpeed);
    this.position = this.position.add(this.velocity);
    this.acceleration = this.acceleration.mul_scalar(0);

    if (this.position.x > WIDTH) this.position.x = 0;
    if (this.position.x < 0) this.position.x = WIDTH;
    if (this.position.y > HEIGHT) this.position.y = 0;
    if (this.position.y < 0) this.position.y = HEIGHT;
  }

  private drawMover() {
    this.moverCtx.save();
    const size = 30;
    const width = 20;
    this.moverCtx.clearRect(0, 0, this.moverCanvas.width, this.moverCanvas.height);
    // draw the mover
    this.moverCtx.moveTo(...this.position.array());
    const endPoint = this.position.add(this.circleCenter);
    const dx = endPoint.x - this.position.x;
    const dy = endPoint.y - this.position.y;
    const angle = Math.atan2(dy, dx);
    this.moverCtx.save();
    this.moverCtx.translate(this.position.x, this.position.y);
    this.moverCtx.rotate(angle);
    this.moverCtx.lineWidth = 4;
    this.moverCtx.beginPath();
    this.moverCtx.moveTo(0, 0);
    this.moverCtx.lineTo(-size, width / 2);
    this.moverCtx.lineTo(-size, -width / 2);
    this.moverCtx.closePath();
    this.moverCtx.strokeStyle = this.orange;
    // this.moverCtx.fillStyle = this.orange;
    this.moverCtx.fill();
    this.moverCtx.stroke();
    this.moverCtx.restore();

    this.moverCtx.strokeStyle = this.orange;
    this.moverCtx.lineWidth = 2;
    this.moverCtx.beginPath();
    this.moverCtx.moveTo(...this.position.array());
    this.moverCtx.lineTo(...this.position.add(this.circleCenter).array());
    this.moverCtx.stroke();

    this.moverCtx.strokeStyle = this.darkorange;
    this.moverCtx.lineWidth = 1;
    this.moverCtx.beginPath();
    this.moverCtx.arc(...this.position.add(this.circleCenter).array(), CIRCLE_RADIUS, 0, Math.PI * 2);
    this.moverCtx.stroke();

    this.moverCtx.fillStyle = this.yellow;
    this.moverCtx.strokeStyle = this.yellow;
    this.moverCtx.lineWidth = 1;
    this.moverCtx.beginPath();
    this.moverCtx.arc(...this.position.add(this.circleCenter).add(this.target).array(), 4, 0, Math.PI * 2);
    this.moverCtx.fill();
    this.moverCtx.stroke();
    this.moverCtx.restore();
  }

  drawWalls() {
    this.wallCtx.clearRect(0, 0, this.wallCanvas.width, this.wallCanvas.height);
    for (let line of this.walls) {
      this.wallCtx.strokeStyle = line.color;
      this.wallCtx.lineCap = 'round';
      this.wallCtx.lineJoin = 'round';
      this.wallCtx.lineWidth = WALL_THICKNESS;
      this.wallCtx.beginPath();
      this.wallCtx.moveTo(line.start.x, line.start.y);
      this.wallCtx.lineTo(line.end.x, line.end.y);
      this.wallCtx.stroke();
    }
  }

  private animate = () => {
    //update the colors
    let rootStyles = getComputedStyle(document.documentElement);
    this.black = rootStyles.getPropertyValue('--black').trim();
    this.orange = rootStyles.getPropertyValue('--orange').trim();
    this.darkorange = rootStyles.getPropertyValue('--darkorange').trim();
    this.yellow = rootStyles.getPropertyValue('--yellow').trim();

    this.calculateForce();
    this.drawMover();
    this.move();
    this.drawWalls();

    this.device.queue.writeBuffer(
      this.paramsBuffer, 0,
      new Float32Array([this.position.x, this.position.y])
    );
    this.device.queue.writeBuffer(
      this.paramsBuffer, 8,
      new Uint32Array([RAYS_COUNT, this.walls.length])
    );

    let colors = this.getColors();
    this.device.queue.writeBuffer(
      this.colorUniformBuffer, 0,
      colors.buffer,
      colors.byteOffset,
      colors.byteLength
    );

    const computeCommandEncoder = this.device.createCommandEncoder();
    const computePass = computeCommandEncoder.beginComputePass();
    computePass.setPipeline(this.computePipeline);
    computePass.setBindGroup(0, this.computeBindGroup);
    computePass.dispatchWorkgroups(Math.ceil(RAYS_COUNT / 64));
    computePass.end();
    this.device.queue.submit([computeCommandEncoder.finish()]);

    const renderCommandEncoder = this.device.createCommandEncoder();

    const raysBufferSize = RAYS_COUNT * 2 * 2 * 4;
    renderCommandEncoder.copyBufferToBuffer(
      this.raysStorageBuffer, 0,
      this.raysVertexBuffer, 0,
      raysBufferSize
    );

    const textureView = this.context.getCurrentTexture().createView();
    const renderPass = renderCommandEncoder.beginRenderPass({
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
        loadOp: 'clear',
        storeOp: 'store',
      }],
    });

    // Draw walls
    renderPass.setPipeline(this.renderPipeline);
    renderPass.setBindGroup(0, this.renderBindGroup);
    renderPass.setVertexBuffer(0, this.wallsVertexBuffer);
    renderPass.draw(this.walls.length * 6);

    // Draw rays
    renderPass.setPipeline(this.rayRenderPipeline);
    renderPass.setVertexBuffer(0, this.raysVertexBuffer); // Use the vertex buffer
    renderPass.draw(RAYS_COUNT * 2);

    renderPass.end();
    this.device.queue.submit([renderCommandEncoder.finish()]);

    requestAnimationFrame(this.animate);
  }

  private parseColor(rgbString: string, alpha: number): Float32Array {
    const regex = /rgb\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)/;
    const matches = rgbString.match(regex);

    if (!matches) {
      throw new Error(`Invalid RGB string format: "${rgbString}"`);
    }

    const r = parseFloat(matches[1]) / 255;
    const g = parseFloat(matches[2]) / 255;
    const b = parseFloat(matches[3]) / 255;

    return new Float32Array([r, g, b, alpha]);
  }

  private getColors(): Float32Array {
    const wallColor = this.parseColor(this.darkorange, 1.0);
    const rayColor = this.parseColor(this.black, 0.2);
    const colorData = new Float32Array(8);
    colorData.set(wallColor, 0);
    colorData.set(rayColor, 4);
    return colorData;
  }
}

async function main() {
  const gpuCanvas = document.querySelector<HTMLCanvasElement>('#webgpu-canvas');
  const moverCanvas = document.querySelector<HTMLCanvasElement>('#mover-canvas');
  const wallCanvas = document.querySelector<HTMLCanvasElement>('#wall-canvas');

  if (!gpuCanvas || !moverCanvas || !wallCanvas) {
    throw new Error('Canvas elements not found');
  }

  gpuCanvas.width = moverCanvas.width = wallCanvas.width = WIDTH;
  gpuCanvas.height = moverCanvas.height = wallCanvas.height = HEIGHT;

  const animator = new WebGPURaycaster(gpuCanvas, moverCanvas, wallCanvas);
  await animator.init();
}

main().catch(err => {
  console.error(err);
  alert('An error occurred. Check the console for details.');
});
