(function () {
  "use strict";

  const readyHandlers = [];
  let ready = false;

  function fireReady() {
    ready = true;
    for (const h of readyHandlers) {
      try { h(); } catch (e) { /* swallow */ }
    }
  }

  async function invoke(method, args) {
    const url = new URL(location.href);
    const base = url.pathname.replace(/\/$/, "");
    const res = await fetch(base + "/invoke", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ method, args: args ?? {} }),
    });
    if (!res.ok) {
      const text = await res.text();
      throw new Error(text || ("HTTP " + res.status));
    }
    const body = await res.json();
    if (!body.ok) throw new Error(body.error || "invoke failed");
    return body.value;
  }

  async function settingsGet() { return invoke("$settings.get", {}); }
  async function settingsSet(patch) { return invoke("$settings.set", patch); }

  function on(event, handler) {
    const channel = "dnea-" + (location.pathname.split("/")[2] || "module");
    const wrapped = (e) => {
      if (e.source !== window.parent) return;
      if (e.data?.channel !== channel) return;
      if (e.data?.event !== event) return;
      handler(e.data.data);
    };
    window.addEventListener("message", wrapped);
    return () => window.removeEventListener("message", wrapped);
  }

  async function requestOrigin(capabilityId, origin) {
    return invoke("$origin.request", { capabilityId, origin });
  }

  async function notify(input) { return invoke("$notify", input); }

  window.dnea = {
    invoke,
    settings: { get: settingsGet, set: settingsSet },
    on,
    requestOrigin,
    notify,
    onReady(handler) {
      if (ready) handler();
      else readyHandlers.push(handler);
    },
    context: {
      moduleId: location.pathname.split("/")[2] || "",
      panelId: location.pathname.split("/")[4] || "",
    },
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", fireReady);
  } else {
    fireReady();
  }
})();