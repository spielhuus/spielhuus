import { noise2D } from '../../js/noise';
import { map } from '../../js/math';

export class BasicNoise {
  private ctx: CanvasRenderingContext2D;
  private scale = 0.02;
  private speedX = 0.3;
  private speedY = 0.1;
  constructor(private canvas: HTMLCanvasElement) {
    this.ctx = canvas.getContext('2d')!;
  }

  draw(time: number) {
    let imageData = this.ctx.createImageData(this.canvas.width, this.canvas.height);
    const data = imageData.data; // Type: Uint8ClampedArray

    const xOffset = time * 0.001 * this.speedX;
    const yOffset = time * 0.001 * this.speedY;
    for (let x = 0; x < this.canvas.width; x++) {
      for (let y = 0; y < this.canvas.height; y++) {
        const index = (y * this.canvas.width + x) * 4;
        const xCoord = x * this.scale + xOffset;
        const yCoord = y * this.scale + yOffset;
        let n = noise2D(xCoord, yCoord);
        // n += 1.0;
        // n /= 2.0;
		    // let bright = Math.round(255*n);
         const bright = Math.round(map(n, -1, 1, 0, 255));
        data[index] = bright;
        data[index + 1] = bright;
        data[index + 2] = bright,
        data[index + 3] = 255;
      }
    }
    this.ctx.putImageData(imageData, 0, 0);
    requestAnimationFrame((dt) => { this.draw(dt) });
  }
}
