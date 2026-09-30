import { browser } from "wxt/browser";
import { Router } from "./router";
import { Envelope } from "./protocol";

export class ConsentController {
  private pending = new Map<string, Envelope>();
  private consentWindowId?: number;
  constructor(private router: Router) {}

  async handle(env: Envelope): Promise<void> {
    const payload = env.payload as { promptId: string };
    this.pending.set(payload.promptId, env);
    try {
      await browser.runtime.sendMessage({ kind: "dnea.consent.show", promptId: payload.promptId });
      return;
    } catch {}
    await this.openConsentPage(payload.promptId);
  }

  private async openConsentPage(promptId: string): Promise<void> {
    const url = browser.runtime.getURL(`/consent.html?promptId=${encodeURIComponent(promptId)}`);
    const win = await browser.windows.create({ url, type: "popup", width: 520, height: 640 });
    this.consentWindowId = win.id;
  }

  fetchPrompt(promptId: string): Envelope | undefined { return this.pending.get(promptId); }

  async respond(promptId: string, response: { granted: boolean; scope?: unknown; rememberFor?: string }): Promise<void> {
    const env = this.pending.get(promptId);
    if (!env) throw new Error("prompt not found");
    this.pending.delete(promptId);
    this.router.sendRuntimeEnvelope({
      type: "dnea/consent/response",
      reply_to: env.id,
      payload: {
        prompt_id: promptId,
        granted: response.granted,
        scope: response.scope,
        remember_for: response.rememberFor,
      },
    });
  }
}