
export class Interval {
  min: number;
  max: number;
  constructor(min: number = Number.NEGATIVE_INFINITY, max: number = Number.POSITIVE_INFINITY) {
    this.min = min;
    this.max = max;
  }
  size(): number {
    return this.max - this.min;
  }

  contains(x: number): boolean {
    return this.min <= x && x <= this.max;
  }

  surrounds(x: number): boolean {
    return this.min < x && x < this.max;
  }

  clamp(x: number): number {
    if (x < this.min) return this.min;
    if (x > this.max) return this.max;
    return x;
  }

  public static readonly empty = new Interval(Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY);
  public static readonly universe = new Interval(Number.NEGATIVE_INFINITY, Number.POSITIVE_INFINITY);
}
