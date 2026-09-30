export type Envelope = {
  v: 1;
  id: string;
  type: string;
  ts: string;
  src: Party;
  dst: Party;
  reply_to?: string;
  cap?: string;
  mod?: string;
  enc?: "identity" | "zstd" | "br";
  ttl_ms?: number;
  payload?: unknown;
  err?: EnvelopeError;
  meta?: Record<string, unknown>;
};

export type Party = "runtime" | "client" | `cs:${string}` | `mod:${string}`;

export type EnvelopeError = {
  code: string;
  message: string;
  retryable?: boolean;
  retry_after_ms?: number;
  details?: unknown;
  cause?: string;
};

export const PROTOCOL_VERSION = 1;
export const DEFAULT_MTU = 1_048_576;
export const OVERHEAD_BUDGET = 4_096;
export const MAX_CHUNKS = 256;
export const CHUNK_TIMEOUT_MS = 30_000;
export const HANDSHAKE_TIMEOUT_MS = 5_000;
export const HEARTBEAT_IDLE_MS = 20_000;
export const HEARTBEAT_TIMEOUT_MS = 5_000;
export const DEFAULT_TTL_MS = 30_000;