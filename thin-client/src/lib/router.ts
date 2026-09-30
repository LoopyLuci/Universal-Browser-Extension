import { Connection } from "./connection";
import { Envelope, PROTOCOL_VERSION } from "./protocol";
import { uuidv7 } from "./crypto";

type Handler = (env: Envelope) => void;

export class Router {
  private handlers = new Map<string, Set<Handler>>();
  constructor(private conn: Connection) { conn.on("*", (env) => this.dispatch(env)); }

  on(type: string, handler: Handler): () => void {
    const set = this.handlers.get(type) ?? new Set();
    set.add(handler);
    this.handlers.set(type, set);
    return () => set.delete(handler);
  }

  private dispatch(env: Envelope): void {
    const set = this.handlers.get(env.type);
    if (set) for (const h of set) h(env);
    const wildcard = this.handlers.get("*");
    if (wildcard) for (const h of wildcard) h(env);
  }

  mintEnvelope(input: {
    type: string; src: string; dst: string;
    cap?: string; mod?: string; ttl_ms?: number; payload?: unknown;
  }): Envelope {
    return {
      v: PROTOCOL_VERSION, id: uuidv7(), type: input.type,
      ts: new Date().toISOString(),
      src: input.src as never, dst: input.dst as never,
      cap: input.cap, mod: input.mod, ttl_ms: input.ttl_ms, payload: input.payload,
    };
  }

  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
    return this.conn.request(env, opts);
  }

  send(env: Envelope): void { this.conn.send(env); }

  sendRuntimeEnvelope(input: { type: string; payload?: unknown; reply_to?: string }): void {
    const env = this.mintEnvelope({
      type: input.type, src: "client", dst: "runtime", payload: input.payload,
    });
    if (input.reply_to) env.reply_to = input.reply_to as never;
    this.send(env);
  }

  sendEnvelope(env: Envelope): void { this.conn.send(env); }
}