import type { DrawOptions } from './utils';
import { Map } from './maps';
import { Vector2 } from './utils';


export function drawRaycast(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, mouseX: number, mouseY: number, map: Map, options: DrawOptions) {
  const gridSize = options.gridSize;
  const playerPos = new Vector2(canvas.width / 2, canvas.height / 2);
  const mousePos = new Vector2(mouseX, mouseY);

  ctx.strokeStyle = 'red';
  ctx.lineWidth = options.lineWidth;
  ctx.beginPath();
  ctx.moveTo(playerPos.x, playerPos.y);
  ctx.lineTo(mouseX, mouseY);
  ctx.stroke();

  ctx.fillStyle = 'red';
  ctx.beginPath();
  ctx.arc(playerPos.x, playerPos.y, options.circle_radius ?? 4, 0, Math.PI * 2);
  ctx.fill();

  const rayDir = mousePos.sub(playerPos);
  if (rayDir.length() === 0) return;

  const normRayDir = rayDir.norm();

  let mapPos = playerPos.mapPos(gridSize);
  // length of ray from one x or y-side to next x or y-side
  const deltaDistX = (normRayDir.x === 0) ? Infinity : Math.abs(gridSize.x / normRayDir.x);
  const deltaDistY = (normRayDir.y === 0) ? Infinity : Math.abs(gridSize.y / normRayDir.y);

  let stepX: number;
  let stepY: number;
  let sideDistX: number;
  let sideDistY: number;

  // calculate step and initial sideDist
  if (normRayDir.x < 0) {
    stepX = -1;
    sideDistX = (playerPos.x - mapPos.x * gridSize.x) / gridSize.x * deltaDistX;
  } else {
    stepX = 1;
    sideDistX = ((mapPos.x + 1.0) * gridSize.x - playerPos.x) / gridSize.x * deltaDistX;
  }
  if (normRayDir.y < 0) {
    stepY = -1;
    sideDistY = (playerPos.y - mapPos.y * gridSize.y) / gridSize.y * deltaDistY;
  } else {
    stepY = 1;
    sideDistY = ((mapPos.y + 1.0) * gridSize.y - playerPos.y) / gridSize.y * deltaDistY;
  }

  let oldPos = playerPos
  let side = 0;
  const maxSteps = 50;
  for (let i = 0; i < maxSteps; i++) {
    if (sideDistX < sideDistY) {
      sideDistX += deltaDistX;
      mapPos.x += stepX;
      side = 0; // hit a vertical line

      ctx.stroke();

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

    oldPos = new Vector2(intersectionX, intersectionY);

    ctx.strokeStyle = 'orange';
    ctx.fillStyle = 'orange';
    ctx.beginPath();
    ctx.arc(intersectionX, intersectionY, options.circle_radius ?? 4, 0, Math.PI * 2);
    ctx.fill();


    let cellPos = new Vector2(intersectionX, intersectionY).mapPos(gridSize);
    if (map.isCell(cellPos)) {
          ctx.strokeStyle = 'orange';
          ctx.fillStyle = 'orange';
          ctx.beginPath();
          let cellSize = map.cellSize(canvas.width, canvas.height);
          ctx.rect(cellPos.x*cellSize.x, cellPos.y*cellSize.y, cellSize.x, cellSize.y);
          ctx.fill();
          break;
    }



    if (intersectionX < 0 || intersectionX > canvas.width || intersectionY < 0 || intersectionY > canvas.height) {
      break;
    }
  }
}
