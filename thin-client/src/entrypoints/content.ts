import { browser } from "wxt/browser";

const port = browser.runtime.connect({ name: "dnea.cs.v1" });

const pending = new Map<string, (msg: unknown) => void>();
const listeners = new Set<(data: unknown) => void>();

port.onMessage.addListener((raw: unknown) => {
  const msg = raw as { kind: string; cid: string; payload?: unknown; err?: unknown };
  if (msg.kind === "dnea.cs/res" || msg.kind === "dnea.cs/err") {
    const resolver = pending.get(msg.cid);
    if (resolver) { pending.delete(msg.cid); resolver(msg); }
    return;
  }
  if (msg.kind === "dnea.cs/evt") {
    for (const fn of listeners) fn(msg.payload);
  }
});

const bridge = {
  invoke(type: string, payload: unknown, opts: { mod?: string; cap?: string; ttl_ms?: number } = {}): Promise<unknown> {
    const cid = crypto.randomUUID();
    return new Promise((resolve, reject) => {
      pending.set(cid, (msg) => {
        const m = msg as { kind: string; payload?: unknown; err?: { code: string; message: string } };
        if (m.kind === "dnea.cs/err") reject(Object.assign(new Error(m.err!.message), { code: m.err!.code }));
        else resolve(m.payload);
      });
      port.postMessage({ kind: "dnea.cs/req", cid, type, ...opts, payload });
    });
  },
  on(_event: string, fn: (data: unknown) => void): () => void {
    listeners.add(fn);
    return () => listeners.delete(fn);
  },
};

(window as unknown as { __dnea: typeof bridge }).__dnea = bridge;

const ALLOWED_PAGE_TYPES = new Set(["focus.getState", "focus.toggle"]);

window.addEventListener("message", (ev) => {
  if (ev.source !== window) return;
  if (ev.origin !== location.origin) return;
  const data = ev.data as { __dnea?: true; channel?: string; cid?: string; type?: string; payload?: unknown };
  if (!data?.__dnea || !data.cid || !data.type) return;
  if (!ALLOWED_PAGE_TYPES.has(data.type)) return;
  bridge.invoke(data.type, data.payload)
    .then((payload) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: true, payload }, location.origin))
    .catch((err) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: false, error: String(err) }, location.origin));
});