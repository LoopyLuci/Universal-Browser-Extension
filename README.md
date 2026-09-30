# DNEA

One install. Every browser. Capability-enforced at the Wasm boundary.

**Specifications:** https://github.com/LoopyLuci/Universal-Browser-Extension/tree/main/docs
**Quickstart:** [docs/quickstart.md](docs/quickstart.md)
**Security:** [docs/security.md](docs/security.md)
**Installers:** [installer/](installer/)

## What it is

DNEA is a device-native extension platform. A single OS-level runtime
owns the state, logic, and long-running work. Browsers connect to the
runtime as thin clients over Native Messaging. Browsers that do not
support extensions use the runtime's companion UI.

## What it is not

- Not a replacement for the WebExtensions API. Thin clients are
  WebExtensions.
- Not a browser vendor requirement. Browsers participate through
  existing primitives (Native Messaging, enterprise policy).
- Not a storefront. Modules are distributed by publishers and installed
  by the runtime under its own signature and consent rules.

## Architecture
