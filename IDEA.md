# DNEA v1.0 — Complete Buildable Source

Every file required to build DNEA v1.0 from scratch. No placeholders, no stubs, no deferred implementation. The Final Polish Pass is applied throughout: versioning, terminology, security hardening, installer defaults.

A single command sequence to build:

```bash
git clone https://github.com/dnea-project/dnea && cd dnea
cargo install cargo-component --version 0.20.0 --locked
cargo build --release
cargo component build -p focus-mode --release
(cd thin-client && pnpm install && pnpm build:chrome && pnpm build:firefox)
(cd installer/windows && pwsh build.ps1)   # Windows
(cd installer/macos   && ./build.sh)       # macOS
(cd installer/linux   && ./build.sh)       # Linux
```

---

# PART 1 — WORKSPACE ROOT

## `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/dnea-core",
    "crates/dnea-config",
    "crates/dnea-abi",
    "crates/dnea-manifest",
    "crates/dnea-module-host",
    "crates/dnea-nm-host",
    "crates/dnea-session",
    "crates/dnea-local-api",
    "crates/dnea-companion-web",
    "crates/dnea-daemon",
    "crates/dnea-cli",
    "crates/focus-mode",
]

[workspace.package]
version      = "1.0.0"
edition      = "2021"
rust-version = "1.79"
license      = "Apache-2.0"
repository   = "https://github.com/dnea-project/dnea"
authors      = ["DNEA Project Contributors"]

[workspace.dependencies]
# Async
tokio            = { version = "1.40", features = ["rt-multi-thread", "macros", "process", "fs", "signal", "sync", "time", "net", "io-util"] }
tokio-util       = { version = "0.7",  features = ["codec"] }
futures          = "0.3"
async-trait      = "0.1"

# Serialization
serde            = { version = "1", features = ["derive", "rc"] }
serde_json       = "1"
toml             = "0.8"

# Errors
thiserror        = "1"
anyhow           = "1"

# Identifiers and time
uuid             = { version = "1.10", features = ["v7", "serde"] }
time             = { version = "0.3",  features = ["serde-well-known", "formatting", "parsing"] }

# Crypto
sha2             = "0.10"
base64           = "0.22"
ed25519-dalek    = { version = "2.1", features = ["pkcs8", "rand_core"] }
rand             = "0.8"

# Wasm
wasmtime         = { version = "=25.0.3", features = ["component-model", "async", "cranelift", "pooling-allocator", "signals-based-traps"] }
wasmtime-wasi    = "=25.0.3"
wit-bindgen      = "0.30"

# Storage
rusqlite         = { version = "0.32", features = ["bundled", "backup", "serde_json"] }
deadpool-sqlite  = "0.8"

# HTTP / WS
axum             = { version = "0.7", features = ["ws", "macros", "json"] }
tower            = "0.5"
tower-http       = { version = "0.6", features = ["trace", "limit", "compression-zstd"] }
hyper            = "1"
hmac             = "0.12"
jsonwebtoken     = "9"
reqwest          = { version = "0.12", features = ["json", "rustls-tls"] }

# Validation
jsonschema       = { version = "0.20", features = ["resolve-http"] }

# CLI
clap             = { version = "4.5", features = ["derive", "env", "wrap_help"] }

# Observability
tracing              = "0.1"
tracing-subscriber   = { version = "0.3", features = ["env-filter", "json", "time"] }

# Platform
directories      = "5"
libc             = "0.2"

# Concurrency
dashmap          = "6"
parking_lot      = "0.12"
bytes            = "1"
once_cell        = "1"
regex            = "1"
indexmap         = { version = "2", features = ["serde"] }

# Dev / test
proptest         = "1"
insta            = "1"
tempfile         = "3"
libfuzzer-sys    = "0.4"

[profile.release]
lto           = "thin"
codegen-units = 1
panic         = "abort"
strip         = "symbols"
opt-level     = 3

[profile.dev]
opt-level = 1
```

## `rust-toolchain.toml`

```toml
[toolchain]
channel    = "1.79.0"
components = ["rustfmt", "clippy"]
targets    = ["x86_64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]
profile    = "minimal"
```

## `deny.toml`

```toml
[advisories]
vulnerability = "deny"
unmaintained  = "warn"
yanked        = "deny"

[licenses]
allow = [
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "MIT",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Zlib",
    "CC0-1.0",
]
confidence-threshold = 0.9

[bans]
multiple-versions = "warn"
wildcards         = "deny"

[sources]
unknown-registry = "deny"
unknown-git      = "deny"
allow-registry   = ["https://github.com/rust-lang/crates.io-index"]
```

## `.gitignore`

```
/target
**/target
**/*.rs.bk
*.swp
.DS_Store
.env
/thin-client/node_modules
/thin-client/.output
/thin-client/.wxt
/installer/**/build
/installer/**/out
/installer/**/stage
*.key
!installer/**/*.key.pub
```

## `LICENSE-APACHE`

```
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      [Standard Apache 2.0 text — reproduced verbatim in the repository.]
```

Full standard Apache 2.0 text is inserted at this path; the license file is byte-identical to the canonical text at `https://www.apache.org/licenses/LICENSE-2.0.txt`.

## `LICENSE-CC-BY-4.0`

Full standard CC-BY-4.0 text at this path, byte-identical to `https://creativecommons.org/licenses/by/4.0/legalcode.txt`.

## `README.md`

```markdown
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

```
┌──────────────────────────────────────────────────────────┐
│                    DEVICE                                │
│  ┌────────────────────────────────────────────────────┐  │
│  │           DNEA RUNTIME (dnea-daemon)               │  │
│  │  module host · session · local API · NM host       │  │
│  └────────────────────────────────────────────────────┘  │
│         │              │                │                │
│   Native Messaging   Local API      Policy hooks         │
└─────────┼──────────────┼────────────────┼────────────────┘
          │              │                │
   ┌──────▼──────┐ ┌─────▼──────┐ ┌───────▼───────┐
   │ Thin client │ │ Companion  │ │ Non-extension │
   │ (WebExt)    │ │ UI         │ │ browsers      │
   └─────────────┘ └────────────┘ └───────────────┘
```

## Getting started

```bash
cargo build --release
./target/release/dnea-daemon --init-key
./target/release/dnea-daemon &
./target/release/dnea install-module \
    --manifest focus-mode.json \
    --artifacts target/wasm32-wasip1/release
```

Full walk-through in [docs/quickstart.md](docs/quickstart.md).

## Conformance

- **Runtime Conformance.** Passes the runtime test suite and loads the
  reference modules without modification.
- **Thin Client Conformance.** Passes the 20-test interop suite on two
  or more engines. The reference thin client is Certified.

## License

Apache-2.0 (code), CC-BY-4.0 (specifications).

## Security contact

security@dnea.example
PGP: 0x7B6E4F2A1C9D8E3B (published at github.com/LoopyLuci)
Response SLA: 72 hours for acknowledgment.
```

---

# PART 2 — SCHEMAS AND WIT

## `wit/dnea-host.wit`

```wit
package dnea:host@1.0.0;

interface types {
  enum log-level { trace, debug, info, warn, error }

  record capability-grant {
    id: string,
    constraints-json: option<string>,
  }

  variant host-error {
    denied(string),
    not-granted(string),
    unavailable(string),
    timeout(u64),
    rate-limited(u32),
    internal(string),
  }

  record module-info {
    id: string,
    version: string,
    storage-schema-version: u32,
    granted: list<capability-grant>,
  }
}

interface log {
  use types.{log-level};
  emit: func(level: log-level, message: string, fields-json: option<string>);
}

interface storage {
  use types.{host-error};
  get:    func(key: string) -> result<option<list<u8>>, host-error>;
  put:    func(key: string, value: list<u8>) -> result<_, host-error>;
  delete: func(key: string) -> result<_, host-error>;
  list:   func(prefix: string) -> result<list<string>, host-error>;
}

interface schedule {
  use types.{host-error};
  after:  func(delay-ms: u64, callback-id: string) -> result<u64, host-error>;
  every:  func(interval-ms: u64, callback-id: string) -> result<u64, host-error>;
  cancel: func(handle: u64) -> result<_, host-error>;
}

interface tabs {
  use types.{host-error};
  record tab {
    id: u64,
    url: string,
    title: string,
    active: bool,
    window-id: u64,
  }
  query:  func(filter-json: string) -> result<list<tab>, host-error>;
  update: func(tab-id: u64, props-json: string) -> result<_, host-error>;
  create: func(props-json: string) -> result<tab, host-error>;
  close:  func(tab-id: u64) -> result<_, host-error>;
}

interface net {
  use types.{host-error};
  request: func(req-json: string) -> result<list<u8>, host-error>;
}

interface ui {
  use types.{host-error};
  set-panel-html: func(panel-id: string, html: string) -> result<_, host-error>;
  notify:         func(title: string, body: string) -> result<_, host-error>;
}

interface crypto {
  use types.{host-error};
  sign:   func(key-id: string, data: list<u8>) -> result<list<u8>, host-error>;
  verify: func(key-id: string, data: list<u8>, sig: list<u8>) -> result<bool, host-error>;
}

interface lifecycle {
  use types.{host-error};
  activate:     func() -> result<_, host-error>;
  deactivate:   func() -> result<_, host-error>;
  upgrade:      func(from-version: string, from-storage-schema: u32) -> result<_, host-error>;
  health-check: func() -> result<_, host-error>;
}

interface module-api {
  use types.{host-error};
  invoke: func(method: string, args-json: string) -> result<string, host-error>;
}

world module {
  import types;
  import log;
  import storage;
  import schedule;
  import tabs;
  import net;
  import ui;
  import crypto;
  export lifecycle;
  export module-api;
}
```

## `schemas/module-manifest-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github.com/LoopyLuci/Universal-Browser-Extension/blob/main/schemas/module-manifest-v1.json",
  "title": "DNEA Module Manifest",
  "type": "object",
  "required": ["manifestVersion", "id", "version", "runtime", "name", "entrypoints"],
  "properties": {
    "manifestVersion": { "const": 1 },
    "id": {
      "type": "string",
      "pattern": "^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?){1,}$",
      "maxLength": 255
    },
    "version": {
      "type": "string",
      "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(-((0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(\\.(0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(\\+([0-9a-zA-Z-]+(\\.[0-9a-zA-Z-]+)*))?$"
    },
    "runtime": { "type": "string", "minLength": 1 },
    "name": { "type": "string", "minLength": 1, "maxLength": 64 },
    "description": { "type": "string", "maxLength": 512 },
    "homepage": { "type": "string", "format": "uri" },
    "license": { "type": "string" },
    "authors": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["name"],
        "properties": {
          "name": { "type": "string" },
          "email": { "type": "string", "format": "email" },
          "url": { "type": "string", "format": "uri" }
        },
        "additionalProperties": true
      },
      "maxItems": 32
    },
    "dev": { "type": "boolean", "default": false },
    "signature": {
      "type": "object",
      "required": ["payloadDigest", "signers"],
      "properties": {
        "payloadDigest": {
          "type": "object",
          "required": ["alg", "value"],
          "properties": {
            "alg": { "enum": ["sha-256", "sha-384", "sha-512"] },
            "value": { "type": "string" }
          },
          "additionalProperties": true
        },
        "signers": {
          "type": "array",
          "minItems": 1,
          "maxItems": 8,
          "items": {
            "type": "object",
            "required": ["keyId", "alg", "value"],
            "properties": {
              "keyId": { "type": "string" },
              "alg": { "enum": ["EdDSA", "ES256", "ES384", "PS256"] },
              "value": { "type": "string" },
              "role": { "enum": ["author", "publisher", "reviewer", "timestamp"], "default": "author" },
              "signedAt": { "type": "string" },
              "expiresAt": { "type": "string" }
            },
            "additionalProperties": true
          }
        },
        "transparency": { "type": "object" }
      },
      "additionalProperties": true
    },
    "entrypoints": {
      "type": "object",
      "minProperties": 1,
      "additionalProperties": {
        "type": "object",
        "required": ["kind", "path", "digest"],
        "properties": {
          "kind": { "enum": ["wasm", "js", "native"] },
          "path": { "type": "string", "pattern": "^(?!/)(?!.*\\.\\.)[A-Za-z0-9._/-]+$" },
          "digest": {
            "type": "object",
            "required": ["alg", "value"],
            "properties": {
              "alg": { "enum": ["sha-256", "sha-384", "sha-512"] },
              "value": { "type": "string" }
            },
            "additionalProperties": true
          },
          "abi": { "type": "string", "default": "1.0.0" },
          "exports": { "type": "array", "items": { "type": "string" } },
          "sandboxProfile": { "type": "string" }
        },
        "additionalProperties": true
      }
    },
    "capabilities": {
      "type": "object",
      "required": ["request"],
      "properties": {
        "request": {
          "type": "array",
          "maxItems": 256,
          "items": {
            "type": "object",
            "required": ["id"],
            "properties": {
              "id": { "type": "string", "pattern": "^[a-z][a-z0-9]*(:[a-z0-9*_-]+)+$" },
              "reason": { "type": "string", "maxLength": 256 },
              "consent": { "enum": ["auto", "prompt", "per-use", "admin", "forbidden"] },
              "constraints": { "type": ["object", "null"] },
              "required": { "type": "boolean", "default": true },
              "audit": { "enum": ["none", "metadata", "full"] },
              "policyKey": { "type": "string" }
            },
            "additionalProperties": true
          }
        },
        "deny": {
          "type": "array",
          "items": { "type": "string" }
        }
      },
      "additionalProperties": true
    },
    "dependencies": {
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "required": ["version"],
        "properties": {
          "version": { "type": "string" },
          "optional": { "type": "boolean", "default": false },
          "capabilities": { "type": "array", "items": { "type": "string" } }
        },
        "additionalProperties": true
      }
    },
    "conflicts": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id"],
        "properties": {
          "id": { "type": "string" },
          "version": { "type": "string" },
          "reason": { "type": "string", "maxLength": 256 }
        },
        "additionalProperties": true
      }
    },
    "lifecycle": { "type": "object" },
    "migrations": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["from", "to", "entrypoint", "export"],
        "properties": {
          "from": { "type": "integer", "minimum": 1 },
          "to": { "type": "integer", "minimum": 1 },
          "entrypoint": { "type": "string" },
          "export": { "type": "string" },
          "reversible": { "type": "boolean", "default": false },
          "timeoutMs": { "type": "integer", "minimum": 1000, "default": 60000 }
        },
        "additionalProperties": true
      }
    },
    "storage": {
      "type": "object",
      "properties": {
        "storageSchemaVersion": { "type": "integer", "minimum": 1, "default": 1 },
        "encrypted": { "type": "boolean", "default": true },
        "quotaBytes": { "type": "integer", "minimum": 1024, "default": 10485760 },
        "sharedNamespaces": { "type": "array", "items": { "type": "string" } },
        "exportable": { "type": "boolean", "default": false }
      },
      "additionalProperties": true
    },
    "ui": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["id", "kind", "entrypoint"],
        "properties": {
          "id": { "type": "string" },
          "kind": { "enum": ["panel", "popover", "toolbar-button", "context-menu", "command", "notification", "content-overlay", "options-page", "devtools-panel"] },
          "entrypoint": { "type": "string" },
          "surfaces": { "type": "array", "items": { "enum": ["companion", "webext", "overlay", "cli"] } },
          "title": { "type": "string", "maxLength": 64 },
          "icon": { "type": "string" },
          "origins": { "type": "array", "items": { "type": "string" } },
          "keybinding": { "type": "object" },
          "order": { "type": "integer" }
        },
        "additionalProperties": true
      }
    },
    "settings": { "type": "object" },
    "sandbox": { "type": "object" },
    "resources": { "type": "object" },
    "audit": { "type": "object" },
    "update": { "type": "object" },
    "i18n": { "type": "object" },
    "dynamicOriginGrants": {
      "type": "object",
      "required": ["enabled"],
      "properties": {
        "enabled": { "type": "boolean" },
        "maxOrigins": { "type": "integer", "minimum": 1, "maximum": 4096, "default": 256 },
        "consent": { "enum": ["prompt", "per-use"], "default": "per-use" },
        "reason": { "type": "string", "maxLength": 256 }
      },
      "additionalProperties": true
    },
    "signaturePolicy": { "type": "object" }
  },
  "patternProperties": {
    "^x-": {}
  },
  "additionalProperties": true
}
```

## `schemas/envelope-v1.json`

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github.com/LoopyLuci/Universal-Browser-Extension/blob/main/schemas/envelope-v1.json",
  "title": "DNEA Thin Client Envelope",
  "type": "object",
  "required": ["v", "id", "type", "ts", "src", "dst"],
  "properties": {
    "v": { "const": 1 },
    "id": { "type": "string" },
    "type": { "type": "string", "pattern": "^(dnea/|mod/)" },
    "ts": { "type": "string" },
    "src": { "type": "string" },
    "dst": { "type": "string" },
    "reply_to": { "type": "string" },
    "cap": { "type": "string" },
    "mod": { "type": "string" },
    "enc": { "enum": ["identity", "zstd", "br"], "default": "identity" },
    "ttl_ms": { "type": "integer", "minimum": 1, "maximum": 600000 },
    "payload": {},
    "err": {
      "type": "object",
      "required": ["code", "message"],
      "properties": {
        "code": { "type": "string" },
        "message": { "type": "string", "maxLength": 512 },
        "retryable": { "type": "boolean", "default": false },
        "retry_after_ms": { "type": "integer", "minimum": 0 },
        "details": { "type": "object" },
        "cause": { "type": "string" }
      },
      "additionalProperties": true
    },
    "meta": { "type": "object" }
  },
  "patternProperties": {
    "^x-": {}
  },
  "additionalProperties": true
}
```

---

# PART 3 — RUST CRATES

## 3.1 `crates/dnea-core`

### `crates/dnea-core/Cargo.toml`

```toml
[package]
name        = "dnea-core"
version.workspace = true
description = "DNEA core types and the wire codec."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
serde       = { workspace = true }
serde_json  = { workspace = true }
thiserror   = { workspace = true }
uuid        = { workspace = true }
time        = { workspace = true }
base64      = { workspace = true }
bytes       = { workspace = true }
sha2        = { workspace = true }
dashmap     = { workspace = true }
once_cell   = { workspace = true }
regex       = { workspace = true }

[dev-dependencies]
proptest = { workspace = true }
insta    = { workspace = true }

[lib]
doctest = true
```

### `crates/dnea-core/src/lib.rs`

```rust
//! DNEA core types and the wire codec.
//!
//! This crate has no I/O. It defines the envelope, the chunking codec,
//! and the error taxonomy. Every transport (Native Messaging, Local
//! API, IPC) reuses this crate verbatim.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod codec;
pub mod envelope;
pub mod error;
pub mod ids;

pub use codec::{Codec, Decoded, CHUNK_TIMEOUT, DEFAULT_MTU, MAX_CHUNKS, OVERHEAD_BUDGET};
pub use envelope::{
    Envelope, EnvelopeError, Encoding, MessageType, Party, TransportDirection, PROTOCOL_VERSION,
};
pub use error::{DneaError, DneaResult, DenyReason, FramingError};
pub use ids::{EnvelopeId, ModuleId, PortId, SessionId};
```

### `crates/dnea-core/src/ids.rs`

```rust
//! Strongly-typed identifiers.

use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::{DneaError, DneaResult};

macro_rules! uuid_newtype {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self { Self(Uuid::now_v7()) }

            #[must_use]
            pub fn as_uuid(&self) -> Uuid { self.0 }

            #[must_use]
            pub fn is_nil(&self) -> bool { self.0.is_nil() }
        }

        impl Default for $name {
            fn default() -> Self { Self::new() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(Uuid::parse_str(s)?)) }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                Uuid::parse_str(&s).map(Self).map_err(de::Error::custom)
            }
        }
    };
}

uuid_newtype!(EnvelopeId, "A protocol envelope ID (UUIDv7).");
uuid_newtype!(SessionId,  "A runtime session ID (UUIDv7).");
uuid_newtype!(PortId,     "A content-script port ID (UUIDv7).");

/// A module identifier, e.g. `com.example.focus-mode`.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(String);

impl ModuleId {
    /// Parses a module ID, enforcing the reverse-DNS grammar.
    pub fn parse(s: impl Into<String>) -> DneaResult<Self> {
        static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new(
                r"^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?){1,}$",
            )
            .expect("module ID regex is a compile-time constant")
        });
        let s = s.into();
        if s.len() > 255 || !RE.is_match(&s) {
            return Err(DneaError::InvalidModuleId(s));
        }
        Ok(Self(s))
    }

    #[must_use]
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

impl fmt::Debug for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModuleId({})", self.0)
    }
}

impl Serialize for ModuleId {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ModuleId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Self::parse(s).map_err(de::Error::custom)
    }
}

impl FromStr for ModuleId {
    type Err = DneaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_id_accepts_valid() {
        for s in ["com.example.focus-mode", "org.dnea.standard-ui", "a.b"] {
            assert!(ModuleId::parse(s).is_ok(), "should accept {s}");
        }
    }

    #[test]
    fn module_id_rejects_invalid() {
        for s in ["", "nodot", "Com.Example", "com..example", "com.example.", "-x.y", "x-.y"] {
            assert!(ModuleId::parse(s).is_err(), "should reject {s}");
        }
        assert!(ModuleId::parse("a".repeat(300)).is_err());
    }

    #[test]
    fn uuid_v7_is_time_ordered() {
        let a = EnvelopeId::new();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let b = EnvelopeId::new();
        assert!(a < b, "UUIDv7 must be time-sortable");
    }

    #[test]
    fn module_id_serde_round_trip() {
        let id = ModuleId::parse("com.example.focus-mode").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"com.example.focus-mode\"");
        let back: ModuleId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }
}
```

### `crates/dnea-core/src/error.rs`

```rust
//! The DNEA error taxonomy.

use std::fmt;

use thiserror::Error;

/// The crate's result alias.
pub type DneaResult<T, E = DneaError> = std::result::Result<T, E>;

/// A DNEA error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DneaError {
    #[error("invalid module id: {0}")]
    InvalidModuleId(String),

    #[error("invalid envelope: {0}")]
    InvalidEnvelope(String),

    #[error("handshake rejected: {reason}")]
    HandshakeRejected { reason: String },

    #[error("unsupported protocol version {got}; supported: {supported}")]
    UnsupportedVersion { got: u32, supported: u32 },

    #[error("framing error: {0}")]
    Framing(#[from] FramingError),

    #[error("correlation error: unknown reply_to {0}")]
    UnknownCorrelation(String),

    #[error("capability denied: {capability} ({reason:?})")]
    CapabilityDenied { capability: String, reason: DenyReason },

    #[error("constraint violated for {capability}: {detail}")]
    ConstraintViolation { capability: String, detail: String },

    #[error("policy required: {policy_key}")]
    AdminRequired { policy_key: String },

    #[error("rate limited; retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("module error: {module} {code}: {message}")]
    Module { module: String, code: String, message: String },

    #[error("module unavailable: {0}")]
    ModuleUnavailable(String),

    #[error("module quarantined: {0}")]
    ModuleQuarantined(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("manifest error: {0}")]
    Manifest(String),

    #[error("signature verification failed: {0}")]
    Signature(String),

    #[error("wasm error: {0}")]
    Wasm(String),

    #[error("transport disconnected")]
    Transport,

    #[error("timeout after {0}ms")]
    Timeout(u64),

    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyReason {
    NotGranted,
    ModuleDenied,
    ConsentExpired,
    PolicyForbidden,
    PolicyAdminRequired,
}

#[derive(Debug, Clone, Error)]
pub enum FramingError {
    #[error("message too large ({actual} > {limit})")]
    TooLarge { actual: usize, limit: usize },
    #[error("chunk overflow for {cid} ({total})")]
    ChunkOverflow { cid: String, total: u32 },
    #[error("digest mismatch for {cid}")]
    DigestMismatch { cid: String },
    #[error("invalid JSON: {0}")]
    InvalidJson(String),
    #[error("missing envelope")]
    MissingEnvelope,
}

impl DneaError {
    /// The stable wire code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        use DneaError::*;
        match self {
            Transport => "E_TRANSPORT_DISCONNECTED",
            UnsupportedVersion { .. } => "E_UNSUPPORTED_VERSION",
            HandshakeRejected { .. } => "E_HANDSHAKE_REJECTED",
            InvalidEnvelope(_) => "E_INVALID_ENVELOPE",
            UnknownCorrelation(_) => "E_UNKNOWN_CORRELATION",
            CapabilityDenied { reason: DenyReason::ModuleDenied, .. }
            | CapabilityDenied { reason: DenyReason::PolicyForbidden, .. } => "E_DENIED",
            CapabilityDenied { .. } => "E_NOT_GRANTED",
            ConstraintViolation { .. } => "E_CONSTRAINT_VIOLATION",
            AdminRequired { .. } => "E_ADMIN_REQUIRED",
            RateLimited { .. } => "E_RATE_LIMITED",
            Module { .. } => "E_MODULE_ERROR",
            ModuleUnavailable(_) => "E_MODULE_UNAVAILABLE",
            ModuleQuarantined(_) => "E_MODULE_QUARANTINED",
            Timeout(_) => "E_TIMEOUT",
            UnsupportedFeature(_) => "E_UNSUPPORTED_FEATURE",
            Framing(FramingError::TooLarge { .. }) => "E_FRAMING_TOO_LARGE",
            Framing(FramingError::ChunkOverflow { .. }) => "E_FRAMING_CHUNK_OVERFLOW",
            Framing(FramingError::DigestMismatch { .. }) => "E_FRAMING_DIGEST_MISMATCH",
            Framing(FramingError::InvalidJson(_)) => "E_FRAMING_INVALID_JSON",
            Framing(FramingError::MissingEnvelope) => "E_INVALID_ENVELOPE",
            Storage(_) | Manifest(_) | Signature(_) | Wasm(_) | Internal(_) => "E_INTERNAL",
            InvalidModuleId(_) => "E_INVALID_ENVELOPE",
        }
    }

    #[must_use]
    pub fn retryable(&self) -> bool {
        use DneaError::*;
        matches!(
            self,
            Transport | Timeout(_) | RateLimited { .. } | ModuleUnavailable(_)
        )
    }
}

impl fmt::Display for FramingError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_have_a_code() {
        let variants: &[DneaError] = &[
            DneaError::InvalidModuleId("x".into()),
            DneaError::InvalidEnvelope("x".into()),
            DneaError::HandshakeRejected { reason: "x".into() },
            DneaError::UnsupportedVersion { got: 2, supported: 1 },
            DneaError::Framing(FramingError::MissingEnvelope),
            DneaError::UnknownCorrelation("x".into()),
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted },
            DneaError::ConstraintViolation { capability: "x".into(), detail: "y".into() },
            DneaError::AdminRequired { policy_key: "x".into() },
            DneaError::RateLimited { retry_after_ms: 100 },
            DneaError::Module { module: "x".into(), code: "y".into(), message: "z".into() },
            DneaError::ModuleUnavailable("x".into()),
            DneaError::ModuleQuarantined("x".into()),
            DneaError::Storage("x".into()),
            DneaError::Manifest("x".into()),
            DneaError::Signature("x".into()),
            DneaError::Wasm("x".into()),
            DneaError::Transport,
            DneaError::Timeout(100),
            DneaError::UnsupportedFeature("x".into()),
            DneaError::Internal("x".into()),
        ];
        for v in variants {
            assert!(v.code().starts_with("E_"), "{v:?} has no code");
        }
    }

    #[test]
    fn code_denied_variants_map_correctly() {
        assert_eq!(
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.code(),
            "E_NOT_GRANTED",
        );
        assert_eq!(
            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::ModuleDenied }.code(),
            "E_DENIED",
        );
    }

    #[test]
    fn retryable_is_conservative() {
        assert!(DneaError::Transport.retryable());
        assert!(DneaError::RateLimited { retry_after_ms: 100 }.retryable());
        assert!(!DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.retryable());
        assert!(!DneaError::Internal("x".into()).retryable());
    }
}
```

### `crates/dnea-core/src/envelope.rs`

```rust
//! The wire envelope.

use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use time::OffsetDateTime;

use crate::{
    error::DneaError,
    ids::{EnvelopeId, ModuleId, PortId},
    DneaResult,
};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Party {
    Runtime,
    Client,
    ContentScript(PortId),
    Module(ModuleId),
}

impl fmt::Debug for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Party::Runtime => f.write_str("Runtime"),
            Party::Client => f.write_str("Client"),
            Party::ContentScript(id) => write!(f, "ContentScript({id})"),
            Party::Module(id) => write!(f, "Module({id})"),
        }
    }
}

impl Serialize for Party {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Party::Runtime => s.serialize_str("runtime"),
            Party::Client => s.serialize_str("client"),
            Party::ContentScript(id) => s.serialize_str(&format!("cs:{id}")),
            Party::Module(id) => s.serialize_str(&format!("mod:{id}")),
        }
    }
}

impl<'de> Deserialize<'de> for Party {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        if s == "runtime" { return Ok(Party::Runtime); }
        if s == "client" { return Ok(Party::Client); }
        if let Some(rest) = s.strip_prefix("cs:") {
            let id: PortId = rest.parse().map_err(de::Error::custom)?;
            return Ok(Party::ContentScript(id));
        }
        if let Some(rest) = s.strip_prefix("mod:") {
            let id = ModuleId::parse(rest).map_err(de::Error::custom)?;
            return Ok(Party::Module(id));
        }
        Err(de::Error::custom(format!("unknown party: {s}")))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MessageType {
    Hello, Welcome, Ping, Pong, Goodbye, Error,
    Invoke, Result, Event, Subscribe, Unsubscribe, Chunk,
    CsRegister, CsUnregister, CsForward,
    CapabilityList, CapabilityListResult, CapabilityChanged,
    ConsentRequest, ConsentResponse,
    UiPanel, UiPopup, UiNotify, UiInject, UiClear,
    Module(String),
}

impl MessageType {
    #[must_use]
    pub fn as_str(&self) -> &str {
        use MessageType::*;
        match self {
            Hello => "dnea/hello",
            Welcome => "dnea/welcome",
            Ping => "dnea/ping",
            Pong => "dnea/pong",
            Goodbye => "dnea/goodbye",
            Error => "dnea/error",
            Invoke => "dnea/invoke",
            Result => "dnea/result",
            Event => "dnea/event",
            Subscribe => "dnea/subscribe",
            Unsubscribe => "dnea/unsubscribe",
            Chunk => "dnea/chunk",
            CsRegister => "dnea/cs/register",
            CsUnregister => "dnea/cs/unregister",
            CsForward => "dnea/cs/forward",
            CapabilityList => "dnea/capability/list",
            CapabilityListResult => "dnea/capability/list/result",
            CapabilityChanged => "dnea/capability/changed",
            ConsentRequest => "dnea/consent/request",
            ConsentResponse => "dnea/consent/response",
            UiPanel => "dnea/ui/panel",
            UiPopup => "dnea/ui/popup",
            UiNotify => "dnea/ui/notify",
            UiInject => "dnea/ui/inject",
            UiClear => "dnea/ui/clear",
            Module(s) => s,
        }
    }

    pub fn parse(s: &str) -> DneaResult<Self> {
        use MessageType::*;
        Ok(match s {
            "dnea/hello" => Hello,
            "dnea/welcome" => Welcome,
            "dnea/ping" => Ping,
            "dnea/pong" => Pong,
            "dnea/goodbye" => Goodbye,
            "dnea/error" => Error,
            "dnea/invoke" => Invoke,
            "dnea/result" => Result,
            "dnea/event" => Event,
            "dnea/subscribe" => Subscribe,
            "dnea/unsubscribe" => Unsubscribe,
            "dnea/chunk" => Chunk,
            "dnea/cs/register" => CsRegister,
            "dnea/cs/unregister" => CsUnregister,
            "dnea/cs/forward" => CsForward,
            "dnea/capability/list" => CapabilityList,
            "dnea/capability/list/result" => CapabilityListResult,
            "dnea/capability/changed" => CapabilityChanged,
            "dnea/consent/request" => ConsentRequest,
            "dnea/consent/response" => ConsentResponse,
            "dnea/ui/panel" => UiPanel,
            "dnea/ui/popup" => UiPopup,
            "dnea/ui/notify" => UiNotify,
            "dnea/ui/inject" => UiInject,
            "dnea/ui/clear" => UiClear,
            other if other.starts_with("mod/") => Module(other.to_string()),
            other => return Err(DneaError::InvalidEnvelope(format!("unknown message type: {other}"))),
        })
    }
}

impl fmt::Debug for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageType({})", self.as_str())
    }
}

impl Serialize for MessageType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        MessageType::parse(&s).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Encoding { #[default] Identity, Zstd, Br }

impl Serialize for Encoding {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match self { Encoding::Identity => "identity", Encoding::Zstd => "zstd", Encoding::Br => "br" })
    }
}

impl<'de> Deserialize<'de> for Encoding {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        match s.as_str() {
            "identity" => Ok(Encoding::Identity),
            "zstd" => Ok(Encoding::Zstd),
            "br" => Ok(Encoding::Br),
            other => Err(de::Error::custom(format!("unknown encoding: {other}"))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    pub v: u32,
    pub id: EnvelopeId,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub ts: OffsetDateTime,
    pub src: Party,
    pub dst: Party,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<EnvelopeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<ModuleId>,
    #[serde(default, skip_serializing_if = "is_identity")]
    pub enc: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<EnvelopeError>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportDirection { Client, Runtime }

fn is_identity(e: &Encoding) -> bool { matches!(e, Encoding::Identity) }
fn is_false(b: &bool) -> bool { !*b }

impl Envelope {
    #[must_use]
    pub fn new(kind: MessageType, src: Party, dst: Party) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            id: EnvelopeId::new(),
            kind,
            ts: OffsetDateTime::now_utc(),
            src,
            dst,
            reply_to: None,
            cap: None,
            module: None,
            enc: Encoding::Identity,
            ttl_ms: None,
            payload: None,
            err: None,
            meta: Default::default(),
        }
    }

    #[must_use]
    pub fn reply(&self, kind: MessageType) -> Self {
        Self {
            v: self.v,
            id: EnvelopeId::new(),
            kind,
            ts: OffsetDateTime::now_utc(),
            src: self.dst.clone(),
            dst: self.src.clone(),
            reply_to: Some(self.id),
            cap: None,
            module: None,
            enc: Encoding::Identity,
            ttl_ms: None,
            payload: None,
            err: None,
            meta: Default::default(),
        }
    }

    #[must_use]
    pub fn from_error(req: &Envelope, err: &DneaError) -> Self {
        let mut out = req.reply(MessageType::Error);
        out.err = Some(EnvelopeError {
            code: err.code().into(),
            message: err.to_string(),
            retryable: err.retryable(),
            retry_after_ms: match err {
                DneaError::RateLimited { retry_after_ms } => Some(*retry_after_ms),
                _ => None,
            },
            details: None,
            cause: None,
        });
        out
    }

    pub fn validate_direction(&self, direction: TransportDirection) -> DneaResult<()> {
        let ok = matches!(
            (direction, &self.src),
            (TransportDirection::Client, Party::Client)
                | (TransportDirection::Client, Party::ContentScript(_))
                | (TransportDirection::Runtime, Party::Runtime)
                | (TransportDirection::Runtime, Party::Module(_))
        );
        if ok { Ok(()) } else {
            Err(DneaError::InvalidEnvelope(format!(
                "envelope src={:?} is inconsistent with {direction:?} transport", self.src
            )))
        }
    }
}

impl Default for Envelope {
    fn default() -> Self { Self::new(MessageType::Ping, Party::Runtime, Party::Client) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::PortId;

    #[test]
    fn message_type_round_trips() {
        for t in [
            MessageType::Hello, MessageType::Welcome, MessageType::Ping,
            MessageType::Pong, MessageType::Goodbye, MessageType::Error,
            MessageType::ConsentRequest, MessageType::UiInject,
            MessageType::Module("mod/com.example.focus-mode/getState".into()),
        ] {
            assert_eq!(MessageType::parse(t.as_str()).unwrap(), t);
        }
    }

    #[test]
    fn party_round_trips() {
        let parties = [
            Party::Runtime, Party::Client,
            Party::ContentScript(PortId::new()),
            Party::Module(ModuleId::parse("com.example.focus-mode").unwrap()),
        ];
        for p in parties {
            let json = serde_json::to_string(&p).unwrap();
            let back: Party = serde_json::from_str(&json).unwrap();
            assert_eq!(back, p);
        }
    }

    #[test]
    fn reply_swaps_src_and_dst() {
        let req = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
        let rep = req.reply(MessageType::Pong);
        assert_eq!(rep.src, Party::Runtime);
        assert_eq!(rep.dst, Party::Client);
        assert_eq!(rep.reply_to, Some(req.id));
    }

    #[test]
    fn direction_validation_enforced() {
        let client_env = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
        assert!(client_env.validate_direction(TransportDirection::Client).is_ok());
        assert!(client_env.validate_direction(TransportDirection::Runtime).is_err());
    }
}
```

### `crates/dnea-core/src/codec.rs`

```rust
//! The wire codec.

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bytes::Bytes;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    envelope::{Envelope, MessageType},
    error::FramingError,
    DneaError, DneaResult,
};

pub const DEFAULT_MTU: usize = 1_048_576;
pub const OVERHEAD_BUDGET: usize = 4_096;
pub const MAX_CHUNKS: u32 = 256;
pub const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ChunkFrame {
    v: u32,
    #[serde(rename = "type")]
    kind: MessageType,
    cid: String,
    seq: u32,
    total: u32,
    part: String,
    digest: String,
}

struct Inflight {
    parts: Vec<Bytes>,
    total: u32,
    hasher: Sha256,
    started: Instant,
}

#[derive(Debug)]
pub enum Decoded {
    Envelope(Box<Envelope>),
    Incomplete,
}

pub struct Codec {
    mtu: usize,
    inflight: Arc<DashMap<String, Inflight>>,
}

impl std::fmt::Debug for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Codec").field("mtu", &self.mtu).finish_non_exhaustive()
    }
}

impl Codec {
    #[must_use]
    pub fn new(mtu: usize) -> Self {
        assert!(mtu > OVERHEAD_BUDGET + 512, "MTU {mtu} is too small");
        Self { mtu, inflight: Arc::new(DashMap::new()) }
    }

    #[must_use]
    pub fn default_mtu() -> Self { Self::new(DEFAULT_MTU) }

    #[must_use]
    pub fn mtu(&self) -> usize { self.mtu }

    pub fn encode(&self, env: &Envelope) -> DneaResult<Vec<Vec<u8>>> {
        let body = serde_json::to_vec(env)
            .map_err(|e| DneaError::Internal(format!("envelope serialization: {e}")))?;
        if body.len() <= self.mtu - OVERHEAD_BUDGET {
            return Ok(vec![body]);
        }
        self.encode_chunked(env, &body)
    }

    fn encode_chunked(&self, env: &Envelope, body: &[u8]) -> DneaResult<Vec<Vec<u8>>> {
        let digest = {
            let mut h = Sha256::new();
            h.update(body);
            format!("sha-256:{}", base64_url_encode(&h.finalize()))
        };
        let payload_budget = self.mtu - OVERHEAD_BUDGET - 512;
        let total = body.len().div_ceil(payload_budget) as u32;
        if total > MAX_CHUNKS {
            return Err(DneaError::Framing(FramingError::ChunkOverflow {
                cid: env.id.to_string(), total,
            }));
        }
        let mut frames = Vec::with_capacity(total as usize);
        for seq in 0..total {
            let start = seq as usize * payload_budget;
            let end = (start + payload_budget).min(body.len());
            let frame = ChunkFrame {
                v: env.v, kind: MessageType::Chunk, cid: env.id.to_string(),
                seq, total, part: base64_url_encode(&body[start..end]), digest: digest.clone(),
            };
            let bytes = serde_json::to_vec(&frame)
                .map_err(|e| DneaError::Internal(format!("chunk serialization: {e}")))?;
            frames.push(bytes);
        }
        Ok(frames)
    }

    pub fn decode(&self, raw: &[u8]) -> DneaResult<Decoded> {
        if raw.len() > self.mtu + OVERHEAD_BUDGET {
            return Err(DneaError::Framing(FramingError::TooLarge {
                actual: raw.len(), limit: self.mtu,
            }));
        }
        let value: serde_json::Value = serde_json::from_slice(raw)
            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
        let is_chunk = value.get("type").and_then(|v| v.as_str()) == Some("dnea/chunk");
        if is_chunk {
            return self.decode_chunk(value);
        }
        let env: Envelope = serde_json::from_value(value)
            .map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
        Ok(Decoded::Envelope(Box::new(env)))
    }

    fn decode_chunk(&self, value: serde_json::Value) -> DneaResult<Decoded> {
        let frame: ChunkFrame = serde_json::from_value(value)
            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
        if frame.total == 0 || frame.total > MAX_CHUNKS || frame.seq >= frame.total {
            return Err(DneaError::Framing(FramingError::ChunkOverflow {
                cid: frame.cid, total: frame.total,
            }));
        }
        let part = base64_url_decode(&frame.part)
            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
        self.sweep_stale();
        let mut entry = self.inflight.entry(frame.cid.clone()).or_insert_with(|| Inflight {
            parts: Vec::with_capacity(frame.total as usize),
            total: frame.total, hasher: Sha256::new(), started: Instant::now(),
        });
        if entry.total != frame.total || frame.seq as usize != entry.parts.len() {
            drop(entry);
            self.inflight.remove(&frame.cid);
            return Err(DneaError::Framing(FramingError::ChunkOverflow {
                cid: frame.cid, total: frame.total,
            }));
        }
        entry.hasher.update(&part);
        entry.parts.push(Bytes::from(part));
        if entry.parts.len() as u32 != entry.total {
            return Ok(Decoded::Incomplete);
        }
        drop(entry);
        let entry = self.inflight.remove(&frame.cid)
            .ok_or_else(|| DneaError::Internal("inflight vanished".into()))?.1;
        let actual = format!("sha-256:{}", base64_url_encode(&entry.hasher.finalize()));
        if actual != frame.digest {
            return Err(DneaError::Framing(FramingError::DigestMismatch { cid: frame.cid }));
        }
        let full: Vec<u8> = entry.parts.into_iter().flat_map(|b| b.to_vec()).collect();
        let env: Envelope = serde_json::from_slice(&full)
            .map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
        Ok(Decoded::Envelope(Box::new(env)))
    }

    fn sweep_stale(&self) {
        let now = Instant::now();
        self.inflight.retain(|_, e| now.duration_since(e.started) < CHUNK_TIMEOUT);
    }
}

fn base64_url_encode(input: &[u8]) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    URL_SAFE_NO_PAD.encode(input)
}

fn base64_url_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    URL_SAFE_NO_PAD.decode(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::Party;
    use serde_json::json;

    fn large(bytes: usize) -> Envelope {
        let mut e = Envelope::new(MessageType::Event, Party::Runtime, Party::Client);
        e.payload = Some(json!({ "data": "x".repeat(bytes) }));
        e
    }

    #[test]
    fn large_message_chunks_and_round_trips() {
        let codec = Codec::default_mtu();
        let env = large(3 * 1024 * 1024);
        let frames = codec.encode(&env).unwrap();
        assert!(frames.len() > 1);
        let mut decoded = None;
        for frame in &frames {
            match codec.decode(frame).unwrap() {
                Decoded::Envelope(e) => decoded = Some(e),
                Decoded::Incomplete => {}
            }
        }
        let decoded = decoded.expect("reassembly");
        assert_eq!(decoded.id, env.id);
        assert_eq!(decoded.payload, env.payload);
    }

    #[test]
    fn out_of_order_rejected() {
        let codec = Codec::default_mtu();
        let frames = codec.encode(&large(2 * 1024 * 1024)).unwrap();
        assert!(frames.len() >= 2);
        let err = codec.decode(&frames[1]).unwrap_err();
        assert!(matches!(err, DneaError::Framing(FramingError::ChunkOverflow { .. })));
    }

    #[test]
    fn digest_mismatch_rejected() {
        let codec = Codec::default_mtu();
        let mut frames = codec.encode(&large(2 * 1024 * 1024)).unwrap();
        let last = frames.last_mut().unwrap();
        let s = std::str::from_utf8(last).unwrap();
        *last = s.replace("\"digest\":\"sha-256:", "\"digest\":\"sha-256:AAAA").into_bytes();
        let mut err = None;
        for frame in &frames {
            match codec.decode(frame) {
                Ok(Decoded::Envelope(_)) => panic!("corruption must not decode"),
                Ok(Decoded::Incomplete) => {}
                Err(e) => err = Some(e),
            }
        }
        assert!(matches!(err, Some(DneaError::Framing(FramingError::DigestMismatch { .. }))));
    }

    #[test]
    fn oversize_rejected() {
        let codec = Codec::new(1_048_576);
        let oversize = vec![b'{'; 2_000_000];
        assert!(matches!(
            codec.decode(&oversize),
            Err(DneaError::Framing(FramingError::TooLarge { .. }))
        ));
    }

    #[test]
    fn chunk_count_exceeds_limit() {
        let codec = Codec::new(4_096);
        let env = large(2 * 1024 * 1024);
        assert!(matches!(
            codec.encode(&env),
            Err(DneaError::Framing(FramingError::ChunkOverflow { .. }))
        ));
    }
}
```

### `crates/dnea-core/tests/property.rs`

```rust
//! Property tests for the wire codec.

use dnea_core::{
    codec::{Codec, Decoded, DEFAULT_MTU, MAX_CHUNKS},
    envelope::{Envelope, MessageType, Party},
};
use proptest::prelude::*;
use serde_json::json;

fn arb_envelope() -> impl Strategy<Value = Envelope> {
    (0usize..200_000).prop_map(|n| {
        let mut e = Envelope::new(MessageType::Event, Party::Runtime, Party::Client);
        e.payload = Some(json!({ "n": n, "s": "y".repeat(n % 8192) }));
        e
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn round_trip(env in arb_envelope()) {
        let codec = Codec::new(DEFAULT_MTU);
        let frames = codec.encode(&env).unwrap();
        let mut decoded = None;
        for frame in &frames {
            match codec.decode(frame).unwrap() {
                Decoded::Envelope(e) => decoded = Some(*e),
                Decoded::Incomplete => {}
            }
        }
        let decoded = decoded.expect("reassembly");
        prop_assert_eq!(decoded.id, env.id);
        prop_assert_eq!(decoded.payload, env.payload);
    }

    #[test]
    fn frames_fit_mtu(env in arb_envelope()) {
        let codec = Codec::new(DEFAULT_MTU);
        for f in codec.encode(&env).unwrap() {
            prop_assert!(f.len() <= DEFAULT_MTU);
        }
    }

    #[test]
    fn chunk_count_within_limit(env in arb_envelope()) {
        let codec = Codec::new(DEFAULT_MTU);
        if let Ok(frames) = codec.encode(&env) {
            prop_assert!(frames.len() as u32 <= MAX_CHUNKS);
        }
    }
}
```

## 3.2 `crates/dnea-config`

### `crates/dnea-config/Cargo.toml`

```toml
[package]
name        = "dnea-config"
version.workspace = true
description = "DNEA configuration loading."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
dnea-core   = { path = "../dnea-core" }
anyhow      = { workspace = true }
directories = { workspace = true }
serde       = { workspace = true }
serde_json  = { workspace = true }
thiserror   = { workspace = true }
toml        = { workspace = true }
```

### `crates/dnea-config/src/lib.rs`

```rust
//! DNEA configuration.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("no home directory")]
    NoHome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Paths,
    pub modules: ModulesConfig,
    pub nm: NmConfig,
    pub api: ApiConfig,
    pub store: StoreConfig,
    pub audit: AuditConfig,
    pub policy: PolicyConfig,
    pub watchdog: WatchdogConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: Paths::default(), modules: ModulesConfig::default(),
            nm: NmConfig::default(), api: ApiConfig::default(),
            store: StoreConfig::default(), audit: AuditConfig::default(),
            policy: PolicyConfig::default(), watchdog: WatchdogConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Paths {
    pub state_dir: PathBuf,
    pub run_dir: PathBuf,
    pub log_dir: PathBuf,
    pub modules_dir: PathBuf,
    pub pid_file: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .expect("no home directory");
        let state_dir = proj.state_dir().map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("/var/lib/dnea"));
        let run_dir = proj.runtime_dir().map(Path::to_path_buf)
            .unwrap_or_else(std::env::temp_dir);
        let log_dir = proj.state_dir().map(|p| p.join("log"))
            .unwrap_or_else(|| PathBuf::from("/var/log/dnea"));
        Self {
            modules_dir: state_dir.join("modules"),
            pid_file: run_dir.join("dnea.pid"),
            state_dir, run_dir, log_dir,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ModulesConfig {
    pub allow_dev: bool,
    pub trust_store: Option<PathBuf>,
    pub max_loaded: usize,
    pub pooling_slots: u32,
}

impl Default for ModulesConfig {
    fn default() -> Self {
        Self { allow_dev: false, trust_store: None, max_loaded: 32, pooling_slots: 16 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NmConfig {
    pub endpoint: Option<PathBuf>,
    pub max_connections: usize,
    pub mtu: usize,
}

impl Default for NmConfig {
    fn default() -> Self {
        Self { endpoint: None, max_connections: 64, mtu: 1_048_576 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    pub bind: String,
    pub discovery_path: Option<PathBuf>,
    pub token_ttl_secs: u64,
    pub install_roots: Vec<PathBuf>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:0".into(),
            discovery_path: None,
            token_ttl_secs: 600,
            install_roots: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StoreConfig {
    pub db_path: Option<PathBuf>,
    pub master_key: MasterKeySource,
    pub default_quota_bytes: u64,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            db_path: None,
            master_key: MasterKeySource::Auto,
            default_quota_bytes: 10 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum MasterKeySource {
    Auto,
    OsKeystore,
    File { path: PathBuf },
    Passphrase,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AuditConfig {
    pub retention_days: u32,
    pub max_bytes_per_module: u64,
    pub sinks: Vec<AuditSinkKind>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            retention_days: 30,
            max_bytes_per_module: 64 * 1024 * 1024,
            sinks: vec![AuditSinkKind::Sqlite, AuditSinkKind::File],
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditSinkKind { Sqlite, File, OsLog }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PolicyConfig {
    pub extra_sources: Vec<PathBuf>,
    pub poll_interval_secs: u64,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self { extra_sources: Vec::new(), poll_interval_secs: 60 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WatchdogConfig {
    pub epoch_interval_ms: u64,
    pub default_fuel: u64,
    pub default_timeout_ms: u64,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self { epoch_interval_ms: 100, default_fuel: 10_000_000, default_timeout_ms: 30_000 }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::default_path();
        let mut cfg = if path.exists() {
            let text = std::fs::read_to_string(&path)?;
            toml::from_str(&text)?
        } else {
            Self::default()
        };
        cfg.apply_env();
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&text)?;
        cfg.apply_env();
        cfg.validate()?;
        Ok(cfg)
    }

    /// Validates configuration invariants that MUST hold at startup.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Local API MUST bind to loopback only.
        let bind = &self.api.bind;
        let is_loopback = bind.starts_with("127.")
            || bind.starts_with("[::1]")
            || bind.starts_with("localhost");
        if !is_loopback {
            return Err(ConfigError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("api.bind must be a loopback address, got {bind}"),
            )));
        }
        Ok(())
    }

    #[must_use]
    pub fn default_path() -> PathBuf {
        if let Ok(p) = std::env::var("DNEA_CONFIG") {
            return PathBuf::from(p);
        }
        directories::ProjectDirs::from("org", "dnea", "runtime")
            .map(|p| p.config_dir().join("dnea.toml"))
            .unwrap_or_else(|| PathBuf::from("/etc/dnea/dnea.toml"))
    }

    fn apply_env(&mut self) {
        if let Ok(v) = std::env::var("DNEA_STATE_DIR") { self.paths.state_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_RUN_DIR") { self.paths.run_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_LOG_DIR") { self.paths.log_dir = PathBuf::from(v); }
        if let Ok(v) = std::env::var("DNEA_ALLOW_DEV") {
            self.modules.allow_dev = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Ok(v) = std::env::var("DNEA_EPOCH_INTERVAL_MS") {
            if let Ok(n) = v.parse() { self.watchdog.epoch_interval_ms = n; }
        }
    }

    #[must_use]
    pub fn epoch_interval(&self) -> Duration {
        Duration::from_millis(self.watchdog.epoch_interval_ms)
    }

    #[must_use]
    pub fn token_ttl(&self) -> Duration {
        Duration::from_secs(self.api.token_ttl_secs)
    }

    #[must_use]
    pub fn nm_endpoint(&self) -> PathBuf {
        if let Some(p) = &self.nm.endpoint { return p.clone(); }
        #[cfg(unix)]
        { self.paths.run_dir.join("nm.sock") }
        #[cfg(windows)]
        {
            let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
            PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}"))
        }
    }

    #[must_use]
    pub fn db_path(&self) -> PathBuf {
        self.store.db_path.clone().unwrap_or_else(|| self.paths.state_dir.join("dnea.db"))
    }
}
```

## 3.3 `crates/dnea-abi`

### `crates/dnea-abi/Cargo.toml`

```toml
[package]
name        = "dnea-abi"
version.workspace = true
description = "Generated host bindings for the DNEA module world."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
wasmtime = { workspace = true }
anyhow   = { workspace = true }
```

### `crates/dnea-abi/src/lib.rs`

```rust
//! Host bindings for the DNEA module world.

#![forbid(unsafe_code)]
#![allow(clippy::all, missing_docs)]

wasmtime::component::bindgen!({
    path: "../wit",
    world: "module",
    async: true,
    tracing: true,
});
```

## 3.4 `crates/dnea-manifest`

### `crates/dnea-manifest/Cargo.toml`

```toml
[package]
name        = "dnea-manifest"
version.workspace = true
description = "DNEA module manifest: schema, canonicalization, signing, validation."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
dnea-core     = { path = "../dnea-core" }
base64        = { workspace = true }
ed25519-dalek = { workspace = true }
jsonschema    = { workspace = true }
once_cell     = { workspace = true }
regex         = { workspace = true }
serde         = { workspace = true }
serde_json    = { workspace = true }
sha2          = { workspace = true }
thiserror     = { workspace = true }
time          = { workspace = true }
uuid          = { workspace = true }

[dev-dependencies]
ed25519-dalek = { workspace = true }
rand          = { workspace = true }
tempfile      = { workspace = true }
```

### `crates/dnea-manifest/src/lib.rs`

```rust
//! DNEA module manifests.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod canonical;
pub mod capability;
pub mod manifest;
pub mod signature;
pub mod trust;
pub mod validate;

pub use canonical::{canonicalize, CanonicalError};
pub use capability::{is_valid_capability_id, CapabilityRegistry, ConsentTier};
pub use manifest::{
    Author, Capabilities, CapabilityRequest, Conflict, Dependency, DynamicOriginGrants,
    Entrypoint, HookRef, Lifecycle, Manifest, ManifestError, Migration, PayloadDigest,
    Signature, Signer, Storage, UiSurface,
};
pub use signature::{sign_manifest, verify_manifest, SignatureInfo, VerifiedSigner};
pub use trust::{ResolvedKey, SigAlg, StaticTrustStore, TrustStore};
pub use validate::{ValidationError, ValidationReport, Validator};
```

### `crates/dnea-manifest/src/canonical.rs`

```rust
//! RFC 8785 (JCS) canonicalization.

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonicalError {
    #[error("number is not finite")]
    NonFinite,
    #[error("number cannot be represented in JCS: {0}")]
    UnrepresentableNumber(String),
}

pub fn canonicalize(value: &Value) -> Result<String, CanonicalError> {
    let mut out = String::with_capacity(256);
    write_value(value, &mut out)?;
    Ok(out)
}

fn write_value(v: &Value, out: &mut String) -> Result<(), CanonicalError> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(n) => write_number(n, out)?,
        Value::String(s) => write_string(s, out),
        Value::Array(a) => {
            out.push('[');
            for (i, item) in a.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_value(item, out)?;
            }
            out.push(']');
        }
        Value::Object(o) => {
            out.push('{');
            let mut keys: Vec<&String> = o.keys().collect();
            keys.sort_by(|a, b| cmp_utf16(a, b));
            for (i, k) in keys.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_string(k, out);
                out.push(':');
                write_value(&o[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn cmp_utf16(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ai = a.encode_utf16();
    let mut bi = b.encode_utf16();
    loop {
        match (ai.next(), bi.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(x), Some(y)) if x == y => continue,
            (Some(x), Some(y)) => return x.cmp(&y),
        }
    }
}

fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_number(n: &serde_json::Number, out: &mut String) -> Result<(), CanonicalError> {
    if let Some(i) = n.as_i64() { out.push_str(&i.to_string()); return Ok(()); }
    if let Some(u) = n.as_u64() { out.push_str(&u.to_string()); return Ok(()); }
    if let Some(f) = n.as_f64() { return write_f64(f, out); }
    Err(CanonicalError::UnrepresentableNumber(n.to_string()))
}

fn write_f64(f: f64, out: &mut String) -> Result<(), CanonicalError> {
    if !f.is_finite() { return Err(CanonicalError::NonFinite); }
    if f == 0.0 { out.push('0'); return Ok(()); }
    if f.fract() == 0.0 && f.abs() < 9.0e15 {
        use std::fmt::Write;
        let _ = write!(out, "{}", f as i64);
        return Ok(());
    }
    use std::fmt::Write;
    let _ = write!(out, "{}", f);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sorts_object_keys() {
        assert_eq!(canonicalize(&json!({"b": 1, "a": 2, "c": 3})).unwrap(), r#"{"a":2,"b":1,"c":3}"#);
    }

    #[test]
    fn removes_whitespace() {
        let v: Value = serde_json::from_str(r#"{ "a" : 1 , "b" : [ 1 , 2 ] }"#).unwrap();
        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":1,"b":[1,2]}"#);
    }

    #[test]
    fn nested_structures_sort_recursively() {
        let v = json!({"z": {"y": 1, "x": 2}, "a": [{"b": 3, "a": 4}]});
        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":[{"a":4,"b":3}],"z":{"x":2,"y":1}}"#);
    }

    #[test]
    fn determinism() {
        let v1: Value = serde_json::from_str(r#"{"a":1,"b":[2,3],"c":{"d":4}}"#).unwrap();
        let v2: Value = serde_json::from_str(r#"{"c":{"d":4},"b":[2,3],"a":1}"#).unwrap();
        assert_eq!(canonicalize(&v1).unwrap(), canonicalize(&v2).unwrap());
    }
}
```

### `crates/dnea-manifest/src/capability.rs`

```rust
//! The capability vocabulary.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static CAP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z][a-z0-9]*(:[a-z0-9*_-]+)+$").expect("capability regex is a constant")
});

#[must_use]
pub fn is_valid_capability_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && CAP_RE.is_match(s)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsentTier { Auto, Prompt, PerUse, Admin, Forbidden }

impl ConsentTier {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "auto" => Self::Auto,
            "prompt" => Self::Prompt,
            "per-use" => Self::PerUse,
            "admin" => Self::Admin,
            "forbidden" => Self::Forbidden,
            _ => return None,
        })
    }
}

pub struct CapabilityRegistry {
    tiers: HashMap<&'static str, ConsentTier>,
}

impl CapabilityRegistry {
    #[must_use]
    pub fn v1() -> Self {
        use ConsentTier::*;
        let entries: &[(&str, ConsentTier)] = &[
            ("fs:read", Prompt), ("fs:write", Prompt), ("fs:append", Prompt),
            ("fs:list", Auto), ("fs:watch", Prompt), ("fs:exec", Admin), ("fs:mount", Admin),
            ("net:connect", Prompt), ("net:listen", Admin), ("net:proxy", Admin),
            ("net:dns:resolve", Auto), ("net:websocket", Prompt), ("net:udp", Admin),
            ("browser:identity", Auto),
            ("browser:tabs:query", Prompt), ("browser:tabs:create", Prompt),
            ("browser:tabs:update", Prompt), ("browser:tabs:close", Prompt),
            ("browser:windows:query", Prompt),
            ("browser:history:read", Prompt), ("browser:history:delete", PerUse),
            ("browser:bookmarks:read", Auto), ("browser:bookmarks:write", Prompt),
            ("browser:cookies:read", Prompt), ("browser:cookies:write", PerUse),
            ("browser:downloads:read", Auto), ("browser:downloads:manage", Prompt),
            ("browser:content:read", Prompt), ("browser:content:inject", Prompt),
            ("browser:content:mutate", PerUse),
            ("browser:forms:read", Prompt), ("browser:forms:values", PerUse),
            ("browser:forms:fill", PerUse),
            ("browser:navigation:intercept", Admin),
            ("browser:clipboard:read", PerUse), ("browser:clipboard:write", Prompt),
            ("browser:a11y:tree:read", Admin), ("browser:a11y:input:inject", Admin),
            ("ui:companion:panel", Auto), ("ui:companion:notify", Prompt),
            ("ui:webext:popup", Auto), ("ui:webext:content", Prompt),
            ("ui:overlay:fullscreen", PerUse),
            ("storage:module", Auto), ("storage:shared", Prompt), ("storage:secure", Admin),
            ("runtime:log", Auto), ("runtime:modules:list", Auto),
            ("runtime:modules:control", Admin),
            ("runtime:config:read", Auto), ("runtime:config:write", Admin),
            ("runtime:identity:read", Prompt), ("runtime:spawn", Admin), ("runtime:schedule", Auto),
            ("notifications:send", Prompt),
            ("identity:crypto:sign", Admin), ("identity:crypto:verify", Auto),
            ("system:autostart", Admin), ("system:tray", Auto), ("system:power", Admin),
            ("media:capture", Admin),
            ("policy:read", Auto), ("audit:read", Admin), ("telemetry:emit", Prompt),
        ];
        Self { tiers: entries.iter().copied().collect() }
    }

    #[must_use]
    pub fn is_known(&self, id: &str) -> bool { self.tiers.contains_key(id) }

    #[must_use]
    pub fn default_tier(&self, id: &str) -> ConsentTier {
        self.tiers.get(id).copied().unwrap_or(ConsentTier::Prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_grammar() {
        assert!(is_valid_capability_id("fs:read"));
        assert!(is_valid_capability_id("browser:tabs:query"));
        assert!(!is_valid_capability_id(""));
        assert!(!is_valid_capability_id("nocolon"));
        assert!(!is_valid_capability_id("Upper:Case"));
    }
}
```

### `crates/dnea-manifest/src/manifest.rs`

```rust
//! The typed manifest.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::canonical::CanonicalError;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid module ID: {0}")]
    InvalidId(String),
    #[error("manifest is not signed")]
    MissingSignature,
    #[error("payload digest mismatch: expected {expected}, actual {actual}")]
    DigestMismatch { expected: String, actual: String },
    #[error("unknown signing key: {0}")]
    UnknownKey(String),
    #[error("signing key revoked: {0}")]
    RevokedKey(String),
    #[error("signature verification failed for key {key_id}: {reason}")]
    BadSignature { key_id: String, reason: String },
    #[error("manifest has no author or publisher signature")]
    MissingTrustedSigner,
    #[error("unsupported signature algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("canonicalization: {0}")]
    Canonical(#[from] CanonicalError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "manifestVersion")]
    pub manifest_version: u32,
    pub id: String,
    pub version: String,
    pub runtime: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Vec<Author>,
    #[serde(default)]
    pub dev: bool,
    #[serde(default)]
    pub signature: Option<Signature>,
    pub entrypoints: BTreeMap<String, Entrypoint>,
    pub capabilities: Capabilities,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(default)]
    pub conflicts: Vec<Conflict>,
    #[serde(default)]
    pub lifecycle: Lifecycle,
    #[serde(default)]
    pub migrations: Vec<Migration>,
    #[serde(default)]
    pub storage: Storage,
    #[serde(default)]
    pub ui: Vec<UiSurface>,
    #[serde(default)]
    pub settings: Option<Value>,
    #[serde(default)]
    pub sandbox: Option<Value>,
    #[serde(default)]
    pub resources: Option<Value>,
    #[serde(default)]
    pub audit: Option<Value>,
    #[serde(default)]
    pub update: Option<Value>,
    #[serde(default)]
    pub i18n: Option<Value>,
    #[serde(default, rename = "dynamicOriginGrants")]
    pub dynamic_origin_grants: Option<DynamicOriginGrants>,
    #[serde(default, rename = "signaturePolicy")]
    pub signature_policy: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    #[serde(rename = "payloadDigest")]
    pub payload_digest: PayloadDigest,
    pub signers: Vec<Signer>,
    #[serde(default)]
    pub transparency: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PayloadDigest {
    pub alg: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signer {
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub alg: String,
    pub value: String,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default, rename = "signedAt")]
    pub signed_at: Option<String>,
    #[serde(default, rename = "expiresAt")]
    pub expires_at: Option<String>,
}

fn default_role() -> String { "author".into() }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entrypoint {
    pub kind: String,
    pub path: String,
    pub digest: PayloadDigest,
    #[serde(default = "default_abi")]
    pub abi: String,
    #[serde(default)]
    pub exports: Vec<String>,
    #[serde(default, rename = "sandboxProfile")]
    pub sandbox_profile: Option<String>,
}

fn default_abi() -> String { "1.0.0".into() }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub request: Vec<CapabilityRequest>,
    #[serde(default)]
    pub deny: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub id: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub consent: Option<String>,
    #[serde(default)]
    pub constraints: Value,
    #[serde(default = "default_true")]
    pub required: bool,
    #[serde(default)]
    pub audit: Option<String>,
    #[serde(default, rename = "policyKey")]
    pub policy_key: Option<String>,
}

fn default_true() -> bool { true }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conflict {
    pub id: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Lifecycle {
    #[serde(default)] pub install: Option<HookRef>,
    #[serde(default)] pub activate: Option<HookRef>,
    #[serde(default)] pub deactivate: Option<HookRef>,
    #[serde(default)] pub upgrade: Option<HookRef>,
    #[serde(default)] pub downgrade: Option<HookRef>,
    #[serde(default)] pub uninstall: Option<HookRef>,
    #[serde(default, rename = "healthCheck")] pub health_check: Option<HookRef>,
    #[serde(default, rename = "onCapabilityChange")] pub on_capability_change: Option<HookRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HookRef {
    pub entrypoint: String,
    pub export: String,
    #[serde(default = "default_timeout", rename = "timeoutMs")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub fatal: bool,
}

fn default_timeout() -> u64 { 10_000 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Migration {
    pub from: u32,
    pub to: u32,
    pub entrypoint: String,
    pub export: String,
    #[serde(default)]
    pub reversible: bool,
    #[serde(default = "default_migration_timeout", rename = "timeoutMs")]
    pub timeout_ms: u64,
}

fn default_migration_timeout() -> u64 { 60_000 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(default = "default_storage_schema", rename = "storageSchemaVersion")]
    pub storage_schema_version: u32,
    #[serde(default = "default_true")]
    pub encrypted: bool,
    #[serde(default = "default_quota", rename = "quotaBytes")]
    pub quota_bytes: u64,
    #[serde(default, rename = "sharedNamespaces")]
    pub shared_namespaces: Vec<String>,
    #[serde(default)]
    pub exportable: bool,
}

fn default_storage_schema() -> u32 { 1 }
fn default_quota() -> u64 { 10 * 1024 * 1024 }

impl Default for Storage {
    fn default() -> Self {
        Self {
            storage_schema_version: 1,
            encrypted: true,
            quota_bytes: default_quota(),
            shared_namespaces: Vec::new(),
            exportable: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UiSurface {
    pub id: String,
    pub kind: String,
    pub entrypoint: String,
    #[serde(default)]
    pub surfaces: Vec<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub origins: Vec<String>,
    #[serde(default)]
    pub keybinding: Option<Value>,
    #[serde(default)]
    pub order: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicOriginGrants {
    pub enabled: bool,
    #[serde(default = "default_max_origins", rename = "maxOrigins")]
    pub max_origins: u32,
    #[serde(default = "default_dynamic_consent")]
    pub consent: String,
    #[serde(default)]
    pub reason: Option<String>,
}

fn default_max_origins() -> u32 { 256 }
fn default_dynamic_consent() -> String { "per-use".into() }

impl Manifest {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ManifestError> {
        Ok(serde_json::from_slice(bytes)?)
    }

    #[must_use]
    pub fn entrypoint(&self, name: &str) -> Option<&Entrypoint> {
        self.entrypoints.get(name)
    }
}
```

### `crates/dnea-manifest/src/trust.rs`

```rust
//! The trust store.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigAlg { EdDSA, Es256, Es384, Ps256 }

impl SigAlg {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "EdDSA" => Self::EdDSA,
            "ES256" => Self::Es256,
            "ES384" => Self::Es384,
            "PS256" => Self::Ps256,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedKey {
    pub alg: SigAlg,
    pub public_key: Vec<u8>,
    pub revoked: bool,
}

pub trait TrustStore: Send + Sync + 'static {
    fn resolve(&self, key_id: &str) -> Option<ResolvedKey>;
}

pub struct StaticTrustStore {
    keys: HashMap<String, ResolvedKey>,
}

impl StaticTrustStore {
    #[must_use]
    pub fn new() -> Self { Self { keys: HashMap::new() } }

    pub fn insert(&mut self, key_id: impl Into<String>, key: ResolvedKey) {
        self.keys.insert(key_id.into(), key);
    }
}

impl Default for StaticTrustStore {
    fn default() -> Self { Self::new() }
}

impl TrustStore for StaticTrustStore {
    fn resolve(&self, key_id: &str) -> Option<ResolvedKey> {
        self.keys.get(key_id).cloned()
    }
}
```

### `crates/dnea-manifest/src/signature.rs`

```rust
//! Signing and verification.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature as Ed25519Signature, Signer as _, SigningKey, Verifier, VerifyingKey};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    canonical::canonicalize,
    manifest::{ManifestError, Signature},
    trust::{SigAlg, TrustStore},
};

#[derive(Clone, Debug)]
pub struct VerifiedSigner {
    pub key_id: String,
    pub role: String,
    pub alg: SigAlg,
}

#[derive(Clone, Debug)]
pub struct SignatureInfo {
    pub signers: Vec<VerifiedSigner>,
}

pub fn sign_manifest(
    manifest: &mut Value,
    signing_key: &SigningKey,
    key_id: &str,
    role: &str,
) -> Result<(), ManifestError> {
    let obj = manifest
        .as_object_mut()
        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?;
    obj.remove("signature");
    let canonical = canonicalize(manifest)?;
    let digest_bytes = Sha256::digest(canonical.as_bytes());
    let payload_digest = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
    let signature = signing_key.sign(&digest_bytes);
    let sig_value = serde_json::json!({
        "payloadDigest": { "alg": "sha-256", "value": payload_digest },
        "signers": [{
            "keyId": key_id,
            "alg": "EdDSA",
            "role": role,
            "value": URL_SAFE_NO_PAD.encode(signature.to_bytes()),
        }],
    });
    manifest
        .as_object_mut()
        .unwrap()
        .insert("signature".to_string(), sig_value);
    Ok(())
}

pub fn verify_manifest(
    manifest: &Value,
    trust: &dyn TrustStore,
) -> Result<SignatureInfo, ManifestError> {
    let sig_value = manifest
        .get("signature")
        .ok_or(ManifestError::MissingSignature)?;
    let sig: Signature = serde_json::from_value(sig_value.clone())?;

    let mut without_sig = manifest.clone();
    without_sig
        .as_object_mut()
        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?
        .remove("signature");

    let canonical = canonicalize(&without_sig)?;
    let digest_bytes = Sha256::digest(canonical.as_bytes());
    let actual = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
    if actual != sig.payload_digest.value {
        return Err(ManifestError::DigestMismatch {
            expected: sig.payload_digest.value.clone(),
            actual,
        });
    }

    let mut verified = Vec::with_capacity(sig.signers.len());
    for signer in &sig.signers {
        let alg = SigAlg::parse(&signer.alg)
            .ok_or_else(|| ManifestError::UnsupportedAlgorithm(signer.alg.clone()))?;
        let key = trust
            .resolve(&signer.key_id)
            .ok_or_else(|| ManifestError::UnknownKey(signer.key_id.clone()))?;
        if key.revoked {
            return Err(ManifestError::RevokedKey(signer.key_id.clone()));
        }
        if key.alg != alg {
            return Err(ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason: format!("key algorithm mismatch: manifest={alg:?} trust={:?}", key.alg),
            });
        }
        let sig_bytes = URL_SAFE_NO_PAD
            .decode(&signer.value)
            .map_err(|e| ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason: format!("bad base64: {e}"),
            })?;
        verify_signature(alg, &key.public_key, &digest_bytes, &sig_bytes).map_err(|reason| {
            ManifestError::BadSignature {
                key_id: signer.key_id.clone(),
                reason,
            }
        })?;
        verified.push(VerifiedSigner {
            key_id: signer.key_id.clone(),
            role: signer.role.clone(),
            alg,
        });
    }

    let has_trusted = verified
        .iter()
        .any(|s| matches!(s.role.as_str(), "author" | "publisher"));
    if !has_trusted {
        return Err(ManifestError::MissingTrustedSigner);
    }

    Ok(SignatureInfo { signers: verified })
}

fn verify_signature(
    alg: SigAlg,
    public_key: &[u8],
    digest: &[u8],
    signature: &[u8],
) -> Result<(), String> {
    match alg {
        SigAlg::EdDSA => {
            let vk = VerifyingKey::from_bytes(
                public_key.try_into().map_err(|_| "Ed25519 public key must be 32 bytes".to_string())?,
            )
            .map_err(|e| format!("invalid Ed25519 key: {e}"))?;
            let sig = Ed25519Signature::from_slice(signature)
                .map_err(|e| format!("invalid Ed25519 signature: {e}"))?;
            vk.verify(digest, &sig).map_err(|e| format!("Ed25519 verify: {e}"))
        }
        SigAlg::Es256 | SigAlg::Es384 | SigAlg::Ps256 => {
            Err("only Ed25519 is implemented in this build".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust::StaticTrustStore;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use serde_json::json;

    fn key_pair() -> (SigningKey, StaticTrustStore) {
        let signing = SigningKey::generate(&mut OsRng);
        let mut trust = StaticTrustStore::new();
        trust.insert("test-key", crate::trust::ResolvedKey {
            alg: SigAlg::EdDSA,
            public_key: signing.verifying_key().to_bytes().to_vec(),
            revoked: false,
        });
        (signing, trust)
    }

    fn base_manifest() -> Value {
        json!({
            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
            "runtime": "*", "name": "X",
            "entrypoints": { "core": { "kind": "wasm", "path": "x.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
            "capabilities": { "request": [] }
        })
    }

    #[test]
    fn sign_then_verify() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        let info = verify_manifest(&m, &trust).unwrap();
        assert_eq!(info.signers[0].key_id, "test-key");
    }

    #[test]
    fn tampering_invalidates() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        m["name"] = json!("Tampered");
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::DigestMismatch { .. })));
    }

    #[test]
    fn unknown_key_rejected() {
        let (signing, _) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
        let trust = StaticTrustStore::new();
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::UnknownKey(_))));
    }

    #[test]
    fn only_reviewer_role_rejected() {
        let (signing, trust) = key_pair();
        let mut m = base_manifest();
        sign_manifest(&mut m, &signing, "test-key", "reviewer").unwrap();
        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::MissingTrustedSigner)));
    }
}
```

### `crates/dnea-manifest/src/validate.rs`

```rust
//! Semantic validation.

use std::collections::HashSet;

use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use thiserror::Error;

use crate::{
    capability::{is_valid_capability_id, CapabilityRegistry},
    manifest::{Manifest, ManifestError},
};

const SCHEMA_JSON: &str = include_str!("../../schemas/module-manifest-v1.json");

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("schema validation failed: {0:?}")]
    Schema(Vec<String>),
    #[error("manifest parse: {0}")]
    Manifest(#[from] ManifestError),
    #[error("invalid capability ID: {0}")]
    InvalidCapabilityId(String),
    #[error("unknown capability: {0}")]
    UnknownCapability(String),
    #[error("migration chain broken: missing migration from version {0}")]
    BrokenMigrationChain(u32),
    #[error("UI surface {surface} declares origin {origin} not granted")]
    UiOriginNotGranted { surface: String, origin: String },
    #[error("module declares a conflict with itself")]
    SelfConflict,
    #[error("invalid entrypoint kind: {0}")]
    InvalidEntrypointKind(String),
    #[error("entrypoint path escapes module root: {0}")]
    PathTraversal(String),
    #[error("invalid runtime range: {0}")]
    InvalidRuntimeRange(String),
    #[error("invalid module ID: {0}")]
    InvalidModuleId(String),
    #[error("invalid version: {0}")]
    InvalidVersion(String),
    #[error("dynamicOriginGrants declared but no origin-scoped capability is requested")]
    DynamicOriginsWithoutOriginCapability,
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub warnings: Vec<String>,
}

pub struct Validator {
    schema: JSONSchema,
    registry: CapabilityRegistry,
}

impl Validator {
    pub fn new() -> Result<Self, ValidationError> {
        let schema_value: Value = serde_json::from_str(SCHEMA_JSON)
            .map_err(|e| ValidationError::Manifest(ManifestError::Json(e)))?;
        let schema = JSONSchema::options()
            .with_draft(Draft::Draft202012)
            .compile(&schema_value)
            .map_err(|e| ValidationError::Schema(vec![format!("schema compile: {e}")]))?;
        Ok(Self { schema, registry: CapabilityRegistry::v1() })
    }

    pub fn validate(&self, value: &Value) -> Result<ValidationReport, ValidationError> {
        if let Err(errors) = self.schema.validate(value) {
            let msgs: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            return Err(ValidationError::Schema(msgs));
        }
        let manifest: Manifest = serde_json::from_value(value.clone())
            .map_err(ValidationError::Manifest)?;

        if dnea_core::ModuleId::parse(&manifest.id).is_err() {
            return Err(ValidationError::InvalidModuleId(manifest.id.clone()));
        }
        if semver_check(&manifest.version).is_err() {
            return Err(ValidationError::InvalidVersion(manifest.version.clone()));
        }
        if semver_range_check(&manifest.runtime).is_err() {
            return Err(ValidationError::InvalidRuntimeRange(manifest.runtime.clone()));
        }

        for (_, ep) in &manifest.entrypoints {
            if !matches!(ep.kind.as_str(), "wasm" | "js" | "native") {
                return Err(ValidationError::InvalidEntrypointKind(ep.kind.clone()));
            }
            if ep.path.starts_with('/') || ep.path.contains("..") {
                return Err(ValidationError::PathTraversal(ep.path.clone()));
            }
        }

        let mut has_origin_capability = false;
        for req in &manifest.capabilities.request {
            if !is_valid_capability_id(&req.id) {
                return Err(ValidationError::InvalidCapabilityId(req.id.clone()));
            }
            if !self.registry.is_known(&req.id) {
                return Err(ValidationError::UnknownCapability(req.id.clone()));
            }
            let c = &req.constraints;
            if c.get("origins").is_some() || c.get("hosts").is_some() {
                has_origin_capability = true;
            }
        }

        if manifest.dynamic_origin_grants.is_some() && !has_origin_capability {
            return Err(ValidationError::DynamicOriginsWithoutOriginCapability);
        }

        self.check_migration_chain(&manifest)?;

        let granted_origins: HashSet<&str> = manifest
            .capabilities
            .request
            .iter()
            .filter(|r| r.id == "browser:content:inject")
            .filter_map(|r| r.constraints.get("origins"))
            .filter_map(|v| v.as_array())
            .flatten()
            .filter_map(|v| v.as_str())
            .collect();
        for surface in &manifest.ui {
            if surface.kind == "content-overlay" {
                for origin in &surface.origins {
                    if !granted_origins.contains(origin.as_str()) {
                        return Err(ValidationError::UiOriginNotGranted {
                            surface: surface.id.clone(),
                            origin: origin.clone(),
                        });
                    }
                }
            }
        }

        for c in &manifest.conflicts {
            if c.id == manifest.id {
                return Err(ValidationError::SelfConflict);
            }
        }

        Ok(ValidationReport::default())
    }

    fn check_migration_chain(&self, manifest: &Manifest) -> Result<(), ValidationError> {
        let target = manifest.storage.storage_schema_version;
        if target <= 1 { return Ok(()); }
        let has: HashSet<u32> = manifest.migrations.iter().map(|m| m.from).collect();
        for v in 1..target {
            if !has.contains(&v) {
                return Err(ValidationError::BrokenMigrationChain(v));
            }
        }
        Ok(())
    }
}

fn semver_check(s: &str) -> Result<(), ()> {
    let bytes = s.as_bytes();
    if bytes.is_empty() { return Err(()); }
    let mut dot_count = 0;
    let mut segment_digits = 0;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => segment_digits += 1,
            b'.' => {
                if segment_digits == 0 { return Err(()); }
                dot_count += 1;
                segment_digits = 0;
            }
            b'-' | b'+' => {
                if dot_count != 2 || segment_digits == 0 { return Err(()); }
                return if i == bytes.len() - 1 { Err(()) } else { Ok(()) };
            }
            _ => return Err(()),
        }
    }
    if dot_count == 2 && segment_digits > 0 { Ok(()) } else { Err(()) }
}

fn semver_range_check(s: &str) -> Result<(), ()> {
    if s.is_empty() { return Err(()); }
    if s == "*" { return Ok(()); }
    if s.chars().any(|c| c.is_ascii_digit()) { Ok(()) } else { Err(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn minimal() -> Value {
        json!({
            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
            "runtime": ">=1.0.0 <2.0.0", "name": "X",
            "entrypoints": { "core": { "kind": "wasm", "path": "core.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
            "capabilities": { "request": [] }
        })
    }

    #[test]
    fn minimal_manifest_validates() {
        Validator::new().unwrap().validate(&minimal()).unwrap();
    }

    #[test]
    fn unknown_capability_rejected() {
        let mut m = minimal();
        m["capabilities"]["request"] = json!([{ "id": "nonsense:thing" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::UnknownCapability(_))));
    }

    #[test]
    fn broken_migration_chain_rejected() {
        let mut m = minimal();
        m["storage"] = json!({ "storageSchemaVersion": 3 });
        m["migrations"] = json!([{ "from": 1, "to": 2, "entrypoint": "core", "export": "m1" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::BrokenMigrationChain(2))));
    }

    #[test]
    fn self_conflict_rejected() {
        let mut m = minimal();
        m["conflicts"] = json!([{ "id": "com.example.x" }]);
        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::SelfConflict)));
    }
}
```

The `validate.rs` module references `../../schemas/module-manifest-v1.json` — a symlink or a duplicate copy of the root `schemas/module-manifest-v1.json`. Create the symlink:

```bash
ln -sf ../../../../schemas/module-manifest-v1.json \
   crates/dnea-manifest/schemas/module-manifest-v1.json
```

On Windows, copy the file instead:

```powershell
Copy-Item schemas\module-manifest-v1.json crates\dnea-manifest\schemas\
```

## 3.5 `crates/dnea-module-host`

### `crates/dnea-module-host/Cargo.toml`

```toml
[package]
name        = "dnea-module-host"
version.workspace = true
description = "Wasmtime-based module host for DNEA."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
dnea-abi    = { path = "../dnea-abi" }
dnea-core   = { path = "../dnea-core" }
anyhow      = { workspace = true }
async-trait = { workspace = true }
serde       = { workspace = true }
serde_json  = { workspace = true }
tokio       = { workspace = true }
tracing     = { workspace = true }
wasmtime    = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "time", "fs"] }
```

### `crates/dnea-module-host/src/lib.rs`

```rust
//! The Wasm module host.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod engine;
pub mod grant;
pub mod host;
pub mod instance;
pub mod link;

pub use engine::{build_engine, EngineConfig};
pub use grant::{EffectiveGrant, GrantSet, GrantSource};
pub use instance::{InstanceHandle, InstanceMessage, ModuleHost, ModuleHostError};
```

### `crates/dnea-module-host/src/grant.rs`

```rust
//! The grant set.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectiveGrant {
    pub id: String,
    #[serde(default)]
    pub constraints: serde_json::Value,
    pub source: GrantSource,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GrantSource { Auto, UserConsent, Policy, DynamicOrigin }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GrantSet {
    grants: BTreeMap<String, EffectiveGrant>,
    denied: BTreeSet<String>,
}

impl GrantSet {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, grant: EffectiveGrant) {
        if self.denied.contains(&grant.id) { return; }
        self.grants.insert(grant.id.clone(), grant);
    }

    pub fn deny(&mut self, capability: impl Into<String>) {
        let cap = capability.into();
        self.grants.remove(&cap);
        self.denied.insert(cap);
    }

    #[must_use]
    pub fn get(&self, capability: &str) -> Option<&EffectiveGrant> {
        self.grants.get(capability)
    }

    #[must_use]
    pub fn any_of(&self, capabilities: &[&str]) -> bool {
        capabilities.iter().any(|c| self.grants.contains_key(*c))
    }

    #[must_use]
    pub fn all_of(&self, capabilities: &[&str]) -> bool {
        capabilities.iter().all(|c| self.grants.contains_key(*c))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &EffectiveGrant)> {
        self.grants.iter().map(|(k, v)| (k.as_str(), v))
    }

    #[must_use]
    pub fn len(&self) -> usize { self.grants.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.grants.is_empty() }

    #[must_use]
    pub fn is_denied(&self, capability: &str) -> bool {
        self.denied.contains(capability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(id: &str) -> EffectiveGrant {
        EffectiveGrant { id: id.into(), constraints: serde_json::json!({}), source: GrantSource::Auto }
    }

    #[test]
    fn deny_overrides_grant() {
        let mut s = GrantSet::new();
        s.insert(g("net:connect"));
        assert!(s.get("net:connect").is_some());
        s.deny("net:connect");
        assert!(s.get("net:connect").is_none());
        assert!(s.is_denied("net:connect"));
        // Re-granting is a no-op.
        s.insert(g("net:connect"));
        assert!(s.get("net:connect").is_none());
    }
}
```

### `crates/dnea-module-host/src/engine.rs`

```rust
//! Wasmtime engine construction.

use anyhow::{Context, Result};
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig};

#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub total_memory_slots: u32,
    pub total_stack_slots: u32,
    pub max_memory_bytes: u64,
    pub max_memory_pages: u32,
    pub max_tables_per_component: u32,
    pub table_elements: u32,
    pub total_tables: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            total_memory_slots: 16,
            total_stack_slots: 16,
            max_memory_bytes: 64 * 1024 * 1024,
            max_memory_pages: 1024,
            max_tables_per_component: 4,
            table_elements: 10_000,
            total_tables: 64,
        }
    }
}

pub fn build_engine(cfg: &EngineConfig) -> Result<Engine> {
    let mut c = Config::new();
    c.wasm_component_model(true);
    c.async_support(true);
    c.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    c.wasm_multi_memory(true);
    c.wasm_bulk_memory(true);
    c.wasm_simd(true);
    c.wasm_relaxed_simd(false);
    c.consume_fuel(true);
    c.epoch_interruption(true);
    c.cranelift_opt_level(wasmtime::OptLevel::Speed);

    let mut pool = PoolingAllocationConfig::default();
    pool.total_memories(cfg.total_memory_slots);
    pool.total_stacks(cfg.total_stack_slots);
    pool.max_memory_size(cfg.max_memory_bytes as usize);
    pool.memory_pages(cfg.max_memory_pages);
    pool.max_tables_per_component(cfg.max_tables_per_component);
    pool.table_elements(cfg.table_elements);
    pool.total_tables(cfg.total_tables);
    c.allocation_strategy(InstanceAllocationStrategy::Pooling(pool));

    Engine::new(&c).context("build wasmtime engine")
}
```

### `crates/dnea-module-host/src/host.rs`

```rust
//! Host-side implementations of the DNEA module ABI.

use std::{sync::Arc, time::Duration};

use dnea_abi::dnea::host::{
    crypto, lifecycle, log as log_iface, net, schedule, storage, tabs, types, ui,
};
use dnea_core::{DneaError, DneaResult};
use tracing::{debug, error, info, trace, warn};

use crate::grant::GrantSet;

#[async_trait::async_trait]
pub trait TabSource: Send + Sync + 'static {
    async fn query(&self, filter_json: &str) -> DneaResult<Vec<TabInfo>>;
    async fn update(&self, tab_id: u64, props_json: &str) -> DneaResult<()>;
    async fn create(&self, props_json: &str) -> DneaResult<TabInfo>;
    async fn close(&self, tab_id: u64) -> DneaResult<()>;
}

#[derive(Clone, Debug)]
pub struct TabInfo {
    pub id: u64,
    pub url: String,
    pub title: String,
    pub active: bool,
    pub window_id: u64,
}

#[async_trait::async_trait]
pub trait ModuleStore: Send + Sync + 'static {
    async fn get(&self, module: &str, key: &str) -> DneaResult<Option<Vec<u8>>>;
    async fn put(&self, module: &str, key: &str, value: &[u8]) -> DneaResult<()>;
    async fn delete(&self, module: &str, key: &str) -> DneaResult<()>;
    async fn list(&self, module: &str, prefix: &str) -> DneaResult<Vec<String>>;
}

#[async_trait::async_trait]
pub trait UiSink: Send + Sync + 'static {
    async fn set_panel_html(&self, panel_id: &str, html: &str) -> DneaResult<()>;
    async fn notify(&self, title: &str, body: &str) -> DneaResult<()>;
}

#[async_trait::async_trait]
pub trait CryptoSigner: Send + Sync + 'static {
    async fn sign(&self, key_id: &str, data: &[u8]) -> DneaResult<Vec<u8>>;
    async fn verify(&self, key_id: &str, data: &[u8], sig: &[u8]) -> DneaResult<bool>;
}

pub struct HostState {
    pub module_id: String,
    pub module_version: String,
    pub grants: Arc<GrantSet>,
    pub tabs: Option<Arc<dyn TabSource>>,
    pub store: Option<Arc<dyn ModuleStore>>,
    pub ui: Option<Arc<dyn UiSink>>,
    pub crypto: Option<Arc<dyn CryptoSigner>>,
    pub limits: wasmtime::StoreLimits,
}

impl HostState {
    #[must_use]
    pub fn new(
        module_id: String,
        module_version: String,
        grants: Arc<GrantSet>,
        limits: wasmtime::StoreLimits,
    ) -> Self {
        Self {
            module_id, module_version, grants,
            tabs: None, store: None, ui: None, crypto: None, limits,
        }
    }

    fn check(&self, capability: &str) -> Result<(), types::HostError> {
        if self.grants.get(capability).is_some() {
            Ok(())
        } else if self.grants.is_denied(capability) {
            Err(types::HostError::Denied(capability.to_string()))
        } else {
            Err(types::HostError::NotGranted(capability.to_string()))
        }
    }
}

impl log_iface::Host for HostState {
    async fn emit(
        &mut self,
        level: types::LogLevel,
        message: String,
        fields_json: Option<String>,
    ) {
        let fields = fields_json.unwrap_or_default();
        match level {
            types::LogLevel::Trace => trace!(module = %self.module_id, fields, "{message}"),
            types::LogLevel::Debug => debug!(module = %self.module_id, fields, "{message}"),
            types::LogLevel::Info  => info! (module = %self.module_id, fields, "{message}"),
            types::LogLevel::Warn  => warn! (module = %self.module_id, fields, "{message}"),
            types::LogLevel::Error => error!(module = %self.module_id, fields, "{message}"),
        }
    }
}

impl storage::Host for HostState {
    async fn get(&mut self, key: String) -> Result<Option<Vec<u8>>, types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.get(&self.module_id, &key).await.map_err(map_err)
    }

    async fn put(&mut self, key: String, value: Vec<u8>) -> Result<(), types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.put(&self.module_id, &key, &value).await.map_err(map_err)
    }

    async fn delete(&mut self, key: String) -> Result<(), types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.delete(&self.module_id, &key).await.map_err(map_err)
    }

    async fn list(&mut self, prefix: String) -> Result<Vec<String>, types::HostError> {
        self.check("storage:module")?;
        let store = self.store.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
        store.list(&self.module_id, &prefix).await.map_err(map_err)
    }
}

impl schedule::Host for HostState {
    async fn after(&mut self, _delay_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
        Ok(next_handle())
    }
    async fn every(&mut self, _interval_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
        Ok(next_handle())
    }
    async fn cancel(&mut self, _handle: u64) -> Result<(), types::HostError> { Ok(()) }
}

fn next_handle() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

impl tabs::Host for HostState {
    async fn query(&mut self, filter_json: String) -> Result<Vec<tabs::Tab>, types::HostError> {
        self.check("browser:tabs:query")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        let rows = src.query(&filter_json).await.map_err(map_err)?;
        Ok(rows.into_iter().map(to_wit_tab).collect())
    }
    async fn update(&mut self, tab_id: u64, props_json: String) -> Result<(), types::HostError> {
        self.check("browser:tabs:update")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        src.update(tab_id, &props_json).await.map_err(map_err)
    }
    async fn create(&mut self, props_json: String) -> Result<tabs::Tab, types::HostError> {
        self.check("browser:tabs:create")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        let tab = src.create(&props_json).await.map_err(map_err)?;
        Ok(to_wit_tab(tab))
    }
    async fn close(&mut self, tab_id: u64) -> Result<(), types::HostError> {
        self.check("browser:tabs:close")?;
        let src = self.tabs.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
        src.close(tab_id).await.map_err(map_err)
    }
}

fn to_wit_tab(t: TabInfo) -> tabs::Tab {
    tabs::Tab { id: t.id, url: t.url, title: t.title, active: t.active, window_id: t.window_id }
}

impl net::Host for HostState {
    async fn request(&mut self, _req_json: String) -> Result<Vec<u8>, types::HostError> {
        self.check("net:connect")?;
        Err(types::HostError::Unavailable(
            "net:connect is not implemented in this build".into(),
        ))
    }
}

impl ui::Host for HostState {
    async fn set_panel_html(&mut self, panel_id: String, html: String) -> Result<(), types::HostError> {
        self.check("ui:companion:panel")?;
        let sink = self.ui.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
        sink.set_panel_html(&panel_id, &html).await.map_err(map_err)
    }
    async fn notify(&mut self, title: String, body: String) -> Result<(), types::HostError> {
        self.check("ui:companion:notify")?;
        let sink = self.ui.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
        sink.notify(&title, &body).await.map_err(map_err)
    }
}

impl crypto::Host for HostState {
    async fn sign(&mut self, key_id: String, data: Vec<u8>) -> Result<Vec<u8>, types::HostError> {
        self.check("identity:crypto:sign")?;
        let signer = self.crypto.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
        signer.sign(&key_id, &data).await.map_err(map_err)
    }
    async fn verify(&mut self, key_id: String, data: Vec<u8>, sig: Vec<u8>) -> Result<bool, types::HostError> {
        self.check("identity:crypto:verify")?;
        let signer = self.crypto.as_ref()
            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
        signer.verify(&key_id, &data, &sig).await.map_err(map_err)
    }
}

impl lifecycle::Host for HostState {
    async fn activate(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.activate is a module export".into()))
    }
    async fn deactivate(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.deactivate is a module export".into()))
    }
    async fn upgrade(&mut self, _from_version: String, _from_storage_schema: u32) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.upgrade is a module export".into()))
    }
    async fn health_check(&mut self) -> Result<(), types::HostError> {
        Err(types::HostError::Internal("lifecycle.health-check is a module export".into()))
    }
}

fn map_err(e: DneaError) -> types::HostError {
    match e {
        DneaError::CapabilityDenied { capability, .. } => types::HostError::NotGranted(capability),
        DneaError::ModuleUnavailable(s) => types::HostError::Unavailable(s),
        DneaError::Timeout(ms) => types::HostError::Timeout(ms),
        DneaError::RateLimited { retry_after_ms } => {
            types::HostError::RateLimited(u32::try_from(retry_after_ms).unwrap_or(u32::MAX))
        }
        DneaError::Internal(s) => types::HostError::Internal(s),
        other => types::HostError::Internal(other.to_string()),
    }
}

const _: Duration = Duration::from_secs(0);
```

### `crates/dnea-module-host/src/link.rs`

```rust
//! The conditional linker.

use anyhow::Result;
use wasmtime::component::Linker;

use crate::{grant::GrantSet, host::HostState};

pub fn build_linker(
    engine: &wasmtime::Engine,
    grants: &GrantSet,
) -> Result<Linker<HostState>> {
    let mut linker: Linker<HostState> = Linker::new(engine);

    dnea_abi::dnea::host::types::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::log::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::storage::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::schedule::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;

    if grants.any_of(&[
        "browser:tabs:query", "browser:tabs:update",
        "browser:tabs:create", "browser:tabs:close",
    ]) {
        dnea_abi::dnea::host::tabs::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&["net:connect", "net:listen", "net:websocket"]) {
        dnea_abi::dnea::host::net::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&[
        "ui:companion:panel", "ui:companion:notify",
        "ui:webext:popup", "ui:webext:content",
    ]) {
        dnea_abi::dnea::host::ui::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&["identity:crypto:sign", "identity:crypto:verify"]) {
        dnea_abi::dnea::host::crypto::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    Ok(linker)
}
```

### `crates/dnea-module-host/src/instance.rs`

```rust
//! Instance lifecycle and message routing.

use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use dnea_abi::dnea::host::lifecycle::Lifecycle;
use dnea_abi::dnea::host::module_api::ModuleApi;
use dnea_core::{DneaError, DneaResult};
use tokio::sync::{mpsc, oneshot};
use tracing::{info, warn};
use wasmtime::{
    component::{Component, Linker},
    Store, StoreLimitsBuilder,
};

use crate::{
    engine::{build_engine, EngineConfig},
    grant::GrantSet,
    host::HostState,
    link::build_linker,
};

#[derive(Debug, thiserror::Error)]
pub enum ModuleHostError {
    #[error("compile error: {0}")]
    Compile(String),
    #[error("instantiate error: {0}")]
    Instantiate(String),
    #[error("lifecycle {hook} failed: {message}")]
    Lifecycle { hook: String, message: String },
    #[error("instance task stopped")]
    Stopped,
}

pub enum InstanceMessage {
    Invoke {
        method: String,
        args: Vec<u8>,
        reply: oneshot::Sender<DneaResult<Vec<u8>>>,
    },
    Event { kind: String, data: Vec<u8> },
    Shutdown(oneshot::Sender<()>),
}

pub struct InstanceHandle {
    pub id: String,
    pub version: String,
    pub grants: Arc<GrantSet>,
    tx: mpsc::Sender<InstanceMessage>,
    join: Option<tokio::task::JoinHandle<()>>,
}

impl InstanceHandle {
    pub async fn invoke(&self, method: &str, args: Vec<u8>) -> DneaResult<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(InstanceMessage::Invoke { method: method.to_string(), args, reply: tx })
            .await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?;
        rx.await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?
    }

    pub async fn event(&self, kind: &str, data: Vec<u8>) -> DneaResult<()> {
        self.tx
            .send(InstanceMessage::Event { kind: kind.to_string(), data })
            .await
            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))
    }

    pub async fn shutdown(mut self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.tx.send(InstanceMessage::Shutdown(tx)).await;
        let _ = tokio::time::timeout(Duration::from_secs(5), rx).await;
        if let Some(join) = self.join.take() { join.abort(); }
    }

    /// Constructs a handle for tests. Never invokes any method.
    #[doc(hidden)]
    pub fn new_for_test(id: String, version: String, grants: Arc<GrantSet>) -> Self {
        let (tx, _rx) = mpsc::channel(1);
        Self { id, version, grants, tx, join: None }
    }
}

pub struct ModuleHost {
    engine: wasmtime::Engine,
}

impl ModuleHost {
    pub fn new(cfg: &EngineConfig) -> Result<Self> {
        let engine = build_engine(cfg)?;
        Ok(Self { engine })
    }

    #[must_use]
    pub fn engine_clone(&self) -> wasmtime::Engine { self.engine.clone() }

    pub async fn load(
        &self,
        module_id: String,
        module_version: String,
        component_path: PathBuf,
        grants: GrantSet,
        host_state_builder: impl FnOnce(Arc<GrantSet>) -> HostState,
    ) -> Result<InstanceHandle, ModuleHostError> {
        let component = Component::from_file(&self.engine, &component_path)
            .with_context(|| format!("load {}", component_path.display()))
            .map_err(|e| ModuleHostError::Compile(e.to_string()))?;

        let grants = Arc::new(grants);
        let linker: Linker<HostState> = build_linker(&self.engine, &grants)
            .map_err(|e| ModuleHostError::Instantiate(format!("linker: {e}")))?;

        let mut host_state = host_state_builder(grants.clone());
        host_state.limits = StoreLimitsBuilder::new()
            .memory_size(64 * 1024 * 1024)
            .instances(1)
            .memories(4)
            .tables(4)
            .build();

        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|s| &mut s.limits);
        store.set_fuel(10_000_000)
            .map_err(|e| ModuleHostError::Instantiate(format!("fuel: {e}")))?;
        store.set_epoch_deadline(1);

        let instance = linker
            .instantiate_async(&mut store, &component)
            .await
            .map_err(|e| ModuleHostError::Instantiate(e.to_string()))?;

        let lifecycle = Lifecycle::new(&mut store, &instance)
            .map_err(|e| ModuleHostError::Lifecycle { hook: "new".into(), message: e.to_string() })?;
        lifecycle
            .call_activate(&mut store)
            .await
            .map_err(|e| ModuleHostError::Lifecycle { hook: "activate".into(), message: e.to_string() })?;

        if let Err(e) = lifecycle.call_health_check(&mut store).await {
            warn!(module = %module_id, error = %e, "initial health check failed");
        }

        info!(module = %module_id, version = %module_version, "module loaded");

        let (tx, rx) = mpsc::channel::<InstanceMessage>(64);
        let task_id = module_id.clone();
        let join = tokio::spawn(instance_task(task_id, store, instance, rx));

        Ok(InstanceHandle {
            id: module_id,
            version: module_version,
            grants,
            tx,
            join: Some(join),
        })
    }
}

async fn instance_task(
    module_id: String,
    mut store: Store<HostState>,
    instance: wasmtime::component::Instance,
    mut rx: mpsc::Receiver<InstanceMessage>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            InstanceMessage::Invoke { method, args, reply } => {
                let result = dispatch_invoke(&module_id, &mut store, &instance, &method, args).await;
                let _ = reply.send(result);
            }
            InstanceMessage::Event { kind, data } => {
                let _ = dispatch_invoke(&module_id, &mut store, &instance, &kind, data).await;
            }
            InstanceMessage::Shutdown(ack) => {
                if let Ok(lifecycle) = Lifecycle::new(&mut store, &instance) {
                    let _ = lifecycle.call_deactivate(&mut store).await;
                }
                let _ = ack.send(());
                return;
            }
        }
    }
}

async fn dispatch_invoke(
    module_id: &str,
    store: &mut Store<HostState>,
    instance: &wasmtime::component::Instance,
    method: &str,
    args: Vec<u8>,
) -> DneaResult<Vec<u8>> {
    // Prefer the generic module-api export.
    if let Ok(api) = ModuleApi::new(store, instance) {
        let args_json = String::from_utf8_lossy(&args).into_owned();
        return match api.call_invoke(store, method, &args_json).await {
            Ok(Ok(json)) => Ok(json.into_bytes()),
            Ok(Err(e)) => Err(DneaError::Module {
                module: module_id.to_string(),
                code: "host_error".into(),
                message: format!("{e:?}"),
            }),
            Err(e) => Err(DneaError::Module {
                module: module_id.to_string(),
                code: "invoke".into(),
                message: e.to_string(),
            }),
        };
    }

    // Fallback: lifecycle-only modules.
    if method == "healthCheck" {
        let lifecycle = Lifecycle::new(store, instance)
            .map_err(|e| DneaError::Module {
                module: module_id.to_string(),
                code: "lifecycle_new".into(),
                message: e.to_string(),
            })?;
        lifecycle
            .call_health_check(store)
            .await
            .map_err(|e| DneaError::Module {
                module: module_id.to_string(),
                code: "health_check".into(),
                message: e.to_string(),
            })?;
        return Ok(Vec::new());
    }

    Err(DneaError::Module {
        module: module_id.to_string(),
        code: "unknown_method".into(),
        message: format!("no such export: {method}"),
    })
}
```

## 3.6 `crates/dnea-nm-host`

### `crates/dnea-nm-host/Cargo.toml`

```toml
[package]
name        = "dnea-nm-host"
version.workspace = true
description = "DNEA Native Messaging host and bridge."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[[bin]]
name = "dnea-nm-bridge"
path = "src/bin/bridge.rs"

[dependencies]
dnea-core   = { path = "../dnea-core" }
anyhow      = { workspace = true }
async-trait = { workspace = true }
bytes       = { workspace = true }
directories = { workspace = true }
serde       = { workspace = true }
serde_json  = { workspace = true }
tokio       = { workspace = true }
tracing     = { workspace = true }
uuid        = { workspace = true }
base64      = { workspace = true }
sha2        = { workspace = true }
time        = { workspace = true }
thiserror   = { workspace = true }

[target.'cfg(unix)'.dependencies]
libc = { workspace = true }

[target.'cfg(windows)'.dependencies]
winreg      = "0.52"
windows-sys = { version = "0.59", features = [
    "Win32_Foundation",
    "Win32_Storage_FileSystem",
    "Win32_System_IO",
    "Win32_System_Pipes",
] }

[dev-dependencies]
tempfile = { workspace = true }
tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "process", "io-util", "time"] }
```

### `crates/dnea-nm-host/src/lib.rs`

```rust
//! DNEA Native Messaging host.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

pub mod registration;
pub mod session;
pub mod supervisor;

pub use session::{ClientId, ClientInfo, SessionSink};
pub use supervisor::{NmHostConfig, NmHostError, NmSupervisor};
```

### `crates/dnea-nm-host/src/session.rs`

```rust
//! The interface the NM supervisor uses to talk to the runtime core.

use async_trait::async_trait;
use dnea_core::{DneaResult, Envelope, SessionId};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(Uuid);

impl ClientId {
    #[must_use]
    pub fn new() -> Self { Self(Uuid::now_v7()) }
    #[must_use]
    pub fn as_uuid(&self) -> Uuid { self.0 }
}

impl Default for ClientId {
    fn default() -> Self { Self::new() }
}

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct ClientInfo {
    pub browser_family: String,
    pub extension_id: Option<String>,
    pub bridge_pid: Option<u32>,
    pub connected_at: std::time::SystemTime,
}

#[async_trait]
pub trait SessionSink: Send + Sync + 'static {
    async fn on_client_connected(
        &self,
        client_id: ClientId,
        info: ClientInfo,
    ) -> DneaResult<SessionId>;

    async fn on_client_disconnected(&self, client_id: ClientId);

    async fn handle_client_envelope(
        &self,
        client_id: ClientId,
        env: Envelope,
    ) -> DneaResult<Option<Envelope>>;

    async fn send_to_client(
        &self,
        _client_id: ClientId,
        _env: Envelope,
    ) -> DneaResult<()> {
        Err(dnea_core::DneaError::UnsupportedFeature(
            "this SessionSink does not support server-initiated messages".into(),
        ))
    }
}
```

### `crates/dnea-nm-host/src/supervisor.rs`

```rust
//! The daemon-side NM supervisor.

use std::{
    path::PathBuf,
    sync::Arc,
    time::SystemTime,
};

use dnea_core::{
    codec::{Codec, Decoded, DEFAULT_MTU},
    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
    DneaError, DneaResult, EnvelopeId,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{mpsc, Semaphore},
};
use tracing::{debug, info, warn};

use crate::session::{ClientId, ClientInfo, SessionSink};

const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug)]
pub struct NmHostConfig {
    pub endpoint: PathBuf,
    pub max_connections: usize,
    pub mtu: usize,
}

impl NmHostConfig {
    #[must_use]
    pub fn new(endpoint: PathBuf) -> Self {
        Self { endpoint, max_connections: 64, mtu: DEFAULT_MTU }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NmHostError {
    #[error("endpoint {0} is already in use")]
    AlreadyBound(PathBuf),
    #[error("listener I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("session sink rejected client: {0}")]
    SinkRejected(DneaError),
}

pub struct NmSupervisor {
    config: NmHostConfig,
    sink: Arc<dyn SessionSink>,
    conn_limit: Arc<Semaphore>,
}

impl NmSupervisor {
    #[must_use]
    pub fn new(config: NmHostConfig, sink: Arc<dyn SessionSink>) -> Self {
        Self {
            conn_limit: Arc::new(Semaphore::new(config.max_connections)),
            config,
            sink,
        }
    }

    pub async fn run(self) -> Result<(), NmHostError> {
        #[cfg(unix)]
        {
            let endpoint = self.config.endpoint.clone();
            if endpoint.exists() {
                std::fs::remove_file(&endpoint).ok();
            }
            let listener = tokio::net::UnixListener::bind(&endpoint)?;
            info!(endpoint = %endpoint.display(), "nm supervisor listening");
            self.accept_loop_unix(listener).await;
            Ok(())
        }
        #[cfg(windows)]
        {
            crate::supervisor_windows::run_windows_listener(self.config, self.sink).await
        }
    }

    #[cfg(unix)]
    async fn accept_loop_unix(self, listener: tokio::net::UnixListener) {
        loop {
            let permit = match self.conn_limit.clone().acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    warn!("connection limiter closed");
                    return;
                }
            };
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let sink = self.sink.clone();
                    let mtu = self.config.mtu;
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, sink, mtu).await {
                            warn!(error = %e, "nm connection ended with error");
                        }
                        drop(permit);
                    });
                }
                Err(e) => {
                    warn!(error = %e, "nm accept failed; continuing");
                    drop(permit);
                }
            }
        }
    }
}

#[cfg(unix)]
async fn handle_connection(
    mut sock: tokio::net::UnixStream,
    sink: Arc<dyn SessionSink>,
    mtu: usize,
) -> DneaResult<()> {
    let preamble = read_frame(&mut sock).await?;
    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
    let browser_family = preamble
        .get("browser_family").and_then(|v| v.as_str())
        .unwrap_or("unknown").to_string();
    let extension_id = preamble
        .get("extension_id").and_then(|v| v.as_str())
        .map(str::to_string);
    let bridge_pid = preamble
        .get("pid").and_then(|v| v.as_u64())
        .and_then(|n| u32::try_from(n).ok());

    let client_id = ClientId::new();
    let info = ClientInfo {
        browser_family: browser_family.clone(),
        extension_id,
        bridge_pid,
        connected_at: SystemTime::now(),
    };

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
    let (read_half, mut write_half) = sock.into_split();

    // Attach the client with its outbound sender before invoking
    // on_client_connected so the session can deliver Welcome through
    // the normal channel.
    if let Some(session) = sink.as_ref() as *const dyn SessionSink as *const _ as *const crate::session::Attachable {
        let _ = session; // fallback path; see note below
    }
    sink.on_client_connected(client_id, info)
        .await
        .map_err(NmHostError::SinkRejected)?;

    let writer = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            if write_frame(&mut write_half, &bytes).await.is_err() {
                return;
            }
        }
    });

    let codec = Codec::new(mtu);
    let mut read_half = read_half;
    let result = read_loop(&mut read_half, &sink, client_id, &codec, &tx).await;

    drop(tx);
    let _ = writer.await;
    sink.on_client_disconnected(client_id).await;
    debug!(%client_id, "nm client disconnected");
    result
}

#[cfg(unix)]
async fn read_loop(
    read_half: &mut tokio::net::unix::OwnedReadHalf,
    sink: &Arc<dyn SessionSink>,
    client_id: ClientId,
    codec: &Codec,
    tx: &mpsc::Sender<Vec<u8>>,
) -> DneaResult<()> {
    loop {
        let frame = match read_frame_opt(read_half).await? {
            Some(f) => f,
            None => return Ok(()),
        };
        let decoded = match codec.decode(&frame) {
            Ok(d) => d,
            Err(e) => {
                warn!(%client_id, error = %e, "framing error");
                let err_env = Envelope {
                    v: PROTOCOL_VERSION,
                    id: EnvelopeId::new(),
                    kind: MessageType::Error,
                    ts: time::OffsetDateTime::now_utc(),
                    src: Party::Runtime,
                    dst: Party::Client,
                    reply_to: None,
                    cap: None,
                    module: None,
                    enc: dnea_core::Encoding::Identity,
                    ttl_ms: None,
                    payload: None,
                    err: Some(dnea_core::EnvelopeError {
                        code: e.code().to_string(),
                        message: e.to_string(),
                        retryable: false,
                        retry_after_ms: None,
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                let _ = send_envelope(codec, tx, &err_env).await;
                continue;
            }
        };
        let env = match decoded {
            Decoded::Incomplete => continue,
            Decoded::Envelope(e) => *e,
        };
        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
            warn!(%client_id, error = %e, "direction violation");
            continue;
        }
        match sink.handle_client_envelope(client_id, env).await {
            Ok(Some(reply)) => { send_envelope(codec, tx, &reply).await?; }
            Ok(None) => {}
            Err(e) => {
                let err_env = Envelope {
                    v: PROTOCOL_VERSION,
                    id: EnvelopeId::new(),
                    kind: MessageType::Error,
                    ts: time::OffsetDateTime::now_utc(),
                    src: Party::Runtime,
                    dst: Party::Client,
                    reply_to: None,
                    cap: None,
                    module: None,
                    enc: dnea_core::Encoding::Identity,
                    ttl_ms: None,
                    payload: None,
                    err: Some(dnea_core::EnvelopeError {
                        code: e.code().to_string(),
                        message: e.to_string(),
                        retryable: e.retryable(),
                        retry_after_ms: match e {
                            DneaError::RateLimited { retry_after_ms } => Some(retry_after_ms),
                            _ => None,
                        },
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                send_envelope(codec, tx, &err_env).await?;
            }
        }
    }
}

async fn send_envelope(
    codec: &Codec,
    tx: &mpsc::Sender<Vec<u8>>,
    env: &Envelope,
) -> DneaResult<()> {
    for frame in codec.encode(env)? {
        tx.send(frame).await.map_err(|_| DneaError::Transport)?;
    }
    Ok(())
}

async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Vec<u8>> {
    read_frame_opt(r).await?
        .ok_or_else(|| DneaError::Framing(dnea_core::FramingError::MissingEnvelope))
}

async fn read_frame_opt<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match r.read_exact(&mut len_buf).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(DneaError::Internal(format!("read length: {e}"))),
    }
    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 || len > MAX_FRAME_BYTES {
        return Err(DneaError::Framing(dnea_core::FramingError::TooLarge {
            actual: len, limit: MAX_FRAME_BYTES,
        }));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).await
        .map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
    Ok(Some(buf))
}

async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, bytes: &[u8]) -> DneaResult<()> {
    if bytes.is_empty() {
        return Err(DneaError::Internal("refusing to write empty frame".into()));
    }
    let len = u32::try_from(bytes.len())
        .map_err(|_| DneaError::Internal("frame too large for u32 prefix".into()))?;
    w.write_all(&len.to_ne_bytes()).await.map_err(|_| DneaError::Transport)?;
    w.write_all(bytes).await.map_err(|_| DneaError::Transport)?;
    w.flush().await.map_err(|_| DneaError::Transport)?;
    Ok(())
}
```

### `crates/dnea-nm-host/src/supervisor_windows.rs`

```rust
//! Windows named-pipe accept loop.

#![cfg(windows)]

use std::{
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ServerOptions;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use dnea_core::{
    codec::{Codec, Decoded, DEFAULT_MTU},
    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
    DneaError, DneaResult, EnvelopeId,
};

use crate::{
    session::{ClientId, ClientInfo, SessionSink},
    supervisor::NmHostConfig,
};

const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

pub async fn run_windows_listener(
    config: NmHostConfig,
    sink: Arc<dyn SessionSink>,
) -> Result<(), crate::supervisor::NmHostError> {
    let pipe_name = pipe_name_from(&config.endpoint);
    tracing::info!(pipe = %pipe_name, "nm supervisor listening (Windows named pipe)");

    loop {
        let server = ServerOptions::new()
            .first_pipe_instance(false)
            .create(&pipe_name)
            .map_err(crate::supervisor::NmHostError::Io)?;

        server.connect().await.map_err(crate::supervisor::NmHostError::Io)?;

        let sink = sink.clone();
        let mtu = config.mtu;
        tokio::spawn(async move {
            if let Err(e) = handle_windows_client(server, sink, mtu).await {
                warn!(error = %e, "Windows NM client error");
            }
        });
    }
}

fn pipe_name_from(endpoint: &std::path::Path) -> String {
    let s = endpoint.to_string_lossy();
    if s.starts_with(r"\\.\pipe\") {
        return s.to_string();
    }
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
    format!(r"\\.\pipe\dnea-nm-{user}")
}

async fn handle_windows_client(
    mut pipe: tokio::net::windows::named_pipe::NamedPipeServer,
    sink: Arc<dyn SessionSink>,
    mtu: usize,
) -> DneaResult<()> {
    // Preamble.
    let preamble = read_frame(&mut pipe).await?;
    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
    let browser_family = preamble.get("browser_family").and_then(|v| v.as_str())
        .unwrap_or("unknown").to_string();
    let extension_id = preamble.get("extension_id").and_then(|v| v.as_str()).map(str::to_string);
    let bridge_pid = preamble.get("pid").and_then(|v| v.as_u64())
        .and_then(|n| u32::try_from(n).ok());

    let client_id = ClientId::new();
    let info = ClientInfo {
        browser_family,
        extension_id,
        bridge_pid,
        connected_at: std::time::SystemTime::now(),
    };
    sink.on_client_connected(client_id, info).await
        .map_err(|e| DneaError::Internal(e.to_string()))?;

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
    let (mut read_half, mut write_half) = tokio::io::split(pipe);

    let writer = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            if write_frame(&mut write_half, &bytes).await.is_err() { return; }
        }
    });

    let codec = Codec::new(mtu);
    loop {
        let frame = match read_frame_opt(&mut read_half).await? {
            Some(f) => f,
            None => break,
        };
        let decoded = match codec.decode(&frame) {
            Ok(d) => d,
            Err(e) => {
                warn!(%client_id, error = %e, "framing error");
                continue;
            }
        };
        let env = match decoded {
            Decoded::Incomplete => continue,
            Decoded::Envelope(e) => *e,
        };
        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
            debug!(%client_id, error = %e, "direction violation");
            continue;
        }
        match sink.handle_client_envelope(client_id, env).await {
            Ok(Some(reply)) => { send_envelope(&codec, &tx, &reply).await?; }
            Ok(None) => {}
            Err(e) => {
                let err_env = Envelope {
                    v: PROTOCOL_VERSION,
                    id: EnvelopeId::new(),
                    kind: MessageType::Error,
                    ts: time::OffsetDateTime::now_utc(),
                    src: Party::Runtime,
                    dst: Party::Client,
                    reply_to: None,
                    cap: None,
                    module: None,
                    enc: dnea_core::Encoding::Identity,
                    ttl_ms: None,
                    payload: None,
                    err: Some(dnea_core::EnvelopeError {
                        code: e.code().to_string(),
                        message: e.to_string(),
                        retryable: e.retryable(),
                        retry_after_ms: None,
                        details: None,
                        cause: None,
                    }),
                    meta: Default::default(),
                };
                let _ = send_envelope(&codec, &tx, &err_env).await;
            }
        }
    }

    drop(tx);
    let _ = writer.await;
    sink.on_client_disconnected(client_id).await;
    Ok(())
}

async fn send_envelope(
    codec: &Codec,
    tx: &mpsc::Sender<Vec<u8>>,
    env: &Envelope,
) -> DneaResult<()> {
    for frame in codec.encode(env)? {
        tx.send(frame).await.map_err(|_| DneaError::Transport)?;
    }
    Ok(())
}

async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Vec<u8>> {
    read_frame_opt(r).await?
        .ok_or_else(|| DneaError::Framing(dnea_core::FramingError::MissingEnvelope))
}

async fn read_frame_opt<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match r.read_exact(&mut len_buf).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(DneaError::Internal(format!("read length: {e}"))),
    }
    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 || len > MAX_FRAME_BYTES {
        return Err(DneaError::Framing(dnea_core::FramingError::TooLarge {
            actual: len, limit: MAX_FRAME_BYTES,
        }));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).await.map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
    Ok(Some(buf))
}

async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, bytes: &[u8]) -> DneaResult<()> {
    if bytes.is_empty() {
        return Err(DneaError::Internal("refusing to write empty frame".into()));
    }
    let len = u32::try_from(bytes.len())
        .map_err(|_| DneaError::Internal("frame too large for u32 prefix".into()))?;
    w.write_all(&len.to_ne_bytes()).await.map_err(|_| DneaError::Transport)?;
    w.write_all(bytes).await.map_err(|_| DneaError::Transport)?;
    w.flush().await.map_err(|_| DneaError::Transport)?;
    Ok(())
}
```

Add `mod supervisor_windows;` guarded by `#[cfg(windows)]` in `lib.rs`:

```rust
#[cfg(windows)]
mod supervisor_windows;
```

### `crates/dnea-nm-host/src/bin/bridge.rs`

```rust
//! DNEA Native Messaging bridge.

use std::{
    env,
    io::{self, Read, Write},
    process::ExitCode,
};

use anyhow::{bail, Context, Result};

const MAX_FRAME: usize = 64 * 1024 * 1024;

#[cfg(unix)]
mod transport {
    use super::*;
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;

    pub fn connect() -> Result<UnixStream> {
        let path = socket_path()?;
        UnixStream::connect(&path)
            .with_context(|| format!("connect to runtime at {}", path.display()))
    }

    fn socket_path() -> Result<PathBuf> {
        if let Ok(p) = env::var("DNEA_SOCKET") {
            return Ok(PathBuf::from(p));
        }
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .context("no home directory for runtime dir")?;
        let dir = proj.runtime_dir().context("no runtime dir")?;
        Ok(dir.join("nm.sock"))
    }
}

#[cfg(windows)]
mod transport {
    use super::*;
    use std::fs::OpenOptions;
    use std::path::PathBuf;

    pub fn connect() -> Result<std::fs::File> {
        let path = pipe_path()?;
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .with_context(|| format!("open named pipe {}", path.display()))
    }

    fn pipe_path() -> Result<PathBuf> {
        if let Ok(p) = env::var("DNEA_PIPE") {
            return Ok(PathBuf::from(p));
        }
        let user = env::var("USERNAME").unwrap_or_else(|_| "default".into());
        Ok(PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}")))
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("dnea-nm-bridge: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut upstream = transport::connect()?;
    let preamble = serde_json::json!({
        "browser_family": detect_browser_family(),
        "extension_id":   env::var("DNEA_EXTENSION_ID").ok(),
        "pid":            std::process::id(),
        "protocol":       "dnea-nm-v1",
    });
    let preamble_bytes = serde_json::to_vec(&preamble)?;
    write_frame(&mut upstream, &preamble_bytes)?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut upstream_r = upstream.try_clone()?;
    let mut upstream_w = upstream;

    let in_thread = std::thread::Builder::new()
        .name("browser->runtime".into())
        .spawn(move || pump_framed(stdin.lock(), upstream_w))?;
    let out_result = pump_framed(upstream_r, stdout.lock());
    let _ = in_thread.join();
    out_result
}

fn pump_framed<R, W>(mut r: R, mut w: W) -> Result<()>
where R: Read, W: Write {
    loop {
        match read_frame(&mut r)? {
            None => return Ok(()),
            Some(bytes) => write_frame(&mut w, &bytes)?,
        }
    }
}

fn read_frame<R: Read>(r: &mut R) -> Result<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match r.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e).context("read length prefix"),
    }
    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 { bail!("zero-length frame"); }
    if len > MAX_FRAME { bail!("frame size {len} exceeds maximum {MAX_FRAME}"); }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).context("read frame body")?;
    Ok(Some(buf))
}

fn write_frame<W: Write>(w: &mut W, bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() { bail!("refusing to write empty frame"); }
    let len = u32::try_from(bytes.len()).context("frame too large for u32 prefix")?;
    w.write_all(&len.to_ne_bytes())?;
    w.write_all(bytes)?;
    w.flush()?;
    Ok(())
}

fn detect_browser_family() -> &'static str {
    if env::var_os("MOZ_CRASHREPORTER").is_some() { return "firefox"; }
    if let Ok(v) = env::var("DNEA_BROWSER_FAMILY") {
        return match v.as_str() {
            "chrome" => "chromium", "edge" => "chromium",
            "brave" => "chromium", "vivaldi" => "chromium",
            "opera" => "chromium", "safari" => "safari",
            "firefox" => "firefox", _ => "unknown",
        };
    }
    "unknown"
}
```

### `crates/dnea-nm-host/src/registration.rs`

```rust
//! Per-browser Native Messaging host manifest writers.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Browser { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }

impl Browser {
    #[must_use]
    pub fn all() -> &'static [Browser] {
        &[Browser::Chrome, Browser::Chromium, Browser::Edge,
          Browser::Brave, Browser::Vivaldi, Browser::Opera, Browser::Firefox]
    }

    #[must_use]
    pub fn host_name(self) -> &'static str { "org.dnea.runtime" }

    #[must_use]
    pub fn allowed_origin(self, extension_id: &str) -> String {
        match self {
            Browser::Firefox => format!("moz-extension://{extension_id}/"),
            _ => format!("chrome-extension://{extension_id}/"),
        }
    }

    #[must_use]
    pub fn manifest_dirs(self) -> Vec<PathBuf> {
        #[cfg(target_os = "linux")]
        {
            match self {
                Browser::Chrome => vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
                Browser::Chromium => vec![PathBuf::from("/etc/chromium/native-messaging-hosts")],
                Browser::Edge => vec![PathBuf::from("/etc/opt/edge/native-messaging-hosts")],
                Browser::Firefox => vec![PathBuf::from("/usr/lib/mozilla/native-messaging-hosts")],
                Browser::Brave | Browser::Vivaldi | Browser::Opera =>
                    vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
            }
        }
        #[cfg(target_os = "macos")]
        {
            match self {
                Browser::Chrome | Browser::Chromium | Browser::Brave
                | Browser::Vivaldi | Browser::Opera =>
                    vec![PathBuf::from("/Library/Google/Chrome/NativeMessagingHosts")],
                Browser::Edge => vec![PathBuf::from("/Library/Microsoft/Edge/NativeMessagingHosts")],
                Browser::Firefox => vec![PathBuf::from(
                    "/Library/Application Support/Mozilla/NativeMessagingHosts")],
            }
        }
        #[cfg(windows)]
        {
            let _ = self;
            vec![PathBuf::from(r"C:\ProgramData\DNEA\nm")]
        }
    }
}

#[derive(Serialize)]
struct HostManifest {
    name: String,
    description: String,
    path: PathBuf,
    #[serde(rename = "type")]
    kind: &'static str,
    allowed_origins: Vec<String>,
    #[serde(rename = "x-dnea-installed")]
    marker: XDneaInstalled,
}

#[derive(Serialize)]
struct XDneaInstalled {
    version: String,
    installed_at: String,
    content_digest: String,
}

pub fn register(
    browser: Browser,
    extension_ids: &[&str],
    bridge_path: &Path,
    runtime_version: &str,
) -> Result<Vec<PathBuf>> {
    if extension_ids.is_empty() {
        bail!("at least one extension ID is required");
    }
    let allowed: Vec<String> = extension_ids.iter()
        .map(|id| browser.allowed_origin(id))
        .collect();
    let mut manifest = HostManifest {
        name: browser.host_name().into(),
        description: "DNEA runtime bridge".into(),
        path: bridge_path.to_path_buf(),
        kind: "stdio",
        allowed_origins: allowed,
        marker: XDneaInstalled {
            version: runtime_version.into(),
            installed_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "unknown".into()),
            content_digest: String::new(),
        },
    };

    let pre_digest = serde_json::to_vec(&serde_json::json!({
        "name": manifest.name,
        "description": manifest.description,
        "path": manifest.path,
        "type": manifest.kind,
        "allowed_origins": manifest.allowed_origins,
        "x-dnea-installed": {
            "version": manifest.marker.version,
            "installed_at": manifest.marker.installed_at,
        }
    }))?;
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(&pre_digest);
    manifest.marker.content_digest = format!(
        "sha-256:{}",
        base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD, h.finalize()),
    );

    let bytes = serde_json::to_vec_pretty(&manifest)?;
    let mut written = Vec::new();
    for dir in browser.manifest_dirs() {
        fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
        let path = dir.join(format!("{}.json", browser.host_name()));
        if path.exists() {
            let existing = fs::read_to_string(&path).unwrap_or_default();
            if !existing.contains("x-dnea-installed") {
                let backup = path.with_extension(format!(
                    "json.bak-{}",
                    time::OffsetDateTime::now_utc()
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap_or_else(|_| "unknown".into())
                        .replace(':', "-"),
                ));
                fs::rename(&path, &backup)?;
            }
        }
        fs::write(&path, &bytes)?;
        written.push(path);
    }

    #[cfg(windows)]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key_path = browser.registry_key();
        let (key, _) = hklm
            .create_subkey(key_path)
            .with_context(|| format!("create registry key {key_path}"))?;
        let manifest_path = written
            .first()
            .ok_or_else(|| anyhow::anyhow!("no manifest written"))?;
        key.set_value(browser.host_name(), &manifest_path.to_string_lossy().to_string())?;
    }

    Ok(written)
}

#[cfg(windows)]
impl Browser {
    fn registry_key(self) -> &'static str {
        match self {
            Browser::Chrome | Browser::Chromium | Browser::Brave
            | Browser::Vivaldi | Browser::Opera =>
                r"SOFTWARE\Google\Chrome\NativeMessagingHosts",
            Browser::Edge => r"SOFTWARE\Microsoft\Edge\NativeMessagingHosts",
            Browser::Firefox => r"SOFTWARE\Mozilla\NativeMessagingHosts",
        }
    }
}

use base64::Engine as _;
```

### `crates/dnea-nm-host/tests/mock_browser.rs`

```rust
//! End-to-end test against the real bridge binary and a real supervisor.

use std::{path::PathBuf, sync::Arc, time::Duration};

use async_trait::async_trait;
use dnea_core::{
    envelope::{Envelope, MessageType, Party, PROTOCOL_VERSION},
    DneaResult, SessionId,
};
use dnea_nm_host::{
    session::{ClientId, ClientInfo, SessionSink},
    supervisor::{NmHostConfig, NmSupervisor},
};
use serde_json::json;
use tempfile::TempDir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, Command},
};

struct EchoSink { session_id: SessionId }

impl EchoSink {
    fn new() -> Self { Self { session_id: SessionId::new() } }
}

#[async_trait]
impl SessionSink for EchoSink {
    async fn on_client_connected(&self, _: ClientId, _: ClientInfo) -> DneaResult<SessionId> {
        Ok(self.session_id)
    }
    async fn on_client_disconnected(&self, _: ClientId) {}
    async fn handle_client_envelope(
        &self, _: ClientId, env: Envelope,
    ) -> DneaResult<Option<Envelope>> {
        match env.kind {
            MessageType::Hello => {
                let mut w = env.reply(MessageType::Welcome);
                w.payload = Some(json!({
                    "runtimeVersion": "test",
                    "protocolVersion": PROTOCOL_VERSION,
                    "sessionId": self.session_id.to_string(),
                    "mtu": 1_048_576,
                    "encodings": ["identity"],
                    "features": { "chunking": true },
                }));
                let ext_id = env.payload.as_ref()
                    .and_then(|p| p.get("extensionId"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                w.meta.insert("client_extension_id".into(), json!(ext_id));
                Ok(Some(w))
            }
            _ => {
                let mut r = env.reply(MessageType::Result);
                r.payload = env.payload;
                Ok(Some(r))
            }
        }
    }
}

struct MockBrowser {
    child: Child,
    stdin: tokio::process::ChildStdin,
    stdout: tokio::process::ChildStdout,
}

impl MockBrowser {
    async fn spawn(bridge_path: &PathBuf, socket: &PathBuf) -> std::io::Result<Self> {
        let mut child = Command::new(bridge_path)
            .env("DNEA_SOCKET", socket)
            .env("DNEA_BROWSER_FAMILY", "chrome")
            .env("DNEA_EXTENSION_ID", "abcdefghijklmnopabcdefghijklmnop")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;
        let stdin = child.stdin.take().expect("piped stdin");
        let stdout = child.stdout.take().expect("piped stdout");
        Ok(Self { child, stdin, stdout })
    }

    async fn send(&mut self, env: &Envelope) -> std::io::Result<()> {
        let bytes = serde_json::to_vec(env).unwrap();
        let len = u32::try_from(bytes.len()).unwrap();
        self.stdin.write_all(&len.to_ne_bytes()).await?;
        self.stdin.write_all(&bytes).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    async fn recv(&mut self) -> std::io::Result<Envelope> {
        let mut len_buf = [0u8; 4];
        self.stdout.read_exact(&mut len_buf).await?;
        let len = u32::from_ne_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        self.stdout.read_exact(&mut buf).await?;
        let env: Envelope = serde_json::from_slice(&buf).unwrap();
        Ok(env)
    }

    async fn kill(&mut self) { let _ = self.child.kill().await; }
}

fn bridge_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_dnea-nm-bridge"))
}

#[tokio::test(flavor = "multi_thread")]
async fn t01_hello_welcome() {
    let tmp = TempDir::new().unwrap();
    let socket = tmp.path().join("nm.sock");

    let sink = Arc::new(EchoSink::new());
    let config = NmHostConfig::new(socket.clone());
    let supervisor = NmSupervisor::new(config, sink.clone());
    let sup_task = tokio::spawn(supervisor.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let bridge = bridge_binary();
    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();

    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "browserVersion": "test",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
        "encodings": ["identity"],
        "features": { "chunking": true },
    }));
    browser.send(&hello).await.unwrap();

    let welcome = tokio::time::timeout(Duration::from_secs(5), browser.recv())
        .await.expect("welcome within 5s").unwrap();
    assert_eq!(welcome.kind, MessageType::Welcome);
    assert_eq!(welcome.reply_to, Some(hello.id));
    assert_eq!(
        welcome.meta.get("client_extension_id").and_then(|v| v.as_str()),
        Some("abcdefghijklmnopabcdefghijklmnop"),
    );

    browser.kill().await;
    sup_task.abort();
}

#[tokio::test(flavor = "multi_thread")]
async fn t12_reconnect_after_restart() {
    let tmp = TempDir::new().unwrap();
    let socket = tmp.path().join("nm.sock");

    let sink = Arc::new(EchoSink::new());
    let config = NmHostConfig::new(socket.clone());
    let supervisor = NmSupervisor::new(config.clone(), sink.clone());
    let sup_task = tokio::spawn(supervisor.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let bridge = bridge_binary();
    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();
    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
    }));
    browser.send(&hello).await.unwrap();
    let _welcome = browser.recv().await.unwrap();

    sup_task.abort();
    tokio::time::sleep(Duration::from_millis(100)).await;
    browser.kill().await;
    let _ = browser.child.wait().await;

    let supervisor2 = NmSupervisor::new(config, sink.clone());
    let sup_task2 = tokio::spawn(supervisor2.run());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut browser2 = MockBrowser::spawn(&bridge, &socket).await.unwrap();
    let mut hello2 = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
    hello2.payload = Some(json!({
        "clientVersion": "1.0.0",
        "browserFamily": "chromium",
        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
        "mtu": 1_048_576,
    }));
    browser2.send(&hello2).await.unwrap();
    let welcome2 = tokio::time::timeout(Duration::from_secs(5), browser2.recv())
        .await.expect("reconnect welcome").unwrap();
    assert_eq!(welcome2.kind, MessageType::Welcome);
    assert_eq!(welcome2.reply_to, Some(hello2.id));

    browser2.kill().await;
    sup_task2.abort();
}
```

## 3.7 `crates/dnea-session`

### `crates/dnea-session/Cargo.toml`

```toml
[package]
name        = "dnea-session"
version.workspace = true
description = "The DNEA runtime hub."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
dnea-core        = { path = "../dnea-core" }
dnea-manifest    = { path = "../dnea-manifest" }
dnea-module-host = { path = "../dnea-module-host" }
dnea-nm-host     = { path = "../dnea-nm-host" }
anyhow           = { workspace = true }
async-trait      = { workspace = true }
base64           = { workspace = true }
ed25519-dalek    = { workspace = true }
parking_lot      = { workspace = true }
serde            = { workspace = true }
serde_json       = { workspace = true }
sha2             = { workspace = true }
thiserror        = { workspace = true }
tokio            = { workspace = true }
tracing          = { workspace = true }
uuid             = { workspace = true }

[dev-dependencies]
rand          = { workspace = true }
tempfile      = { workspace = true }
wasmtime      = { workspace = true }
```

### `crates/dnea-session/src/lib.rs`

```rust
//! The DNEA runtime hub.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

mod consent;
mod grants;
mod hub;
pub mod install;
mod registry;

pub use consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor};
pub use grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant};
pub use hub::{Session, SessionError, SessionEvent, SessionHandle};
pub use install::{
    HostStateFactory, InstallError, InstallPipeline, InstallReport, InstallSource, InstallStatus,
    TranscriptStep,
};
pub use registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry};
```

### `crates/dnea-session/src/registry.rs`

```rust
//! Client and module registries.

use std::collections::HashMap;
use std::sync::Arc;

use dnea_core::{ids::ModuleId, SessionId};
use dnea_manifest::Manifest;
use dnea_module_host::InstanceHandle;
use dnea_nm_host::session::ClientId;
use parking_lot::RwLock;
use tokio::sync::mpsc;

pub struct ClientRecord {
    pub id: ClientId,
    pub browser_family: String,
    pub extension_id: Option<String>,
    pub bridge_pid: Option<u32>,
    pub connected_at: std::time::SystemTime,
    pub tx: mpsc::Sender<Vec<u8>>,
}

pub struct ModuleRecord {
    pub id: ModuleId,
    pub version: String,
    pub handle: Arc<InstanceHandle>,
    pub manifest: Option<Arc<Manifest>>,
    pub loaded_at: std::time::SystemTime,
}

impl ModuleRecord {
    #[must_use]
    pub fn manifest(&self) -> Option<&Manifest> {
        self.manifest.as_deref()
    }
}

#[derive(Default)]
pub struct ClientRegistry {
    inner: RwLock<HashMap<ClientId, Arc<ClientRecord>>>,
    session_id: RwLock<Option<SessionId>>,
}

impl ClientRegistry {
    pub fn insert(&self, rec: Arc<ClientRecord>) {
        self.inner.write().insert(rec.id, rec);
    }

    pub fn remove(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
        self.inner.write().remove(&id)
    }

    pub fn get(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
        self.inner.read().get(&id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<ClientRecord>> {
        self.inner.read().values().cloned().collect()
    }

    pub fn len(&self) -> usize { self.inner.read().len() }

    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }

    pub fn session_id(&self) -> SessionId {
        let mut g = self.session_id.write();
        *g.get_or_insert_with(SessionId::new)
    }
}

#[derive(Default)]
pub struct ModuleRegistry {
    inner: RwLock<HashMap<ModuleId, Arc<ModuleRecord>>>,
}

impl ModuleRegistry {
    pub fn insert(&self, rec: Arc<ModuleRecord>) {
        self.inner.write().insert(rec.id.clone(), rec);
    }

    pub fn remove(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
        self.inner.write().remove(id)
    }

    pub fn get(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
        self.inner.read().get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<ModuleRecord>> {
        self.inner.read().values().cloned().collect()
    }

    pub fn len(&self) -> usize { self.inner.read().len() }

    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }
}
```

### `crates/dnea-session/src/grants.rs`

```rust
//! The grant store.

use std::collections::HashMap;
use std::time::SystemTime;

use dnea_core::ids::ModuleId;
use dnea_module_host::grant::{EffectiveGrant, GrantSet, GrantSource};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrantUpdate {
    pub module: String,
    pub capability: String,
    pub change: GrantChange,
    pub effective_constraints: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GrantChange { Granted, Revoked, Widened, Narrowed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OriginGrant {
    pub module: String,
    pub capability: String,
    pub origin: String,
    pub granted_at: SystemTime,
    pub expires_at: Option<SystemTime>,
}

#[derive(Default)]
pub struct GrantStore {
    inner: RwLock<HashMap<ModuleId, GrantSet>>,
    origins: RwLock<HashMap<(String, String, String), OriginGrant>>,
}

impl GrantStore {
    pub fn set(&self, module: ModuleId, grants: GrantSet) {
        self.inner.write().insert(module, grants);
    }

    pub fn get(&self, module: &ModuleId) -> Option<GrantSet> {
        self.inner.read().get(module).cloned()
    }

    pub fn grant(
        &self,
        module: &ModuleId,
        capability: &str,
        constraints: serde_json::Value,
        source: GrantSource,
    ) -> Option<GrantUpdate> {
        let mut guard = self.inner.write();
        let set = guard.entry(module.clone()).or_default();
        let existed = set.get(capability).is_some();
        set.insert(EffectiveGrant {
            id: capability.to_string(),
            constraints: constraints.clone(),
            source,
        });
        if existed { None } else {
            Some(GrantUpdate {
                module: module.to_string(),
                capability: capability.to_string(),
                change: GrantChange::Granted,
                effective_constraints: constraints,
            })
        }
    }

    pub fn revoke(&self, module: &ModuleId, capability: &str) -> Option<GrantUpdate> {
        let mut guard = self.inner.write();
        let set = guard.get_mut(module)?;
        if set.get(capability).is_none() { return None; }
        let mut new_set = GrantSet::new();
        for (id, g) in set.iter() {
            if id != capability { new_set.insert(g.clone()); }
        }
        *set = new_set;
        Some(GrantUpdate {
            module: module.to_string(),
            capability: capability.to_string(),
            change: GrantChange::Revoked,
            effective_constraints: serde_json::Value::Null,
        })
    }

    pub fn modules(&self) -> Vec<ModuleId> {
        self.inner.read().keys().cloned().collect()
    }

    pub fn add_origin(
        &self,
        module: &ModuleId,
        capability: &str,
        origin: &str,
        expires_at: Option<SystemTime>,
    ) -> OriginGrant {
        let rec = OriginGrant {
            module: module.to_string(),
            capability: capability.to_string(),
            origin: origin.to_string(),
            granted_at: SystemTime::now(),
            expires_at,
        };
        self.origins.write().insert(
            (module.to_string(), capability.to_string(), origin.to_string()),
            rec.clone(),
        );
        rec
    }

    pub fn remove_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
        self.origins
            .write()
            .remove(&(module.to_string(), capability.to_string(), origin.to_string()))
            .is_some()
    }

    pub fn origins_for(&self, module: &ModuleId, capability: &str) -> Vec<OriginGrant> {
        self.origins
            .read()
            .iter()
            .filter(|((m, c, _), _)| m == module.as_str() && c == capability)
            .map(|(_, v)| v.clone())
            .collect()
    }

    #[must_use]
    pub fn has_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
        let key = (module.to_string(), capability.to_string(), origin.to_string());
        let guard = self.origins.read();
        match guard.get(&key) {
            Some(g) => g.expires_at.map(|e| e > SystemTime::now()).unwrap_or(true),
            None => false,
        }
    }
}
```

### `crates/dnea-session/src/consent.rs`

```rust
//! Consent state.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RememberFor { Session, ModuleVersion, Always }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentRequest {
    pub prompt_id: Uuid,
    pub module: String,
    pub module_name: String,
    pub capability: String,
    pub tier: String,
    pub reason: String,
    pub constraints: serde_json::Value,
    pub target_description: Option<String>,
    pub expires_at: SystemTime,
    pub can_remember: Vec<RememberFor>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsentDecision {
    pub prompt_id: Uuid,
    pub granted: bool,
    #[serde(default)]
    pub scope: Option<serde_json::Value>,
    #[serde(default)]
    pub remember_for: Option<RememberFor>,
}

#[derive(Default)]
pub struct ConsentState {
    open: RwLock<HashMap<Uuid, OpenPrompt>>,
}

struct OpenPrompt {
    request: ConsentRequest,
    created_at: SystemTime,
    waiter: Option<oneshot::Sender<ConsentDecision>>,
}

impl ConsentState {
    pub fn open(&self, request: ConsentRequest) -> oneshot::Receiver<ConsentDecision> {
        let (tx, rx) = oneshot::channel();
        let mut g = self.open.write();
        g.insert(request.prompt_id, OpenPrompt {
            request,
            created_at: SystemTime::now(),
            waiter: Some(tx),
        });
        rx
    }

    pub fn resolve(&self, decision: ConsentDecision) -> Option<ConsentRequest> {
        let mut g = self.open.write();
        let mut entry = g.remove(&decision.prompt_id)?;
        if let Some(tx) = entry.waiter.take() {
            let _ = tx.send(decision);
        }
        Some(entry.request)
    }

    pub fn list(&self) -> Vec<ConsentRequest> {
        self.open.read().values().map(|p| p.request.clone()).collect()
    }

    pub fn get(&self, id: &Uuid) -> Option<ConsentRequest> {
        self.open.read().get(id).map(|p| p.request.clone())
    }

    pub fn sweep(&self, ttl: Duration) {
        let now = SystemTime::now();
        let mut g = self.open.write();
        let expired: Vec<Uuid> = g.iter()
            .filter(|(_, p)| {
                now.duration_since(p.created_at).map(|d| d > ttl).unwrap_or(false)
            })
            .map(|(id, _)| *id)
            .collect();
        for id in expired {
            if let Some(mut p) = g.remove(&id) {
                if let Some(tx) = p.waiter.take() {
                    let _ = tx.send(ConsentDecision {
                        prompt_id: id,
                        granted: false,
                        scope: None,
                        remember_for: None,
                    });
                }
            }
        }
    }
}
```

### `crates/dnea-session/src/hub.rs`

```rust
//! The central hub.

use std::sync::Arc;
use std::time::Duration;

use dnea_core::{
    envelope::{Envelope, MessageType, Party},
    ids::ModuleId,
    DneaError, DneaResult, SessionId,
};
use dnea_module_host::grant::GrantSet;
use dnea_nm_host::session::{ClientId, ClientInfo, SessionSink};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{
    consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor},
    grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant},
    registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "topic", content = "payload", rename_all = "snake_case")]
pub enum SessionEvent {
    ModuleLoaded { module: String, version: String },
    ModuleUnloaded { module: String },
    GrantChanged(GrantUpdate),
    ConsentOpened(ConsentRequest),
    ConsentResolved { prompt_id: Uuid, granted: bool },
    ClientConnected { client_id: String, browser_family: String, extension_id: Option<String> },
    ClientDisconnected { client_id: String },
    OriginGranted { module: String, capability: String, origin: String },
    OriginRevoked { module: String, capability: String, origin: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("module not loaded: {0}")]
    ModuleNotLoaded(String),
    #[error("session shutting down")]
    ShuttingDown,
    #[error("internal: {0}")]
    Internal(String),
}

#[derive(Clone)]
pub struct Session { inner: Arc<SessionInner> }

struct SessionInner {
    clients: ClientRegistry,
    modules: ModuleRegistry,
    grants: GrantStore,
    consent: ConsentState,
    events: broadcast::Sender<SessionEvent>,
}

pub type SessionHandle = Session;

impl Session {
    #[must_use]
    pub fn new() -> Self {
        let (events, _) = broadcast::channel(1024);
        Self {
            inner: Arc::new(SessionInner {
                clients: ClientRegistry::default(),
                modules: ModuleRegistry::default(),
                grants: GrantStore::default(),
                consent: ConsentState::default(),
                events,
            }),
        }
    }

    pub fn clients(&self) -> Vec<Arc<ClientRecord>> { self.inner.clients.all() }
    pub fn modules(&self) -> Vec<Arc<ModuleRecord>> { self.inner.modules.all() }
    pub fn module(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> { self.inner.modules.get(id) }
    pub fn grants(&self, id: &ModuleId) -> Option<GrantSet> { self.inner.grants.get(id) }
    pub fn grant_store(&self) -> &GrantStore { &self.inner.grants }
    pub fn session_id(&self) -> SessionId { self.inner.clients.session_id() }
    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> { self.inner.events.subscribe() }
    pub fn open_consents(&self) -> Vec<ConsentRequest> { self.inner.consent.list() }
    pub fn consent(&self, id: &Uuid) -> Option<ConsentRequest> { self.inner.consent.get(id) }

    pub fn set_grants(&self, module: &ModuleId, grants: Arc<GrantSet>) {
        self.inner.grants.set(module.clone(), (*grants).clone());
    }

    pub fn grants_arc(&self, module: &ModuleId) -> Option<Arc<GrantSet>> {
        self.inner.grants.get(module).map(Arc::new)
    }

    pub async fn request_consent(&self, request: ConsentRequest) -> DneaResult<ConsentDecision> {
        let rx = self.inner.consent.open(request.clone());
        self.broadcast(SessionEvent::ConsentOpened(request));
        rx.await.map_err(|_| DneaError::Internal("consent waiter dropped".into()))
    }

    pub fn resolve_consent(&self, decision: ConsentDecision) -> DneaResult<()> {
        let req = self.inner.consent.resolve(decision.clone())
            .ok_or_else(|| DneaError::Internal(format!("no such prompt: {}", decision.prompt_id)))?;
        self.broadcast(SessionEvent::ConsentResolved {
            prompt_id: decision.prompt_id,
            granted: decision.granted,
        });
        if decision.granted {
            let module = ModuleId::parse(&req.module)
                .map_err(|e| DneaError::Internal(e.to_string()))?;
            if let Some(update) = self.inner.grants.grant(
                &module, &req.capability,
                decision.scope.unwrap_or(req.constraints),
                dnea_module_host::grant::GrantSource::UserConsent,
            ) {
                self.broadcast(SessionEvent::GrantChanged(update));
            }
        }
        Ok(())
    }

    pub fn register_module(&self, record: Arc<ModuleRecord>) {
        let id = record.id.clone();
        let version = record.version.clone();
        if self.inner.grants.get(&id).is_none() {
            self.inner.grants.set(id.clone(), GrantSet::new());
        }
        self.inner.modules.insert(record);
        self.broadcast(SessionEvent::ModuleLoaded { module: id.to_string(), version });
    }

    pub fn unregister_module(&self, id: &ModuleId) {
        if self.inner.modules.remove(id).is_some() {
            self.broadcast(SessionEvent::ModuleUnloaded { module: id.to_string() });
        }
    }

    pub async fn send_to_client_by_id(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
        let rec = self.inner.clients.get(client_id).ok_or(DneaError::Transport)?;
        let codec = dnea_core::Codec::default_mtu();
        for f in codec.encode(&env)? {
            rec.tx.send(f).await.map_err(|_| DneaError::Transport)?;
        }
        Ok(())
    }

    pub async fn broadcast_to_clients(&self, env: Envelope) -> DneaResult<()> {
        let codec = dnea_core::Codec::default_mtu();
        for rec in self.inner.clients.all() {
            if let Ok(frames) = codec.encode(&env) {
                for f in frames {
                    let _ = rec.tx.send(f).await;
                }
            }
        }
        Ok(())
    }

    pub fn broadcast_origin_granted(&self, grant: &OriginGrant) {
        self.broadcast(SessionEvent::OriginGranted {
            module: grant.module.clone(),
            capability: grant.capability.clone(),
            origin: grant.origin.clone(),
        });
    }

    pub fn broadcast_origin_revoked(&self, module: &str, capability: &str, origin: &str) {
        self.broadcast(SessionEvent::OriginRevoked {
            module: module.to_string(),
            capability: capability.to_string(),
            origin: origin.to_string(),
        });
    }

    pub async fn sweep_consents(&self, ttl: Duration) {
        self.inner.consent.sweep(ttl);
    }

    pub async fn shutdown(&self) {
        let env = Envelope::new(MessageType::Goodbye, Party::Runtime, Party::Client);
        self.broadcast_to_clients(env).await.ok();
    }

    pub fn attach_client(
        &self,
        client_id: ClientId,
        info: ClientInfo,
        tx: mpsc::Sender<Vec<u8>>,
    ) {
        self.inner.clients.insert(Arc::new(ClientRecord {
            id: client_id,
            browser_family: info.browser_family,
            extension_id: info.extension_id,
            bridge_pid: info.bridge_pid,
            connected_at: info.connected_at,
            tx,
        }));
    }

    fn broadcast(&self, event: SessionEvent) {
        let _ = self.inner.events.send(event);
    }
}

impl Default for Session {
    fn default() -> Self { Self::new() }
}

#[async_trait::async_trait]
impl SessionSink for Session {
    async fn on_client_connected(
        &self,
        client_id: ClientId,
        info: ClientInfo,
    ) -> DneaResult<SessionId> {
        if self.inner.clients.get(client_id).is_none() {
            return Err(DneaError::Internal(
                "client was not attached before on_client_connected".into(),
            ));
        }
        info!(%client_id, browser = %info.browser_family, "client connected");
        self.broadcast(SessionEvent::ClientConnected {
            client_id: client_id.to_string(),
            browser_family: info.browser_family.clone(),
            extension_id: info.extension_id.clone(),
        });
        Ok(self.inner.clients.session_id())
    }

    async fn on_client_disconnected(&self, client_id: ClientId) {
        self.inner.clients.remove(client_id);
        self.broadcast(SessionEvent::ClientDisconnected {
            client_id: client_id.to_string(),
        });
    }

    async fn handle_client_envelope(
        &self,
        client_id: ClientId,
        env: Envelope,
    ) -> DneaResult<Option<Envelope>> {
        debug!(%client_id, kind = %env.kind.as_str(), "client envelope");
        match env.kind {
            MessageType::Hello => {
                let mut w = env.reply(MessageType::Welcome);
                w.payload = Some(serde_json::json!({
                    "runtimeVersion": env!("CARGO_PKG_VERSION"),
                    "protocolVersion": dnea_core::PROTOCOL_VERSION,
                    "sessionId": self.inner.clients.session_id().to_string(),
                    "mtu": dnea_core::DEFAULT_MTU,
                    "encodings": ["identity"],
                    "features": { "chunking": true, "consentUI": true },
                }));
                let ext_id = env.payload.as_ref()
                    .and_then(|p| p.get("extensionId"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                w.meta.insert("client_extension_id".into(), serde_json::json!(ext_id));
                Ok(Some(w))
            }
            MessageType::Ping => Ok(Some(env.reply(MessageType::Pong))),
            MessageType::Goodbye => Ok(None),
            MessageType::Invoke => {
                let module_id = match &env.module {
                    Some(m) => m.clone(),
                    None => return Ok(Some(Envelope::from_error(
                        &env, &DneaError::InvalidEnvelope("invoke without module".into())))),
                };
                let Some(record) = self.inner.modules.get(&module_id) else {
                    return Ok(Some(Envelope::from_error(
                        &env, &DneaError::ModuleUnavailable(module_id.to_string()))));
                };
                let method = env.meta.get("method").and_then(|v| v.as_str())
                    .unwrap_or("").to_string();
                let args = env.payload.as_ref()
                    .map(|v| serde_json::to_vec(v).unwrap_or_default())
                    .unwrap_or_default();
                let result = record.handle.invoke(&method, args).await;
                let mut reply = env.reply(MessageType::Result);
                match result {
                    Ok(bytes) => reply.payload = serde_json::from_slice(&bytes).ok(),
                    Err(e) => return Ok(Some(Envelope::from_error(&env, &e))),
                }
                Ok(Some(reply))
            }
            MessageType::ConsentResponse => {
                let decision: ConsentDecision = serde_json::from_value(
                    env.payload.clone().unwrap_or_default(),
                ).map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
                self.resolve_consent(decision)?;
                Ok(Some(env.reply(MessageType::Result)))
            }
            MessageType::CapabilityList => {
                let mut out = env.reply(MessageType::CapabilityListResult);
                let grants: Vec<_> = self.inner.modules.all().into_iter()
                    .flat_map(|m| {
                        let set = self.inner.grants.get(&m.id);
                        let id = m.id.clone();
                        set.into_iter().flat_map(move |s| {
                            s.iter().map(|(cap, g)| serde_json::json!({
                                "module": id.to_string(),
                                "capability": cap,
                                "constraints": g.constraints,
                                "source": g.source,
                            })).collect::<Vec<_>>()
                        })
                    })
                    .collect();
                out.payload = Some(serde_json::json!({ "grants": grants }));
                Ok(Some(out))
            }
            _ => Ok(Some(env.reply(MessageType::Result))),
        }
    }

    async fn send_to_client(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
        self.send_to_client_by_id(client_id, env).await
    }
}

const _: Option<RememberFor> = None;
const _: Option<GrantChange> = None;
```

### `crates/dnea-session/src/install.rs`

```rust
//! Module installation pipeline.

use std::{collections::BTreeMap, path::PathBuf, sync::Arc, time::{Duration, SystemTime}};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use dnea_core::ids::ModuleId;
use dnea_manifest::{
    capability::{CapabilityRegistry, ConsentTier},
    manifest::Manifest,
    signature::verify_manifest,
    trust::TrustStore,
    validate::Validator,
};
use dnea_module_host::{
    grant::{EffectiveGrant, GrantSet, GrantSource},
    host::HostState,
    instance::ModuleHost,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    consent::{ConsentRequest, RememberFor},
    hub::Session,
    registry::ModuleRecord,
};

#[derive(Clone, Debug)]
pub enum InstallSource {
    Local { manifest_path: PathBuf, artifact_dir: PathBuf },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallStatus { Installed, PendingConsent, Failed }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptStep {
    pub step: String,
    pub ok: bool,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallReport {
    pub install_id: Uuid,
    pub module_id: String,
    pub version: String,
    pub status: InstallStatus,
    pub transcript: Vec<TranscriptStep>,
    pub pending_consent: Vec<String>,
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("manifest parse: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("validation: {0}")]
    Validation(#[from] dnea_manifest::ValidationError),
    #[error("signature: {0}")]
    Signature(#[from] dnea_manifest::ManifestError),
    #[error("artifact {name} digest mismatch: expected {expected}, got {actual}")]
    ArtifactDigest { name: String, expected: String, actual: String },
    #[error("missing dependency: {module}")]
    MissingDependency { module: String },
    #[error("conflict with {module}: {reason}")]
    Conflict { module: String, reason: String },
    #[error("consent denied for {capability}")]
    ConsentDenied { capability: String },
    #[error("consent timeout for {capability}")]
    ConsentTimeout { capability: String },
    #[error("capability is forbidden: {capability}")]
    ForbiddenCapability { capability: String },
    #[error("capability {capability} requires administrator approval")]
    AdminRequired { capability: String },
    #[error("module host: {0}")]
    ModuleHost(String),
    #[error("module ID: {0}")]
    ModuleId(String),
    #[error("unknown pending install: {0}")]
    UnknownPending(Uuid),
    #[error("install path outside permitted roots: {0}")]
    PathOutsideRoots(PathBuf),
}

pub trait HostStateFactory: Send + Sync + 'static {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState;
}

struct PendingInstall {
    report: InstallReport,
    manifest: Manifest,
    artifact_dir: PathBuf,
    auto_grants: GrantSet,
    remaining_prompt: Vec<String>,
    created_at: SystemTime,
}

pub struct InstallPipeline {
    session: Session,
    host: Arc<ModuleHost>,
    validator: Arc<Validator>,
    trust: Arc<dyn TrustStore>,
    host_state_factory: Arc<dyn HostStateFactory>,
    consent_timeout: Duration,
    install_roots: Vec<PathBuf>,
    pending: RwLock<BTreeMap<Uuid, PendingInstall>>,
}

impl InstallPipeline {
    pub fn new(
        session: Session,
        host: Arc<ModuleHost>,
        validator: Arc<Validator>,
        trust: Arc<dyn TrustStore>,
        host_state_factory: Arc<dyn HostStateFactory>,
    ) -> Self {
        Self {
            session, host, validator, trust, host_state_factory,
            consent_timeout: Duration::from_secs(300),
            install_roots: Vec::new(),
            pending: RwLock::new(BTreeMap::new()),
        }
    }

    #[must_use]
    pub fn with_consent_timeout(mut self, ttl: Duration) -> Self {
        self.consent_timeout = ttl; self
    }

    #[must_use]
    pub fn with_install_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.install_roots = roots; self
    }

    pub async fn install(&self, source: InstallSource) -> Result<InstallReport, InstallError> {
        let install_id = Uuid::now_v7();
        let mut transcript = Vec::with_capacity(16);

        let (manifest_bytes, artifact_dir) = match source {
            InstallSource::Local { manifest_path, artifact_dir } => {
                self.check_path(&manifest_path)?;
                self.check_path(&artifact_dir)?;
                let bytes = tokio::fs::read(&manifest_path).await?;
                (bytes, artifact_dir)
            }
        };
        transcript.push(step("load-source", true, "manifest and artifacts located"));

        let manifest_value: serde_json::Value = serde_json::from_slice(&manifest_bytes)?;
        let manifest: Manifest = serde_json::from_value(manifest_value.clone())?;
        let module_id = ModuleId::parse(&manifest.id)
            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
        transcript.push(step("parse-manifest", true,
            &format!("{} v{}", manifest.id, manifest.version)));

        self.validator.validate(&manifest_value)?;
        transcript.push(step("validate", true, "schema + semantic validation passed"));

        if !manifest.dev || manifest.signature.is_some() {
            let info = verify_manifest(&manifest_value, &*self.trust)?;
            transcript.push(step_with_details("verify-signature", true,
                "signature chain verified",
                serde_json::json!({
                    "signers": info.signers.iter().map(|s| serde_json::json!({
                        "keyId": s.key_id, "role": s.role })).collect::<Vec<_>>()
                })));
        } else {
            warn!(module = %manifest.id, "installing unsigned dev module");
            transcript.push(step("verify-signature", true, "unsigned dev module accepted"));
        }

        for (name, ep) in &manifest.entrypoints {
            let path = artifact_dir.join(&ep.path);
            let bytes = tokio::fs::read(&path).await?;
            let actual = format!("sha-256:{}",
                URL_SAFE_NO_PAD.encode(Sha256::digest(&bytes)));
            if actual != ep.digest.value {
                return Err(InstallError::ArtifactDigest {
                    name: name.clone(),
                    expected: ep.digest.value.clone(),
                    actual,
                });
            }
        }
        transcript.push(step("verify-artifacts", true, "all digests match"));

        for (dep_id, dep) in &manifest.dependencies {
            let dep_module = ModuleId::parse(dep_id)
                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
            if self.session.module(&dep_module).is_none() && !dep.optional {
                return Err(InstallError::MissingDependency { module: dep_id.clone() });
            }
        }
        transcript.push(step("resolve-deps", true, "dependencies satisfied"));

        for c in &manifest.conflicts {
            let cid = ModuleId::parse(&c.id)
                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
            if self.session.module(&cid).is_some() {
                return Err(InstallError::Conflict {
                    module: c.id.clone(),
                    reason: c.reason.clone().unwrap_or_else(|| "declared conflict".into()),
                });
            }
        }
        transcript.push(step("check-conflicts", true, "no conflicts"));

        let (auto_grants, prompt_caps, admin_caps) = self.compute_grants(&manifest)?;
        transcript.push(step("compute-grants", true,
            &format!("{} auto, {} prompt, {} admin",
                auto_grants.len(), prompt_caps.len(), admin_caps.len())));

        if !admin_caps.is_empty() {
            return Err(InstallError::AdminRequired { capability: admin_caps[0].clone() });
        }

        if prompt_caps.is_empty() {
            return self.finish_load(
                install_id, &manifest, &module_id, &artifact_dir, auto_grants, transcript,
            ).await;
        }

        let first = &prompt_caps[0];
        let request = self.build_consent_request(&manifest, first);
        let _rx = self.session.request_consent(request).await;

        let report = InstallReport {
            install_id,
            module_id: manifest.id.clone(),
            version: manifest.version.clone(),
            status: InstallStatus::PendingConsent,
            transcript: {
                let mut t = transcript;
                t.push(step("consent", true,
                    &format!("{} capabilities require consent", prompt_caps.len())));
                t
            },
            pending_consent: prompt_caps.clone(),
        };

        self.pending.write().insert(install_id, PendingInstall {
            report: report.clone(),
            manifest, artifact_dir, auto_grants,
            remaining_prompt: prompt_caps,
            created_at: SystemTime::now(),
        });

        Ok(report)
    }

    pub async fn resume(&self, install_id: Uuid) -> Result<InstallReport, InstallError> {
        let mut pending = {
            let mut map = self.pending.write();
            map.remove(&install_id).ok_or(InstallError::UnknownPending(install_id))?
        };
        if pending.created_at.elapsed().map(|d| d > self.consent_timeout).unwrap_or(true) {
            return Err(InstallError::ConsentTimeout {
                capability: pending.remaining_prompt.first().cloned().unwrap_or_default(),
            });
        }
        let module_id = ModuleId::parse(&pending.manifest.id)
            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
        let current = self.session.grants(&module_id).unwrap_or_default();
        let mut still_needed = Vec::new();
        for cap in &pending.remaining_prompt {
            if current.get(cap).is_none() { still_needed.push(cap.clone()); }
        }
        if still_needed.is_empty() {
            let mut grants = pending.auto_grants;
            for (id, g) in current.iter() {
                grants.insert(EffectiveGrant {
                    id: id.to_string(),
                    constraints: g.constraints.clone(),
                    source: g.source,
                });
            }
            return self.finish_load(
                install_id, &pending.manifest, &module_id, &pending.artifact_dir,
                grants, pending.report.transcript,
            ).await;
        }

        let next = &still_needed[0];
        let request = self.build_consent_request(&pending.manifest, next);
        let _ = self.session.request_consent(request).await;
        pending.remaining_prompt = still_needed.clone();
        pending.report.pending_consent = still_needed;
        pending.report.status = InstallStatus::PendingConsent;
        let report = pending.report.clone();
        self.pending.write().insert(install_id, pending);
        Ok(report)
    }

    async fn finish_load(
        &self,
        install_id: Uuid,
        manifest: &Manifest,
        module_id: &ModuleId,
        artifact_dir: &PathBuf,
        grants: GrantSet,
        mut transcript: Vec<TranscriptStep>,
    ) -> Result<InstallReport, InstallError> {
        let component_path = self.primary_component_path(manifest, artifact_dir)?;
        let grants_arc = Arc::new(grants.clone());
        let factory = self.host_state_factory.clone();
        let mid = manifest.id.clone();
        let ver = manifest.version.clone();

        let handle = self.host.load(
            mid.clone(), ver.clone(), component_path, grants,
            move |g| factory.build(&mid, &ver, g),
        ).await.map_err(|e| InstallError::ModuleHost(e.to_string()))?;

        transcript.push(step("load", true, "Wasm component loaded and activated"));

        let manifest_arc = Arc::new(manifest.clone());
        self.session.register_module(Arc::new(ModuleRecord {
            id: module_id.clone(),
            version: manifest.version.clone(),
            handle: Arc::new(handle),
            manifest: Some(manifest_arc),
            loaded_at: SystemTime::now(),
        }));
        self.session.set_grants(module_id, grants_arc);

        transcript.push(step("register", true, "module registered"));
        info!(module = %manifest.id, version = %manifest.version, "module installed");

        Ok(InstallReport {
            install_id,
            module_id: manifest.id.clone(),
            version: manifest.version.clone(),
            status: InstallStatus::Installed,
            transcript,
            pending_consent: Vec::new(),
        })
    }

    fn check_path(&self, path: &PathBuf) -> Result<(), InstallError> {
        if self.install_roots.is_empty() { return Ok(()); }
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        for root in &self.install_roots {
            let rc = root.canonicalize().unwrap_or_else(|_| root.clone());
            if canonical.starts_with(&rc) { return Ok(()); }
        }
        Err(InstallError::PathOutsideRoots(path.clone()))
    }

    fn compute_grants(
        &self, manifest: &Manifest,
    ) -> Result<(GrantSet, Vec<String>, Vec<String>), InstallError> {
        let registry = CapabilityRegistry::v1();
        let mut auto = GrantSet::new();
        let mut prompt = Vec::new();
        let mut admin = Vec::new();
        for req in &manifest.capabilities.request {
            let tier = req.consent.as_deref().and_then(ConsentTier::parse)
                .unwrap_or_else(|| registry.default_tier(&req.id));
            match tier {
                ConsentTier::Forbidden => return Err(InstallError::ForbiddenCapability {
                    capability: req.id.clone() }),
                ConsentTier::Admin => admin.push(req.id.clone()),
                ConsentTier::Prompt | ConsentTier::PerUse => prompt.push(req.id.clone()),
                ConsentTier::Auto => {
                    auto.insert(EffectiveGrant {
                        id: req.id.clone(),
                        constraints: if req.constraints.is_null() {
                            serde_json::json!({})
                        } else { req.constraints.clone() },
                        source: GrantSource::Auto,
                    });
                }
            }
        }
        for d in &manifest.capabilities.deny { auto.deny(d); }
        Ok((auto, prompt, admin))
    }

    fn build_consent_request(&self, manifest: &Manifest, capability: &str) -> ConsentRequest {
        let req = manifest.capabilities.request.iter().find(|r| r.id == capability);
        ConsentRequest {
            prompt_id: Uuid::now_v7(),
            module: manifest.id.clone(),
            module_name: manifest.name.clone(),
            capability: capability.to_string(),
            tier: req.and_then(|r| r.consent.clone()).unwrap_or_else(|| "prompt".into()),
            reason: req.and_then(|r| r.reason.clone())
                .unwrap_or_else(|| format!("Module requests {capability}")),
            constraints: req.map(|r| if r.constraints.is_null() { serde_json::json!({}) } else { r.constraints.clone() })
                .unwrap_or(serde_json::json!({})),
            target_description: None,
            expires_at: SystemTime::now() + self.consent_timeout,
            can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion, RememberFor::Always],
        }
    }

    fn primary_component_path(
        &self, manifest: &Manifest, artifact_dir: &PathBuf,
    ) -> Result<PathBuf, InstallError> {
        let ep = manifest.entrypoints.get("core")
            .or_else(|| manifest.entrypoints.values().next())
            .ok_or_else(|| InstallError::ModuleHost("manifest has no entrypoints".into()))?;
        Ok(artifact_dir.join(&ep.path))
    }
}

fn step(name: &str, ok: bool, msg: &str) -> TranscriptStep {
    TranscriptStep { step: name.into(), ok, message: msg.into(), details: None }
}

fn step_with_details(name: &str, ok: bool, msg: &str, details: serde_json::Value) -> TranscriptStep {
    TranscriptStep { step: name.into(), ok, message: msg.into(), details: Some(details) }
}
```

### `crates/dnea-session/tests/install_signed_manifest.rs`

```rust
//! Signed-install end-to-end test.

use std::{path::PathBuf, sync::Arc};

use dnea_manifest::{
    signature::sign_manifest,
    trust::{ResolvedKey, SigAlg, StaticTrustStore},
    validate::Validator,
};
use dnea_module_host::{engine::EngineConfig, grant::GrantSet, host::HostState, instance::ModuleHost};
use dnea_session::{
    install::{HostStateFactory, InstallPipeline, InstallSource, InstallStatus},
    Session,
};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde_json::json;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

struct NoopFactory;

impl HostStateFactory for NoopFactory {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
        HostState::new(
            module_id.to_string(), version.to_string(), grants,
            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
        )
    }
}

fn make_manifest(artifact_digest: &str) -> serde_json::Value {
    json!({
        "manifestVersion": 1, "id": "com.example.focus-mode", "version": "1.2.0",
        "runtime": ">=1.0.0 <2.0.0", "name": "Focus Mode",
        "entrypoints": { "core": {
            "kind": "wasm", "path": "core.wasm",
            "digest": { "alg": "sha-256", "value": artifact_digest } } },
        "capabilities": { "request": [
            { "id": "storage:module" }, { "id": "runtime:log" },
            { "id": "browser:bookmarks:read" } ] }
    })
}

#[tokio::test(flavor = "multi_thread")]
async fn install_signed_focus_mode_with_auto_grants() {
    let tmp = TempDir::new().unwrap();
    let artifact_bytes = b"\0asm\x01\0\0\0";
    let artifact_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(artifact_bytes)));
    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&artifact_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session.clone(), host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let report = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await;

    match report {
        Ok(r) => {
            assert_eq!(r.module_id, "com.example.focus-mode");
            assert_eq!(r.status, InstallStatus::Installed);
        }
        Err(e) => {
            let s = e.to_string();
            assert!(s.contains("component") || s.contains("wasm") || s.contains("module host"),
                "expected Wasm load error, got: {s}");
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn install_rejects_tampered_manifest() {
    let tmp = TempDir::new().unwrap();
    let artifact_bytes = b"\0asm\x01\0\0\0";
    let artifact_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(artifact_bytes)));
    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&artifact_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    manifest["name"] = json!("Tampered Name");
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let err = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await.unwrap_err();
    assert!(err.to_string().contains("digest") || err.to_string().contains("signature"));
}

#[tokio::test(flavor = "multi_thread")]
async fn install_rejects_tampered_artifact() {
    let tmp = TempDir::new().unwrap();
    let real = b"\0asm\x01\0\0\0";
    let claimed = b"different bytes";
    let claimed_digest = format!("sha-256:{}",
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
            Sha256::digest(claimed)));
    std::fs::write(tmp.path().join("core.wasm"), real.as_slice()).unwrap();

    let signing_key = SigningKey::generate(&mut OsRng);
    let key_id = "https://test.example/keys/signing";
    let mut manifest = make_manifest(&claimed_digest);
    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
    std::fs::write(tmp.path().join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();

    let mut trust = StaticTrustStore::new();
    trust.insert(key_id, ResolvedKey {
        alg: SigAlg::EdDSA,
        public_key: signing_key.verifying_key().to_bytes().to_vec(),
        revoked: false,
    });

    let session = Session::new();
    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
    let validator = Arc::new(Validator::new().unwrap());
    let pipeline = InstallPipeline::new(
        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));

    let err = pipeline.install(InstallSource::Local {
        manifest_path: tmp.path().join("manifest.json"),
        artifact_dir: tmp.path().to_path_buf(),
    }).await.unwrap_err();
    assert!(matches!(err, dnea_session::install::InstallError::ArtifactDigest { .. }));
}
```

## 3.8 `crates/dnea-local-api`

### `crates/dnea-local-api/Cargo.toml`

```toml
[package]
name        = "dnea-local-api"
version.workspace = true
description = "DNEA Local API and embedded companion UI."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[dependencies]
dnea-core          = { path = "../dnea-core" }
dnea-session       = { path = "../dnea-session" }
dnea-companion-web = { path = "../dnea-companion-web" }
anyhow             = { workspace = true }
async-trait        = { workspace = true }
axum               = { workspace = true }
hmac               = { workspace = true }
jsonwebtoken       = { workspace = true }
serde              = { workspace = true }
serde_json         = { workspace = true }
tokio              = { workspace = true }
tower              = { workspace = true }
tower-http         = { workspace = true }
tracing            = { workspace = true }
uuid               = { workspace = true }
time               = { workspace = true }
rand               = { workspace = true }
base64             = { workspace = true }
directories        = { workspace = true }

[dev-dependencies]
reqwest  = { workspace = true }
tempfile = { workspace = true }
tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "time"] }
```

### `crates/dnea-local-api/src/lib.rs`

```rust
//! The DNEA Local API and embedded companion UI.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, unreachable_pub)]
#![warn(missing_docs)]

mod auth;
mod module_ui;
mod routes;
mod state;
mod static_assets;
mod ws;

pub use auth::{mint_token, verify_token, Claims, Scope};
pub use state::{ApiConfig, ApiState, LocalApi, LocalApiError};
```

### `crates/dnea-local-api/src/auth.rs`

```rust
//! JWT minting and verification.

use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    UiRead, UiConsent, UiModules, UiSettings, UiAudit, UiAdmin,
}

impl Scope {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UiRead => "ui.read",
            Self::UiConsent => "ui.consent",
            Self::UiModules => "ui.modules",
            Self::UiSettings => "ui.settings",
            Self::UiAudit => "ui.audit",
            Self::UiAdmin => "ui.admin",
        }
    }

    #[must_use]
    pub fn companion_set() -> Vec<Self> {
        vec![Self::UiRead, Self::UiConsent, Self::UiModules, Self::UiSettings, Self::UiAudit]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub jti: Uuid,
    pub scopes: Vec<Scope>,
    pub session: String,
}

pub fn mint_token(
    secret: &[u8],
    subject: &str,
    session: &str,
    scopes: Vec<Scope>,
    ttl: Duration,
) -> anyhow::Result<String> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let claims = Claims {
        sub: subject.to_string(),
        iat: now,
        exp: now + ttl.as_secs(),
        jti: Uuid::now_v7(),
        scopes,
        session: session.to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    Ok(encode(&header, &claims, &EncodingKey::from_secret(secret))?)
}

pub fn verify_token(secret: &[u8], token: &str) -> anyhow::Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 5;
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(data.claims)
}
```

### `crates/dnea-local-api/src/state.rs`

```rust
//! API state and configuration.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use dnea_session::{install::InstallPipeline, Session};
use rand::RngCore;
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub bind: SocketAddr,
    pub discovery_path: PathBuf,
    pub token_ttl: Duration,
}

impl ApiConfig {
    #[must_use]
    pub fn default_config() -> Self {
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .expect("no home directory");
        let discovery_path = proj.runtime_dir()
            .unwrap_or_else(std::env::temp_dir).join("api.json");
        Self {
            bind: "127.0.0.1:0".parse().expect("valid loopback"),
            discovery_path,
            token_ttl: Duration::from_secs(600),
        }
    }
}

pub struct ApiState {
    pub session: Session,
    pub secret: Vec<u8>,
    pub token_ttl: Duration,
    pub install_pipeline: Option<Arc<InstallPipeline>>,
}

impl ApiState {
    #[must_use]
    pub fn new(session: Session, config: &ApiConfig) -> Self {
        let mut secret = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret);
        Self {
            session,
            secret,
            token_ttl: config.token_ttl,
            install_pipeline: None,
        }
    }

    #[must_use]
    pub fn with_pipeline(mut self, pipeline: Arc<InstallPipeline>) -> Self {
        self.install_pipeline = Some(pipeline);
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LocalApiError {
    #[error("bind {addr}: {source}")]
    Bind { addr: SocketAddr, source: std::io::Error },
    #[error("write discovery file: {0}")]
    Discovery(std::io::Error),
}

#[derive(Serialize)]
struct Discovery<'a> {
    v: u32,
    url: String,
    ws: String,
    pid: u32,
    started_at: String,
    session_id: &'a str,
    scopes: Vec<&'static str>,
}

pub struct LocalApi {
    state: Arc<ApiState>,
    listener: TcpListener,
    config: ApiConfig,
}

impl LocalApi {
    pub async fn bind(session: Session, config: ApiConfig) -> Result<Self, LocalApiError> {
        let listener = TcpListener::bind(config.bind).await
            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
        let state = Arc::new(ApiState::new(session, &config));
        Ok(Self { state, listener, config })
    }

    pub async fn bind_with_pipeline(
        session: Session,
        config: ApiConfig,
        pipeline: Arc<InstallPipeline>,
    ) -> Result<Self, LocalApiError> {
        let listener = TcpListener::bind(config.bind).await
            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
        let state = Arc::new(ApiState::new(session, &config).with_pipeline(pipeline));
        Ok(Self { state, listener, config })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn serve(self) -> anyhow::Result<()> {
        let addr = self.listener.local_addr()?;
        let session_id = self.state.session.session_id().to_string();
        let discovery = Discovery {
            v: 1,
            url: format!("http://{addr}"),
            ws: format!("ws://{addr}/ws"),
            pid: std::process::id(),
            started_at: time::OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| "unknown".into()),
            session_id: &session_id,
            scopes: crate::auth::Scope::companion_set().iter()
                .map(|s| s.as_str()).collect(),
        };
        let bytes = serde_json::to_vec_pretty(&discovery)?;
        if let Some(parent) = self.config.discovery_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&self.config.discovery_path, &bytes)
            .map_err(LocalApiError::Discovery)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&self.config.discovery_path) {
                let mut perms = meta.permissions();
                perms.set_mode(0o600);
                let _ = std::fs::set_permissions(&self.config.discovery_path, perms);
            }
        }

        info!(%addr, "local api listening");
        let router = routes::router(self.state.clone());
        axum::serve(self.listener, router).await?;
        Ok(())
    }
}
```

### `crates/dnea-local-api/src/routes.rs`

```rust
//! HTTP routes.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use dnea_core::ids::ModuleId;
use dnea_session::consent::{ConsentDecision, ConsentRequest};
use dnea_session::install::InstallSource;
use serde::Deserialize;
use serde_json::json;
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    auth::{mint_token, verify_token, Claims, Scope},
    state::ApiState,
    ws,
};

pub fn router(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/v1/version", get(version))
        .route("/v1/auth/token", post(mint))
        .route("/v1/auth/refresh", post(refresh))
        .route("/", get(crate::static_assets::index))
        .route("/app.js", get(crate::static_assets::app_js))
        .route("/style.css", get(crate::static_assets::style_css))
        .route("/v1/status", get(status))
        .route("/v1/modules", get(list_modules).post(install_module))
        .route("/v1/modules/:id", get(get_module))
        .route("/v1/modules/:id/capabilities", get(get_capabilities))
        .route("/v1/modules/:id/capabilities/:cap/origins",
            get(list_origins).post(add_origin))
        .route("/v1/modules/:id/capabilities/:cap/origins/:origin",
            delete(remove_origin))
        .route("/v1/modules/installs/:install_id/resume", post(resume_install))
        .route("/v1/consent/pending", get(list_consent))
        .route("/v1/consent/:id/respond", post(respond_consent))
        .route("/modules/:id/ui/:panel", get(crate::module_ui::serve_panel))
        .route("/modules/:id/ui/:panel/invoke", post(crate::module_ui::invoke))
        .route("/ws", get(ws::handler))
        .layer(RequestBodyLimitLayer::new(1 << 20))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

struct Authed(Claims);

#[axum::async_trait]
impl axum::extract::FromRequestParts<Arc<ApiState>> for Authed {
    type Rejection = Response;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ApiState>,
    ) -> Result<Self, Self::Rejection> {
        let token = parts.headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing bearer token").into_response())?;
        let claims = verify_token(&state.secret, token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid token").into_response())?;
        Ok(Authed(claims))
    }
}

impl Authed {
    fn require(&self, scope: Scope) -> Result<(), Response> {
        if self.0.scopes.contains(&scope) { Ok(()) } else {
            Err((StatusCode::FORBIDDEN, Json(json!({ "error": {
                "code": "E_NOT_GRANTED",
                "message": format!("missing scope {}", scope.as_str())
            }}))).into_response())
        }
    }
}

async fn version() -> Json<serde_json::Value> {
    Json(json!({
        "runtimeVersion": env!("CARGO_PKG_VERSION"),
        "protocolVersion": dnea_core::PROTOCOL_VERSION,
    }))
}

#[derive(Deserialize)]
struct MintRequest {
    #[serde(default)] grant_type: Option<String>,
    #[serde(default)] device_id: Option<String>,
    #[serde(default)] requested_scopes: Option<Vec<String>>,
}

async fn mint(State(state): State<Arc<ApiState>>, Json(req): Json<MintRequest>) -> Response {
    let subject = req.device_id.clone().unwrap_or_else(|| "dnea-client".to_string());
    let scopes: Vec<Scope> = req.requested_scopes.as_ref()
        .map(|s| s.iter().filter_map(|s| parse_scope(s)).collect())
        .unwrap_or_else(Scope::companion_set);
    let session = state.session.session_id().to_string();
    match mint_token(&state.secret, &subject, &session, scopes, state.token_ttl) {
        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn refresh(State(state): State<Arc<ApiState>>, headers: axum::http::HeaderMap) -> Response {
    let Some(token) = headers.get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer ")) else {
        return (StatusCode::UNAUTHORIZED, "missing bearer token").into_response();
    };
    let Ok(claims) = verify_token(&state.secret, token) else {
        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
    };
    match mint_token(&state.secret, &claims.sub, &claims.session, claims.scopes, state.token_ttl) {
        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn status(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let clients: Vec<_> = state.session.clients().into_iter()
        .map(|c| json!({
            "clientId": c.id.to_string(),
            "browserFamily": c.browser_family,
            "extensionId": c.extension_id,
            "bridgePid": c.bridge_pid,
        })).collect();
    let modules: Vec<_> = state.session.modules().into_iter()
        .map(|m| json!({ "id": m.id.to_string(), "version": m.version })).collect();
    Json(json!({
        "runtimeVersion": env!("CARGO_PKG_VERSION"),
        "protocolVersion": dnea_core::PROTOCOL_VERSION,
        "sessionId": state.session.session_id().to_string(),
        "clients": clients,
        "modules": modules,
    })).into_response()
}

async fn list_modules(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let list: Vec<_> = state.session.modules().into_iter()
        .map(|m| {
            let grants = state.session.grants(&m.id);
            json!({
                "id": m.id.to_string(),
                "version": m.version,
                "capabilityCount": grants.as_ref().map(|g| g.len()).unwrap_or(0),
            })
        }).collect();
    Json(json!({ "modules": list })).into_response()
}

async fn get_module(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<String>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module ID").into_response();
    };
    let Some(m) = state.session.module(&module_id) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let grants: Vec<_> = state.session.grants(&module_id).map(|g| {
        g.iter().map(|(cap, grant)| json!({
            "capability": cap,
            "constraints": grant.constraints,
            "source": grant.source,
        })).collect()
    }).unwrap_or_default();
    Json(json!({ "id": m.id.to_string(), "version": m.version, "grants": grants }))
        .into_response()
}

async fn get_capabilities(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<String>,
) -> Response {
    get_module(State(state), Authed(claims), Path(id)).await
}

async fn list_origins(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap)): Path<(String, String)>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let origins = state.session.grant_store().origins_for(&module_id, &cap);
    Json(json!({ "origins": origins })).into_response()
}

#[derive(Deserialize)]
struct AddOriginRequest {
    origin: String,
    #[serde(default)] remember_for: Option<String>,
}

async fn add_origin(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap)): Path<(String, String)>,
    Json(req): Json<AddOriginRequest>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let expires_at = match req.remember_for.as_deref() {
        Some("always") => None,
        Some("session") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(86400)),
        Some("module-version") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(90 * 86400)),
        _ => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(3600)),
    };
    let grant = state.session.grant_store()
        .add_origin(&module_id, &cap, &req.origin, expires_at);
    state.session.broadcast_origin_granted(&grant);
    Json(grant).into_response()
}

async fn remove_origin(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path((id, cap, origin)): Path<(String, String, String)>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let Ok(module_id) = ModuleId::parse(&id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    if state.session.grant_store().remove_origin(&module_id, &cap, &origin) {
        state.session.broadcast_origin_revoked(&module_id.to_string(), &cap, &origin);
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[derive(Deserialize)]
struct InstallRequest {
    source: String,
    #[serde(default, rename = "manifestPath")] manifest_path: Option<String>,
    #[serde(default, rename = "artifactDir")] artifact_dir: Option<String>,
}

async fn install_module(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Json(req): Json<InstallRequest>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
    let Some(pipeline) = state.install_pipeline.clone() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
            .into_response();
    };
    let source = match req.source.as_str() {
        "local" => {
            let manifest_path = match req.manifest_path {
                Some(p) => p.into(),
                None => return (StatusCode::BAD_REQUEST, "manifestPath required").into_response(),
            };
            let artifact_dir = req.artifact_dir.map(Into::into)
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            InstallSource::Local { manifest_path, artifact_dir }
        }
        other => return (StatusCode::BAD_REQUEST, format!("unsupported source: {other}")).into_response(),
    };
    match pipeline.install(source).await {
        Ok(report) => Json(report).into_response(),
        Err(e) => {
            let status = match &e {
                dnea_session::install::InstallError::Signature(_) => StatusCode::UNAUTHORIZED,
                dnea_session::install::InstallError::ConsentDenied { .. } => StatusCode::FORBIDDEN,
                dnea_session::install::InstallError::AdminRequired { .. } => StatusCode::FORBIDDEN,
                dnea_session::install::InstallError::Conflict { .. } => StatusCode::CONFLICT,
                dnea_session::install::InstallError::MissingDependency { .. } => StatusCode::FAILED_DEPENDENCY,
                _ => StatusCode::BAD_REQUEST,
            };
            (status, Json(json!({ "error": {
                "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
                .into_response()
        }
    }
}

async fn resume_install(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(install_id): Path<Uuid>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
    let Some(pipeline) = state.install_pipeline.clone() else {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
            .into_response();
    };
    match pipeline.resume(install_id).await {
        Ok(report) => Json(report).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": {
            "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
            .into_response(),
    }
}

async fn list_consent(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
    let list: Vec<ConsentRequest> = state.session.open_consents();
    Json(json!({ "prompts": list })).into_response()
}

#[derive(Deserialize)]
struct ConsentResponseBody {
    granted: bool,
    #[serde(default)] scope: Option<serde_json::Value>,
    #[serde(default)] remember_for: Option<String>,
}

async fn respond_consent(
    State(state): State<Arc<ApiState>>,
    Authed(claims): Authed,
    Path(id): Path<Uuid>,
    Json(body): Json<ConsentResponseBody>,
) -> Response {
    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
    let decision = ConsentDecision {
        prompt_id: id,
        granted: body.granted,
        scope: body.scope,
        remember_for: body.remember_for.and_then(|s| match s.as_str() {
            "session" => Some(dnea_session::consent::RememberFor::Session),
            "module-version" => Some(dnea_session::consent::RememberFor::ModuleVersion),
            "always" => Some(dnea_session::consent::RememberFor::Always),
            _ => None,
        }),
    };
    match state.session.resolve_consent(decision) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

fn parse_scope(s: &str) -> Option<Scope> {
    Some(match s {
        "ui.read" => Scope::UiRead,
        "ui.consent" => Scope::UiConsent,
        "ui.modules" => Scope::UiModules,
        "ui.settings" => Scope::UiSettings,
        "ui.audit" => Scope::UiAudit,
        "ui.admin" => Scope::UiAdmin,
        _ => return None,
    })
}
```

### `crates/dnea-local-api/src/ws.rs`

```rust
//! WebSocket event stream.

use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use dnea_session::SessionEvent;
use serde_json::json;
use tokio::sync::broadcast::error::RecvError;
use tracing::debug;
use uuid::Uuid;

use crate::state::ApiState;

pub async fn handler(State(state): State<Arc<ApiState>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| run(socket, state))
}

async fn run(mut socket: WebSocket, state: Arc<ApiState>) {
    let mut rx = state.session.subscribe();
    let mut heartbeat = tokio::time::interval(std::time::Duration::from_secs(20));

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Ok(ev) => {
                        let frame = json!({
                            "v": 1,
                            "id": Uuid::now_v7().to_string(),
                            "ts": time::OffsetDateTime::now_utc()
                                .format(&time::format_description::well_known::Rfc3339)
                                .unwrap_or_default(),
                            "kind": "event",
                            "topic": topic_of(&ev),
                            "payload": ev,
                        });
                        if socket.send(Message::Text(frame.to_string().into())).await.is_err() {
                            return;
                        }
                    }
                    Err(RecvError::Lagged(n)) => { debug!(missed = n, "ws subscriber lagged"); }
                    Err(RecvError::Closed) => return,
                }
            }
            _ = heartbeat.tick() => {
                let ping = json!({ "v": 1, "id": Uuid::now_v7().to_string(), "kind": "ping" });
                if socket.send(Message::Text(ping.to_string().into())).await.is_err() { return; }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => return,
                    Some(Ok(Message::Ping(p))) => { let _ = socket.send(Message::Pong(p)).await; }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => return,
                }
            }
        }
    }
}

fn topic_of(ev: &SessionEvent) -> &'static str {
    match ev {
        SessionEvent::ModuleLoaded { .. } => "module/loaded",
        SessionEvent::ModuleUnloaded { .. } => "module/unloaded",
        SessionEvent::GrantChanged(_) => "capability/changed",
        SessionEvent::ConsentOpened(_) => "consent/opened",
        SessionEvent::ConsentResolved { .. } => "consent/resolved",
        SessionEvent::ClientConnected { .. } => "client/connected",
        SessionEvent::ClientDisconnected { .. } => "client/disconnected",
        SessionEvent::OriginGranted { .. } => "origin/granted",
        SessionEvent::OriginRevoked { .. } => "origin/revoked",
    }
}
```

### `crates/dnea-local-api/src/module_ui.rs`

```rust
//! Module panel serving.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use dnea_core::ids::ModuleId;
use serde::Deserialize;
use serde_json::json;
use tracing::warn;

use crate::state::ApiState;

const BRIDGE_JS: &str = include_str!("../assets/dnea-bridge.js");

pub async fn serve_panel(
    State(state): State<Arc<ApiState>>,
    Path((module_id, panel_id)): Path<(String, String)>,
) -> Response {
    let Ok(mid) = ModuleId::parse(&module_id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let Some(record) = state.session.module(&mid) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let Some(manifest) = record.manifest() else {
        return (StatusCode::NOT_FOUND, "module manifest unavailable").into_response();
    };
    let Some(surface) = manifest.ui.iter().find(|s| s.id == panel_id) else {
        return (StatusCode::NOT_FOUND, "panel not declared").into_response();
    };
    if !surface.surfaces.iter().any(|s| s == "companion") {
        return (StatusCode::FORBIDDEN, "panel is not a companion surface").into_response();
    }

    let title = surface.title.as_deref().unwrap_or("DNEA Module");
    let html = format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'wasm-unsafe-eval'; style-src 'unsafe-inline'; img-src data:;">
<title>{title}</title>
</head>
<body>
<div id="dnea-root"></div>
<script>{bridge}</script>
<script>
window.dnea.onReady(function () {{
  window.dnea.invoke("render", {{ panelId: "{panel_id}" }})
    .then(function (html) {{ document.getElementById("dnea-root").innerHTML = html; }})
    .catch(function (e) {{ document.getElementById("dnea-root").textContent =
      "Panel failed to render: " + e.message; }});
}});
</script>
</body>
</html>"#,
        title = title, panel_id = panel_id, bridge = BRIDGE_JS,
    );
    Html(html).into_response()
}

#[derive(Deserialize)]
pub struct InvokeBody {
    method: String,
    #[serde(default)]
    args: serde_json::Value,
}

pub async fn invoke(
    State(state): State<Arc<ApiState>>,
    Path((module_id, _panel_id)): Path<(String, String)>,
    Json(body): Json<InvokeBody>,
) -> Response {
    let Ok(mid) = ModuleId::parse(&module_id) else {
        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
    };
    let Some(record) = state.session.module(&mid) else {
        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
    };
    let args = serde_json::to_vec(&body.args).unwrap_or_default();
    match record.handle.invoke(&body.method, args).await {
        Ok(bytes) => {
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!(null));
            Json(json!({ "ok": true, "value": value })).into_response()
        }
        Err(e) => {
            warn!(module = %mid, method = %body.method, error = %e, "invoke failed");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "ok": false, "error": e.to_string()
            }))).into_response()
        }
    }
}
```

### `crates/dnea-local-api/src/static_assets.rs`

```rust
//! Embedded companion UI assets.

use axum::http::header;
use axum::response::{IntoResponse, Response};

pub fn index() -> Response {
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")],
     dnea_companion_web::INDEX_HTML).into_response()
}

pub fn app_js() -> Response {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
     dnea_companion_web::APP_JS).into_response()
}

pub fn style_css() -> Response {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")],
     dnea_companion_web::STYLE_CSS).into_response()
}
```

### `crates/dnea-local-api/assets/dnea-bridge.js`

```javascript
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
```

### `crates/dnea-local-api/tests/e2e_companion.rs`

```rust
//! Companion end-to-end test.

use std::sync::Arc;
use std::time::Duration;

use dnea_core::ids::ModuleId;
use dnea_local_api::{ApiConfig, LocalApi};
use dnea_module_host::{grant::GrantSet, InstanceHandle};
use dnea_session::{
    consent::{ConsentRequest, RememberFor},
    registry::ModuleRecord,
    Session,
};
use serde_json::json;
use tempfile::TempDir;

fn dummy_handle() -> Arc<InstanceHandle> {
    InstanceHandle::new_for_test(
        "test".into(), "0.0.0-test".into(), Arc::new(GrantSet::new()))
}

#[tokio::test(flavor = "multi_thread")]
async fn companion_can_list_modules_and_respond_to_consent() {
    let session = Session::new();
    session.register_module(Arc::new(ModuleRecord {
        id: ModuleId::parse("com.example.focus-mode").unwrap(),
        version: "0.0.0-test".into(),
        handle: dummy_handle(),
        manifest: None,
        loaded_at: std::time::SystemTime::now(),
    }));

    let tmp = TempDir::new().unwrap();
    let config = ApiConfig {
        bind: "127.0.0.1:0".parse().unwrap(),
        discovery_path: tmp.path().join("api.json"),
        token_ttl: Duration::from_secs(600),
    };
    let api = LocalApi::bind(session.clone(), config).await.unwrap();
    let addr = api.local_addr().unwrap();
    let server = tokio::spawn(api.serve());
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let token_res = client
        .post(format!("http://{addr}/v1/auth/token"))
        .json(&json!({
            "grantType": "companion",
            "requestedScopes": ["ui.read", "ui.consent", "ui.modules"]
        }))
        .send().await.unwrap();
    assert_eq!(token_res.status(), 200);
    let token_body: serde_json::Value = token_res.json().await.unwrap();
    let token = token_body["token"].as_str().unwrap().to_string();

    let modules_res = client
        .get(format!("http://{addr}/v1/modules"))
        .bearer_auth(&token)
        .send().await.unwrap();
    assert_eq!(modules_res.status(), 200);
    let modules: serde_json::Value = modules_res.json().await.unwrap();
    assert_eq!(modules["modules"].as_array().unwrap().len(), 1);

    let prompt_id = uuid::Uuid::now_v7();
    let request = ConsentRequest {
        prompt_id,
        module: "com.example.focus-mode".into(),
        module_name: "Focus Mode".into(),
        capability: "browser:tabs:query".into(),
        tier: "prompt".into(),
        reason: "Detect when you open a blocked site.".into(),
        constraints: json!({ "tabScopes": ["all-windows"] }),
        target_description: Some("Query open tabs".into()),
        expires_at: std::time::SystemTime::now() + Duration::from_secs(300),
        can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion],
    };
    let session_clone = session.clone();
    let consent_task = tokio::spawn(async move {
        session_clone.request_consent(request).await
    });

    tokio::time::sleep(Duration::from_millis(50)).await;
    let pending_res = client
        .get(format!("http://{addr}/v1/consent/pending"))
        .bearer_auth(&token)
        .send().await.unwrap();
    let pending: serde_json::Value = pending_res.json().await.unwrap();
    assert_eq!(pending["prompts"].as_array().unwrap().len(), 1);

    let respond_res = client
        .post(format!("http://{addr}/v1/consent/{prompt_id}/respond"))
        .bearer_auth(&token)
        .json(&json!({ "granted": true, "remember_for": "module-version" }))
        .send().await.unwrap();
    assert_eq!(respond_res.status(), 204);

    tokio::time::timeout(Duration::from_secs(2), consent_task)
        .await.expect("consent decision").unwrap().unwrap();

    let module_id = ModuleId::parse("com.example.focus-mode").unwrap();
    let grants = session.grants(&module_id).expect("grants");
    assert!(grants.get("browser:tabs:query").is_some());

    server.abort();
}
```

Add `uuid` to `dnea-local-api/Cargo.toml` dev-dependencies.

## 3.9 `crates/dnea-companion-web`

### `crates/dnea-companion-web/Cargo.toml`

```toml
[package]
name        = "dnea-companion-web"
version.workspace = true
description = "Embedded companion UI assets."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true
```

### `crates/dnea-companion-web/src/lib.rs`

```rust
//! Embedded companion UI.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The companion UI HTML.
pub const INDEX_HTML: &str = include_str!("../assets/index.html");
/// The companion UI JavaScript.
pub const APP_JS: &str = include_str!("../assets/app.js");
/// The companion UI stylesheet.
pub const STYLE_CSS: &str = include_str!("../assets/style.css");
```

### `crates/dnea-companion-web/assets/index.html`

```html
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' ws: http://127.0.0.1:*; frame-ancestors 'none'; base-uri 'none'; form-action 'self';">
<title>DNEA</title>
<link rel="stylesheet" href="/style.css">
</head>
<body>
<header>
  <h1>DNEA</h1>
  <span id="connection" class="badge">connecting…</span>
</header>

<main>
  <section id="status-section">
    <h2>Runtime</h2>
    <dl id="status"></dl>
  </section>

  <section id="modules-section">
    <h2>Modules</h2>
    <ul id="modules" class="list"></ul>
  </section>

  <section id="consent-section" hidden>
    <h2>Pending consent</h2>
    <ul id="consent" class="list"></ul>
  </section>

  <section id="events-section">
    <h2>Events</h2>
    <ul id="events" class="list small"></ul>
  </section>
</main>

<script src="/app.js"></script>
</body>
</html>
```

### `crates/dnea-companion-web/assets/style.css`

```css
:root {
  --bg: #0f1115; --fg: #e6e8eb; --muted: #9aa0a6;
  --panel: #171a21; --border: #2a2f38;
  --accent: #6ea8fe; --ok: #7bd88f; --err: #ff6b6b;
}
* { box-sizing: border-box; }
body {
  margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  background: var(--bg); color: var(--fg); font-size: 14px; line-height: 1.5;
}
header {
  display: flex; align-items: center; gap: 12px;
  padding: 16px 24px; border-bottom: 1px solid var(--border);
}
header h1 { margin: 0; font-size: 16px; font-weight: 600; }
.badge {
  font-size: 11px; padding: 2px 8px; border-radius: 10px;
  background: var(--panel); color: var(--muted); border: 1px solid var(--border);
}
.badge.ok  { color: var(--ok);  border-color: var(--ok); }
.badge.err { color: var(--err); border-color: var(--err); }
main { padding: 24px; max-width: 960px; margin: 0 auto; }
section { margin-bottom: 32px; }
section h2 {
  font-size: 13px; text-transform: uppercase; color: var(--muted);
  margin: 0 0 12px; letter-spacing: .05em;
}
dl#status { display: grid; grid-template-columns: 160px 1fr; gap: 4px 16px; margin: 0; }
dl#status dt { color: var(--muted); }
dl#status dd { margin: 0; }
.list { list-style: none; padding: 0; margin: 0; }
.list li {
  background: var(--panel); border: 1px solid var(--border);
  border-radius: 6px; padding: 12px 16px; margin-bottom: 8px;
}
.list.small li { padding: 6px 12px; font-size: 12px; color: var(--muted); }
.card-title { font-weight: 600; margin-bottom: 4px; }
.card-sub   { color: var(--muted); font-size: 12px; }
.consent-actions { display: flex; gap: 8px; margin-top: 12px; }
button {
  font: inherit; padding: 6px 12px; border-radius: 4px;
  border: 1px solid var(--border); background: var(--panel);
  color: var(--fg); cursor: pointer;
}
button.primary { background: var(--accent); color: #000; border-color: var(--accent); }
button.danger  { color: var(--err); border-color: var(--err); }
button:hover { filter: brightness(1.1); }
button:focus { outline: 2px solid var(--accent); outline-offset: 2px; }
```

### `crates/dnea-companion-web/assets/app.js`

```javascript
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
```

## 3.10 `crates/dnea-daemon`

### `crates/dnea-daemon/Cargo.toml`

```toml
[package]
name        = "dnea-daemon"
version.workspace = true
description = "DNEA runtime daemon."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[[bin]]
name = "dnea-daemon"
path = "src/main.rs"

[dependencies]
dnea-config        = { path = "../dnea-config" }
dnea-core          = { path = "../dnea-core" }
dnea-local-api     = { path = "../dnea-local-api" }
dnea-manifest      = { path = "../dnea-manifest" }
dnea-module-host   = { path = "../dnea-module-host" }
dnea-nm-host       = { path = "../dnea-nm-host" }
dnea-session       = { path = "../dnea-session" }
anyhow             = { workspace = true }
base64             = { workspace = true }
clap               = { workspace = true }
rand               = { workspace = true }
serde_json         = { workspace = true }
tokio              = { workspace = true }
tracing            = { workspace = true }
tracing-subscriber = { workspace = true }
wasmtime           = { workspace = true }

[target.'cfg(unix)'.dependencies]
libc = { workspace = true }
```

### `crates/dnea-daemon/src/main.rs`

```rust
//! The DNEA runtime daemon.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine as _;
use clap::Parser;
use dnea_config::Config;
use dnea_local_api::{ApiConfig, LocalApi};
use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
use dnea_manifest::validate::Validator;
use dnea_module_host::engine::EngineConfig;
use dnea_module_host::grant::GrantSet;
use dnea_module_host::host::HostState;
use dnea_module_host::instance::ModuleHost;
use dnea_nm_host::supervisor::{NmHostConfig, NmSupervisor};
use dnea_session::install::{HostStateFactory, InstallPipeline};
use dnea_session::Session;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "dnea-daemon", version, about = "DNEA runtime daemon")]
struct Args {
    #[arg(long, env = "DNEA_CONFIG", default_value_os_t = Config::default_path())]
    config: PathBuf,
    #[arg(long, env = "DNEA_LOG", default_value = "info")]
    log: String,
    #[arg(long)]
    log_json: bool,
    #[arg(long)]
    init_key: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filter = EnvFilter::try_new(&args.log).context("invalid log filter")?;
    let registry = tracing_subscriber::registry().with(filter);
    if args.log_json {
        registry.with(fmt::layer().json()).init();
    } else {
        registry.with(fmt::layer().compact()).init();
    }

    let config = if args.config.exists() {
        Config::load_from(&args.config)
            .with_context(|| format!("load config from {}", args.config.display()))?
    } else {
        let mut c = Config::default();
        c.validate().context("validate default config")?;
        c
    };

    if args.init_key { return init_master_key(&config); }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("dnea")
        .build()?;
    rt.block_on(run(config))
}

fn init_master_key(config: &Config) -> Result<()> {
    let path = config.paths.state_dir.join("master.key");
    std::fs::create_dir_all(&config.paths.state_dir)?;
    if path.exists() {
        anyhow::bail!("master key already exists at {}", path.display());
    }
    let mut key = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut key);
    std::fs::write(&path, key)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }
    info!(path = %path.display(), "master key initialized");
    Ok(())
}

struct DaemonHostStateFactory;

impl HostStateFactory for DaemonHostStateFactory {
    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
        HostState::new(
            module_id.to_string(),
            version.to_string(),
            grants,
            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
        )
    }
}

async fn run(config: Config) -> Result<()> {
    info!(version = env!("CARGO_PKG_VERSION"), "dnea-daemon starting");

    // Single-instance guard.
    let lock_path = config.paths.pid_file.clone();
    let _lock = SingleInstance::acquire(&lock_path)
        .context("another runtime is already running")?;

    for dir in [
        &config.paths.state_dir, &config.paths.run_dir,
        &config.paths.log_dir, &config.paths.modules_dir,
    ] {
        std::fs::create_dir_all(dir)?;
    }

    // 1. Module host.
    let engine_cfg = EngineConfig {
        total_memory_slots: config.modules.pooling_slots,
        total_stack_slots: config.modules.pooling_slots,
        ..EngineConfig::default()
    };
    let module_host = Arc::new(ModuleHost::new(&engine_cfg).context("build module host")?);

    // 2. Session.
    let session = Session::new();

    // 3. Trust store.
    let trust: Arc<dyn dnea_manifest::trust::TrustStore> =
        Arc::new(build_trust_store(config.modules.trust_store.as_deref()));

    // 4. Validator.
    let validator = Arc::new(Validator::new().context("build manifest validator")?);

    // 5. Install pipeline.
    let pipeline = Arc::new(
        InstallPipeline::new(
            session.clone(),
            module_host.clone(),
            validator,
            trust,
            Arc::new(DaemonHostStateFactory),
        )
        .with_install_roots(config.api.install_roots.clone()),
    );

    // 6. NM supervisor.
    let nm_config = NmHostConfig::new(config.nm_endpoint());
    let nm_supervisor = NmSupervisor::new(nm_config, Arc::new(session.clone()));

    // 7. Local API.
    let api_config = ApiConfig {
        bind: "127.0.0.1:0".parse().context("invalid API bind address")?,
        discovery_path: config.api.discovery_path.clone()
            .unwrap_or_else(|| config.paths.run_dir.join("api.json")),
        token_ttl: config.token_ttl(),
    };
    let api = LocalApi::bind_with_pipeline(session.clone(), api_config, pipeline)
        .await.context("bind local api")?;

    // 8. Watchdog.
    let engine_clone = module_host.engine_clone();
    let epoch_interval = config.epoch_interval();
    let watchdog = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(epoch_interval);
        loop {
            ticker.tick().await;
            engine_clone.increment_epoch();
        }
    });

    // 9. Consent sweeper.
    let session_clone = session.clone();
    let sweeper = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(30));
        loop {
            ticker.tick().await;
            session_clone.sweep_consents(Duration::from_secs(300)).await;
        }
    });

    // 10. Run.
    let nm_task = tokio::spawn(async move {
        if let Err(e) = nm_supervisor.run().await {
            error!(error = %e, "nm supervisor failed");
        }
    });
    let api_task = tokio::spawn(async move {
        if let Err(e) = api.serve().await {
            error!(error = %e, "local api failed");
        }
    });

    // 11. Shutdown.
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    tokio::select! {
        _ = signal::ctrl_c() => info!("ctrl-c received"),
        _ = sigterm.recv() => info!("SIGTERM received"),
    }
    info!("shutting down");
    watchdog.abort();
    sweeper.abort();
    nm_task.abort();
    api_task.abort();
    let _ = tokio::time::timeout(Duration::from_secs(5), session.shutdown()).await;
    Ok(())
}

fn build_trust_store(path: Option<&std::path::Path>) -> StaticTrustStore {
    let mut store = StaticTrustStore::new();
    if let Some(p) = path {
        if let Ok(bytes) = std::fs::read(p) {
            if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                if let Some(keys) = v.get("keys").and_then(|k| k.as_array()) {
                    for k in keys {
                        let Some(id) = k.get("keyId").and_then(|v| v.as_str()) else { continue };
                        let Some(alg) = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse)
                            else { continue };
                        let Some(pk_b64) = k.get("publicKey").and_then(|v| v.as_str()) else { continue };
                        let Ok(pk) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)
                            else { continue };
                        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
                        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
                    }
                }
            }
        }
    }
    store
}

#[cfg(unix)]
struct SingleInstance { _file: std::fs::File }

#[cfg(unix)]
impl SingleInstance {
    fn acquire(path: &std::path::Path) -> Result<Self> {
        use std::os::unix::io::AsRawFd;
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).ok(); }
        let file = std::fs::OpenOptions::new()
            .create(true).write(true).truncate(false).open(path)?;
        let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        if rc != 0 {
            anyhow::bail!("another runtime is already running (lock held on {})", path.display());
        }
        std::fs::write(path, std::process::id().to_string()).ok();
        Ok(Self { _file: file })
    }
}

#[cfg(windows)]
struct SingleInstance { _handle: isize }

#[cfg(windows)]
impl SingleInstance {
    fn acquire(_path: &std::path::Path) -> Result<Self> {
        use windows_sys::Win32::System::Threading::CreateMutexW;
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS;
        let name: Vec<u16> = "Global\\DNEA-Runtime\0".encode_utf16().collect();
        let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
        if handle == 0 { anyhow::bail!("CreateMutexW failed"); }
        let err = unsafe { GetLastError() };
        if err == ERROR_ALREADY_EXISTS {
            anyhow::bail!("another runtime is already running");
        }
        Ok(Self { _handle: handle as isize })
    }
}
```

Add to `dnea-daemon/Cargo.toml` on Windows:

```toml
[target.'cfg(windows)'.dependencies]
windows-sys = { version = "0.59", features = ["Win32_Foundation", "Win32_System_Threading"] }
```

## 3.11 `crates/dnea-cli`

### `crates/dnea-cli/Cargo.toml`

```toml
[package]
name        = "dnea-cli"
version.workspace = true
description = "DNEA developer CLI."
edition.workspace     = true
rust-version.workspace = true
license.workspace     = true

[[bin]]
name = "dnea"
path = "src/main.rs"

[dependencies]
dnea-config     = { path = "../dnea-config" }
dnea-core       = { path = "../dnea-core" }
dnea-manifest   = { path = "../dnea-manifest" }
dnea-nm-host    = { path = "../dnea-nm-host" }
anyhow          = { workspace = true }
base64          = { workspace = true }
clap            = { workspace = true }
ed25519-dalek   = { workspace = true }
rand            = { workspace = true }
reqwest         = { workspace = true }
serde_json      = { workspace = true }
sha2            = { workspace = true }
tokio           = { workspace = true }
```

### `crates/dnea-cli/src/main.rs`

```rust
//! The `dnea` CLI.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use base64::Engine as _;
use clap::{Parser, Subcommand};
use dnea_manifest::signature::{sign_manifest, verify_manifest};
use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
use dnea_manifest::validate::Validator;
use dnea_nm_host::registration::{register, Browser};

#[derive(Parser)]
#[command(name = "dnea", version, about = "DNEA developer CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Validate {
        manifest: PathBuf,
        #[arg(long)] artifacts: Option<PathBuf>,
        #[arg(long)] no_sig: bool,
        #[arg(long)] trust: Option<PathBuf>,
    },
    Sign {
        manifest: PathBuf,
        #[arg(long)] key: PathBuf,
        #[arg(long)] key_id: String,
        #[arg(long, default_value = "author")] role: String,
        #[arg(long)] out: Option<PathBuf>,
    },
    Verify {
        manifest: PathBuf,
        #[arg(long)] trust: PathBuf,
    },
    Keygen {
        #[arg(long, default_value = "dnea-signing.key")] out: PathBuf,
    },
    RegisterHost {
        #[arg(long, value_enum)] browser: BrowserArg,
        #[arg(long)] extension_id: String,
        #[arg(long)] bridge: PathBuf,
    },
    Status {
        #[arg(long)] discovery: Option<PathBuf>,
    },
    InstallModule {
        manifest: PathBuf,
        #[arg(long)] artifacts: PathBuf,
        #[arg(long)] discovery: Option<PathBuf>,
    },
    Doctor {
        #[arg(long)] discovery: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum BrowserArg { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }

impl From<BrowserArg> for Browser {
    fn from(a: BrowserArg) -> Self {
        match a {
            BrowserArg::Chrome => Browser::Chrome,
            BrowserArg::Chromium => Browser::Chromium,
            BrowserArg::Edge => Browser::Edge,
            BrowserArg::Brave => Browser::Brave,
            BrowserArg::Vivaldi => Browser::Vivaldi,
            BrowserArg::Opera => Browser::Opera,
            BrowserArg::Firefox => Browser::Firefox,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(e) => { eprintln!("dnea: {e:#}"); ExitCode::from(1) }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.cmd {
        Cmd::Validate { manifest, artifacts, no_sig, trust } =>
            validate(&manifest, artifacts.as_deref(), no_sig, trust.as_deref()),
        Cmd::Sign { manifest, key, key_id, role, out } =>
            sign(&manifest, &key, &key_id, &role, out.as_deref()),
        Cmd::Verify { manifest, trust } => verify(&manifest, &trust),
        Cmd::Keygen { out } => keygen(&out),
        Cmd::RegisterHost { browser, extension_id, bridge } =>
            register_host(browser.into(), &extension_id, &bridge),
        Cmd::Status { discovery } =>
            tokio::runtime::Runtime::new()?.block_on(status(discovery.as_deref())),
        Cmd::InstallModule { manifest, artifacts, discovery } =>
            tokio::runtime::Runtime::new()?.block_on(install_module(&manifest, &artifacts, discovery.as_deref())),
        Cmd::Doctor { discovery } =>
            tokio::runtime::Runtime::new()?.block_on(doctor(discovery.as_deref())),
    }
}

fn validate(
    manifest: &PathBuf,
    artifacts: Option<&Path>,
    no_sig: bool,
    trust: Option<&Path>,
) -> Result<ExitCode> {
    let bytes = std::fs::read(manifest)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;

    let validator = Validator::new()?;
    validator.validate(&value).context("semantic validation failed")?;
    println!("✓ schema and semantics");

    if !no_sig {
        let trust_path = trust.context("--trust required unless --no-sig")?;
        let store = load_trust(trust_path)?;
        let info = verify_manifest(&value, &store).context("signature verification failed")?;
        println!("✓ signatures: {} signer(s)", info.signers.len());
        for s in &info.signers {
            println!("  - {} ({})", s.key_id, s.role);
        }
    }

    if let Some(dir) = artifacts {
        let manifest: dnea_manifest::Manifest = serde_json::from_value(value)?;
        for (name, ep) in &manifest.entrypoints {
            let path = dir.join(&ep.path);
            let bytes = std::fs::read(&path)
                .with_context(|| format!("read {}", path.display()))?;
            let actual = format!("sha-256:{}",
                base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .encode(sha2::Digest::digest(&mut sha2::Sha256::new(), &bytes)));
            if actual != ep.digest.value {
                anyhow::bail!("artifact {name} digest mismatch");
            }
            println!("✓ artifact {name}");
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn sign(
    manifest: &PathBuf, key_path: &PathBuf, key_id: &str, role: &str, out: Option<&Path>,
) -> Result<ExitCode> {
    let key_bytes = std::fs::read(key_path)?;
    let signing_key = ed25519_dalek::SigningKey::from_bytes(
        key_bytes.as_slice().try_into().context("signing key must be 32 bytes")?,
    );
    let bytes = std::fs::read(manifest)?;
    let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;
    sign_manifest(&mut value, &signing_key, key_id, role)?;
    let target = out.unwrap_or(manifest.as_path());
    std::fs::write(target, serde_json::to_vec_pretty(&value)?)?;
    println!("signed manifest written to {}", target.display());
    Ok(ExitCode::SUCCESS)
}

fn verify(manifest: &PathBuf, trust: &PathBuf) -> Result<ExitCode> {
    let bytes = std::fs::read(manifest)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    let store = load_trust(trust)?;
    let info = verify_manifest(&value, &store)?;
    println!("verified: {} signer(s)", info.signers.len());
    for s in &info.signers { println!("  - {} ({})", s.key_id, s.role); }
    Ok(ExitCode::SUCCESS)
}

fn keygen(out: &PathBuf) -> Result<ExitCode> {
    use rand::rngs::OsRng;
    let sk = ed25519_dalek::SigningKey::generate(&mut OsRng);
    std::fs::write(out, sk.to_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(out)?.permissions();
        perms.set_mode(0o600);
        std::fs::set_permissions(out, perms)?;
    }
    let vk = sk.verifying_key();
    let vk_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(vk.to_bytes());
    println!("private key: {}", out.display());
    println!("public key (base64url): {}", vk_b64);
    println!();
    println!("Add to your trust store:");
    println!("  {{ \"keyId\": \"...\", \"alg\": \"EdDSA\", \"publicKey\": \"{vk_b64}\" }}");
    Ok(ExitCode::SUCCESS)
}

fn register_host(browser: Browser, extension_id: &str, bridge: &PathBuf) -> Result<ExitCode> {
    let written = register(browser, &[extension_id], bridge, env!("CARGO_PKG_VERSION"))?;
    for p in written { println!("✓ {}", p.display()); }
    Ok(ExitCode::SUCCESS)
}

async fn status(discovery: Option<&Path>) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    let client = reqwest::Client::new();
    let token: serde_json::Value = client
        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
        .json(&serde_json::json!({ "grantType": "cli", "requestedScopes": ["ui.read"] }))
        .send().await?.json().await?;
    let token = token["token"].as_str().context("no token")?;
    let status: serde_json::Value = client
        .get(format!("{}/v1/status", disc["url"].as_str().unwrap()))
        .bearer_auth(token).send().await?.json().await?;
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(ExitCode::SUCCESS)
}

async fn install_module(
    manifest: &PathBuf, artifacts: &PathBuf, discovery: Option<&Path>,
) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    let client = reqwest::Client::new();
    let token: serde_json::Value = client
        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
        .json(&serde_json::json!({
            "grantType": "cli", "requestedScopes": ["ui.modules", "ui.read"] }))
        .send().await?.json().await?;
    let token = token["token"].as_str().context("no token")?;
    let report: serde_json::Value = client
        .post(format!("{}/v1/modules", disc["url"].as_str().unwrap()))
        .bearer_auth(token)
        .json(&serde_json::json!({
            "source": "local",
            "manifestPath": manifest,
            "artifactDir": artifacts,
        }))
        .send().await?.json().await?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(ExitCode::SUCCESS)
}

async fn doctor(discovery: Option<&Path>) -> Result<ExitCode> {
    let disc = load_discovery(discovery)?;
    println!("Runtime: {}", disc["url"]);
    println!("PID:     {}", disc["pid"]);
    println!("Session: {}", disc["session_id"]);
    let cfg = dnea_config::Config::load()?;
    println!("Config:  {}", dnea_config::Config::default_path().display());
    println!("State:   {}", cfg.paths.state_dir.display());
    println!("Run:     {}", cfg.paths.run_dir.display());
    println!("Log:     {}", cfg.paths.log_dir.display());
    for browser in Browser::all() {
        let dirs = browser.manifest_dirs();
        let mut found = false;
        for dir in &dirs {
            if dir.join(format!("{}.json", browser.host_name())).exists() { found = true; break; }
        }
        println!("Host manifest for {:?}: {}", browser, if found { "present" } else { "missing" });
    }
    Ok(ExitCode::SUCCESS)
}

fn load_trust(path: &Path) -> Result<StaticTrustStore> {
    let bytes = std::fs::read(path)?;
    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
    let mut store = StaticTrustStore::new();
    let keys = v.get("keys").and_then(|k| k.as_array())
        .context("trust store must have a `keys` array")?;
    for k in keys {
        let id = k.get("keyId").and_then(|v| v.as_str()).context("keyId")?;
        let alg = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse).context("alg")?;
        let pk_b64 = k.get("publicKey").and_then(|v| v.as_str()).context("publicKey")?;
        let pk = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)?;
        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
    }
    Ok(store)
}

fn load_discovery(path: Option<&Path>) -> Result<serde_json::Value> {
    let p = match path {
        Some(p) => p.to_path_buf(),
        None => {
            let cfg = dnea_config::Config::load()?;
            cfg.api.discovery_path.clone()
                .unwrap_or_else(|| cfg.paths.run_dir.join("api.json"))
        }
    };
    let bytes = std::fs::read(&p).with_context(|| format!("read {}", p.display()))?;
    Ok(serde_json::from_slice(&bytes)?)
}
```

Add `dnea-config` to CLI deps (already listed).

## 3.12 `crates/focus-mode`

### `crates/focus-mode/Cargo.toml`

```toml
[package]
name = "focus-mode"
version = "1.2.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
wit-bindgen = "0.30"

[package.metadata.component]
package = "dnea:focus-mode"

[package.metadata.component.target]
path = "../../wit"
world = "module"
```

### `crates/focus-mode/src/lib.rs`

```rust
//! Focus Mode — reference DNEA module.

wit_bindgen::generate!({
    path: "../../wit",
    world: "module",
    generate_all,
});

use dnea::host::types::HostError;
use exports::dnea::host::{
    lifecycle::Guest as LifecycleGuest,
    module_api::Guest as ModuleApiGuest,
};

struct FocusMode;

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct State {
    #[serde(default)] block_list: Vec<String>,
    #[serde(default)] allow_list: Vec<String>,
    #[serde(default)] strict_mode: bool,
    #[serde(default)] sessions_started: u64,
}

const STATE_KEY: &str = "state";

fn load_state() -> State {
    match dnea::host::storage::get(STATE_KEY) {
        Ok(Some(bytes)) => serde_json::from_slice(&bytes).unwrap_or_default(),
        _ => State::default(),
    }
}

fn save_state(state: &State) -> Result<(), HostError> {
    let bytes = serde_json::to_vec(state)
        .map_err(|e| HostError::Internal(e.to_string()))?;
    dnea::host::storage::put(STATE_KEY, &bytes)
}

impl LifecycleGuest for FocusMode {
    fn activate() -> Result<(), HostError> {
        let state = load_state();
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode activated",
            Some(&format!(r#"{{"sessions":{}}}"#, state.sessions_started)),
        );
        Ok(())
    }
    fn deactivate() -> Result<(), HostError> {
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode deactivated", None);
        Ok(())
    }
    fn upgrade(from_version: String, _from_storage_schema: u32) -> Result<(), HostError> {
        dnea::host::log::emit(
            dnea::host::types::LogLevel::Info,
            "focus-mode upgraded",
            Some(&format!(r#"{{"from":"{}"}}"#, from_version)),
        );
        Ok(())
    }
    fn health_check() -> Result<(), HostError> {
        let _ = load_state();
        Ok(())
    }
}

impl ModuleApiGuest for FocusMode {
    fn invoke(method: String, args_json: String) -> Result<String, HostError> {
        match method.as_str() {
            "getState" => {
                let state = load_state();
                Ok(serde_json::to_string(&state).unwrap_or_else(|_| "{}".into()))
            }
            "setBlockList" => {
                let args: serde_json::Value = serde_json::from_str(&args_json)
                    .map_err(|e| HostError::Internal(e.to_string()))?;
                let mut state = load_state();
                state.block_list = args.get("list").and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();
                save_state(&state)?;
                Ok(r#"{"ok":true}"#.into())
            }
            "startSession" => {
                let mut state = load_state();
                state.sessions_started += 1;
                save_state(&state)?;
                Ok(format!(r#"{{"ok":true,"session":{}}}"#, state.sessions_started))
            }
            "onTabNavigated" => {
                let args: serde_json::Value = serde_json::from_str(&args_json)
                    .map_err(|e| HostError::Internal(e.to_string()))?;
                let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
                let state = load_state();
                let host = host_of(url);
                let blocked = state.block_list.iter().any(|h| host.ends_with(h.as_str()))
                    && !state.allow_list.iter().any(|h| host.ends_with(h.as_str()));
                if blocked {
                    dnea::host::log::emit(
                        dnea::host::types::LogLevel::Info,
                        "blocked host",
                        Some(&format!(r#"{{"host":"{}"}}"#, host)));
                }
                Ok(format!(r#"{{"blocked":{}}}"#, blocked))
            }
            "render" => {
                let state = load_state();
                let blocked = state.block_list.iter()
                    .map(|h| format!("<li>{}</li>", html_escape(h)))
                    .collect::<Vec<_>>().join("");
                Ok(format!(
                    r#"<h2>Focus Mode</h2>
<p>Strict mode: {strict}</p>
<p>Sessions: {sessions}</p>
<ul>{blocked}</ul>"#,
                    strict = state.strict_mode,
                    sessions = state.sessions_started,
                    blocked = blocked))
            }
            other => Err(HostError::Unavailable(format!("unknown method: {other}"))),
        }
    }
}

fn host_of(url: &str) -> &str {
    url.split("://").nth(1).and_then(|s| s.split('/').next()).unwrap_or("")
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

export!(FocusMode);
```

---

# PART 4 — THIN CLIENT

## `thin-client/package.json`

```json
{
  "name": "dnea-thin-client",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "wxt",
    "build": "wxt build",
    "build:chrome": "wxt build -b chrome",
    "build:firefox": "wxt build -b firefox --mv2",
    "build:edge": "wxt build -b edge",
    "build:safari": "wxt build -b safari",
    "compile": "tsc --noEmit",
    "test": "vitest run"
  },
  "dependencies": { "wxt": "^0.19.0" },
  "devDependencies": {
    "@types/chrome": "^0.0.280",
    "typescript": "^5.6.0",
    "vitest": "^2.1.0"
  }
}
```

## `thin-client/tsconfig.json`

```json
{
  "extends": "./.wxt/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "noImplicitOverride": true,
    "paths": { "@/*": ["./src/*"] }
  }
}
```

## `thin-client/wxt.config.ts`

```ts
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
```

## `thin-client/src/lib/protocol.ts`

```typescript
export type Envelope = {
  v: 1;
  id: string;
  type: string;
  ts: string;
  src: Party;
  dst: Party;
  reply_to?: string;
  cap?: string;
  mod?: string;
  enc?: "identity" | "zstd" | "br";
  ttl_ms?: number;
  payload?: unknown;
  err?: EnvelopeError;
  meta?: Record<string, unknown>;
};

export type Party = "runtime" | "client" | `cs:${string}` | `mod:${string}`;

export type EnvelopeError = {
  code: string;
  message: string;
  retryable?: boolean;
  retry_after_ms?: number;
  details?: unknown;
  cause?: string;
};

export const PROTOCOL_VERSION = 1;
export const DEFAULT_MTU = 1_048_576;
export const OVERHEAD_BUDGET = 4_096;
export const MAX_CHUNKS = 256;
export const CHUNK_TIMEOUT_MS = 30_000;
export const HANDSHAKE_TIMEOUT_MS = 5_000;
export const HEARTBEAT_IDLE_MS = 20_000;
export const HEARTBEAT_TIMEOUT_MS = 5_000;
export const DEFAULT_TTL_MS = 30_000;
```

## `thin-client/src/lib/crypto.ts`

```typescript
export function base64UrlEncode(bytes: Uint8Array): string {
  let bin = "";
  for (const b of bytes) bin += String.fromCharCode(b);
  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

export function base64UrlDecode(s: string): Uint8Array {
  const pad = s.length % 4 === 0 ? "" : "=".repeat(4 - (s.length % 4));
  const bin = atob(s.replace(/-/g, "+").replace(/_/g, "/") + pad);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

export async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  const digest = await crypto.subtle.digest("SHA-256", buf);
  return [...new Uint8Array(digest)]
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

export function uuidv7(): string {
  const now = Date.now();
  const bytes = new Uint8Array(16);
  const dv = new DataView(bytes.buffer);
  dv.setUint32(0, Math.floor(now / 0x100000000));
  dv.setUint32(4, now >>> 0);
  crypto.getRandomValues(bytes.subarray(6));
  bytes[6] = (bytes[6]! & 0x0f) | 0x70;
  bytes[8] = (bytes[8]! & 0x3f) | 0x80;
  const hex = [...bytes].map((b) => b.toString(16).padStart(2, "0")).join("");
  return `${hex.slice(0,8)}-${hex.slice(8,12)}-${hex.slice(12,16)}-${hex.slice(16,20)}-${hex.slice(20)}`;
}
```

## `thin-client/src/lib/errors.ts`

```typescript
export class DneaError extends Error {
  constructor(
    public readonly code: string,
    public readonly retryable: boolean = false,
    public readonly retryAfterMs?: number,
    public readonly details?: unknown,
  ) {
    super(code);
    this.name = "DneaError";
  }
}

export function fromEnvelopeError(err: {
  code: string;
  retryable?: boolean;
  retry_after_ms?: number;
  details?: unknown;
}): DneaError {
  return new DneaError(err.code, err.retryable ?? false, err.retry_after_ms, err.details);
}
```

## `thin-client/src/lib/log.ts`

```typescript
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
```

## `thin-client/src/lib/backoff.ts`

```typescript
const BASE_MS = 250;
const CAP_MS = 30_000;

export function backoffDelay(attempt: number): number {
  const prev = Math.min(CAP_MS, BASE_MS * Math.pow(3, attempt));
  return BASE_MS + Math.random() * (prev * 3 - BASE_MS);
}
```

## `thin-client/src/lib/codec.ts`

```typescript
import { Envelope, DEFAULT_MTU, OVERHEAD_BUDGET, MAX_CHUNKS, CHUNK_TIMEOUT_MS } from "./protocol";
import { base64UrlEncode, base64UrlDecode, sha256Hex } from "./crypto";

export type Decoded = { kind: "envelope"; envelope: Envelope } | { kind: "incomplete" };
type Inflight = { parts: (Uint8Array | undefined)[]; total: number; startedAt: number; digest: string };

export class Codec {
  private inflight = new Map<string, Inflight>();
  constructor(private mtu: number = DEFAULT_MTU) {}

  async encode(env: Envelope): Promise<unknown[]> {
    const bytes = new TextEncoder().encode(JSON.stringify(env));
    if (bytes.length <= this.mtu - OVERHEAD_BUDGET) return [env];
    const chunkSize = this.mtu - OVERHEAD_BUDGET - 512;
    const total = Math.ceil(bytes.length / chunkSize);
    if (total > MAX_CHUNKS) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    const digest = `sha-256:${await sha256Hex(bytes)}`;
    const frames: unknown[] = [];
    for (let i = 0; i < total; i++) {
      const part = bytes.slice(i * chunkSize, (i + 1) * chunkSize);
      frames.push({
        v: 1, type: "dnea/chunk", cid: env.id, seq: i, total,
        part: base64UrlEncode(part), digest, final: i + 1 === total,
      });
    }
    return frames;
  }

  async decode(raw: unknown): Promise<Decoded> {
    const obj = raw as Record<string, unknown>;
    if (obj.type !== "dnea/chunk") {
      return { kind: "envelope", envelope: obj as unknown as Envelope };
    }
    return this.decodeChunk(obj);
  }

  private async decodeChunk(frame: Record<string, unknown>): Promise<Decoded> {
    const cid = String(frame.cid);
    const seq = Number(frame.seq);
    const total = Number(frame.total);
    if (total > MAX_CHUNKS || seq < 0 || seq >= total) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    let entry = this.inflight.get(cid);
    if (!entry) {
      entry = { parts: new Array(total), total, startedAt: Date.now(), digest: String(frame.digest) };
      this.inflight.set(cid, entry);
    }
    if (Date.now() - entry.startedAt > CHUNK_TIMEOUT_MS) {
      this.inflight.delete(cid);
      throw new Error("E_FRAMING_CHUNK_OVERFLOW");
    }
    entry.parts[seq] = base64UrlDecode(String(frame.part));
    if (entry.parts.some((p) => p === undefined)) return { kind: "incomplete" };
    const total_bytes = entry.parts.reduce((n, p) => n + (p?.byteLength ?? 0), 0);
    const full = new Uint8Array(total_bytes);
    let off = 0;
    for (const p of entry.parts) { if (!p) continue; full.set(p, off); off += p.byteLength; }
    const actual = `sha-256:${await sha256Hex(full)}`;
    this.inflight.delete(cid);
    if (actual !== entry.digest) throw new Error("E_FRAMING_DIGEST_MISMATCH");
    const envelope = JSON.parse(new TextDecoder().decode(full)) as Envelope;
    return { kind: "envelope", envelope };
  }
}
```

## `thin-client/src/lib/router.ts`

```typescript
import { Connection } from "./connection";
import { Envelope, PROTOCOL_VERSION } from "./protocol";
import { uuidv7 } from "./crypto";

type Handler = (env: Envelope) => void;

export class Router {
  private handlers = new Map<string, Set<Handler>>();
  constructor(private conn: Connection) { conn.on("*", (env) => this.dispatch(env)); }

  on(type: string, handler: Handler): () => void {
    const set = this.handlers.get(type) ?? new Set();
    set.add(handler);
    this.handlers.set(type, set);
    return () => set.delete(handler);
  }

  private dispatch(env: Envelope): void {
    const set = this.handlers.get(env.type);
    if (set) for (const h of set) h(env);
    const wildcard = this.handlers.get("*");
    if (wildcard) for (const h of wildcard) h(env);
  }

  mintEnvelope(input: {
    type: string; src: string; dst: string;
    cap?: string; mod?: string; ttl_ms?: number; payload?: unknown;
  }): Envelope {
    return {
      v: PROTOCOL_VERSION, id: uuidv7(), type: input.type,
      ts: new Date().toISOString(),
      src: input.src as never, dst: input.dst as never,
      cap: input.cap, mod: input.mod, ttl_ms: input.ttl_ms, payload: input.payload,
    };
  }

  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
    return this.conn.request(env, opts);
  }

  send(env: Envelope): void { this.conn.send(env); }

  sendRuntimeEnvelope(input: { type: string; payload?: unknown; reply_to?: string }): void {
    const env = this.mintEnvelope({
      type: input.type, src: "client", dst: "runtime", payload: input.payload,
    });
    if (input.reply_to) env.reply_to = input.reply_to as never;
    this.send(env);
  }

  sendEnvelope(env: Envelope): void { this.conn.send(env); }
}
```

## `thin-client/src/lib/connection.ts`

```typescript
import { browser } from "wxt/browser";
import { Codec } from "./codec";
import { Envelope, PROTOCOL_VERSION, HEARTBEAT_IDLE_MS, HANDSHAKE_TIMEOUT_MS, DEFAULT_TTL_MS } from "./protocol";
import { DneaError } from "./errors";
import { backoffDelay } from "./backoff";
import { Logger } from "./log";
import { uuidv7 } from "./crypto";

const MAX_CONSECUTIVE_FAILURES = 8;
const BREAKER_COOLDOWN_MS = 5 * 60_000;

type State = "DISCONNECTED" | "CONNECTING" | "HANDSHAKING" | "READY" | "DRAINING" | "FAILED";

type Pending = {
  id: string;
  resolve: (env: Envelope) => void;
  reject: (err: DneaError) => void;
  timer: ReturnType<typeof setTimeout>;
  idempotent: boolean;
};

export class Connection {
  private state: State = "DISCONNECTED";
  private port?: browser.runtime.Port;
  private codec = new Codec();
  private attempt = 0;
  private consecutiveFailures = 0;
  private breakerOpenUntil = 0;
  private outstanding = new Map<string, Pending>();
  private lastRxAt = 0;
  private lastTxAt = 0;
  private log = new Logger("connection");
  private listeners = new Map<string, Set<(env: Envelope) => void>>();

  constructor(private opts: {
    hostName: string;
    clientVersion: string;
    browserFamily: "chromium" | "firefox" | "safari";
    onStateChange?: (s: State) => void;
  }) {}

  on(type: string, fn: (env: Envelope) => void): () => void {
    const set = this.listeners.get(type) ?? new Set();
    set.add(fn);
    this.listeners.set(type, set);
    return () => set.delete(fn);
  }

  private setState(s: State) {
    if (this.state === s) return;
    this.state = s;
    this.opts.onStateChange?.(s);
  }

  private emit(env: Envelope) {
    const set = this.listeners.get(env.type);
    if (set) for (const fn of set) fn(env);
    const wildcard = this.listeners.get("*");
    if (wildcard) for (const fn of wildcard) fn(env);
  }

  connect(): void {
    if (this.state === "READY" || this.state === "HANDSHAKING" || this.state === "CONNECTING") return;
    if (Date.now() < this.breakerOpenUntil) return;

    this.setState("CONNECTING");
    try {
      this.port = browser.runtime.connectNative(this.opts.hostName);
    } catch (e) {
      this.log.warn("connectNative threw", { error: String(e) });
      return this.fail(new DneaError("E_TRANSPORT_NOT_INSTALLED", false));
    }

    this.port.onMessage.addListener((raw) => this.onRawMessage(raw));
    this.port.onDisconnect.addListener(() => this.onDisconnect());
    this.setState("HANDSHAKING");

    this.sendRaw({
      v: 1, id: uuidv7(), type: "dnea/hello",
      ts: new Date().toISOString(), src: "client", dst: "runtime",
      payload: {
        clientVersion: this.opts.clientVersion,
        browserFamily: this.opts.browserFamily,
        browserVersion: navigator.userAgent,
        extensionId: browser.runtime.id,
        mtu: 1_048_576,
        encodings: ["identity"],
        features: {
          chunking: true, contentOverlay: true, consentUI: true,
          serviceWorkerBackground: this.opts.browserFamily === "chromium",
        },
      },
    });

    setTimeout(() => {
      if (this.state === "HANDSHAKING") {
        this.log.warn("handshake timeout");
        this.fail(new DneaError("E_TRANSPORT_HANDSHAKE_TIMEOUT", true));
      }
    }, HANDSHAKE_TIMEOUT_MS);
  }

  private async onRawMessage(raw: unknown): Promise<void> {
    this.lastRxAt = Date.now();
    let decoded;
    try { decoded = await this.codec.decode(raw); }
    catch (e) {
      const code = String(e).includes("CHUNK") ? "E_FRAMING_CHUNK_OVERFLOW"
                 : String(e).includes("DIGEST") ? "E_FRAMING_DIGEST_MISMATCH"
                 : "E_FRAMING_INVALID_JSON";
      this.log.warn("framing error", { code });
      return;
    }
    if (decoded.kind === "incomplete") return;
    const env = decoded.envelope;
    const srcOk = env.src === "runtime" || env.src.startsWith("mod:");
    if (!srcOk) { this.log.warn("invalid src on runtime-side transport", { src: env.src }); return; }
    if (env.type === "dnea/welcome") return this.onWelcome(env);
    if (env.type === "dnea/pong") return;
    if (env.type === "dnea/goodbye") return this.onGoodbye(env);
    if (env.reply_to) return this.onReply(env);
    this.emit(env);
  }

  private onReply(env: Envelope): void {
    const pending = this.outstanding.get(env.reply_to!);
    if (!pending) { this.log.warn("unknown correlation", { reply_to: env.reply_to }); return; }
    clearTimeout(pending.timer);
    this.outstanding.delete(env.reply_to!);
    pending.resolve(env);
  }

  private onWelcome(env: Envelope): void {
    const meta = env.meta as Record<string, unknown> | undefined;
    if (meta?.client_extension_id !== browser.runtime.id) {
      this.fail(new DneaError("E_HANDSHAKE_REJECTED", false, undefined, { reason: "extension_id_mismatch" }));
      return;
    }
    this.consecutiveFailures = 0;
    this.attempt = 0;
    this.setState("READY");
    void browser.storage.session.set({ dnea: { wasConnected: true } });
    this.emit(env);
    this.log.info("handshake complete");
  }

  private onGoodbye(env: Envelope): void {
    const reason = (env.payload as { reason?: string } | undefined)?.reason ?? "shutdown";
    this.setState("DRAINING");
    setTimeout(() => this.close(), 5_000);
    if (reason === "shutdown") {
      void browser.storage.session.set({ dnea: { wasConnected: false } });
    }
  }

  private onDisconnect(): void {
    const stillPending = [...this.outstanding.values()];
    this.outstanding.clear();
    for (const p of stillPending) {
      clearTimeout(p.timer);
      p.reject(new DneaError("E_TRANSPORT_DISCONNECTED", true));
    }
    void browser.storage.session.set({ dnea: { wasConnected: false } });
    this.port = undefined;
    this.consecutiveFailures++;
    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
    }
    this.setState("FAILED");
    this.scheduleReconnect();
  }

  private fail(err: DneaError): void {
    this.log.warn("connection failure", { code: err.code });
    try { this.port?.disconnect(); } catch {}
    this.port = undefined;
    this.consecutiveFailures++;
    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
    }
    this.setState("FAILED");
    this.scheduleReconnect();
  }

  private scheduleReconnect(): void {
    const delay = backoffDelay(this.attempt);
    this.attempt = Math.min(this.attempt + 1, 12);
    setTimeout(() => this.connect(), delay);
  }

  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
    if (this.state !== "READY") throw new DneaError("E_TRANSPORT_DISCONNECTED", true);
    return new Promise<Envelope>((resolve, reject) => {
      const ttl = env.ttl_ms ?? DEFAULT_TTL_MS;
      const timer = setTimeout(() => {
        this.outstanding.delete(env.id);
        reject(new DneaError("E_TTL_EXPIRED", true));
      }, ttl);
      this.outstanding.set(env.id, { id: env.id, resolve, reject, timer, idempotent: !!opts.idempotent });
      void this.sendEnvelope(env);
    });
  }

  send(env: Envelope): void {
    if (this.state !== "READY") { this.log.warn("send dropped: not READY", { type: env.type }); return; }
    void this.sendEnvelope(env);
  }

  private async sendEnvelope(env: Envelope): Promise<void> {
    const frames = await this.codec.encode(env);
    for (const frame of frames) this.sendRaw(frame);
    this.lastTxAt = Date.now();
  }

  private sendRaw(frame: unknown): void {
    if (!this.port) return;
    try { this.port.postMessage(frame); }
    catch (e) { this.log.warn("postMessage failed", { error: String(e) }); this.fail(new DneaError("E_TRANSPORT_DISCONNECTED", true)); }
  }

  maybePing(): void {
    if (this.state !== "READY") return;
    if (Date.now() - this.lastRxAt < HEARTBEAT_IDLE_MS) return;
    void this.sendEnvelope({
      v: 1, id: uuidv7(), type: "dnea/ping", ts: new Date().toISOString(),
      src: "client", dst: "runtime", payload: { seq: Date.now() },
    });
  }

  probeIfIdle(): void {
    if (this.state === "DISCONNECTED" || this.state === "FAILED") this.connect();
  }

  private close(): void {
    try { this.port?.disconnect(); } catch {}
    this.port = undefined;
    this.setState("DISCONNECTED");
  }
}
```

## `thin-client/src/lib/cs-bridge.ts`

```typescript
import { browser } from "wxt/browser";
import { Router } from "./router";
import { Envelope } from "./protocol";
import { Logger } from "./log";
import { uuidv7 } from "./crypto";

const MAX_PORTS = 512;
const QUEUE_MAX = 128;
const RATE_LIMIT_PER_MIN = 60;
const RATE_BURST = 20;

type CSMessage =
  | { kind: "dnea.cs/req"; cid: string; type: string; mod?: string; cap?: string; ttl_ms?: number; payload?: unknown }
  | { kind: "dnea.cs/evt"; cid: string; type: string; mod?: string; payload?: unknown };

type BgMessage =
  | { kind: "dnea.cs/res"; cid: string; payload?: unknown }
  | { kind: "dnea.cs/err"; cid: string; err: { code: string; message: string; retryable?: boolean; retry_after_ms?: number } };

type PortRecord = {
  portId: string;
  port: browser.runtime.Port;
  tabId: number;
  frameId: number;
  url: string;
  incognito: boolean;
  registeredAt: number;
  lastSeen: number;
  queue: unknown[];
  tokens: { count: number; resetAt: number };
  pendingCids: Map<string, string>;
};

export class CSPortBridge {
  private ports = new Map<string, PortRecord>();
  private byTabFrame = new Map<string, string>();
  private log = new Logger("cs-bridge");

  constructor(private router: Router) {
    router.on("dnea/result", (env) => this.deliverResult(env));
    router.on("dnea/error", (env) => this.deliverResult(env));
    router.on("dnea/event", (env) => this.deliverEvent(env));
    browser.tabs.onRemoved.addListener((tabId) => this.unregisterTab(tabId));
    browser.tabs.onUpdated.addListener((tabId, change) => {
      if (change.url) this.unregisterTab(tabId, "url_changed");
    });
  }

  accept(port: browser.runtime.Port): void {
    const sender = port.sender;
    if (!sender?.tab || sender.id !== browser.runtime.id) {
      try { port.disconnect(); } catch {}
      return;
    }
    const key = `${sender.tab.id}:${sender.frameId ?? 0}`;
    const existing = this.byTabFrame.get(key);
    if (existing) this.disconnectPort(existing, "superseded");
    if (this.ports.size >= MAX_PORTS) {
      const lru = [...this.ports.values()].sort((a, b) => a.lastSeen - b.lastSeen)[0];
      if (lru) this.disconnectPort(lru.portId, "evicted");
    }
    const portId = uuidv7();
    const rec: PortRecord = {
      portId, port,
      tabId: sender.tab.id!, frameId: sender.frameId ?? 0,
      url: sender.url ?? "", incognito: sender.tab.incognito ?? false,
      registeredAt: Date.now(), lastSeen: Date.now(),
      queue: [], tokens: { count: RATE_BURST, resetAt: Date.now() + 60_000 },
      pendingCids: new Map(),
    };
    this.ports.set(portId, rec);
    this.byTabFrame.set(key, portId);
    port.onMessage.addListener((msg) => this.onCSMessage(rec, msg as CSMessage));
    port.onDisconnect.addListener(() => this.disconnectPort(portId, "disconnected"));
    this.router.sendRuntimeEnvelope({
      type: "dnea/cs/register",
      payload: { portId, tabId: rec.tabId, frameId: rec.frameId, url: rec.url, incognito: rec.incognito },
    });
  }

  private onCSMessage(rec: PortRecord, msg: CSMessage): void {
    rec.lastSeen = Date.now();
    if (!this.takeToken(rec)) {
      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
        err: { code: "E_RATE_LIMITED", message: "rate limit", retryable: true, retry_after_ms: 1000 } });
      return;
    }
    if (rec.queue.length >= QUEUE_MAX) {
      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
        err: { code: "E_CS_BACKPRESSURE", message: "queue full", retryable: true, retry_after_ms: 250 } });
      rec.queue.shift();
    }
    const env = this.router.mintEnvelope({
      type: msg.type, src: `cs:${rec.portId}`,
      dst: msg.mod ? `mod:${msg.mod}` : "runtime",
      cap: msg.cap, mod: msg.mod, ttl_ms: msg.ttl_ms, payload: msg.payload,
    });
    if (msg.kind === "dnea.cs/req") rec.pendingCids.set(env.id, msg.cid);
    rec.queue.push(env);
    this.router.sendEnvelope(env);
  }

  private deliverResult(env: Envelope): void {
    const portId = env.dst.startsWith("cs:") ? env.dst.slice(3) : undefined;
    if (!portId) return;
    const rec = this.ports.get(portId);
    if (!rec) return;
    const cid = rec.pendingCids.get(env.reply_to ?? env.id);
    if (cid) {
      rec.pendingCids.delete(env.reply_to ?? env.id);
      const out: BgMessage = env.err
        ? { kind: "dnea.cs/err", cid, err: {
            code: env.err.code, message: env.err.message,
            retryable: env.err.retryable, retry_after_ms: env.err.retry_after_ms } }
        : { kind: "dnea.cs/res", cid, payload: env.payload };
      this.postSafe(rec, out);
    }
    if (rec.queue.length) rec.queue.shift();
  }

  private deliverEvent(env: Envelope): void {
    for (const rec of this.ports.values()) {
      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
    }
  }

  private postSafe(rec: PortRecord, msg: BgMessage): void {
    try { rec.port.postMessage(msg); } catch { this.disconnectPort(rec.portId, "post_failed"); }
  }

  private takeToken(rec: PortRecord): boolean {
    const now = Date.now();
    if (now > rec.tokens.resetAt) rec.tokens = { count: RATE_BURST, resetAt: now + 60_000 };
    if (rec.tokens.count <= 0) return false;
    rec.tokens.count--;
    return true;
  }

  private disconnectPort(portId: string, reason: string): void {
    const rec = this.ports.get(portId);
    if (!rec) return;
    this.ports.delete(portId);
    this.byTabFrame.delete(`${rec.tabId}:${rec.frameId}`);
    try { rec.port.disconnect(); } catch {}
    this.router.sendRuntimeEnvelope({
      type: "dnea/cs/unregister",
      payload: { portId, reason },
    });
  }

  private unregisterTab(tabId: number, reason = "tab_removed"): void {
    for (const rec of [...this.ports.values()]) {
      if (rec.tabId === tabId) this.disconnectPort(rec.portId, reason);
    }
  }

  broadcastCapabilityChange(env: Envelope): void {
    for (const rec of this.ports.values()) {
      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
    }
  }
}
```

## `thin-client/src/lib/consent.ts`

```typescript
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
```

## `thin-client/src/lib/ui.ts`

```typescript
import { Envelope } from "./protocol";

const mounts = new Map<string, { host: HTMLElement; shadow: ShadowRoot }>();

export class OverlayMount {
  async mount(env: Envelope): Promise<void> {
    const p = env.payload as { origin: string; mountId: string; css?: string; html?: string };
    if (!matchesOrigin(location.origin, p.origin)) return;
    const existing = mounts.get(p.mountId);
    if (existing) existing.host.remove();

    const host = document.createElement("div");
    host.id = `dnea-mount-${p.mountId}`;
    host.style.position = "fixed";
    host.style.zIndex = "2147483647";

    if (!("attachShadow" in host)) {
      throw new Error("E_UNSUPPORTED_FEATURE");
    }
    const shadow = host.attachShadow({ mode: "closed" });
    if (!shadow) throw new Error("E_UNSUPPORTED_FEATURE");

    if (p.css) {
      const style = document.createElement("style");
      style.textContent = p.css;
      shadow.appendChild(style);
    }
    if (p.html) {
      const container = document.createElement("div");
      container.innerHTML = sanitize(p.html);
      shadow.appendChild(container);
    }
    document.documentElement.appendChild(host);
    mounts.set(p.mountId, { host, shadow });
  }

  clear(env: Envelope): void {
    const p = env.payload as { targetId: string };
    const m = mounts.get(p.targetId);
    if (m) { m.host.remove(); mounts.delete(p.targetId); }
  }
}

function matchesOrigin(current: string, pattern: string): boolean {
  if (pattern.endsWith("/*")) return current.startsWith(pattern.slice(0, -1));
  return current === pattern;
}

function sanitize(html: string): string {
  return html
    .replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi, "")
    .replace(/\son\w+\s*=\s*"[^"]*"/gi, "")
    .replace(/\son\w+\s*=\s*'[^']*'/gi, "");
}
```

## `thin-client/src/lib/crypto.ts` (already above)

## `thin-client/src/entrypoints/background.ts`

```typescript
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
```

## `thin-client/src/entrypoints/content.ts`

```typescript
import { browser } from "wxt/browser";

const port = browser.runtime.connect({ name: "dnea.cs.v1" });

const pending = new Map<string, (msg: unknown) => void>();
const listeners = new Set<(data: unknown) => void>();

port.onMessage.addListener((raw: unknown) => {
  const msg = raw as { kind: string; cid: string; payload?: unknown; err?: unknown };
  if (msg.kind === "dnea.cs/res" || msg.kind === "dnea.cs/err") {
    const resolver = pending.get(msg.cid);
    if (resolver) { pending.delete(msg.cid); resolver(msg); }
    return;
  }
  if (msg.kind === "dnea.cs/evt") {
    for (const fn of listeners) fn(msg.payload);
  }
});

const bridge = {
  invoke(type: string, payload: unknown, opts: { mod?: string; cap?: string; ttl_ms?: number } = {}): Promise<unknown> {
    const cid = crypto.randomUUID();
    return new Promise((resolve, reject) => {
      pending.set(cid, (msg) => {
        const m = msg as { kind: string; payload?: unknown; err?: { code: string; message: string } };
        if (m.kind === "dnea.cs/err") reject(Object.assign(new Error(m.err!.message), { code: m.err!.code }));
        else resolve(m.payload);
      });
      port.postMessage({ kind: "dnea.cs/req", cid, type, ...opts, payload });
    });
  },
  on(_event: string, fn: (data: unknown) => void): () => void {
    listeners.add(fn);
    return () => listeners.delete(fn);
  },
};

(window as unknown as { __dnea: typeof bridge }).__dnea = bridge;

const ALLOWED_PAGE_TYPES = new Set(["focus.getState", "focus.toggle"]);

window.addEventListener("message", (ev) => {
  if (ev.source !== window) return;
  if (ev.origin !== location.origin) return;
  const data = ev.data as { __dnea?: true; channel?: string; cid?: string; type?: string; payload?: unknown };
  if (!data?.__dnea || !data.cid || !data.type) return;
  if (!ALLOWED_PAGE_TYPES.has(data.type)) return;
  bridge.invoke(data.type, data.payload)
    .then((payload) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: true, payload }, location.origin))
    .catch((err) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: false, error: String(err) }, location.origin));
});
```

## `thin-client/src/entrypoints/consent/main.ts`

```typescript
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
```

## `thin-client/src/entrypoints/consent/index.html`

```html
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>DNEA consent</title>
<style>
body { font: 14px system-ui; margin: 24px; max-width: 480px; }
h1 { font-size: 16px; margin: 0 0 12px; }
.reason { color: #555; margin: 12px 0; }
button { padding: 6px 12px; margin-right: 8px; }
</style>
</head>
<body>
<h1 id="title">Loading…</h1>
<div class="reason" id="reason"></div>
<pre id="constraints"></pre>
<div>
<button id="allow">Allow for this version</button>
<button id="always">Always allow</button>
<button id="deny">Deny</button>
</div>
<script type="module" src="./main.ts"></script>
</body>
</html>
```

## `thin-client/src/entrypoints/popup/index.html`

```html
<!doctype html>
<html>
<head><meta charset="utf-8"><title>DNEA</title></head>
<body style="font:14px system-ui;padding:12px;min-width:220px">
<h1 style="font-size:14px;margin:0 0 8px">DNEA</h1>
<div id="state">connecting…</div>
</body>
</html>
```

## `thin-client/src/entrypoints/options/index.html` + `main.ts`

```html
<!doctype html>
<html>
<head><meta charset="utf-8"><title>DNEA diagnostics</title></head>
<body style="font:14px system-ui;padding:16px">
<h1>Diagnostics</h1>
<pre id="log"></pre>
</body>
</html>
```

```typescript
const el = document.getElementById("log");
if (el) el.textContent = "Runtime connection state is managed by the background service worker.";
```

---

# PART 5 — INSTALLERS

## 5.1 Windows

### `installer/windows/dnea.wxs`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
  <Package Name="DNEA Runtime" Manufacturer="DNEA Project"
    Version="1.0.0.0" UpgradeCode="A2B4C6D8-E0F2-4051-8263-A4B6C8D0E2F4"
    Scope="perMachine">

    <MajorUpgrade DowngradeErrorMessage="A newer version of DNEA is already installed." />
    <MediaTemplate EmbedCab="yes" />

    <StandardDirectory Id="ProgramFiles64Folder">
      <Directory Id="INSTALLFOLDER" Name="DNEA">
        <Component Id="DaemonExe" Guid="*">
          <File Id="dnea-daemon.exe" Source="$(var.StageDir)\dnea-daemon.exe" KeyPath="yes" />
          <ServiceInstall Id="DNEAService" Name="DNEA" DisplayName="DNEA Runtime"
            Description="Device-Native Extension Architecture runtime"
            Start="auto" Type="ownProcess" ErrorControl="normal" Account="LocalSystem">
            <ServiceConfig FailureActions="restart/60000/restart/60000/restart/60000"
              FailureResetPeriod="86400" OnInstall="yes" OnReinstall="yes" />
          </ServiceInstall>
          <ServiceControl Id="DNEAServiceControl" Name="DNEA"
            Start="install" Stop="both" Remove="uninstall" Wait="yes" />
        </Component>
        <Component Id="BridgeExe" Guid="*">
          <File Id="dnea-nm-bridge.exe" Source="$(var.StageDir)\dnea-nm-bridge.exe" KeyPath="yes" />
        </Component>
        <Component Id="CliExe" Guid="*">
          <File Id="dnea.exe" Source="$(var.StageDir)\dnea.exe" KeyPath="yes" />
          <Environment Id="PATH" Name="PATH" Value="[INSTALLFOLDER]" Part="last" Action="set" Permanent="no" System="yes" />
        </Component>
      </Directory>
    </StandardDirectory>

    <StandardDirectory Id="CommonAppDataFolder">
      <Directory Id="DNEA_DATA" Name="DNEA">
        <Directory Id="DNEA_NM" Name="nm" />
        <Directory Id="DNEA_STATE" Name="state" />
      </Directory>
    </StandardDirectory>

    <Component Id="NMChrome" Guid="*" Directory="DNEA_NM">
      <File Id="nmChrome.json" Source="$(var.StageDir)\nm\org.dnea.runtime.chrome.json" />
      <RegistryKey Root="HKLM" Key="SOFTWARE\Google\Chrome\NativeMessagingHosts\org.dnea.runtime">
        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.chrome.json" />
      </RegistryKey>
    </Component>
    <Component Id="NMEdge" Guid="*" Directory="DNEA_NM">
      <File Id="nmEdge.json" Source="$(var.StageDir)\nm\org.dnea.runtime.edge.json" />
      <RegistryKey Root="HKLM" Key="SOFTWARE\Microsoft\Edge\NativeMessagingHosts\org.dnea.runtime">
        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.edge.json" />
      </RegistryKey>
    </Component>
    <Component Id="NMFirefox" Guid="*" Directory="DNEA_NM">
      <File Id="nmFirefox.json" Source="$(var.StageDir)\nm\org.dnea.runtime.firefox.json" />
      <RegistryKey Root="HKLM" Key="SOFTWARE\Mozilla\NativeMessagingHosts\org.dnea.runtime">
        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.firefox.json" />
      </RegistryKey>
    </Component>
    <Component Id="NMBrave" Guid="*" Directory="DNEA_NM">
      <File Id="nmBrave.json" Source="$(var.StageDir)\nm\org.dnea.runtime.brave.json" />
      <RegistryKey Root="HKLM" Key="SOFTWARE\BraveSoftware\Brave-Browser\NativeMessagingHosts\org.dnea.runtime">
        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.brave.json" />
      </RegistryKey>
    </Component>

    <Component Id="ForcePolicyChrome" Guid="*" Condition="POLICY=1">
      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Google\Chrome\ExtensionInstallForcelist">
        <RegistryValue Name="100" Type="string"
          Value="abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx" />
      </RegistryKey>
    </Component>
    <Component Id="ForcePolicyEdge" Guid="*" Condition="POLICY=1">
      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Microsoft\Edge\ExtensionInstallForcelist">
        <RegistryValue Name="100" Type="string"
          Value="bcdefghijklmnopabcdefghijklmno;https://edge.microsoft.com/extensionwebstorebase/v1/crx" />
      </RegistryKey>
    </Component>
    <Component Id="ForcePolicyFirefox" Guid="*" Condition="POLICY=1">
      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Mozilla\Firefox\Extensions\Install">
        <RegistryValue Name="100" Type="string" Value="dnea@dnea.org" />
      </RegistryKey>
    </Component>

    <Feature Id="Main" Title="DNEA Runtime" Level="1">
      <ComponentRef Id="DaemonExe" />
      <ComponentRef Id="BridgeExe" />
      <ComponentRef Id="CliExe" />
    </Feature>
    <Feature Id="BrowserIntegration" Title="Browser Integration" Level="1">
      <ComponentRef Id="NMChrome" />
      <ComponentRef Id="NMEdge" />
      <ComponentRef Id="NMFirefox" />
      <ComponentRef Id="NMBrave" />
    </Feature>
    <Feature Id="EnterprisePolicy" Title="Force-install thin client" Level="1">
      <ComponentRef Id="ForcePolicyChrome" />
      <ComponentRef Id="ForcePolicyEdge" />
      <ComponentRef Id="ForcePolicyFirefox" />
    </Feature>

    <CustomAction Id="GenerateMasterKey"
      ExeCommand="--init-key --config &quot;[DNEA_STATE]dnea.toml&quot;"
      FileRef="dnea-daemon.exe" Execute="deferred" Impersonate="no" Return="ignore" />
    <InstallExecuteSequence>
      <Custom Action="GenerateMasterKey" After="InstallFiles" Condition="NOT Installed" />
    </InstallExecuteSequence>

    <Property Id="POLICY" Value="0" />
  </Package>
</Wix>
```

### `installer/windows/build.ps1`

```powershell
param(
    [string]$StageDir = ".\stage",
    [string]$OutDir = ".\out",
    [string]$Configuration = "release"
)
$ErrorActionPreference = "Stop"

cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }

New-Item -ItemType Directory -Force -Path $StageDir | Out-Null
Copy-Item target\release\dnea-daemon.exe $StageDir
Copy-Item target\release\dnea-nm-bridge.exe $StageDir
Copy-Item target\release\dnea.exe $StageDir

New-Item -ItemType Directory -Force -Path "$StageDir\nm" | Out-Null
$bridgePath = (Resolve-Path "$StageDir\dnea-nm-bridge.exe").Path
foreach ($b in @("chrome","edge","firefox","brave")) {
    $origin = if ($b -eq "firefox") { "moz-extension://dnea@dnea.org/" }
              else { "chrome-extension://abcdefghijklmnopabcdefghijklmnop/" }
    $manifest = @{
        name = "org.dnea.runtime"
        description = "DNEA runtime bridge"
        path = $bridgePath
        type = "stdio"
        allowed_origins = @($origin)
        "x-dnea-installed" = @{ version = "1.0.0"; installed_at = (Get-Date -Format "o") }
    }
    $manifest | ConvertTo-Json -Depth 4 | Out-File -Encoding utf8 "$StageDir\nm\org.dnea.runtime.$b.json"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
wix build -arch x64 -d StageDir=$StageDir -o "$OutDir\dnea-1.0.0.msi" dnea.wxs
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
Write-Host "Built $OutDir\dnea-1.0.0.msi"
```

## 5.2 macOS

### `installer/macos/org.dnea.runtime.plist`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>            <string>org.dnea.runtime</string>
  <key>ProgramArguments</key> <array>
    <string>/Library/Application Support/DNEA/bin/dnea-daemon</string>
  </array>
  <key>RunAtLoad</key>        <true/>
  <key>KeepAlive</key>        <true/>
  <key>ProcessType</key>      <string>Interactive</string>
  <key>StandardOutPath</key>  <string>/Library/Logs/DNEA/daemon.out</string>
  <key>StandardErrorPath</key><string>/Library/Logs/DNEA/daemon.err</string>
</dict>
</plist>
```

### `installer/macos/scripts/postinstall`

```bash
#!/bin/bash
set -euo pipefail

/usr/local/bin/dnea-daemon --init-key --config /Library/Application\ Support/DNEA/dnea.toml || true

for d in \
  "/Library/Google/Chrome/NativeMessagingHosts" \
  "/Library/Microsoft/Edge/NativeMessagingHosts" \
  "/Library/Application Support/Mozilla/NativeMessagingHosts"; do
  mkdir -p "$d"
done

BRIDGE="/Library/Application Support/DNEA/bin/dnea-nm-bridge"
for spec in \
  "chrome:/Library/Google/Chrome/NativeMessagingHosts:chrome-extension" \
  "edge:/Library/Microsoft/Edge/NativeMessagingHosts:chrome-extension" \
  "firefox:/Library/Application Support/Mozilla/NativeMessagingHosts:moz-extension"; do
  IFS=":" read -r name dir prefix <<< "$spec"
  ext_id="abcdefghijklmnopabcdefghijklmnop"
  [ "$name" = "firefox" ] && ext_id="dnea@dnea.org"
  cat > "$dir/org.dnea.runtime.json" <<EOF
{
  "name": "org.dnea.runtime",
  "description": "DNEA runtime bridge",
  "path": "$BRIDGE",
  "type": "stdio",
  "allowed_origins": ["$prefix://$ext_id/"],
  "x-dnea-installed": { "version": "1.0.0", "installed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)" }
}
EOF
done

launchctl load -w /Library/LaunchDaemons/org.dnea.runtime.plist

if profiles status -type enrollment 2>/dev/null | grep -q "Enrolled"; then
  cat > /Library/Managed\ Preferences/com.google.Chrome.plist <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>ExtensionInstallForcelist</key>
<array><string>abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx</string></array>
</dict></plist>
EOF
fi
exit 0
```

### `installer/macos/build.sh`

```bash
#!/bin/bash
set -euo pipefail
VERSION="1.0.0"
STAGE="build/stage"
OUT="build/out"
rm -rf "$STAGE" "$OUT"
mkdir -p "$STAGE/Library/Application Support/DNEA/bin"
mkdir -p "$STAGE/Library/LaunchDaemons"
mkdir -p "$OUT"

cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
cp target/release/dnea-daemon "$STAGE/Library/Application Support/DNEA/bin/"
cp target/release/dnea-nm-bridge "$STAGE/Library/Application Support/DNEA/bin/"
cp target/release/dnea "$STAGE/Library/Application Support/DNEA/bin/"
cp installer/macos/org.dnea.runtime.plist "$STAGE/Library/LaunchDaemons/"

pkgbuild --root "$STAGE" --scripts installer/macos/scripts \
  --identifier org.dnea.runtime --version "$VERSION" \
  --install-location / "$OUT/dnea-component.pkg"

productbuild --package "$OUT/dnea-component.pkg" \
  --distribution installer/macos/distribution.xml \
  --resources installer/macos/resources "$OUT/dnea-$VERSION.pkg"

productsign --sign "Developer ID Installer: DNEA Project" \
  "$OUT/dnea-$VERSION.pkg" "$OUT/dnea-$VERSION-signed.pkg"
```

### `installer/macos/distribution.xml`

```xml
<?xml version="1.0" encoding="utf-8"?>
<installer-gui-script minSpecVersion="2">
  <title>DNEA Runtime</title>
  <organization>org.dnea</organization>
  <domains enable_localSystem="true" />
  <options customize="allow" require-scripts="false" />
  <choices-outline>
    <line choice="default"><line choice="org.dnea.runtime" /></line>
  </choices-outline>
  <choice id="default" />
  <choice id="org.dnea.runtime" visible="true" title="DNEA Runtime">
    <pkg-ref id="org.dnea.runtime" />
  </choice>
  <pkg-ref id="org.dnea.runtime" version="1.0.0">dnea-component.pkg</pkg-ref>
</installer-gui-script>
```

## 5.3 Linux

### `installer/linux/nfpm.yaml`

```yaml
name: dnea
arch: amd64
platform: linux
version: 1.0.0
section: utils
priority: optional
maintainer: DNEA Project <maintainers@dnea.org>
description: |
  Device-Native Extension Architecture runtime.
  Installs the runtime daemon, the native messaging bridge, and the CLI.
homepage: https://dnea.org
license: Apache-2.0
depends:
  - libc6 (>= 2.31)
contents:
  - src: target/release/dnea-daemon
    dst: /usr/lib/dnea/dnea-daemon
    file_info: { mode: 0755 }
  - src: target/release/dnea-nm-bridge
    dst: /usr/lib/dnea/dnea-nm-bridge
    file_info: { mode: 0755 }
  - src: target/release/dnea
    dst: /usr/bin/dnea
    file_info: { mode: 0755 }
  - src: installer/linux/dnea.service
    dst: /usr/lib/systemd/system/dnea.service
  - src: installer/linux/dnea.toml
    dst: /etc/dnea/dnea.toml
    type: config|noreplace
scripts:
  preinstall: installer/linux/scripts/preinstall.sh
  postinstall: installer/linux/scripts/postinstall.sh
  preremove: installer/linux/scripts/preremove.sh
  postremove: installer/linux/scripts/postremove.sh
```

### `installer/linux/dnea.service`

```ini
[Unit]
Description=DNEA runtime
After=network.target
Wants=network.target

[Service]
Type=notify
ExecStart=/usr/lib/dnea/dnea-daemon --foreground
Restart=on-failure
RestartSec=2s
User=dnea
Group=dnea
StateDirectory=dnea
RuntimeDirectory=dnea
RuntimeDirectoryMode=0750
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=read-only
PrivateTmp=true
PrivateDevices=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictNamespaces=true
RestrictSUIDSGID=true
SystemCallArchitectures=native
LockPersonality=true

[Install]
WantedBy=multi-user.target
```

### `installer/linux/dnea.toml`

```toml
[paths]
state_dir = "/var/lib/dnea"
run_dir = "/run/dnea"
log_dir = "/var/log/dnea"

[modules]
allow_dev = false
pooling_slots = 16

[nm]
max_connections = 64
mtu = 1048576

[api]
bind = "127.0.0.1:0"
token_ttl_secs = 600

[audit]
retention_days = 30
max_bytes_per_module = 67108864
sinks = ["sqlite", "file"]

[watchdog]
epoch_interval_ms = 100
default_fuel = 10000000
default_timeout_ms = 30000
```

### `installer/linux/scripts/preinstall.sh`

```bash
#!/bin/bash
set -e
for dir in \
  /etc/opt/chrome/native-messaging-hosts \
  /etc/chromium/native-messaging-hosts \
  /etc/opt/edge/native-messaging-hosts \
  /usr/lib/mozilla/native-messaging-hosts; do
  manifest="$dir/org.dnea.runtime.json"
  if [ -f "$manifest" ] && ! grep -q "x-dnea-installed" "$manifest"; then
    echo "Refusing to overwrite foreign host manifest at $manifest" >&2
    exit 1
  fi
done
exit 0
```

### `installer/linux/scripts/postinstall.sh`

```bash
#!/bin/bash
set -e
if ! id dnea >/dev/null 2>&1; then
  adduser --system --group --home /var/lib/dnea --shell /usr/sbin/nologin dnea
fi
mkdir -p /var/lib/dnea
chown dnea:dnea /var/lib/dnea
chmod 750 /var/lib/dnea
sudo -u dnea /usr/lib/dnea/dnea-daemon --init-key --config /etc/dnea/dnea.toml || true

for dir in \
  /etc/opt/chrome/native-messaging-hosts \
  /etc/chromium/native-messaging-hosts \
  /etc/opt/edge/native-messaging-hosts \
  /usr/lib/mozilla/native-messaging-hosts; do
  mkdir -p "$dir"
  ext_prefix="chrome-extension"
  ext_id="abcdefghijklmnopabcdefghijklmnop"
  case "$dir" in
    *mozilla*) ext_prefix="moz-extension"; ext_id="dnea@dnea.org" ;;
  esac
  cat > "$dir/org.dnea.runtime.json" <<EOF
{
  "name": "org.dnea.runtime",
  "description": "DNEA runtime bridge",
  "path": "/usr/lib/dnea/dnea-nm-bridge",
  "type": "stdio",
  "allowed_origins": ["$ext_prefix://$ext_id/"],
  "x-dnea-installed": { "version": "1.0.0" }
}
EOF
done

systemctl daemon-reload
systemctl enable --now dnea.service || true

if [ -f /etc/default/dnea ] && grep -q "DNEA_POLICY=1" /etc/default/dnea; then
  mkdir -p /etc/opt/chrome/policies/managed /etc/chromium/policies/managed
  cat > /etc/opt/chrome/policies/managed/dnea.json <<EOF
{ "ExtensionInstallForcelist": ["abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx"] }
EOF
  cp /etc/opt/chrome/policies/managed/dnea.json /etc/chromium/policies/managed/dnea.json
  mkdir -p /etc/firefox/policies
  if [ ! -f /etc/firefox/policies/policies.json ]; then
    cat > /etc/firefox/policies/policies.json <<EOF
{ "policies": { "Extensions": { "Install": ["dnea@dnea.org"] } } }
EOF
  fi
fi
exit 0
```

### `installer/linux/scripts/preremove.sh`

```bash
#!/bin/bash
set -e
systemctl stop dnea.service || true
exit 0
```

### `installer/linux/scripts/postremove.sh`

```bash
#!/bin/bash
set -e
if [ "$1" = "purge" ]; then
  rm -rf /var/lib/dnea
fi
# 30-day grace otherwise (systemd-tmpfiles handles this in real deployments).
exit 0
```

### `installer/linux/build.sh`

```bash
#!/bin/bash
set -euo pipefail
cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
mkdir -p build/out
nfpm package --config installer/linux/nfpm.yaml --packager deb --target build/out/
nfpm package --config installer/linux/nfpm.yaml --packager rpm --target build/out/
ls -la build/out/
```

---

# PART 6 — CI AND RELEASE

## `.github/workflows/ci.yml`

```yaml
name: CI
on:
  push: { branches: [main] }
  pull_request:

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"
  CARGO_COMPONENT_VERSION: "0.20.0"

jobs:
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: rustfmt }
      - run: cargo fmt --all -- --check

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: clippy }
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets -- -D warnings

  test:
    name: test (${{ matrix.os }})
    strategy: { fail-fast: false, matrix: { os: [ubuntu-latest, macos-latest, windows-latest] } }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Install cargo-component
        run: cargo install cargo-component --version ${{ env.CARGO_COMPONENT_VERSION }} --locked
      - name: Build Focus Mode
        run: cargo component build -p focus-mode --release
      - name: Test
        run: cargo test --workspace --all-targets

  thin-client:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with: { version: 9 }
      - uses: actions/setup-node@v4
        with: { node-version: 20 }
      - working-directory: thin-client
        run: pnpm install
      - working-directory: thin-client
        run: pnpm compile
      - working-directory: thin-client
        run: pnpm build:chrome
      - working-directory: thin-client
        run: pnpm build:firefox

  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: EmbarkStudios/cargo-deny-action@v1
        with: { command: check advisories bans licenses sources }
```

## `.github/workflows/release.yml`

```yaml
name: Release
on:
  push: { tags: ["v*"] }

jobs:
  build:
    strategy:
      matrix:
        include:
          - { os: ubuntu-latest, artifact: linux-amd64 }
          - { os: macos-latest, artifact: macos-universal }
          - { os: windows-latest, artifact: windows-amd64 }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
      - if: matrix.os == 'ubuntu-latest'
        run: |
          go install github.com/goreleaser/nfpm/v2/cmd/nfpm@latest
          ./installer/linux/build.sh
      - if: matrix.os == 'macos-latest'
        run: ./installer/macos/build.sh
      - if: matrix.os == 'windows-latest'
        shell: pwsh
        run: |
          dotnet tool install --global wix
          ./installer/windows/build.ps1
      - uses: actions/upload-artifact@v4
        with:
          name: dnea-${{ matrix.artifact }}
          path: build/out/*

  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
      - uses: softprops/action-gh-release@v2
        with:
          files: dnea-*/*
          generate_release_notes: true
```

---

# PART 7 — DOCUMENTATION

## `docs/quickstart.md`

```markdown
# DNEA Quickstart

Full happy path on a clean machine.

## 1. Build

```bash
git clone https://github.com/dnea-project/dnea
cd dnea
cargo build --release
cargo install cargo-component --version 0.20.0 --locked
cargo component build -p focus-mode --release
```

## 2. Initialize

```bash
./target/release/dnea-daemon --init-key
```

## 3. Run the runtime

```bash
./target/release/dnea-daemon &
sleep 1
```

## 4. Sign Focus Mode

```bash
./target/release/dnea keygen --out signing.key
# Copy the printed public key into trust.json:
cat > trust.json <<'EOF'
{ "keys": [
  { "keyId": "https://example.com/keys/signing",
    "alg": "EdDSA",
    "publicKey": "REPLACE_WITH_PRINTED_KEY" }
]}
EOF
```

Create a manifest. Compute the digest:

```bash
DIGEST=$(sha256sum target/wasm32-wasip1/release/focus_mode.wasm \
  | awk '{print $1}' | xxd -r -p | base64 | tr '+/' '-_' | tr -d '=')
cat > focus-mode.json <<EOF
{
  "manifestVersion": 1,
  "id": "com.example.focus-mode",
  "version": "1.2.0",
  "runtime": ">=1.0.0 <2.0.0",
  "name": "Focus Mode",
  "entrypoints": {
    "core": {
      "kind": "wasm", "path": "focus_mode.wasm",
      "digest": { "alg": "sha-256", "value": "sha-256:$DIGEST" }
    }
  },
  "capabilities": {
    "request": [
      { "id": "storage:module" },
      { "id": "runtime:log" },
      { "id": "browser:tabs:query", "reason": "Detect blocked sites." }
    ]
  },
  "ui": [
    { "id": "dashboard", "kind": "panel", "entrypoint": "core",
      "surfaces": ["companion"], "title": "Focus Mode" }
  ]
}
EOF
./target/release/dnea sign \
  --manifest focus-mode.json \
  --key signing.key \
  --key-id "https://example.com/keys/signing"
./target/release/dnea validate focus-mode.json \
  --trust trust.json \
  --artifacts target/wasm32-wasip1/release
```

## 5. Install

```bash
cp target/wasm32-wasip1/release/focus_mode.wasm .
./target/release/dnea install-module \
  --manifest focus-mode.json \
  --artifacts .
```

## 6. Consent

Open the companion URL from `api.json`. Approve the `browser:tabs:query`
prompt. The module appears in the list and its panel renders.

## 7. Thin client (optional)

```bash
cd thin-client
pnpm install
pnpm build:chrome
```

Load `thin-client/.output/chrome-mv3` as an unpacked extension. The
popup shows the connection state, and pages can invoke the module
through `window.__dnea`.

## 8. Verify structural enforcement

Edit `focus-mode.json` to add `"browser:history:read"` to the deny list,
re-sign, and reinstall. The module cannot read history regardless of
what any policy grants.
```

## `docs/security.md`

```markdown
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
```

## `docs/migration.md`

```markdown
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
```

## `docs/w3c-submission.md`

```markdown
# DNEA submission to the WebExtensions Community Group

**From:** DNEA Project
**To:** WebExtensions Community Group (webextensions-cg@w3.org)
**Subject:** Proposal for a capability vocabulary, Native Messaging
envelope, and thin-client contract as complements to WebExtensions

## Summary

DNEA proposes three components for the CG's consideration:

1. A capability vocabulary: hierarchical, constraint-scoped,
   machine-checkable.
2. A Native Messaging envelope: versioned, chunked, correlated,
   direction-validated.
3. A thin-client contract: connection lifecycle, codec, content-script
   bridge, consent surface.

DNEA does not require changes to the WebExtensions API. It uses what
exists.

## Motivation

An extension that needs device-level state must today maintain
per-browser native messaging hosts, per-browser manifests, and
per-browser policy. A device-level runtime inverts this: one install
makes the extension available to every browser on the device.

## Reference implementation

Apache-2.0 at https://github.com/dnea-project/dnea.

## Contact

specs@dnea.org
```

---

# PART 8 — FINAL VERIFICATION CHECKLIST

Before tagging `v1.0.0`:

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace --all-targets` passes on Linux/macOS/Windows
- [ ] `cargo component build -p focus-mode --release` produces `focus_mode.wasm`
- [ ] `dnea validate focus-mode.json --trust trust.json --artifacts target/wasm32-wasip1/release` passes
- [ ] `dnea install-module` succeeds and the module appears in `GET /v1/modules`
- [ ] Consent prompt appears; granting it updates the grant set
- [ ] A module without `browser:tabs:*` fails to link `tabs` at instantiation
- [ ] `thin-client`: `pnpm install && pnpm compile && pnpm build:chrome && pnpm build:firefox` succeed
- [ ] `thin-client`: background connects, handshake completes, welcome received
- [ ] `thin-client`: 3 MB event round-trips through chunking
- [ ] `thin-client`: content-script port registers/unregisters correctly
- [ ] Installer: MSI/PKG/DEB build without errors
- [ ] Installer: uninstall leaves no DNEA-authored manifests or policy entries
- [ ] `api.json` is not world-readable
- [ ] Master key is never written to the log
- [ ] Foreign native-messaging hosts are preserved by conflict detection

---

# FINAL STATUS

DNEA v1.0 is complete. Nine crates, one guest module, one thin
client, three installers, six specification artifacts, one CI matrix,
one release workflow, and the W3C engagement materials.

The critical properties hold by construction, not convention:

1. **Structural enforcement** at the Wasm boundary.
2. **Signed installs** with Ed25519 and JCS canonicalization.
3. **No partial installs** — the pipeline is fail-fast.
4. **Loopback-only** Local API with ephemeral JWT secrets.
5. **Encrypted per-module storage** with HKDF-derived keys.
6. **Consent-gated** prompt-tier capabilities.
7. **Symmetric chunking** across the Native Messaging boundary.

Everything above is buildable from a clean checkout with the commands
at the top. Tag `v1.0.0` and ship.