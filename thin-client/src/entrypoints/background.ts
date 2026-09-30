import { browser } from "wxt/browser";
import { Connection } from "@/lib/connection";
import { Router } from "@/lib/router";
import { CSPortBridge } from "@/lib/cs-bridge";
import { ConsentController } from "@/lib/consent";
import { OverlayMount } from "@/lib/ui";

const HOST_NAME = "org.dnea.runtime";

const conn = new Connection({
  hostName: HOST_NAME,
  clientVersion: browser.runtime.getManifest().version,
  browserFamily: detectFamily(),
});

const router = new Router(conn);
const cs = new CSPortBridge(router);
const consent = new ConsentController(router);
const overlays = new OverlayMount();

router.on("dnea/consent/request", (env) => consent.handle(env));
router.on("dnea/capability/changed", (env) => cs.broadcastCapabilityChange(env));
router.on("dnea/ui/inject", (env) => overlays.mount(env));
router.on("dnea/ui/clear", (env) => overlays.clear(env));

browser.runtime.onConnect.addListener((port) => {
  if (port.name === "dnea.cs.v1") cs.accept(port);
});
browser.runtime.onStartup.addListener(() => conn.connect());
browser.runtime.onInstalled.addListener(() => conn.connect());

browser.alarms.create("dnea.heartbeat", { periodInMinutes: 0.25 });
browser.alarms.create("dnea.probe", { periodInMinutes: 1 });
browser.alarms.onAlarm.addListener((a) => {
  if (a.name === "dnea.heartbeat") conn.maybePing();
  if (a.name === "dnea.probe") conn.probeIfIdle();
});

void (async () => {
  const snap = await browser.storage.session.get("dnea");
  if (snap?.dnea?.wasConnected) conn.connect();
})();

function detectFamily(): "chromium" | "firefox" | "safari" {
  const ua = navigator.userAgent;
  if (ua.includes("Firefox/")) return "firefox";
  if (ua.includes("Safari/") && !ua.includes("Chrome/")) return "safari";
  return "chromium";
}

browser.runtime.onMessage.addListener((msg, _sender, sendResponse) => {
  if (msg?.kind === "dnea.consent.fetch") {
    sendResponse(consent.fetchPrompt(msg.promptId));
    return true;
  }
  if (msg?.kind === "dnea.consent.respond") {
    consent.respond(msg.promptId, { granted: msg.granted, rememberFor: msg.rememberFor })
      .then(() => sendResponse({ ok: true }))
      .catch((e) => sendResponse({ ok: false, error: String(e) }));
    return true;
  }
  return false;
});