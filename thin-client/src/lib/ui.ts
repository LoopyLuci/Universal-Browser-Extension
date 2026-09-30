import { Envelope } from "./protocol";

const mounts = new Map<string, { host: HTMLElement; shadow: ShadowRoot }>();

export class OverlayMount {
  async mount(env: Envelope): Promise<void> {
    const p = env.payload as { origin: string; mountId: string; css?: string; html?: string };
    if (!matchesOrigin(location.origin, p.origin)) return;
    const existing = mounts.get(p.mountId);
    if (existing) existing.host.remove();

    const host = document.createElement("div");
    host.id = `dnea-mount-${p.mountId}`;
    host.style.position = "fixed";
    host.style.zIndex = "2147483647";

    if (!("attachShadow" in host)) {
      throw new Error("E_UNSUPPORTED_FEATURE");
    }
    const shadow = host.attachShadow({ mode: "closed" });
    if (!shadow) throw new Error("E_UNSUPPORTED_FEATURE");

    if (p.css) {
      const style = document.createElement("style");
      style.textContent = p.css;
      shadow.appendChild(style);
    }
    if (p.html) {
      const container = document.createElement("div");
      container.innerHTML = sanitize(p.html);
      shadow.appendChild(container);
    }
    document.documentElement.appendChild(host);
    mounts.set(p.mountId, { host, shadow });
  }

  clear(env: Envelope): void {
    const p = env.payload as { targetId: string };
    const m = mounts.get(p.targetId);
    if (m) { m.host.remove(); mounts.delete(p.targetId); }
  }
}

function matchesOrigin(current: string, pattern: string): boolean {
  if (pattern.endsWith("/*")) return current.startsWith(pattern.slice(0, -1));
  return current === pattern;
}

function sanitize(html: string): string {
  return html
    .replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi, "")
    .replace(/\son\w+\s*=\s*"[^"]*"/gi, "")
    .replace(/\son\w+\s*=\s*'[^']*'/gi, "");
}