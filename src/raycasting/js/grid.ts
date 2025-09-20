import type { DrawOptions } from './utils';
import  { Map } from './maps';

export function drawGrid(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, map: Map, options: DrawOptions) {
  const lineColor = options.lineColor;
  const lineWidth = options.lineWidth;

  const cellWidth = canvas.width / map.width;
  const cellHeight = canvas.height / map.height;

  ctx.strokeStyle = lineColor;
  ctx.lineWidth = lineWidth;

  // Draw vertical lines
  for (let x = cellWidth; x < canvas.width; x += cellWidth) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }

  // Draw horizontal lines
  for (let y = cellHeight; y < canvas.height; y += cellHeight) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
}

