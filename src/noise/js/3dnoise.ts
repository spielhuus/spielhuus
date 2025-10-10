import { noise3D } from '../../js/noise'; // Assuming you have noise3D
import { map } from '../../js/math';

export class DDDNoise {
  private ctx: CanvasRenderingContext2D;
  private scale = 0.02; // Controls the "zoom" of the noise. Smaller is more zoomed out.
  private speed = 0.001; // Controls how fast the animation plays.

  constructor(private canvas: HTMLCanvasElement) {
    this.ctx = canvas.getContext('2d')!;
  }

  draw(time: number) {
    // We use the total elapsed time 'time' for smooth animation, not dt.
    const imageData = this.ctx.createImageData(this.canvas.width, this.canvas.height);
    const data = imageData.data;

    const timeOffset = time * this.speed; // This is our 'z' dimension, which moves forward smoothly.

    for (let x = 0; x < this.canvas.width; x++) {
      for (let y = 0; y < this.canvas.height; y++) {
        // Get a noise value for the (x, y) position and the current time (z).
        // We multiply by 'scale' to control the level of detail.
        let n = noise3D(x * this.scale, y * this.scale, timeOffset);

        // Map the noise value from its typical range [-1, 1] to [0, 255] for color.
        const bright = Math.floor((n + 1) * 0.5 * 255);

        const index = (y * this.canvas.width + x) * 4;
        data[index] = bright;       // R
        data[index + 1] = bright;   // G
        data[index + 2] = bright;   // B
        data[index + 3] = 255;      // A
      }
    }
    this.ctx.putImageData(imageData, 0, 0);

    // Continue the animation loop
    requestAnimationFrame((t) => this.draw(t));
  }

  // A helper to start the animation
  start() {
    requestAnimationFrame((t) => this.draw(t));
  }
}
