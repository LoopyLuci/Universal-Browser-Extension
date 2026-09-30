# Security Model

## Threat model

DNEA assumes untrusted modules, untrusted pages, untrusted networks,
and partially-trusted local users.

## Explicit non-goals

- Protecting against a fully compromised OS kernel or hypervisor.
- Protecting against a malicious user who already controls the runtime's account.
- Providing confidentiality against an attacker who can read runtime process memory.

## Invariants

1. The Local API MUST bind exclusively to `127.0.0.1` or `::1`.
2. `api.json` is created with mode `0600` on Unix, or an ACL granting
   access only to the creating user on Windows.
3. JWT secrets are ephemeral; they are never written to disk.
4. Module storage keys are HKDF-derived with info string
   `dnea-store-v1:<module-id>`. Renaming a module yields a different
   key space.
5. Wasm modules are linked only against the interfaces covered by their
   effective grants. Ungranted imports fail instantiation.
6. `capabilities.deny` is enforceable and cannot be overridden by a
   later grant.
7. Consent prompts expire after 300 seconds and are swept every 30
   seconds.
8. Audit logs never contain JWT tokens, master-key material, or
   unredacted consent payloads at `info` level or higher.

## Enforcement points

- **Wasm structural enforcement.** Link-time.
- **Capability intersection.** Effective grant = `(request ∩ policy ∩
  consent) − deny`.
- **Signed modules.** Ed25519 signature verified against a trust store.
- **Sandboxed module UI.** Per-module origin, closed Shadow DOM, strict
  CSP.
- **Loopback-only API.** Bootstrap tokens require reading `api.json`,
  which is user-owned with `0600`.
- **Content-script isolation.** Content scripts never touch Native
  Messaging; they go through the background.

## What is not defended against

- Malicious signed modules. Consent is the human defense.
- Compromised OS accounts.
- Compromised signing keys.
- Supply-chain attacks on a module's build (DNEA verifies signatures,
  not build provenance).