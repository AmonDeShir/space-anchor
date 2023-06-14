/** 
 * Performs linear interpolation between two values.
 *
 * ## Arguments
 *
 * * `start` - The starting value.
 * * `end` - The ending value.
 * * `t` - The interpolation factor in the range [0, 1].
 */ 
export function lerp(start: number, end: number, t: number): number {
  return start + (end - start) * t;
}

/**
 * Clamps a value between a minimum and maximum range.
 * 
 * ## Arguments
 * 
 * * `value` - The value to be clamped.
 * * `min` - The minimum value of the range.
 * * `max` - The maximum value of the range.
 */
export function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

/**
 * Converts a time value to a percentage between a start and end value.
 *
 * ## Arguments
 * 
 * * `value` - The time value to be converted.
 * * `start` - The start value of the range.
 * * `end` - The end value of the range.
 */
export function timeToPercent(value: number, start: number, end: number) {
  return (value - start) / (end - start);
}

/** 
 * Performs linear interpolation (lerp) between two angles.
 *
 * ## Arguments
 *
 * * `start` - The starting value.
 * * `end` - The ending value.
 * * `t` - The interpolation factor in the range [0, 1].
 */ 
export function lerpAngle(start: number, end: number, t: number): number {
  return start + shortAngleDist(start, end) * t
}

/** 
 * Calculates the shortest angle distance between two angles.
 *
 * ## Arguments
 *
 * * `from` - The starting angle in radians.
 * * `to` - The target angle in radians.
 *
 */
export function shortAngleDist(from: number, to: number): number {
  const max_angle: number = Math.PI * 2.0;
  const difference = remEuclid(to - from, max_angle);
  
  return remEuclid(2.0 * difference, max_angle) - difference;
}

/**
 * Calculates the remainder of division of two floating-point numbers.
 * The result is always positive.
 * 
 * ## Arguments
 * 
 * * `a` - The dividend.
 * * `b` - The divisor.
 * 
 * ## Examples
 * @example
 * console.log(remEuclid(7.5, 3.2));    // Output: 1.0999999
 * console.log(remEuclid(-7.5, 3.2));   // Output: 2.1000001
 * console.log(remEuclid(7.5, -3.2));   // Output: 1.0999999
 */
export function remEuclid(a: number, b: number): number {
  const result = a % b;
  return result >= 0 ? result : result + b;
}

/**
 * Performs circular arc interpolation between two points.
 *
 * ## Arguments
 * 
 * * `y1` - The initial y-coordinate.
 * * `z1` - The initial z-coordinate.
 * * `y2` - The final y-coordinate.
 * * `z2` - The final z-coordinate.
 * * `t` - The interpolation factor in the range [0, 1].
 */
export function circularArcInterpolation(y1: number, z1: number, y2: number, z2: number, t: number): { y: number, z: number } {
  const centerY = (y1 + y2) / 2;
  const centerZ = (z1 + z2) / 2;
  const radius = Math.sqrt(Math.pow(y2 - y1, 2) + Math.pow(z2 - z1, 2)) / 2;
  const angle = lerp(0, -Math.PI, t); 

  const y = centerY + radius * Math.cos(angle);
  const z = centerZ + radius * Math.sin(angle);

  return { y, z };
}