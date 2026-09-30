(() => {
  "use strict";

  const $ = (sel) => document.querySelector(sel);
  const state = { token: null, tokenExpiresAt: 0, ws: null, reconnectDelay: 500, refreshFails: 0 };

  async function ensureToken() {
    if (state.token && Date.now() < state.tokenExpiresAt - 60_000) return;
    const res = await fetch("/v1/auth/token", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        grantType: "companion",
        requestedScopes: ["ui.read", "ui.consent", "ui.modules", "ui.settings", "ui.audit"],
      }),
    });
    if (!res.ok) throw new Error("auth failed: " + res.status);
    const body = await res.json();
    state.token = body.token;
    state.tokenExpiresAt = Date.now() + body.expiresIn * 1000;
    state.refreshFails = 0;
  }

  async function api(path, init = {}) {
    await ensureToken();
    const headers = Object.assign(
      { "authorization": "Bearer " + state.token, "content-type": "application/json" },
      init.headers || {});
    const res = await fetch(path, Object.assign({}, init, { headers }));
    if (res.status === 401) {
      state.token = null;
      state.refreshFails++;
      if (state.refreshFails >= 3) {
        setConnection("auth failed", "err");
        throw new Error("auth failed repeatedly");
      }
      return api(path, init);
    }
    return res;
  }

  function setConnection(text, cls) {
    const el = $("#connection");
    el.textContent = text;
    el.className = "badge " + (cls || "");
  }

  async function refreshStatus() {
    const res = await api("/v1/status");
    if (!res.ok) return;
    const s = await res.json();
    const dl = $("#status");
    dl.innerHTML = "";
    for (const [k, v] of [
      ["Version", s.runtimeVersion],
      ["Protocol", "v" + s.protocolVersion],
      ["Session", s.sessionId],
      ["Clients", String(s.clients.length)],
      ["Modules", String(s.modules.length)],
    ]) {
      const dt = document.createElement("dt"); dt.textContent = k;
      const dd = document.createElement("dd"); dd.textContent = v;
      dl.appendChild(dt); dl.appendChild(dd);
    }
  }

  async function refreshModules() {
    const res = await api("/v1/modules");
    if (!res.ok) return;
    const { modules } = await res.json();
    const ul = $("#modules");
    ul.innerHTML = "";
    if (modules.length === 0) {
      const li = document.createElement("li");
      li.className = "card-sub";
      li.textContent = "No modules loaded.";
      ul.appendChild(li);
      return;
    }
    for (const m of modules) {
      const li = document.createElement("li");
      const t = document.createElement("div");
      t.className = "card-title";
      t.textContent = m.id;
      const s = document.createElement("div");
      s.className = "card-sub";
      s.textContent = "v" + m.version + " · " + m.capabilityCount + " grants";
      li.appendChild(t); li.appendChild(s);
      ul.appendChild(li);
    }
  }

  async function refreshConsent() {
    const res = await api("/v1/consent/pending");
    if (!res.ok) return;
    const { prompts } = await res.json();
    const section = $("#consent-section");
    const ul = $("#consent");
    ul.innerHTML = "";
    if (prompts.length === 0) { section.hidden = true; return; }
    section.hidden = false;
    for (const p of prompts) {
      const li = document.createElement("li");
      const t = document.createElement("div");
      t.className = "card-title";
      t.textContent = p.module_name + " wants " + p.capability;
      const r = document.createElement("div");
      r.className = "card-sub";
      r.textContent = p.reason;
      const actions = document.createElement("div");
      actions.className = "consent-actions";

      const allow = document.createElement("button");
      allow.className = "primary";
      allow.textContent = "Allow for this version";
      allow.onclick = () => respond(p.prompt_id, true, "module-version");

      const always = document.createElement("button");
      always.textContent = "Always allow";
      always.onclick = () => respond(p.prompt_id, true, "always");

      const deny = document.createElement("button");
      deny.className = "danger";
      deny.textContent = "Deny";
      deny.onclick = () => respond(p.prompt_id, false);

      actions.appendChild(allow);
      actions.appendChild(always);
      actions.appendChild(deny);
      li.appendChild(t); li.appendChild(r); li.appendChild(actions);
      ul.appendChild(li);
    }
  }

  async function respond(promptId, granted, rememberFor) {
    await api("/v1/consent/" + encodeURIComponent(promptId) + "/respond", {
      method: "POST",
      body: JSON.stringify({ granted, remember_for: rememberFor || null }),
    });
    await refreshConsent();
  }

  function pushEvent(ev) {
    const ul = $("#events");
    const li = document.createElement("li");
    const time = new Date().toLocaleTimeString();
    li.textContent = time + " — " + (ev.topic || "event");
    ul.prepend(li);
    while (ul.children.length > 50) ul.removeChild(ul.lastChild);
  }

  function connectWs() {
    const proto = location.protocol === "https:" ? "wss:" : "ws:";
    const url = proto + "//" + location.host + "/ws";
    const ws = new WebSocket(url);
    state.ws = ws;
    setConnection("connecting…", "");

    ws.onopen = () => {
      setConnection("connected", "ok");
      state.reconnectDelay = 500;
    };
    ws.onmessage = (e) => {
      try {
        const frame = JSON.parse(e.data);
        if (frame.kind === "event") {
          pushEvent(frame);
          if (frame.topic === "consent/opened" || frame.topic === "consent/resolved") {
            refreshConsent();
          }
          if (frame.topic && frame.topic.startsWith("module/")) {
            refreshModules();
          }
        }
      } catch (_) { /* ignore */ }
    };
    ws.onclose = () => {
      setConnection("disconnected", "err");
      setTimeout(connectWs, state.reconnectDelay);
      state.reconnectDelay = Math.min(state.reconnectDelay * 2, 30_000);
    };
    ws.onerror = () => ws.close();
  }

  async function main() {
    try { await ensureToken(); }
    catch (e) { setConnection("auth failed", "err"); return; }
    await refreshStatus();
    await refreshModules();
    await refreshConsent();
    connectWs();
    setInterval(refreshStatus, 5000);
    setInterval(refreshModules, 10_000);
  }

  main();
})();