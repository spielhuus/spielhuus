import { Vector2 } from '../../js/vector';
import type { DrawOptions } from './utils';

export class Map {
  width: number;
  height: number;
  data: Array<Array<number>>;
  constructor(data: Array<Array<number>>) {
    this.height = data.length;
    this.width = 0;
    for (let row of data) {
      this.width = Math.max(this.width, row.length);
    }
    this.data = data;
  }
  cellSize(width: number, height: number): Vector2 {
    return new Vector2(width / this.width, height / this.height);
  }
  draw(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, options: DrawOptions) {

    const cellWidth = canvas.width / this.width;
    const cellHeight = canvas.height / this.height;

    ctx.strokeStyle = options.lineColor;
    ctx.lineWidth = options.lineWidth;

    // Draw vertical lines
    for (let x = 0; x < this.width + 1; x++) {
      ctx.beginPath();
      ctx.moveTo(x * cellWidth, 0);
      ctx.lineTo(x * cellWidth, canvas.height);
      ctx.stroke();
    }

    // Draw horizontal lines
    for (let y = 0; y < this.height + 1; y++) {
      ctx.beginPath();
      ctx.moveTo(0, y * cellHeight);
      ctx.lineTo(canvas.width, y * cellHeight);
      ctx.stroke();
    }

    //draw the walls
    let cellSize = this.cellSize(canvas.width, canvas.height);
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        if (this.data[y][x] > 0) {
          ctx.strokeStyle = options.wallColor;
          ctx.fillStyle = options.wallColor;
          ctx.beginPath();
          ctx.rect(x * cellSize.x, y * cellSize.y, cellSize.x, cellSize.y);
          ctx.fill();
        }
      }
    }
  }
  isCell(pos: Vector2): boolean {
    return this.data[pos.y][pos.x] > 0;
  }
}

export function createLevel(): Map {
  return new Map(
    [
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
      [0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
      [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
      [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
    ]
  );
}
