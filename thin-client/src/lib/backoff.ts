const BASE_MS = 250;
const CAP_MS = 30_000;

export function backoffDelay(attempt: number): number {
  const prev = Math.min(CAP_MS, BASE_MS * Math.pow(3, attempt));
  return BASE_MS + Math.random() * (prev * 3 - BASE_MS);
}