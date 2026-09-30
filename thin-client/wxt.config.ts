import { defineConfig } from "wxt";

export default defineConfig({
  manifestVersion: 3,
  manifest: ({ browser, manifestVersion }) => ({
    name: "DNEA",
    description: "Device-Native Extension Architecture thin client.",
    version: "1.0.0",
    permissions: ["nativeMessaging", "storage", "alarms", "notifications", "scripting"],
    host_permissions: ["http://*/*", "https://*/*"],
    background: manifestVersion === 3
      ? { service_worker: "background.js", type: "module" }
      : { scripts: ["background.js"], type: "module" },
    content_scripts: [{
      matches: ["http://*/*", "https://*/*"],
      js: ["content.js"],
      run_at: "document_start",
      all_frames: true,
      match_about_blank: true,
    }],
    action: { default_title: "DNEA", default_popup: "popup.html" },
    options_ui: { page: "options.html", open_in_tab: true },
    web_accessible_resources: [{
      resources: ["consent.html"],
      matches: ["http://*/*", "https://*/*"],
    }],
    ...(browser === "firefox" ? {
      browser_specific_settings: {
        gecko: { id: "dnea@dnea.org", strict_min_version: "115.0" },
      },
    } : {}),
  }),
  vite: () => ({
    build: { target: "es2022", minify: "esbuild", sourcemap: false },
  }),
});