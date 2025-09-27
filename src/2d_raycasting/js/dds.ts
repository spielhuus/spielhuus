import type { DrawOptions } from './utils';
import { Vector2 } from './utils';

export function drawGrid(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, options: DrawOptions = {}) {
  // Define default values
  const gridSize = options.gridSize ?? 20;
  const lineColor = options.lineColor ?? '#eeee';
  const lineWidth = options.lineWidth ?? 1;

  // Configure the drawing style
  ctx.strokeStyle = lineColor;
  ctx.lineWidth = lineWidth;

  // Draw vertical lines
  for (let x = gridSize; x < canvas.width; x += gridSize) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }

  // Draw horizontal lines
  for (let y = gridSize; y < canvas.height; y += gridSize) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
}

export function drawRaycast(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, mouseX: number, mouseY: number, options: DrawOptions = {}) {
  const gridSize = options.gridSize ?? 20;
  const playerPos = new Vector2(canvas.width / 2, canvas.height / 2);
  const mousePos = new Vector2(mouseX, mouseY);

  const rayDir = mousePos.sub(playerPos);
  if (rayDir.length() === 0) return;

  const normRayDir = rayDir.norm();

  let mapPos = playerPos.mapPos(gridSize);
  console.log(`GridSize: ${gridSize}, MapPos: ${mapPos.x} x ${mapPos.y}`);

  // length of ray from one x or y-side to next x or y-side
  const deltaDistX = (normRayDir.x === 0) ? Infinity : Math.abs(gridSize / normRayDir.x);
  const deltaDistY = (normRayDir.y === 0) ? Infinity : Math.abs(gridSize / normRayDir.y);

  let stepX: number;
  let stepY: number;
  let sideDistX: number;
  let sideDistY: number;

  // calculate step and initial sideDist
  if (normRayDir.x < 0) {
    stepX = -1;
    sideDistX = (playerPos.x - mapPos.x * gridSize) / gridSize * deltaDistX;
  } else {
    stepX = 1;
    sideDistX = ((mapPos.x + 1.0) * gridSize - playerPos.x) / gridSize * deltaDistX;
  }
  if (normRayDir.y < 0) {
    stepY = -1;
    sideDistY = (playerPos.y - mapPos.y * gridSize) / gridSize * deltaDistY;
  } else {
    stepY = 1;
    sideDistY = ((mapPos.y + 1.0) * gridSize - playerPos.y) / gridSize * deltaDistY;
  }

  ctx.strokeStyle = 'red';
  ctx.lineWidth = options.lineWidth ?? 1;
  ctx.beginPath();
  ctx.moveTo(playerPos.x, playerPos.y);
  ctx.lineTo(mouseX, mouseY);
  ctx.stroke();

  ctx.fillStyle = 'red';
  ctx.beginPath();
  ctx.arc(playerPos.x, playerPos.y, options.circle_radius ?? 4, 0, Math.PI * 2);
  ctx.fill();

  let side = 0;
  const maxSteps = 50;
  for (let i = 0; i < maxSteps; i++) {
    // jump to next map square, either in x-direction, or in y-direction
    if (sideDistX < sideDistY) {
      sideDistX += deltaDistX;
      mapPos.x += stepX;
      side = 0; // hit a vertical line
    } else {
      sideDistY += deltaDistY;
      mapPos.y += stepY;
      side = 1; // hit a horizontal line
    }

    let perpWallDist;
    if (side === 0) {
      perpWallDist = (sideDistX - deltaDistX);
    } else {
      perpWallDist = (sideDistY - deltaDistY);
    }

    const intersectionX = playerPos.x + perpWallDist * normRayDir.x;
    const intersectionY = playerPos.y + perpWallDist * normRayDir.y;

    ctx.fillStyle = 'orange';
    ctx.beginPath();
    ctx.arc(intersectionX, intersectionY, options.circle_radius ?? 4, 0, Math.PI * 2);
    ctx.fill();

    if (intersectionX < 0 || intersectionX > canvas.width || intersectionY < 0 || intersectionY > canvas.height) {
      break;
    }
  }
}
