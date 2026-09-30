export class DneaError extends Error {
  constructor(
    public readonly code: string,
    public readonly retryable: boolean = false,
    public readonly retryAfterMs?: number,
    public readonly details?: unknown,
  ) {
    super(code);
    this.name = "DneaError";
  }
}

export function fromEnvelopeError(err: {
  code: string;
  retryable?: boolean;
  retry_after_ms?: number;
  details?: unknown;
}): DneaError {
  return new DneaError(err.code, err.retryable ?? false, err.retry_after_ms, err.details);
}