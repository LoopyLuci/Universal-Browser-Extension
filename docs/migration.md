# Migrating a WebExtension to DNEA

## Mapping

| WebExtension | DNEA |
|---|---|
| `chrome.storage` | `storage:module` + `dnea::host::storage` |
| `chrome.tabs` | `browser:tabs:*` |
| `chrome.runtime.sendMessage` | Envelope over Native Messaging |
| Permissions in `manifest.json` | Capabilities in `module-manifest.json` |
| `chrome.runtime.onInstalled` | `lifecycle.install` |

## Path

1. Extract logic that is not browser-API interaction.
2. Map permissions to capabilities.
3. Build the thin client (use `thin-client/`).
4. Build the module (use `crates/focus-mode/` as a template).
5. Sign the manifest and distribute the signed module.

## What you gain

- One install works on every browser.
- Capabilities are enforced structurally.
- Modules can do work the browser sandbox forbids.
- Long-running work continues when no browser is open.

## What you lose

- The simplicity of a single `manifest.json`.
- Direct `chrome.*` access from module code.
- Unsigned updates.