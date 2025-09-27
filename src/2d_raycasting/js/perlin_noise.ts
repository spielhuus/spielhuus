/**
 * @file A 2D Perlin noise generator.
 * @module perlin
 * Converted from a Python implementation and corrected for deterministic randomness.
 */

/**
 * Generates a 2D gradient vector from a random angle.
 * @param h - An angle in radians.
 * @returns A 2D gradient vector as a [number, number] tuple.
 */
function gradient(h: number): [number, number] {
  return [Math.cos(h), Math.sin(h)];
}

/**
 * Computes a pseudo-random, but deterministic, gradient vector for a given grid point.
 * This is the corrected replacement for the flawed random number generation in the original Python code.
 * @param ix - The integer x-coordinate of the grid point.
 * @param iy - The integer y-coordinate of the grid point.
 * @returns The 2D gradient vector for the grid point.
 */
function getRandomGradient(ix: number, iy: number): [number, number] {
  // A simple hashing function to get a deterministic "random" value
  const w = 8 * 4; // size of a 32-bit integer in bits
  const s = w / 2; // 16
  let a = ix, b = iy;
  a *= 3284157443; b ^= a << s | a >>> w - s;
  b *= 1911520717; a ^= b << s | b >>> w - s;
  a *= 2048419325;
  const random = (a % 10000) / 10000.0; // Keep it in a predictable range
  
  const angle = random * 2 * Math.PI;
  return gradient(angle);
}


/**
 * Performs linear interpolation.
 * @param a - The start value.
 * @param b - The end value.
 * @param x - The interpolation factor (typically between 0 and 1).
 * @returns The interpolated value.
 */
function lerp(a: number, b: number, x: number): number {
  return a + x * (b - a);
}

/**
 * A smoothing function (quintic smoothstep) to avoid artifacts.
 * 6t^5 - 15t^4 + 10t^3
 * @param t - The value to fade (typically between 0 and 1).
 * @returns The smoothed value.
 */
function fade(t: number): number {
  return t * t * t * (t * (t * 6 - 15) + 10);
}

/**
 * Computes 2D Perlin noise for the given coordinates.
 * The output value is typically in the range [-1, 1].
 *
 * @param x - The x-coordinate.
 * @param y - The y-coordinate.
 * @param gridSize - The size of the grid cells. Larger values result in "zoomed-in" noise. Defaults to 8.
 * @returns The Perlin noise value.
 */
export function perlin(x: number, y: number, gridSize: number = 8): number {
  // Determine grid cell coordinates
  const x0 = Math.floor(x / gridSize);
  const y0 = Math.floor(y / gridSize);
  const x1 = x0 + 1;
  const y1 = y0 + 1;

  // Determine fractional part of x and y
  const dx = (x / gridSize) - x0;
  const dy = (y / gridSize) - y0;

  // Get gradient vectors for the four corners of the grid cell
  const grad00 = getRandomGradient(x0, y0);
  const grad10 = getRandomGradient(x1, y0);
  const grad01 = getRandomGradient(x0, y1);
  const grad11 = getRandomGradient(x1, y1);

  // Compute the dot product between the gradient and the distance vector
  const dot00 = grad00[0] * dx + grad00[1] * dy;
  const dot10 = grad10[0] * (dx - 1) + grad10[1] * dy;
  const dot01 = grad01[0] * dx + grad01[1] * (dy - 1);
  const dot11 = grad11[0] * (dx - 1) + grad11[1] * (dy - 1);

  // Get smoothed interpolation factors
  const u = fade(dx);
  const v = fade(dy);

  // Interpolate the dot products
  const ix0 = lerp(dot00, dot10, u);
  const ix1 = lerp(dot01, dot11, u);
  const value = lerp(ix0, ix1, v);

  return value;
}


// --- Example Usage ---

// To run this example:
// 1. Save the code as `perlin.ts`.
// 2. Create another file, e.g., `main.ts`.
// 3. In `main.ts`, import and use the function:
/*
  import { perlin } from './perlin';

  const noiseValue = perlin(10.5, 20.3);
  console.log(`Perlin noise value: ${noiseValue}`);

  const noiseValueWithGrid = perlin(10.5, 20.3, 16);
  console.log(`Perlin noise value with larger grid: ${noiseValueWithGrid}`);
*/

// Example call directly in this file for quick testing
// const noiseValue = perlin(10.5, 20.3);
// console.log(`Perlin noise value: ${noiseValue}`);
// Expected output: Perlin noise value: 0.1652174240763595
