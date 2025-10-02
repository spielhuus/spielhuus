import type { DrawOptions } from './utils';
import { Map } from './maps';
import { Vector2 } from '../../js/vector';
import { Player } from './player';

export class Canvas2 {
  map = new Map([
    [0, 0, 1, 0, 0, 0, 0, 2, 2, 0],
    [0, 0, 0, 1, 1, 1, 2, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 1, 1, 1, 0, 0],
  ]);
  player = new Player(7.5, 5.5, new Vector2(1.0, 1.0).norm());

  constructor() { }

  public draw(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, mousePos: Vector2, options: DrawOptions) {

    //prepare the map
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    // drawGrid(canvas, ctx, this.map, options);
    this.map.draw(canvas, ctx, options);

    let cellSize = this.map.cellSize(canvas.width, canvas.height);
    let dir = mousePos.clone().sub(this.player.pos.clone().mul(cellSize));
    if (dir.length() === 0) return;

    // draw the direction arrow
    dir = dir.clone().norm();


    let plane = new Vector2(-dir.y, dir.x).mulScalar(0.66);
    const WIDTH = 800; //example screen width
    for (let x = 0; x < WIDTH; x++) {
      //calculate ray position and direction
      let cameraX = 2 * x / WIDTH - 1; //x-coordinate in camera space
      let rayDir = new Vector2(
        dir.x + plane.x * cameraX,
        dir.y + plane.y * cameraX,
      );
      const pStart = this.player.pos.clone().mul(cellSize);
      const pEnd = this.player.pos.clone().add(rayDir.mulScalar(100)).mul(cellSize);

      let mapPos = new Vector2(Math.floor(this.player.pos.x), Math.floor(this.player.pos.y));

      ctx.strokeStyle = "#aaaaaa11";
      ctx.lineWidth = 4;

      ctx.beginPath();
      ctx.moveTo(pStart.x, pStart.y);
      ctx.lineTo(pEnd.x, pEnd.y);
      ctx.stroke();

      //draw the player
      ctx.fillStyle = 'red';
      ctx.strokeStyle = 'red';
      ctx.beginPath();
      ctx.arc(this.player.pos.x * cellSize.x, this.player.pos.y * cellSize.y, options.circle_radius ?? 4, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      let deltaDist = new Vector2(
        (rayDir.x == 0) ? 1e30 : Math.abs(1 / rayDir.x),
        (rayDir.y == 0) ? 1e30 : Math.abs(1 / rayDir.y),
      );

      let sideDist = new Vector2();
      let step = new Vector2();

      //calculate step and initial sideDist
      if (rayDir.x < 0) {
        step.x = -1;
        sideDist.x = (this.player.pos.x - mapPos.x) * deltaDist.x;
      }
      else {
        step.x = 1;
        sideDist.x = (mapPos.x + 1.0 - this.player.pos.x) * deltaDist.x;
      }
      if (rayDir.y < 0) {
        step.y = -1;
        sideDist.y = (this.player.pos.y - mapPos.y) * deltaDist.y;
      }
      else {
        step.y = 1;
        sideDist.y = (mapPos.y + 1.0 - this.player.pos.y) * deltaDist.y;
      }

      let hit = false;
      let side = 0;

      //perform DDA
      while (!hit) {

        let target;
        if (sideDist.x <= sideDist.y) {
          target = rayDir.clone().mulScalar(sideDist.x);
          sideDist.x += deltaDist.x;
          mapPos.x += step.x;
          side = 0;
        }
        else {
          target = rayDir.clone().mulScalar(sideDist.y);
          sideDist.y += deltaDist.y;
          mapPos.y += step.y;
          side = 1;
        }

        if (mapPos.x < 0 || mapPos.y < 0 || mapPos.x >= this.map.height || mapPos.y >= this.map.height) {
          break;
        }

        //Check if ray has hit a wall
        if (this.map.data[mapPos.y][mapPos.x] > 0) {
          ctx.strokeStyle = 'white';
          ctx.fillStyle = 'white';
          ctx.beginPath();
          ctx.arc(...this.player.pos.clone().add(target).mul(cellSize).array(), 0.5, 0, Math.PI * 2);
          ctx.fill();
          ctx.stroke();

          hit = true;
        }
      }
    }
  }
}
