import { browser } from "wxt/browser";

const promptId = new URLSearchParams(location.search).get("promptId")!;

(async () => {
  const env = await browser.runtime.sendMessage({ kind: "dnea.consent.fetch", promptId });
  if (!env) {
    document.getElementById("title")!.textContent = "This consent request has expired.";
    return;
  }
  const p = env.payload as { moduleName: string; capabilityId: string; reason: string; constraints: unknown };
  document.getElementById("title")!.textContent = `${p.moduleName} wants ${p.capabilityId}`;
  document.getElementById("reason")!.textContent = p.reason;
  document.getElementById("constraints")!.textContent = JSON.stringify(p.constraints, null, 2);
  const respond = async (granted: boolean, rememberFor?: string) => {
    await browser.runtime.sendMessage({ kind: "dnea.consent.respond", promptId, granted, rememberFor });
    window.close();
  };
  document.getElementById("allow")!.onclick = () => respond(true, "module-version");
  document.getElementById("always")!.onclick = () => respond(true, "always");
  document.getElementById("deny")!.onclick = () => respond(false);
})();