export class Vector2 {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
  sub(that: Vector2): Vector2 {
    return new Vector2(this.x - that.x, this.y - that.y);
  }
  div(that: Vector2): Vector2 {
    return new Vector2(this.x / that.x, this.y / that.y);
  }
  mul(that: Vector2): Vector2 {
    return new Vector2(this.x * that.x, this.y * that.y);
  }
  length(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }
  norm(): Vector2 {
    let len = this.length();
    if (len == 0) { return new Vector2(0, 0); }
    return new Vector2(this.x / len, this.y / len);
  }
  scale(f: number): Vector2 {
    return new Vector2(this.x * f, this.y * f);
  }
  mapPos(gridSize: Vector2): Vector2 {
    return new Vector2(Math.floor(this.x / gridSize.x), Math.floor(this.y / gridSize.y))
  }
}

/**
 * Linearly interpolates between two numbers a and b by a given amount t.
 *
 * @param a The start value.
 * @param b The end value.
 * @param t The interpolation amount, typically between 0.0 and 1.0.
 *          If t=0.0, the result is a.
 *          If t=1.0, the result is b.
 *          Values of t outside [0, 1] will extrapolate.
 * @returns The interpolated value.
 */
export function lerp(a: number, b: number, t: number): number {
  return a * (1 - t) + b * t;
}

export interface DrawOptions {
  gridSize: Vector2;
  lineColor: string;
  lineWidth: number;
  line_radius: number;
  circle_radius: number;
  circle_color: string;
}
