/** 
 * A 2D vector class for mathematical operations in Cartesian space. 
 * Provides common vector operations including arithmetic, normalization, 
 * rotation, and distance calculations. All operations modify the vector 
 * in-place unless otherwise specified. 
 */
export class Vector2 {
  x: number;
  y: number;

  /** 
   * Creates a new Vector2 instance 
   * @param x - The x-component (default: 0) 
   * @param y - The y-component (default: 0) 
   */
  constructor(x: number = 0, y: number = 0) {
    this.x = x;
    this.y = y;
  }

  /** 
   * Returns a vector representing maximum possible values (Infinity, Infinity) 
   * Useful for comparison operations or boundary checks 
   */
  public static max(): Vector2 {
    return new Vector2(Infinity, Infinity);
  }

  /** 
   * Copies values from another vector into this instance 
   * @param other - The source vector to copy from 
   * @returns This vector for method chaining 
   */
  copyFrom(other: Vector2): this {
    this.x = other.x;
    this.y = other.y;
    return this;
  }

  /** 
   * Sets the vector components directly 
   * @param x - New x-component 
   * @param y - New y-component 
   * @returns This vector for method chaining 
   */
  set(x: number, y: number) {
    this.x = x;
    this.y = y;
    return this;
  }

  /** 
   * Adds another vector to this vector (in-place) 
   * @param that - The vector to add 
   * @returns This vector for method chaining 
   */
  add(that: Vector2): Vector2 {
    this.x += that.x;
    this.y += that.y;
    return this;
  }

  /** 
   * Subtracts another vector from this vector (in-place) 
   * @param that - The vector to subtract 
   * @returns This vector for method chaining 
   */
  sub(that: Vector2): Vector2 {
    this.x -= that.x;
    this.y -= that.y;
    return this;
  }

  /** 
   * Divides this vector by another vector component-wise (in-place) 
   * @param that - The divisor vector 
   * @returns This vector for method chaining 
   */
  div(that: Vector2): Vector2 {
    this.x /= that.x;
    this.y /= that.y;
    return this;
  }

  /** 
   * Divides this vector by a scalar value (in-place) 
   * @param that - The scalar divisor 
   * @returns This vector for method chaining 
   */
  divScalar(that: number): Vector2 {
    this.x /= that;
    this.y /= that;
    return this;
  }

  /** 
   * Multiplies this vector by another vector component-wise (in-place) 
   * @param that - The multiplier vector 
   * @returns This vector for method chaining 
   */
  mul(that: Vector2): Vector2 {
    this.x *= that.x;
    this.y *= that.y;
    return this;
  }

  /** 
   * Multiplies this vector by a scalar value (in-place) 
   * @param that - The scalar multiplier 
   * @returns This vector for method chaining 
   */
  mulScalar(that: number): Vector2 {
    this.x *= that;
    this.y *= that;
    return this;
  }

  /**  
   * Computes the dot product with another vector 
   * @param that - The other vector to compute the dot product with 
   * @returns The dot product (scalar result) 
   */
  dot(that: Vector2): number {
    return this.x * that.x + this.y * that.y;
  }

  /** 
   * Limits the vector's magnitude to a maximum value (in-place) 
   * If the current magnitude exceeds the limit, scales the vector down 
   * @param max - The maximum allowed magnitude 
   * @returns This vector for method chaining 
   */
  limit(max: number): Vector2 {
    const maxSq = max * max;
    const lengthSq = this.x * this.x + this.y * this.y;

    if (lengthSq > maxSq && lengthSq > 0) {
      return this.divScalar(Math.sqrt(lengthSq)).mulScalar(max);
    }
    return this;
  }

  /** 
   * Normalizes the vector to unit length (in-place) 
   * If the vector has zero length, returns a vector with components (0,0) 
   * @returns This vector for method chaining 
   */
  norm(): Vector2 {
    return this.divScalar(this.length() || 1);
  }

  /** 
   * Calculates the magnitude (length) of the vector 
   * @returns The Euclidean length of the vector 
   */
  length(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }

  /** 
   * Calculates the Euclidean distance to another vector 
   * @param other - The target vector 
   * @returns The distance between this vector and the target 
   */
  distance(other: Vector2): number {
    const dx = this.x - other.x;
    const dy = this.y - other.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  /** 
   * Calculates the squared Euclidean distance to another vector 
   * More efficient than distance() when comparing distances 
   * @param v - The target vector 
   * @returns The squared distance between this vector and the target 
   */
  distanceToSquared(v: Vector2) {
    const dx = this.x - v.x, dy = this.y - v.y;
    return dx * dx + dy * dy;
  }

  /** 
   * Converts degrees to radians (internal utility) 
   * @param degrees - Angle in degrees 
   * @returns Angle in radians 
   */
  private static degreesToRadians(degrees: number): number {
    return degrees * (Math.PI / 180);
  }

  /** 
   * Rotates the vector around the origin (in-place) 
   * @param angleInDegrees - Rotation angle in degrees 
   * @returns This vector for method chaining 
   */
  public rotate(angleInDegrees: number): Vector2 {
    const angleInRadians = Vector2.degreesToRadians(angleInDegrees);

    const cos = Math.cos(angleInRadians);
    const sin = Math.sin(angleInRadians);

    const x = this.x;
    this.x = x * cos - this.y * sin;
    this.y = x * sin + this.y * cos;

    return this;
  }

  /** 
   * Creates a new vector with identical components 
   * @returns A new Vector2 instance with the same values 
   */
  clone(): Vector2 {
    return new Vector2(this.x, this.y);
  }

  /** 
   * Returns the vector components as a tuple 
   * @returns [x, y] tuple representation 
   */
  array(): [number, number] {
    return [this.x, this.y];
  }
}

// 3d Vector
export class Vector3 {
  x: number;
  y: number;
  z: number;

  /** 
   * Creates a new Vector2 instance 
   * @param x - The x-component (default: 0) 
   * @param y - The y-component (default: 0) 
   * @param z - The z-component (default: 0) 
   */
  constructor(x: number = 0, y: number = 0, z: number = 0) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  /**
  * Adds another vector to this vector (in-place)
  * @param that - The vector to add
  * @returns This vector for method chaining
  */
  add(that: Vector3): Vector3 {
    this.x += that.x;
    this.y += that.y;
    this.z += that.z;
    return this;
  }

  /**
  * Subtracts another vector to this vector (in-place)
  * @param that - The vector to subtract
  * @returns This vector for method chaining
  */
  sub(that: Vector3): Vector3 {
    this.x -= that.x;
    this.y -= that.y;
    this.z -= that.z;
    return this;
  }

  /**
  * Multiplies this vector by a scalar value (in-place)
  * @param that - The scalar multiplier
  * @returns This vector for method chaining
  */
  mulScalar(that: number): Vector3 {
    this.x *= that;
    this.y *= that;
    this.z *= that;
    return this;
  }

  /** 
   * Divides this vector by a scalar (in-place) 
   * @param that - The scalar to divide by 
   * @returns This vector for method chaining 
   */
  divScalar(that: number): Vector3 {
    this.x /= that;
    this.y /= that;
    this.z /= that;
    return this;
  }

  /** 
   * Calculates the magnitude (length) of this vector. 
   * Also known as the Euclidean norm. 
   * @returns The length of the vector. 
   */
  length(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
  }

  /** 
   * Calculates the magnitude (length) of this vector. 
   * Also known as the Euclidean norm. 
   * @returns The squared length of the vector. 
   */
  lengthSquared(): number {
    return this.x * this.x + this.y * this.y + this.z * this.z;
  }

  /** 
   * Computes the dot product of this vector and another vector 
   * @param that - The vector to compute the dot product with 
   * @returns The scalar result of the dot product 
   */
  dot(that: Vector3): number {
    return this.x * that.x + this.y * that.y + this.z * that.z;
  }

  /** 
   * Normalizes this vector (in-place), making its length equal to 1. 
   * If the vector's length is zero, it remains a zero vector. 
   * @returns This vector for method chaining. 
   */
  norm(): Vector3 {
    const len = this.length();

    // Guard against division by zero 
    if (len > 0) {
      this.divScalar(len);
    }

    return this;
  }

  /** 
   * Creates a new vector with identical components 
   * @returns A new Vector3 instance with the same values 
   */
  clone(): Vector3 {
    return new Vector3(this.x, this.y, this.z);
  }

  static random(): Vector3 {
    return new Vector3(Math.random(), Math.random(), Math.random());
  }
  static randomRange(min: number, max: number): Vector3 {
    return new Vector3(
      min + Math.random() * (max - min),
      min + Math.random() * (max - min),
      min + Math.random() * (max - min)
    );
  }
  static random_unit_vector(): Vector3 {
    while (true) {
      let p = Vector3.randomRange(-1, 1);
      let lensq = p.lengthSquared();
      if (1e-160 < lensq && lensq <= 1)
        return p.divScalar(Math.sqrt(lensq));
    }
  }
}
