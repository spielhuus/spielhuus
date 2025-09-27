export class Vector2 {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
  add(that: Vector2): Vector2 {
    return new Vector2(this.x + that.x, this.y + that.y);
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
  mag() {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }
  limit(max: number): Vector2 {
    if (this.mag() > max) {
      return this.norm().mul_scalar(max);
    }
    return this;
  }
  mul_scalar(that: number): Vector2 {
    return new Vector2(this.x * that, this.y * that);
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
  distance(other: Vector2): number {
    const dx = this.x - other.x;
    const dy = this.y - other.y;
    return Math.sqrt(dx * dx + dy * dy);
  }
  direction(angle: number): Vector2 {
    const deltaX = Math.cos(angle);
    const deltaY = Math.sin(angle);
    return new Vector2(
      this.x + deltaX,
      this.y + deltaY,
    );
  }
  array(): [number, number] {
    return [this.x, this.y];
  }
  public static max(): Vector2 {
    return new Vector2(Number.MAX_VALUE, Number.MAX_VALUE);
  }

}

