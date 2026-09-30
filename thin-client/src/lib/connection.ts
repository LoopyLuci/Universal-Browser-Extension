import { browser } from "wxt/browser";
import { Codec } from "./codec";
import { Envelope, PROTOCOL_VERSION, HEARTBEAT_IDLE_MS, HANDSHAKE_TIMEOUT_MS, DEFAULT_TTL_MS } from "./protocol";
import { DneaError } from "./errors";
import { backoffDelay } from "./backoff";
import { Logger } from "./log";
import { uuidv7 } from "./crypto";

const MAX_CONSECUTIVE_FAILURES = 8;
const BREAKER_COOLDOWN_MS = 5 * 60_000;

type State = "DISCONNECTED" | "CONNECTING" | "HANDSHAKING" | "READY" | "DRAINING" | "FAILED";

type Pending = {
  id: string;
  resolve: (env: Envelope) => void;
  reject: (err: DneaError) => void;
  timer: ReturnType<typeof setTimeout>;
  idempotent: boolean;
};

export class Connection {
  private state: State = "DISCONNECTED";
  private port?: browser.runtime.Port;
  private codec = new Codec();
  private attempt = 0;
  private consecutiveFailures = 0;
  private breakerOpenUntil = 0;
  private outstanding = new Map<string, Pending>();
  private lastRxAt = 0;
  private lastTxAt = 0;
  private log = new Logger("connection");
  private listeners = new Map<string, Set<(env: Envelope) => void>>();

  constructor(private opts: {
    hostName: string;
    clientVersion: string;
    browserFamily: "chromium" | "firefox" | "safari";
    onStateChange?: (s: State) => void;
  }) {}

  on(type: string, fn: (env: Envelope) => void): () => void {
    const set = this.listeners.get(type) ?? new Set();
    set.add(fn);
    this.listeners.set(type, set);
    return () => set.delete(fn);
  }

  private setState(s: State) {
    if (this.state === s) return;
    this.state = s;
    this.opts.onStateChange?.(s);
  }

  private emit(env: Envelope) {
    const set = this.listeners.get(env.type);
    if (set) for (const fn of set) fn(env);
    const wildcard = this.listeners.get("*");
    if (wildcard) for (const fn of wildcard) fn(env);
  }

  connect(): void {
    if (this.state === "READY" || this.state === "HANDSHAKING" || this.state === "CONNECTING") return;
    if (Date.now() < this.breakerOpenUntil) return;

    this.setState("CONNECTING");
    try {
      this.port = browser.runtime.connectNative(this.opts.hostName);
    } catch (e) {
      this.log.warn("connectNative threw", { error: String(e) });
      return this.fail(new DneaError("E_TRANSPORT_NOT_INSTALLED", false));
    }

    this.port.onMessage.addListener((raw) => this.onRawMessage(raw));
    this.port.onDisconnect.addListener(() => this.onDisconnect());
    this.setState("HANDSHAKING");

    this.sendRaw({
      v: 1, id: uuidv7(), type: "dnea/hello",
      ts: new Date().toISOString(), src: "client", dst: "runtime",
      payload: {
        clientVersion: this.opts.clientVersion,
        browserFamily: this.opts.browserFamily,
        browserVersion: navigator.userAgent,
        extensionId: browser.runtime.id,
        mtu: 1_048_576,
        encodings: ["identity"],
        features: {
          chunking: true, contentOverlay: true, consentUI: true,
          serviceWorkerBackground: this.opts.browserFamily === "chromium",
        },
      },
    });

    setTimeout(() => {
      if (this.state === "HANDSHAKING") {
        this.log.warn("handshake timeout");
        this.fail(new DneaError("E_TRANSPORT_HANDSHAKE_TIMEOUT", true));
      }
    }, HANDSHAKE_TIMEOUT_MS);
  }

  private async onRawMessage(raw: unknown): Promise<void> {
    this.lastRxAt = Date.now();
    let decoded;
    try { decoded = await this.codec.decode(raw); }
    catch (e) {
      const code = String(e).includes("CHUNK") ? "E_FRAMING_CHUNK_OVERFLOW"
                 : String(e).includes("DIGEST") ? "E_FRAMING_DIGEST_MISMATCH"
                 : "E_FRAMING_INVALID_JSON";
      this.log.warn("framing error", { code });
      return;
    }
    if (decoded.kind === "incomplete") return;
    const env = decoded.envelope;
    const srcOk = env.src === "runtime" || env.src.startsWith("mod:");
    if (!srcOk) { this.log.warn("invalid src on runtime-side transport", { src: env.src }); return; }
    if (env.type === "dnea/welcome") return this.onWelcome(env);
    if (env.type === "dnea/pong") return;
    if (env.type === "dnea/goodbye") return this.onGoodbye(env);
    if (env.reply_to) return this.onReply(env);
    this.emit(env);
  }

  private onReply(env: Envelope): void {
    const pending = this.outstanding.get(env.reply_to!);
    if (!pending) { this.log.warn("unknown correlation", { reply_to: env.reply_to }); return; }
    clearTimeout(pending.timer);
    this.outstanding.delete(env.reply_to!);
    pending.resolve(env);
  }

  private onWelcome(env: Envelope): void {
    const meta = env.meta as Record<string, unknown> | undefined;
    if (meta?.client_extension_id !== browser.runtime.id) {
      this.fail(new DneaError("E_HANDSHAKE_REJECTED", false, undefined, { reason: "extension_id_mismatch" }));
      return;
    }
    this.consecutiveFailures = 0;
    this.attempt = 0;
    this.setState("READY");
    void browser.storage.session.set({ dnea: { wasConnected: true } });
    this.emit(env);
    this.log.info("handshake complete");
  }

  private onGoodbye(env: Envelope): void {
    const reason = (env.payload as { reason?: string } | undefined)?.reason ?? "shutdown";
    this.setState("DRAINING");
    setTimeout(() => this.close(), 5_000);
    if (reason === "shutdown") {
      void browser.storage.session.set({ dnea: { wasConnected: false } });
    }
  }

  private onDisconnect(): void {
    const stillPending = [...this.outstanding.values()];
    this.outstanding.clear();
    for (const p of stillPending) {
      clearTimeout(p.timer);
      p.reject(new DneaError("E_TRANSPORT_DISCONNECTED", true));
    }
    void browser.storage.session.set({ dnea: { wasConnected: false } });
    this.port = undefined;
    this.consecutiveFailures++;
    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
    }
    this.setState("FAILED");
    this.scheduleReconnect();
  }

  private fail(err: DneaError): void {
    this.log.warn("connection failure", { code: err.code });
    try { this.port?.disconnect(); } catch {}
    this.port = undefined;
    this.consecutiveFailures++;
    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
    }
    this.setState("FAILED");
    this.scheduleReconnect();
  }

  private scheduleReconnect(): void {
    const delay = backoffDelay(this.attempt);
    this.attempt = Math.min(this.attempt + 1, 12);
    setTimeout(() => this.connect(), delay);
  }

  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
    if (this.state !== "READY") throw new DneaError("E_TRANSPORT_DISCONNECTED", true);
    return new Promise<Envelope>((resolve, reject) => {
      const ttl = env.ttl_ms ?? DEFAULT_TTL_MS;
      const timer = setTimeout(() => {
        this.outstanding.delete(env.id);
        reject(new DneaError("E_TTL_EXPIRED", true));
      }, ttl);
      this.outstanding.set(env.id, { id: env.id, resolve, reject, timer, idempotent: !!opts.idempotent });
      void this.sendEnvelope(env);
    });
  }

  send(env: Envelope): void {
    if (this.state !== "READY") { this.log.warn("send dropped: not READY", { type: env.type }); return; }
    void this.sendEnvelope(env);
  }

  private async sendEnvelope(env: Envelope): Promise<void> {
    const frames = await this.codec.encode(env);
    for (const frame of frames) this.sendRaw(frame);
    this.lastTxAt = Date.now();
  }

  private sendRaw(frame: unknown): void {
    if (!this.port) return;
    try { this.port.postMessage(frame); }
    catch (e) { this.log.warn("postMessage failed", { error: String(e) }); this.fail(new DneaError("E_TRANSPORT_DISCONNECTED", true)); }
  }

  maybePing(): void {
    if (this.state !== "READY") return;
    if (Date.now() - this.lastRxAt < HEARTBEAT_IDLE_MS) return;
    void this.sendEnvelope({
      v: 1, id: uuidv7(), type: "dnea/ping", ts: new Date().toISOString(),
      src: "client", dst: "runtime", payload: { seq: Date.now() },
    });
  }

  probeIfIdle(): void {
    if (this.state === "DISCONNECTED" || this.state === "FAILED") this.connect();
  }

  private close(): void {
    try { this.port?.disconnect(); } catch {}
    this.port = undefined;
    this.setState("DISCONNECTED");
  }
}