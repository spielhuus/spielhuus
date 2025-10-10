// src/WireframeTerrain.ts

import { noise2D } from '../../js/noise';

// Interface for optional configuration parameters
interface TerrainOptions {
  cols?: number;
  rows?: number;
  cellSize?: number;
  noiseScale?: number;
  amplitude?: number;
  color?: string;
  lineWidth?: number;
}

export class WireframeTerrain {
  // Canvas and rendering context
  private ctx: CanvasRenderingContext2D;

  // Terrain generation parameters
  private cols: number;
  private rows: number;
  private cellSize: number;
  private noiseScale: number;
  private amplitude: number;
  private color: string;
  private lineWidth: number;

  // Internal state
  private terrain: number[][] = [];
  private time: number = 0;

  constructor(private canvas: HTMLCanvasElement, options: TerrainOptions = {}) {
    this.ctx = canvas.getContext('2d')!;

    // Set parameters from options or use defaults
    this.cols = options.cols ?? 60;
    this.rows = options.rows ?? 60;
    this.cellSize = options.cellSize ?? 20;
    this.noiseScale = options.noiseScale ?? 7;
    this.amplitude = options.amplitude ?? 120;
    this.color = options.color ?? '#00ff7f';
    this.lineWidth = options.lineWidth ?? 1;

    // Initial setup
    this.handleResize();
    window.addEventListener('resize', this.handleResize.bind(this));
  }

  /**
   * Starts the animation loop.
   */
  public start(): void {
    this.animate();
  }

  /**
   * The main animation loop, called on every frame.
   */
  private animate(): void {
    this.generateTerrain();
    this.draw();
    this.time++;
    requestAnimationFrame(this.animate.bind(this));
  }

  /**
   * (Re)generates the height map using Perlin noise.
   * The time offset creates the animation effect.
   */
  private generateTerrain(): void {
    this.terrain = [];
    const timeOffset = this.time * 0.5; // Controls animation speed

    for (let y = 0; y < this.rows; y++) {
      this.terrain[y] = [];
      for (let x = 0; x < this.cols; x++) {
        const noiseVal = noise2D(
          (x / this.cols) * this.noiseScale + timeOffset,
          (y / this.rows) * this.noiseScale
        );
        this.terrain[y][x] = noiseVal * this.amplitude;
      }
    }
  }

  /**
   * Projects a 3D point (x, y, z) to a 2D screen point using isometric projection.
   */
  private project(x: number, y: number, z: number): { x: number; y: number } {
    const angle = Math.PI / 6; // 30-degree angle for isometric view
    const screenX = (x - y) * Math.cos(angle);
    const screenY = (x + y) * Math.sin(angle) - z;
    return { x: screenX, y: screenY };
  }

  /**
   * Clears the canvas and draws the entire wireframe mesh.
   */
  private draw(): void {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    this.ctx.strokeStyle = this.color;
    this.ctx.lineWidth = this.lineWidth;

    // Center the terrain on the canvas
    const offsetX = this.canvas.width / 2;
    const offsetY = this.canvas.height / 2 - this.rows * this.cellSize * 0.2;

    this.ctx.beginPath();
    for (let y = 0; y < this.rows - 1; y++) {
      for (let x = 0; x < this.cols - 1; x++) {
        // Get 3D coordinates for the current point and its neighbors
        const p1 = { x: x * this.cellSize, y: y * this.cellSize, z: this.terrain[y][x] };
        const p2 = { x: (x + 1) * this.cellSize, y: y * this.cellSize, z: this.terrain[y][x + 1] };
        const p3 = { x: x * this.cellSize, y: (y + 1) * this.cellSize, z: this.terrain[y + 1][x] };

        // Project them into 2D screen space
        const proj1 = this.project(p1.x, p1.y, p1.z);
        const proj2 = this.project(p2.x, p2.y, p2.z);
        const proj3 = this.project(p3.x, p3.y, p3.z);

        // Draw lines connecting the points
        this.ctx.moveTo(proj1.x + offsetX, proj1.y + offsetY);
        this.ctx.lineTo(proj2.x + offsetX, proj2.y + offsetY);

        this.ctx.moveTo(proj1.x + offsetX, proj1.y + offsetY);
        this.ctx.lineTo(proj3.x + offsetX, proj3.y + offsetY);
      }
    }
    this.ctx.stroke();
  }

  /**
   * Handles window resize events to make the canvas responsive.
   */
  private handleResize(): void {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }
}
