const MAX_ENTRIES = 100;

export class Logger {
  private entries: { ts: number; level: string; msg: string; extra?: unknown }[] = [];
  constructor(private scope: string) {}
  info(msg: string, extra?: unknown): void { this.record("info", msg, extra); }
  warn(msg: string, extra?: unknown): void { this.record("warn", msg, extra); }
  error(msg: string, extra?: unknown): void { this.record("error", msg, extra); }
  snapshot() { return this.entries.slice(); }
  private record(level: string, msg: string, extra?: unknown): void {
    this.entries.push({ ts: Date.now(), level, msg, extra });
    if (this.entries.length > MAX_ENTRIES) this.entries.shift();
    if (level === "error") console.error(`[${this.scope}]`, msg, extra);
    else if (level === "warn") console.warn(`[${this.scope}]`, msg, extra);
    else console.debug(`[${this.scope}]`, msg, extra);
  }
}