import { Vector2 } from '../../js/vector';
export class Player {
  pos: Vector2;
  constructor(x: number, y: number, public dir: Vector2) {
    this.pos = new Vector2(x, y);
  }
}
