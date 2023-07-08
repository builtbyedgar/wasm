export function convertRange(
  oldMin: number,
  oldMax: number,
  newMin: number,
  newMax: number,
  oldValue: number
): number {
  return ((oldValue - oldMin) * (newMax - newMin)) / (oldMax - oldMin) + newMin
}
