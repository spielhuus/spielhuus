export class Vector2 {
  x: number;
  y: number;

  constructor(x: number = 0, y: number = 0) {
    this.x = x;
    this.y = y;
  }
  public static max(): Vector2 {
    return new Vector2(Number.MAX_VALUE, Number.MAX_VALUE);
  }
  add(that: Vector2): Vector2 {
    this.x += that.x;
    this.y += that.y;
    return this;
  }
  sub(that: Vector2): Vector2 {
    this.x -= that.x;
    this.y -= that.y;
    return this;
  }
  div(that: Vector2): Vector2 {
    this.x /= that.x;
    this.y /= that.y;
    return this;
  }
  divScalar(that: number): Vector2 {
    this.x /= that;
    this.y /= that;
    return this;
  }
  mul(that: Vector2): Vector2 {
    this.x *= that.x;
    this.y *= that.y;
    return this;
  }
  limit(max: number): Vector2 {
    if (this.length() > max) {
      this.norm().mulScalar(max);
    }
    return this;
  }
  mulScalar(that: number): Vector2 {
    this.x *= that;
    this.y *= that;
    return this;
  }
  norm(): Vector2 {
    return this.divScalar( this.length() || 1 );
  }
  length(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }
  // mag(): number {
  //   return Math.sqrt(this.x * this.x + this.y * this.y);
  // }
  distance(other: Vector2): number {
    const dx = this.x - other.x;
    const dy = this.y - other.y;
    return Math.sqrt(dx * dx + dy * dy);
  }
  // direction(angle: number): Vector2 {
  //   const deltaX = Math.cos(angle);
  //   const deltaY = Math.sin(angle);
  //   return new Vector2(
  //     this.x + deltaX,
  //     this.y + deltaY,
  //   );
  // }
  private static degreesToRadians(degrees: number): number {
    return degrees * (Math.PI / 180);
  }
  public rotate(angleInDegrees: number): Vector2 {
    const angleInRadians = Vector2.degreesToRadians(angleInDegrees);

    const cos = Math.cos(angleInRadians);
    const sin = Math.sin(angleInRadians);

    const x = this.x;
    this.x = x * cos - this.y * sin;
    this.y = x * sin + this.y * cos;

    return this;
  }
  clone(): Vector2 {
    return new Vector2(this.x, this.y);
  }
  array(): [number, number] {
    return [this.x, this.y];
  }
}

