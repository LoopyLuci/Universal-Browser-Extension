import { Envelope, DEFAULT_MTU, OVERHEAD_BUDGET, MAX_CHUNKS, CHUNK_TIMEOUT_MS } from "./protocol";
import { base64UrlEncode, base64UrlDecode, sha256Hex } from "./crypto";

export type Decoded = { kind: "envelope"; envelope: Envelope } | { kind: "incomplete" };
type Inflight = { parts: (Uint8Array | undefined)[]; total: number; startedAt: number; digest: string };

export class Codec {
  private inflight = new Map<string, Inflight>();
  constructor(private mtu: number = DEFAULT_MTU) {}

  async encode(env: Envelope): Promise<unknown[]> {
    const bytes = new TextEncoder().encode(JSON.stringify(env));
    if (bytes.length <= this.mtu - OVERHEAD_BUDGET) return [env];
    const chunkSize = this.mtu - OVERHEAD_BUDGET - 512;
    const total = Math.ceil(bytes.length / chunkSize);
    if (total > MAX_CHUNKS) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    const digest = `sha-256:${await sha256Hex(bytes)}`;
    const frames: unknown[] = [];
    for (let i = 0; i < total; i++) {
      const part = bytes.slice(i * chunkSize, (i + 1) * chunkSize);
      frames.push({
        v: 1, type: "dnea/chunk", cid: env.id, seq: i, total,
        part: base64UrlEncode(part), digest, final: i + 1 === total,
      });
    }
    return frames;
  }

  async decode(raw: unknown): Promise<Decoded> {
    const obj = raw as Record<string, unknown>;
    if (obj.type !== "dnea/chunk") {
      return { kind: "envelope", envelope: obj as unknown as Envelope };
    }
    return this.decodeChunk(obj);
  }

  private async decodeChunk(frame: Record<string, unknown>): Promise<Decoded> {
    const cid = String(frame.cid);
    const seq = Number(frame.seq);
    const total = Number(frame.total);
    if (total > MAX_CHUNKS || seq < 0 || seq >= total) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    let entry = this.inflight.get(cid);
    if (!entry) {
      entry = { parts: new Array(total), total, startedAt: Date.now(), digest: String(frame.digest) };
      this.inflight.set(cid, entry);
    }
    if (Date.now() - entry.startedAt > CHUNK_TIMEOUT_MS) {
      this.inflight.delete(cid);
      throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    }
    entry.parts[seq] = base64UrlDecode(String(frame.part));
    if (entry.parts.some((p) => p === undefined)) return { kind: "incomplete" };
    const total_bytes = entry.parts.reduce((n, p) => n + (p?.byteLength ?? 0), 0);
    const full = new Uint8Array(total_bytes);
    let off = 0;
    for (const p of entry.parts) { if (!p) continue; full.set(p, off); off += p.byteLength; }
    const actual = `sha-256:${await sha256Hex(full)}`;
    this.inflight.delete(cid);
    if (actual !== entry.digest) throw new Error("E_FRAMING_DIGEST_MISMATCH");
    const envelope = JSON.parse(new TextDecoder().decode(full)) as Envelope;
    return { kind: "envelope", envelope };
  }
}