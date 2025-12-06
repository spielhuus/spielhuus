

export class Color {
  r: number;
  g: number;
  b: number;
  a: number;

  constructor(r: number = 0, g: number = 0, b: number = 0, a: number = 0) {
    this.r = r;
    this.g = g;
    this.b = b;
    this.a = a;
  }

  /**
  * Adds another color to this color (in-place)
  * @param that - The color to add
  * @returns This color for method chaining
  */
  add(that: Color): Color {
    this.r += that.r;
    this.g += that.g;
    this.b += that.b;
    this.a += that.a;
    return this;
  }

  /**
  * Adds an scalar to this color (in-place)
  * @param that - The scalar to add
  * @returns This color for method chaining
  */
  addScalar(that: number): Color {
    this.r += that;
    this.g += that;
    this.b += that;
    this.a += that;
    return this;
  }

  /**
  * Subtracts another color to this color (in-place)
  * @param that - The color to subtract
  * @returns This color for method chaining
  */
  sub(that: Color): Color {
    this.r -= that.r;
    this.g -= that.g;
    this.b -= that.b;
    this.a -= that.a;
    return this;
  }

  /**
  * Multiplies this color by a scalar value (in-place)
  * @param that - The scalar multiplier
  * @returns This color for method chaining
  */
  mulScalar(that: number): Color {
    this.r *= that;
    this.g *= that;
    this.b *= that;
    this.a *= that;
    return this;
  }

  /** 
   * Creates a new color with identical components 
   * @returns A new Color instance with the same values 
   */
  clone(): Color {
    return new Color(this.r, this.g, this.b, this.a);
  }
}
