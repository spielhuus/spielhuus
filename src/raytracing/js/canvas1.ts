import { Vector3 } from '../../js/vector.ts';
import type { DrawOptions } from './utils';
// import { Map } from './maps';
import { Vector2 } from '../../js/vector';
import { Color } from '../../js/color.ts';
import { HitableList } from './hitable_list.ts';
import type { IHitable } from './hitable.ts';
import { HitRecord } from './hitable.ts';
import { Sphere } from './sphere.ts';
import { Interval } from './interval.ts';
import { Camera } from './camera.ts';
// import { Player } from './player';

const SCREEN_WIDTH = 640;
const SCREEN_HEIGHT = 480;

export class Ray {
  origin: Vector3;
  direction: Vector3;

  constructor(origin: Vector3, direction: Vector3) {
    this.origin = origin;
    this.direction = direction;
  }

  public at(t: number): Vector3 {
    return this.origin.clone().add(this.direction.clone().mulScalar(t))
  }
}

export class Canvas1 {
  private renderingStarted = false;
  private rendered = false;
  private renderProgress = 0;
  private isRendering = false;
  private finalImageData: ImageData | null = null;
  private camera: Camera;

  imageData: ImageData;
  keyPressed: string | undefined;
  lastTime = 0;
  maxVisibleDistance = 10.0;
  minBrightness = 0.1;
  isActive = false;
  private renderCanvas: HTMLCanvasElement;
  private renderCtx: CanvasRenderingContext2D;
  private rendered = false;
  private world: HitableList;
  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D, private options: DrawOptions) {
    // this.renderCanvas = document.createElement('canvas');
    // this.renderCanvas.width = SCREEN_WIDTH;
    // this.renderCanvas.height = SCREEN_HEIGHT;
    // this.renderCtx = this.renderCanvas.getContext('2d')!;
    // canvas.addEventListener('keydown', (event: KeyboardEvent) => {
    //   this.keyPressed = event.code;
    // });
    //
    // canvas.addEventListener('keyup', () => {
    //   this.keyPressed = undefined;
    // });
    //
    // this.imageData = this.ctx.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);

    //create the world
    this.world = new HitableList();
    this.world.add(new Sphere(new Vector3(0, 0.1, -1), 0.5));
    this.world.add(new Sphere(new Vector3(0, -100.5, -1), 100));

    // Create and initialize the camera
    this.camera = new Camera();
    this.camera.samples_per_pixel = 50; // Lower for faster testing
    this.camera.max_depth = 20;
    this.camera.initialize(this.canvas.width, this.canvas.height);
    // Set up focus listeners to control the active state
    canvas.addEventListener('focus', () => this.isActive = true);
    canvas.addEventListener('blur', () => this.isActive = false);

    // Start the main animation loop
    requestAnimationFrame((time) => this.draw(time));
  }

  private drawProgressBar() {
    const width = this.canvas.width;
    const height = this.canvas.height;
    const barWidth = width * 0.8;
    const barHeight = 40;
    const barX = (width - barWidth) / 2;
    const barY = (height - barHeight) / 2;

    // Black background
    this.ctx.fillStyle = '#222';
    this.ctx.fillRect(0, 0, width, height);

    // Progress bar border
    this.ctx.strokeStyle = 'white';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(barX, barY, barWidth, barHeight);

    // Progress bar fill
    this.ctx.fillStyle = '#6495ED'; // Cornflower Blue
    this.ctx.fillRect(barX, barY, barWidth * this.renderProgress, barHeight);

    // Progress percentage text
    const percent = (this.renderProgress * 100).toFixed(0);
    const text = `Rendering... ${percent}%`;
    this.ctx.fillStyle = 'white';
    this.ctx.font = '20px sans-serif';
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';
    this.ctx.fillText(text, width / 2, barY + barHeight / 2);
  }

  private startRendering() {
    if (this.isRendering || this.rendered) return;
    console.log("Starting render...");
    this.isRendering = true;
    this.rendered = false;
    this.renderProgress = 0;
    this.camera.startRender(this.ctx, this.world);
  }

  public draw(currentTime: number) {
    this.lastTime = this.lastTime || currentTime;
    const deltaTime = (currentTime - this.lastTime) / 1000;
    this.lastTime = currentTime;

    // Start rendering automatically if it hasn't started yet.
    if (!this.isRendering && !this.rendered) {
      this.startRendering();
    }

    // If we are actively rendering, process the next chunk.
    if (this.isRendering) {
      const result = this.camera.renderChunk();
      this.renderProgress = result.progress;
      this.finalImageData = result.imageData;

      // Draw the partially rendered image
      this.ctx.putImageData(this.finalImageData, 0, 0);

      // Draw the progress bar on top
      this.drawProgressBar();

      // Check if rendering is complete
      if (this.renderProgress >= 1) {
        console.log("Render finished.");
        this.isRendering = false;
        this.rendered = true;
      }
    }

    // If rendering is finished, just show the final image.
    if (this.rendered && this.finalImageData) {
      this.ctx.putImageData(this.finalImageData, 0, 0);
    }

    // Display FPS
    this.ctx.font = this.options.title_text_font;
    this.ctx.fillStyle = this.options.title_text_style;
    this.ctx.textBaseline = 'top';
    this.ctx.fillText(`${Math.round(1 / deltaTime)} FPS`, 10, 10);

    // Keep the animation loop going
    requestAnimationFrame((time) => this.draw(time));
  }


  // public draw(currentTime: number) {
  //   console.log("draw canvas1");
  //
  //   if (this.lastTime === 0) {
  //     this.lastTime = currentTime / 1000;
  //   }
  //
  //   const deltaTime = currentTime / 1000 - this.lastTime;
  //
  //   this.ctx.font = this.options.title_text_font;
  //   this.ctx.fillStyle = this.options.title_text_style;
  //   this.ctx.textBaseline = 'top';
  //   this.ctx.fillText(`${Math.round(1 / (currentTime / 1000 - this.lastTime))} FPS`, 10, 10);
  //   this.lastTime = currentTime / 1000;
  //
  //   if (!this.isActive) {
  //     // this.ctx.fillStyle = this.options.pause_color;
  //     // this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
  //     // this.ctx.fillText("click to start", this.canvas.width / 2 - 20, this.canvas.height / 2);
  //     if (!this.rendered) {
  //       let cam = new Camera();
  //       cam.render(this.canvas, this.ctx, this.world);
  //       this.rendered = true;
  //     }
  //   } else {
  //     requestAnimationFrame((dt) => { this.draw(dt) });
  //   }
  // }
}
