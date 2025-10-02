import type { DrawOptions } from './utils';
import { Map } from './maps';
import { Vector2 } from '../../js/vector';
import { Player } from './player';

export class Canvas1 {
  map = new Map([
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
    [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
  ]);
  player = new Player(7.5, 5.5, new Vector2(1.0, 1.0).norm());
  constructor() { }

  public draw(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, mousePos: Vector2, options: DrawOptions) {

    //prepare the map
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    this.map.draw(canvas, ctx, options);

    let cellSize = this.map.cellSize(canvas.width, canvas.height);
    let absPlayerPos = this.player.pos.clone().mul(cellSize);
    let mapPos = new Vector2(Math.floor(this.player.pos.x), Math.floor(this.player.pos.y));
    let rayDir = mousePos.sub(absPlayerPos);
    rayDir = rayDir.norm();
    if (rayDir.length() === 0) return;

    // draw the direction arrow

    //draw the ray direction arrow
    const startPoint = absPlayerPos;
    const endPoint = this.player.pos.clone().add(rayDir.mulScalar(0.5)).mul(cellSize);

    ctx.strokeStyle = 'orange'; 
    ctx.fillStyle = 'orange';
    ctx.lineWidth = 4;

    ctx.beginPath();
    ctx.moveTo(startPoint.x, startPoint.y);
    ctx.lineTo(endPoint.x, endPoint.y);
    ctx.stroke();

    const headLength = 15;
    const headAngle = Math.PI / 6;

    const angle1 = Math.atan2(rayDir.y, rayDir.x) + Math.PI - headAngle;
    const angle2 = Math.atan2(rayDir.y, rayDir.x) + Math.PI + headAngle;

    const headPoint1 = new Vector2(
      endPoint.x + headLength * Math.cos(angle1),
      endPoint.y + headLength * Math.sin(angle1)
    );

    const headPoint2 = new Vector2(
      endPoint.x + headLength * Math.cos(angle2),
      endPoint.y + headLength * Math.sin(angle2)
    );

    ctx.beginPath();
    ctx.moveTo(endPoint.x, endPoint.y);
    ctx.lineTo(headPoint1.x, headPoint1.y);
    ctx.lineTo(headPoint2.x, headPoint2.y);
    ctx.closePath();
    ctx.fill();

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

    let sideDist = new Vector2(0, 0);
    let step = new Vector2(0, 0);

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

      if (mapPos.x < 0 || mapPos.y < 0 || mapPos.x >= this.map.width || mapPos.y >= this.map.height) {
        break;
      }

      //Check if ray has hit a wall
      if (this.map.data[mapPos.y][mapPos.x] > 0) {

        ctx.strokeStyle = 'green';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(...absPlayerPos.array());
        ctx.lineTo(...this.player.pos.clone().add(target).mul(cellSize).array());
        ctx.stroke();

        ctx.strokeStyle = 'red';
        ctx.fillStyle = 'red';
        ctx.beginPath();
        ctx.arc(...this.player.pos.clone().add(target).mul(cellSize).array(), options.circle_radius ?? 4, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();

        hit = true;

      } else {
        ctx.strokeStyle = 'lightgrey';
        ctx.fillStyle = 'lightgrey';
        ctx.beginPath();
        ctx.arc(...this.player.pos.clone().add(target).mul(cellSize).array(), options.circle_radius ?? 6, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
      }
    }
  }
}
