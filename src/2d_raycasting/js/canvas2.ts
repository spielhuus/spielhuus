import type { DrawOptions } from './utils';
import { Vector2, lerp } from './utils';

export function drawViewRaycast(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, mouseX: number, mouseY: number, options: DrawOptions) {
for (let x = 0; x < canvas.width; x ++) {
  let cameraX = lerp(-1.0, 1.0, x / canvas.width);
  let rayDirX = dirX + planeX * cameraX;
   let rayDirY = dirY + planeY * cameraX;

}


  // const gridSize = options.gridSize ?? 20;
  // const playerPos = new Vector2(canvas.width / 2, canvas.height / 2);
  // const mousePos = new Vector2(mouseX, mouseY);
  //
  // const rayDir = mousePos.sub(playerPos);
  // if (rayDir.length() === 0) return;
  //
  // const normRayDir = rayDir.norm();
  //
  // let mapPos = playerPos.mapPos(gridSize);
  // console.log(`GridSize: ${gridSize}, MapPos: ${mapPos.x} x ${mapPos.y}`);
  //
  // // length of ray from one x or y-side to next x or y-side
  // const deltaDistX = (normRayDir.x === 0) ? Infinity : Math.abs(gridSize.x / normRayDir.x);
  // const deltaDistY = (normRayDir.y === 0) ? Infinity : Math.abs(gridSize.y / normRayDir.y);
  //
  // let stepX: number;
  // let stepY: number;
  // let sideDistX: number;
  // let sideDistY: number;
  //
  // // calculate step and initial sideDist
  // if (normRayDir.x < 0) {
  //   stepX = -1;
  //   sideDistX = (playerPos.x - mapPos.x * gridSize.x) / gridSize.x * deltaDistX;
  // } else {
  //   stepX = 1;
  //   sideDistX = ((mapPos.x + 1.0) * gridSize.x - playerPos.x) / gridSize.x * deltaDistX;
  // }
  // if (normRayDir.y < 0) {
  //   stepY = -1;
  //   sideDistY = (playerPos.y - mapPos.y * gridSize.y) / gridSize.y * deltaDistY;
  // } else {
  //   stepY = 1;
  //   sideDistY = ((mapPos.y + 1.0) * gridSize.y - playerPos.y) / gridSize.y * deltaDistY;
  // }
  //
  // ctx.strokeStyle = 'red';
  // ctx.lineWidth = options.lineWidth ?? 1;
  // ctx.beginPath();
  // ctx.moveTo(playerPos.x, playerPos.y);
  // ctx.lineTo(mouseX, mouseY);
  // ctx.stroke();
  //
  // ctx.fillStyle = 'red';
  // ctx.beginPath();
  // ctx.arc(playerPos.x, playerPos.y, options.circle_radius ?? 4, 0, Math.PI * 2);
  // ctx.fill();
  //
  // let side = 0;
  // const maxSteps = 50;
  // for (let i = 0; i < maxSteps; i++) {
  //   // jump to next map square, either in x-direction, or in y-direction
  //   if (sideDistX < sideDistY) {
  //     sideDistX += deltaDistX;
  //     mapPos.x += stepX;
  //     side = 0; // hit a vertical line
  //   } else {
  //     sideDistY += deltaDistY;
  //     mapPos.y += stepY;
  //     side = 1; // hit a horizontal line
  //   }
  //
  //   let perpWallDist;
  //   if (side === 0) {
  //     perpWallDist = (sideDistX - deltaDistX);
  //   } else {
  //     perpWallDist = (sideDistY - deltaDistY);
  //   }
  //
  //   const intersectionX = playerPos.x + perpWallDist * normRayDir.x;
  //   const intersectionY = playerPos.y + perpWallDist * normRayDir.y;
  //
  //   ctx.fillStyle = 'orange';
  //   ctx.beginPath();
  //   ctx.arc(intersectionX, intersectionY, options.circle_radius ?? 4, 0, Math.PI * 2);
  //   ctx.fill();
  //
  //   if (intersectionX < 0 || intersectionX > canvas.width || intersectionY < 0 || intersectionY > canvas.height) {
  //     break;
  //   }
  //}
}
