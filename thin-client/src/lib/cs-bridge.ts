import { browser } from "wxt/browser";
import { Router } from "./router";
import { Envelope } from "./protocol";
import { Logger } from "./log";
import { uuidv7 } from "./crypto";

const MAX_PORTS = 512;
const QUEUE_MAX = 128;
const RATE_LIMIT_PER_MIN = 60;
const RATE_BURST = 20;

type CSMessage =
  | { kind: "dnea.cs/req"; cid: string; type: string; mod?: string; cap?: string; ttl_ms?: number; payload?: unknown }
  | { kind: "dnea.cs/evt"; cid: string; type: string; mod?: string; payload?: unknown };

type BgMessage =
  | { kind: "dnea.cs/res"; cid: string; payload?: unknown }
  | { kind: "dnea.cs/err"; cid: string; err: { code: string; message: string; retryable?: boolean; retry_after_ms?: number } };

type PortRecord = {
  portId: string;
  port: browser.runtime.Port;
  tabId: number;
  frameId: number;
  url: string;
  incognito: boolean;
  registeredAt: number;
  lastSeen: number;
  queue: unknown[];
  tokens: { count: number; resetAt: number };
  pendingCids: Map<string, string>;
};

export class CSPortBridge {
  private ports = new Map<string, PortRecord>();
  private byTabFrame = new Map<string, string>();
  private log = new Logger("cs-bridge");

  constructor(private router: Router) {
    router.on("dnea/result", (env) => this.deliverResult(env));
    router.on("dnea/error", (env) => this.deliverResult(env));
    router.on("dnea/event", (env) => this.deliverEvent(env));
    browser.tabs.onRemoved.addListener((tabId) => this.unregisterTab(tabId));
    browser.tabs.onUpdated.addListener((tabId, change) => {
      if (change.url) this.unregisterTab(tabId, "url_changed");
    });
  }

  accept(port: browser.runtime.Port): void {
    const sender = port.sender;
    if (!sender?.tab || sender.id !== browser.runtime.id) {
      try { port.disconnect(); } catch {}
      return;
    }
    const key = `${sender.tab.id}:${sender.frameId ?? 0}`;
    const existing = this.byTabFrame.get(key);
    if (existing) this.disconnectPort(existing, "superseded");
    if (this.ports.size >= MAX_PORTS) {
      const lru = [...this.ports.values()].sort((a, b) => a.lastSeen - b.lastSeen)[0];
      if (lru) this.disconnectPort(lru.portId, "evicted");
    }
    const portId = uuidv7();
    const rec: PortRecord = {
      portId, port,
      tabId: sender.tab.id!, frameId: sender.frameId ?? 0,
      url: sender.url ?? "", incognito: sender.tab.incognito ?? false,
      registeredAt: Date.now(), lastSeen: Date.now(),
      queue: [], tokens: { count: RATE_BURST, resetAt: Date.now() + 60_000 },
      pendingCids: new Map(),
    };
    this.ports.set(portId, rec);
    this.byTabFrame.set(key, portId);
    port.onMessage.addListener((msg) => this.onCSMessage(rec, msg as CSMessage));
    port.onDisconnect.addListener(() => this.disconnectPort(portId, "disconnected"));
    this.router.sendRuntimeEnvelope({
      type: "dnea/cs/register",
      payload: { portId, tabId: rec.tabId, frameId: rec.frameId, url: rec.url, incognito: rec.incognito },
    });
  }

  private onCSMessage(rec: PortRecord, msg: CSMessage): void {
    rec.lastSeen = Date.now();
    if (!this.takeToken(rec)) {
      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
        err: { code: "E_RATE_LIMITED", message: "rate limit", retryable: true, retry_after_ms: 1000 } });
      return;
    }
    if (rec.queue.length >= QUEUE_MAX) {
      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
        err: { code: "E_CS_BACKPRESSURE", message: "queue full", retryable: true, retry_after_ms: 250 } });
      rec.queue.shift();
    }
    const env = this.router.mintEnvelope({
      type: msg.type, src: `cs:${rec.portId}`,
      dst: msg.mod ? `mod:${msg.mod}` : "runtime",
      cap: msg.cap, mod: msg.mod, ttl_ms: msg.ttl_ms, payload: msg.payload,
    });
    if (msg.kind === "dnea.cs/req") rec.pendingCids.set(env.id, msg.cid);
    rec.queue.push(env);
    this.router.sendEnvelope(env);
  }

  private deliverResult(env: Envelope): void {
    const portId = env.dst.startsWith("cs:") ? env.dst.slice(3) : undefined;
    if (!portId) return;
    const rec = this.ports.get(portId);
    if (!rec) return;
    const cid = rec.pendingCids.get(env.reply_to ?? env.id);
    if (cid) {
      rec.pendingCids.delete(env.reply_to ?? env.id);
      const out: BgMessage = env.err
        ? { kind: "dnea.cs/err", cid, err: {
            code: env.err.code, message: env.err.message,
            retryable: env.err.retryable, retry_after_ms: env.err.retry_after_ms } }
        : { kind: "dnea.cs/res", cid, payload: env.payload };
      this.postSafe(rec, out);
    }
    if (rec.queue.length) rec.queue.shift();
  }

  private deliverEvent(env: Envelope): void {
    for (const rec of this.ports.values()) {
      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
    }
  }

  private postSafe(rec: PortRecord, msg: BgMessage): void {
    try { rec.port.postMessage(msg); } catch { this.disconnectPort(rec.portId, "post_failed"); }
  }

  private takeToken(rec: PortRecord): boolean {
    const now = Date.now();
    if (now > rec.tokens.resetAt) rec.tokens = { count: RATE_BURST, resetAt: now + 60_000 };
    if (rec.tokens.count <= 0) return false;
    rec.tokens.count--;
    return true;
  }

  private disconnectPort(portId: string, reason: string): void {
    const rec = this.ports.get(portId);
    if (!rec) return;
    this.ports.delete(portId);
    this.byTabFrame.delete(`${rec.tabId}:${rec.frameId}`);
    try { rec.port.disconnect(); } catch {}
    this.router.sendRuntimeEnvelope({
      type: "dnea/cs/unregister",
      payload: { portId, reason },
    });
  }

  private unregisterTab(tabId: number, reason = "tab_removed"): void {
    for (const rec of [...this.ports.values()]) {
      if (rec.tabId === tabId) this.disconnectPort(rec.portId, reason);
    }
  }

  broadcastCapabilityChange(env: Envelope): void {
    for (const rec of this.ports.values()) {
      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
    }
  }
}