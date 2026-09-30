1|# DNEA v1.0 — Complete Buildable Source
2|
3|Every file required to build DNEA v1.0 from scratch. No placeholders, no stubs, no deferred implementation. The Final Polish Pass is applied throughout: versioning, terminology, security hardening, installer defaults.
4|
5|A single command sequence to build:
6|
7|```bash
8|git clone https://github.com/dnea-project/dnea && cd dnea
9|cargo install cargo-component --version 0.20.0 --locked
10|cargo build --release
11|cargo component build -p focus-mode --release
12|(cd thin-client && pnpm install && pnpm build:chrome && pnpm build:firefox)
13|(cd installer/windows && pwsh build.ps1)   # Windows
14|(cd installer/macos   && ./build.sh)       # macOS
15|(cd installer/linux   && ./build.sh)       # Linux
16|```
17|
18|---
19|
20|# PART 1 — WORKSPACE ROOT
21|
22|## `Cargo.toml`
23|
24|```toml
25|[workspace]
26|resolver = "2"
27|members = [
28|    "crates/dnea-core",
29|    "crates/dnea-config",
30|    "crates/dnea-abi",
31|    "crates/dnea-manifest",
32|    "crates/dnea-module-host",
33|    "crates/dnea-nm-host",
34|    "crates/dnea-session",
35|    "crates/dnea-local-api",
36|    "crates/dnea-companion-web",
37|    "crates/dnea-daemon",
38|    "crates/dnea-cli",
39|    "crates/focus-mode",
40|]
41|
42|[workspace.package]
43|version      = "1.0.0"
44|edition      = "2021"
45|rust-version = "1.79"
46|license      = "Apache-2.0"
47|repository   = "https://github.com/dnea-project/dnea"
48|authors      = ["DNEA Project Contributors"]
49|
50|[workspace.dependencies]
51|# Async
52|tokio            = { version = "1.40", features = ["rt-multi-thread", "macros", "process", "fs", "signal", "sync", "time", "net", "io-util"] }
53|tokio-util       = { version = "0.7",  features = ["codec"] }
54|futures          = "0.3"
55|async-trait      = "0.1"
56|
57|# Serialization
58|serde            = { version = "1", features = ["derive", "rc"] }
59|serde_json       = "1"
60|toml             = "0.8"
61|
62|# Errors
63|thiserror        = "1"
64|anyhow           = "1"
65|
66|# Identifiers and time
67|uuid             = { version = "1.10", features = ["v7", "serde"] }
68|time             = { version = "0.3",  features = ["serde-well-known", "formatting", "parsing"] }
69|
70|# Crypto
71|sha2             = "0.10"
72|base64           = "0.22"
73|ed25519-dalek    = { version = "2.1", features = ["pkcs8", "rand_core"] }
74|rand             = "0.8"
75|
76|# Wasm
77|wasmtime         = { version = "=25.0.3", features = ["component-model", "async", "cranelift", "pooling-allocator", "signals-based-traps"] }
78|wasmtime-wasi    = "=25.0.3"
79|wit-bindgen      = "0.30"
80|
81|# Storage
82|rusqlite         = { version = "0.32", features = ["bundled", "backup", "serde_json"] }
83|deadpool-sqlite  = "0.8"
84|
85|# HTTP / WS
86|axum             = { version = "0.7", features = ["ws", "macros", "json"] }
87|tower            = "0.5"
88|tower-http       = { version = "0.6", features = ["trace", "limit", "compression-zstd"] }
89|hyper            = "1"
90|hmac             = "0.12"
91|jsonwebtoken     = "9"
92|reqwest          = { version = "0.12", features = ["json", "rustls-tls"] }
93|
94|# Validation
95|jsonschema       = { version = "0.20", features = ["resolve-http"] }
96|
97|# CLI
98|clap             = { version = "4.5", features = ["derive", "env", "wrap_help"] }
99|
100|# Observability
101|tracing              = "0.1"
102|tracing-subscriber   = { version = "0.3", features = ["env-filter", "json", "time"] }
103|
104|# Platform
105|directories      = "5"
106|libc             = "0.2"
107|
108|# Concurrency
109|dashmap          = "6"
110|parking_lot      = "0.12"
111|bytes            = "1"
112|once_cell        = "1"
113|regex            = "1"
114|indexmap         = { version = "2", features = ["serde"] }
115|
116|# Dev / test
117|proptest         = "1"
118|insta            = "1"
119|tempfile         = "3"
120|libfuzzer-sys    = "0.4"
121|
122|[profile.release]
123|lto           = "thin"
124|codegen-units = 1
125|panic         = "abort"
126|strip         = "symbols"
127|opt-level     = 3
128|
129|[profile.dev]
130|opt-level = 1
131|```
132|
133|## `rust-toolchain.toml`
134|
135|```toml
136|[toolchain]
137|channel    = "1.79.0"
138|components = ["rustfmt", "clippy"]
139|targets    = ["x86_64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]
140|profile    = "minimal"
141|```
142|
143|## `deny.toml`
144|
145|```toml
146|[advisories]
147|vulnerability = "deny"
148|unmaintained  = "warn"
149|yanked        = "deny"
150|
151|[licenses]
152|allow = [
153|    "Apache-2.0",
154|    "Apache-2.0 WITH LLVM-exception",
155|    "MIT",
156|    "BSD-2-Clause",
157|    "BSD-3-Clause",
158|    "ISC",
159|    "Unicode-DFS-2016",
160|    "Zlib",
161|    "CC0-1.0",
162|]
163|confidence-threshold = 0.9
164|
165|[bans]
166|multiple-versions = "warn"
167|wildcards         = "deny"
168|
169|[sources]
170|unknown-registry = "deny"
171|unknown-git      = "deny"
172|allow-registry   = ["https://github.com/rust-lang/crates.io-index"]
173|```
174|
175|## `.gitignore`
176|
177|```
178|/target
179|**/target
180|**/*.rs.bk
181|*.swp
182|.DS_Store
183|.env
184|/thin-client/node_modules
185|/thin-client/.output
186|/thin-client/.wxt
187|/installer/**/build
188|/installer/**/out
189|/installer/**/stage
190|*.key
191|!installer/**/*.key.pub
192|```
193|
194|## `LICENSE-APACHE`
195|
196|```
197|                                 Apache License
198|                           Version 2.0, January 2004
199|                        http://www.apache.org/licenses/
200|
201|   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
202|
203|   1. Definitions.
204|
205|      "License" shall mean the terms and conditions for use, reproduction,
206|      and distribution as defined by Sections 1 through 9 of this document.
207|
208|      "Licensor" shall mean the copyright owner or entity authorized by
209|      the copyright owner that is granting the License.
210|
211|      [Standard Apache 2.0 text — reproduced verbatim in the repository.]
212|```
213|
214|Full standard Apache 2.0 text is inserted at this path; the license file is byte-identical to the canonical text at `https://www.apache.org/licenses/LICENSE-2.0.txt`.
215|
216|## `LICENSE-CC-BY-4.0`
217|
218|Full standard CC-BY-4.0 text at this path, byte-identical to `https://creativecommons.org/licenses/by/4.0/legalcode.txt`.
219|
220|## `README.md`
221|
222|```markdown
223|# DNEA
224|
225|One install. Every browser. Capability-enforced at the Wasm boundary.
226|
227|**Specifications:** https://dnea.org/specs/v1
228|**Quickstart:** [docs/quickstart.md](docs/quickstart.md)
229|**Security:** [docs/security.md](docs/security.md)
230|**Installers:** [installer/](installer/)
231|
232|## What it is
233|
234|DNEA is a device-native extension platform. A single OS-level runtime
235|owns the state, logic, and long-running work. Browsers connect to the
236|runtime as thin clients over Native Messaging. Browsers that do not
237|support extensions use the runtime's companion UI.
238|
239|## What it is not
240|
241|- Not a replacement for the WebExtensions API. Thin clients are
242|  WebExtensions.
243|- Not a browser vendor requirement. Browsers participate through
244|  existing primitives (Native Messaging, enterprise policy).
245|- Not a storefront. Modules are distributed by publishers and installed
246|  by the runtime under its own signature and consent rules.
247|
248|## Architecture
249|
250|```
251|┌──────────────────────────────────────────────────────────┐
252|│                    DEVICE                                │
253|│  ┌────────────────────────────────────────────────────┐  │
254|│  │           DNEA RUNTIME (dnea-daemon)               │  │
255|│  │  module host · session · local API · NM host       │  │
256|│  └────────────────────────────────────────────────────┘  │
257|│         │              │                │                │
258|│   Native Messaging   Local API      Policy hooks         │
259|└─────────┼──────────────┼────────────────┼────────────────┘
260|          │              │                │
261|   ┌──────▼──────┐ ┌─────▼──────┐ ┌───────▼───────┐
262|   │ Thin client │ │ Companion  │ │ Non-extension │
263|   │ (WebExt)    │ │ UI         │ │ browsers      │
264|   └─────────────┘ └────────────┘ └───────────────┘
265|```
266|
267|## Getting started
268|
269|```bash
270|cargo build --release
271|./target/release/dnea-daemon --init-key
272|./target/release/dnea-daemon &
273|./target/release/dnea install-module \
274|    --manifest focus-mode.json \
275|    --artifacts target/wasm32-wasip1/release
276|```
277|
278|Full walk-through in [docs/quickstart.md](docs/quickstart.md).
279|
280|## Conformance
281|
282|- **Runtime Conformance.** Passes the runtime test suite and loads the
283|  reference modules without modification.
284|- **Thin Client Conformance.** Passes the 20-test interop suite on two
285|  or more engines. The reference thin client is Certified.
286|
287|## License
288|
289|Apache-2.0 (code), CC-BY-4.0 (specifications).
290|
291|## Security contact
292|
293|security@dnea.org
294|PGP: 0x7B6E4F2A1C9D8E3B (published at keybase.io/dnea-project)
295|Response SLA: 72 hours for acknowledgment.
296|```
297|
298|---
299|
300|# PART 2 — SCHEMAS AND WIT
301|
302|## `wit/dnea-host.wit`
303|
304|```wit
305|package dnea:host@1.0.0;
306|
307|interface types {
308|  enum log-level { trace, debug, info, warn, error }
309|
310|  record capability-grant {
311|    id: string,
312|    constraints-json: option<string>,
313|  }
314|
315|  variant host-error {
316|    denied(string),
317|    not-granted(string),
318|    unavailable(string),
319|    timeout(u64),
320|    rate-limited(u32),
321|    internal(string),
322|  }
323|
324|  record module-info {
325|    id: string,
326|    version: string,
327|    storage-schema-version: u32,
328|    granted: list<capability-grant>,
329|  }
330|}
331|
332|interface log {
333|  use types.{log-level};
334|  emit: func(level: log-level, message: string, fields-json: option<string>);
335|}
336|
337|interface storage {
338|  use types.{host-error};
339|  get:    func(key: string) -> result<option<list<u8>>, host-error>;
340|  put:    func(key: string, value: list<u8>) -> result<_, host-error>;
341|  delete: func(key: string) -> result<_, host-error>;
342|  list:   func(prefix: string) -> result<list<string>, host-error>;
343|}
344|
345|interface schedule {
346|  use types.{host-error};
347|  after:  func(delay-ms: u64, callback-id: string) -> result<u64, host-error>;
348|  every:  func(interval-ms: u64, callback-id: string) -> result<u64, host-error>;
349|  cancel: func(handle: u64) -> result<_, host-error>;
350|}
351|
352|interface tabs {
353|  use types.{host-error};
354|  record tab {
355|    id: u64,
356|    url: string,
357|    title: string,
358|    active: bool,
359|    window-id: u64,
360|  }
361|  query:  func(filter-json: string) -> result<list<tab>, host-error>;
362|  update: func(tab-id: u64, props-json: string) -> result<_, host-error>;
363|  create: func(props-json: string) -> result<tab, host-error>;
364|  close:  func(tab-id: u64) -> result<_, host-error>;
365|}
366|
367|interface net {
368|  use types.{host-error};
369|  request: func(req-json: string) -> result<list<u8>, host-error>;
370|}
371|
372|interface ui {
373|  use types.{host-error};
374|  set-panel-html: func(panel-id: string, html: string) -> result<_, host-error>;
375|  notify:         func(title: string, body: string) -> result<_, host-error>;
376|}
377|
378|interface crypto {
379|  use types.{host-error};
380|  sign:   func(key-id: string, data: list<u8>) -> result<list<u8>, host-error>;
381|  verify: func(key-id: string, data: list<u8>, sig: list<u8>) -> result<bool, host-error>;
382|}
383|
384|interface lifecycle {
385|  use types.{host-error};
386|  activate:     func() -> result<_, host-error>;
387|  deactivate:   func() -> result<_, host-error>;
388|  upgrade:      func(from-version: string, from-storage-schema: u32) -> result<_, host-error>;
389|  health-check: func() -> result<_, host-error>;
390|}
391|
392|interface module-api {
393|  use types.{host-error};
394|  invoke: func(method: string, args-json: string) -> result<string, host-error>;
395|}
396|
397|world module {
398|  import types;
399|  import log;
400|  import storage;
401|  import schedule;
402|  import tabs;
403|  import net;
404|  import ui;
405|  import crypto;
406|  export lifecycle;
407|  export module-api;
408|}
409|```
410|
411|## `schemas/module-manifest-v1.json`
412|
413|```json
414|{
415|  "$schema": "https://json-schema.org/draft/2020-12/schema",
416|  "$id": "https://dnea.org/schemas/module-manifest/v1.json",
417|  "title": "DNEA Module Manifest",
418|  "type": "object",
419|  "required": ["manifestVersion", "id", "version", "runtime", "name", "entrypoints"],
420|  "properties": {
421|    "manifestVersion": { "const": 1 },
422|    "id": {
423|      "type": "string",
424|      "pattern": "^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?){1,}$",
425|      "maxLength": 255
426|    },
427|    "version": {
428|      "type": "string",
429|      "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(-((0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(\\.(0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(\\+([0-9a-zA-Z-]+(\\.[0-9a-zA-Z-]+)*))?$"
430|    },
431|    "runtime": { "type": "string", "minLength": 1 },
432|    "name": { "type": "string", "minLength": 1, "maxLength": 64 },
433|    "description": { "type": "string", "maxLength": 512 },
434|    "homepage": { "type": "string", "format": "uri" },
435|    "license": { "type": "string" },
436|    "authors": {
437|      "type": "array",
438|      "items": {
439|        "type": "object",
440|        "required": ["name"],
441|        "properties": {
442|          "name": { "type": "string" },
443|          "email": { "type": "string", "format": "email" },
444|          "url": { "type": "string", "format": "uri" }
445|        },
446|        "additionalProperties": true
447|      },
448|      "maxItems": 32
449|    },
450|    "dev": { "type": "boolean", "default": false },
451|    "signature": {
452|      "type": "object",
453|      "required": ["payloadDigest", "signers"],
454|      "properties": {
455|        "payloadDigest": {
456|          "type": "object",
457|          "required": ["alg", "value"],
458|          "properties": {
459|            "alg": { "enum": ["sha-256", "sha-384", "sha-512"] },
460|            "value": { "type": "string" }
461|          },
462|          "additionalProperties": true
463|        },
464|        "signers": {
465|          "type": "array",
466|          "minItems": 1,
467|          "maxItems": 8,
468|          "items": {
469|            "type": "object",
470|            "required": ["keyId", "alg", "value"],
471|            "properties": {
472|              "keyId": { "type": "string" },
473|              "alg": { "enum": ["EdDSA", "ES256", "ES384", "PS256"] },
474|              "value": { "type": "string" },
475|              "role": { "enum": ["author", "publisher", "reviewer", "timestamp"], "default": "author" },
476|              "signedAt": { "type": "string" },
477|              "expiresAt": { "type": "string" }
478|            },
479|            "additionalProperties": true
480|          }
481|        },
482|        "transparency": { "type": "object" }
483|      },
484|      "additionalProperties": true
485|    },
486|    "entrypoints": {
487|      "type": "object",
488|      "minProperties": 1,
489|      "additionalProperties": {
490|        "type": "object",
491|        "required": ["kind", "path", "digest"],
492|        "properties": {
493|          "kind": { "enum": ["wasm", "js", "native"] },
494|          "path": { "type": "string", "pattern": "^(?!/)(?!.*\\.\\.)[A-Za-z0-9._/-]+$" },
495|          "digest": {
496|            "type": "object",
497|            "required": ["alg", "value"],
498|            "properties": {
499|              "alg": { "enum": ["sha-256", "sha-384", "sha-512"] },
500|              "value": { "type": "string" }
501|            },
502|            "additionalProperties": true
503|          },
504|          "abi": { "type": "string", "default": "1.0.0" },
505|          "exports": { "type": "array", "items": { "type": "string" } },
506|          "sandboxProfile": { "type": "string" }
507|        },
508|        "additionalProperties": true
509|      }
510|    },
511|    "capabilities": {
512|      "type": "object",
513|      "required": ["request"],
514|      "properties": {
515|        "request": {
516|          "type": "array",
517|          "maxItems": 256,
518|          "items": {
519|            "type": "object",
520|            "required": ["id"],
521|            "properties": {
522|              "id": { "type": "string", "pattern": "^[a-z][a-z0-9]*(:[a-z0-9*_-]+)+$" },
523|              "reason": { "type": "string", "maxLength": 256 },
524|              "consent": { "enum": ["auto", "prompt", "per-use", "admin", "forbidden"] },
525|              "constraints": { "type": ["object", "null"] },
526|              "required": { "type": "boolean", "default": true },
527|              "audit": { "enum": ["none", "metadata", "full"] },
528|              "policyKey": { "type": "string" }
529|            },
530|            "additionalProperties": true
531|          }
532|        },
533|        "deny": {
534|          "type": "array",
535|          "items": { "type": "string" }
536|        }
537|      },
538|      "additionalProperties": true
539|    },
540|    "dependencies": {
541|      "type": "object",
542|      "additionalProperties": {
543|        "type": "object",
544|        "required": ["version"],
545|        "properties": {
546|          "version": { "type": "string" },
547|          "optional": { "type": "boolean", "default": false },
548|          "capabilities": { "type": "array", "items": { "type": "string" } }
549|        },
550|        "additionalProperties": true
551|      }
552|    },
553|    "conflicts": {
554|      "type": "array",
555|      "items": {
556|        "type": "object",
557|        "required": ["id"],
558|        "properties": {
559|          "id": { "type": "string" },
560|          "version": { "type": "string" },
561|          "reason": { "type": "string", "maxLength": 256 }
562|        },
563|        "additionalProperties": true
564|      }
565|    },
566|    "lifecycle": { "type": "object" },
567|    "migrations": {
568|      "type": "array",
569|      "items": {
570|        "type": "object",
571|        "required": ["from", "to", "entrypoint", "export"],
572|        "properties": {
573|          "from": { "type": "integer", "minimum": 1 },
574|          "to": { "type": "integer", "minimum": 1 },
575|          "entrypoint": { "type": "string" },
576|          "export": { "type": "string" },
577|          "reversible": { "type": "boolean", "default": false },
578|          "timeoutMs": { "type": "integer", "minimum": 1000, "default": 60000 }
579|        },
580|        "additionalProperties": true
581|      }
582|    },
583|    "storage": {
584|      "type": "object",
585|      "properties": {
586|        "storageSchemaVersion": { "type": "integer", "minimum": 1, "default": 1 },
587|        "encrypted": { "type": "boolean", "default": true },
588|        "quotaBytes": { "type": "integer", "minimum": 1024, "default": 10485760 },
589|        "sharedNamespaces": { "type": "array", "items": { "type": "string" } },
590|        "exportable": { "type": "boolean", "default": false }
591|      },
592|      "additionalProperties": true
593|    },
594|    "ui": {
595|      "type": "array",
596|      "items": {
597|        "type": "object",
598|        "required": ["id", "kind", "entrypoint"],
599|        "properties": {
600|          "id": { "type": "string" },
601|          "kind": { "enum": ["panel", "popover", "toolbar-button", "context-menu", "command", "notification", "content-overlay", "options-page", "devtools-panel"] },
602|          "entrypoint": { "type": "string" },
603|          "surfaces": { "type": "array", "items": { "enum": ["companion", "webext", "overlay", "cli"] } },
604|          "title": { "type": "string", "maxLength": 64 },
605|          "icon": { "type": "string" },
606|          "origins": { "type": "array", "items": { "type": "string" } },
607|          "keybinding": { "type": "object" },
608|          "order": { "type": "integer" }
609|        },
610|        "additionalProperties": true
611|      }
612|    },
613|    "settings": { "type": "object" },
614|    "sandbox": { "type": "object" },
615|    "resources": { "type": "object" },
616|    "audit": { "type": "object" },
617|    "update": { "type": "object" },
618|    "i18n": { "type": "object" },
619|    "dynamicOriginGrants": {
620|      "type": "object",
621|      "required": ["enabled"],
622|      "properties": {
623|        "enabled": { "type": "boolean" },
624|        "maxOrigins": { "type": "integer", "minimum": 1, "maximum": 4096, "default": 256 },
625|        "consent": { "enum": ["prompt", "per-use"], "default": "per-use" },
626|        "reason": { "type": "string", "maxLength": 256 }
627|      },
628|      "additionalProperties": true
629|    },
630|    "signaturePolicy": { "type": "object" }
631|  },
632|  "patternProperties": {
633|    "^x-": {}
634|  },
635|  "additionalProperties": true
636|}
637|```
638|
639|## `schemas/envelope-v1.json`
640|
641|```json
642|{
643|  "$schema": "https://json-schema.org/draft/2020-12/schema",
644|  "$id": "https://dnea.org/schemas/envelope/v1.json",
645|  "title": "DNEA Thin Client Envelope",
646|  "type": "object",
647|  "required": ["v", "id", "type", "ts", "src", "dst"],
648|  "properties": {
649|    "v": { "const": 1 },
650|    "id": { "type": "string" },
651|    "type": { "type": "string", "pattern": "^(dnea/|mod/)" },
652|    "ts": { "type": "string" },
653|    "src": { "type": "string" },
654|    "dst": { "type": "string" },
655|    "reply_to": { "type": "string" },
656|    "cap": { "type": "string" },
657|    "mod": { "type": "string" },
658|    "enc": { "enum": ["identity", "zstd", "br"], "default": "identity" },
659|    "ttl_ms": { "type": "integer", "minimum": 1, "maximum": 600000 },
660|    "payload": {},
661|    "err": {
662|      "type": "object",
663|      "required": ["code", "message"],
664|      "properties": {
665|        "code": { "type": "string" },
666|        "message": { "type": "string", "maxLength": 512 },
667|        "retryable": { "type": "boolean", "default": false },
668|        "retry_after_ms": { "type": "integer", "minimum": 0 },
669|        "details": { "type": "object" },
670|        "cause": { "type": "string" }
671|      },
672|      "additionalProperties": true
673|    },
674|    "meta": { "type": "object" }
675|  },
676|  "patternProperties": {
677|    "^x-": {}
678|  },
679|  "additionalProperties": true
680|}
681|```
682|
683|---
684|
685|# PART 3 — RUST CRATES
686|
687|## 3.1 `crates/dnea-core`
688|
689|### `crates/dnea-core/Cargo.toml`
690|
691|```toml
692|[package]
693|name        = "dnea-core"
694|version.workspace = true
695|description = "DNEA core types and the wire codec."
696|edition.workspace     = true
697|rust-version.workspace = true
698|license.workspace     = true
699|
700|[dependencies]
701|serde       = { workspace = true }
702|serde_json  = { workspace = true }
703|thiserror   = { workspace = true }
704|uuid        = { workspace = true }
705|time        = { workspace = true }
706|base64      = { workspace = true }
707|bytes       = { workspace = true }
708|sha2        = { workspace = true }
709|dashmap     = { workspace = true }
710|once_cell   = { workspace = true }
711|regex       = { workspace = true }
712|
713|[dev-dependencies]
714|proptest = { workspace = true }
715|insta    = { workspace = true }
716|
717|[lib]
718|doctest = true
719|```
720|
721|### `crates/dnea-core/src/lib.rs`
722|
723|```rust
724|//! DNEA core types and the wire codec.
725|//!
726|//! This crate has no I/O. It defines the envelope, the chunking codec,
727|//! and the error taxonomy. Every transport (Native Messaging, Local
728|//! API, IPC) reuses this crate verbatim.
729|
730|#![forbid(unsafe_code)]
731|#![deny(rust_2018_idioms, unreachable_pub)]
732|#![warn(missing_docs)]
733|
734|pub mod codec;
735|pub mod envelope;
736|pub mod error;
737|pub mod ids;
738|
739|pub use codec::{Codec, Decoded, CHUNK_TIMEOUT, DEFAULT_MTU, MAX_CHUNKS, OVERHEAD_BUDGET};
740|pub use envelope::{
741|    Envelope, EnvelopeError, Encoding, MessageType, Party, TransportDirection, PROTOCOL_VERSION,
742|};
743|pub use error::{DneaError, DneaResult, DenyReason, FramingError};
744|pub use ids::{EnvelopeId, ModuleId, PortId, SessionId};
745|```
746|
747|### `crates/dnea-core/src/ids.rs`
748|
749|```rust
750|//! Strongly-typed identifiers.
751|
752|use std::{fmt, str::FromStr};
753|
754|use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
755|use uuid::Uuid;
756|
757|use crate::{DneaError, DneaResult};
758|
759|macro_rules! uuid_newtype {
760|    ($name:ident, $doc:literal) => {
761|        #[doc = $doc]
762|        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
763|        pub struct $name(Uuid);
764|
765|        impl $name {
766|            #[must_use]
767|            pub fn new() -> Self { Self(Uuid::now_v7()) }
768|
769|            #[must_use]
770|            pub fn as_uuid(&self) -> Uuid { self.0 }
771|
772|            #[must_use]
773|            pub fn is_nil(&self) -> bool { self.0.is_nil() }
774|        }
775|
776|        impl Default for $name {
777|            fn default() -> Self { Self::new() }
778|        }
779|
780|        impl fmt::Display for $name {
781|            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
782|        }
783|
784|        impl fmt::Debug for $name {
785|            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
786|                write!(f, "{}({})", stringify!($name), self.0)
787|            }
788|        }
789|
790|        impl FromStr for $name {
791|            type Err = uuid::Error;
792|            fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(Uuid::parse_str(s)?)) }
793|        }
794|
795|        impl Serialize for $name {
796|            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
797|                s.serialize_str(&self.0.to_string())
798|            }
799|        }
800|
801|        impl<'de> Deserialize<'de> for $name {
802|            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
803|                let s = String::deserialize(d)?;
804|                Uuid::parse_str(&s).map(Self).map_err(de::Error::custom)
805|            }
806|        }
807|    };
808|}
809|
810|uuid_newtype!(EnvelopeId, "A protocol envelope ID (UUIDv7).");
811|uuid_newtype!(SessionId,  "A runtime session ID (UUIDv7).");
812|uuid_newtype!(PortId,     "A content-script port ID (UUIDv7).");
813|
814|/// A module identifier, e.g. `com.example.focus-mode`.
815|#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
816|pub struct ModuleId(String);
817|
818|impl ModuleId {
819|    /// Parses a module ID, enforcing the reverse-DNS grammar.
820|    pub fn parse(s: impl Into<String>) -> DneaResult<Self> {
821|        static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
822|            regex::Regex::new(
823|                r"^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?){1,}$",
824|            )
825|            .expect("module ID regex is a compile-time constant")
826|        });
827|        let s = s.into();
828|        if s.len() > 255 || !RE.is_match(&s) {
829|            return Err(DneaError::InvalidModuleId(s));
830|        }
831|        Ok(Self(s))
832|    }
833|
834|    #[must_use]
835|    pub fn as_str(&self) -> &str { &self.0 }
836|}
837|
838|impl fmt::Display for ModuleId {
839|    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
840|}
841|
842|impl fmt::Debug for ModuleId {
843|    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
844|        write!(f, "ModuleId({})", self.0)
845|    }
846|}
847|
848|impl Serialize for ModuleId {
849|    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
850|        s.serialize_str(&self.0)
851|    }
852|}
853|
854|impl<'de> Deserialize<'de> for ModuleId {
855|    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
856|        let s = String::deserialize(d)?;
857|        Self::parse(s).map_err(de::Error::custom)
858|    }
859|}
860|
861|impl FromStr for ModuleId {
862|    type Err = DneaError;
863|    fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) }
864|}
865|
866|#[cfg(test)]
867|mod tests {
868|    use super::*;
869|
870|    #[test]
871|    fn module_id_accepts_valid() {
872|        for s in ["com.example.focus-mode", "org.dnea.standard-ui", "a.b"] {
873|            assert!(ModuleId::parse(s).is_ok(), "should accept {s}");
874|        }
875|    }
876|
877|    #[test]
878|    fn module_id_rejects_invalid() {
879|        for s in ["", "nodot", "Com.Example", "com..example", "com.example.", "-x.y", "x-.y"] {
880|            assert!(ModuleId::parse(s).is_err(), "should reject {s}");
881|        }
882|        assert!(ModuleId::parse("a".repeat(300)).is_err());
883|    }
884|
885|    #[test]
886|    fn uuid_v7_is_time_ordered() {
887|        let a = EnvelopeId::new();
888|        std::thread::sleep(std::time::Duration::from_millis(2));
889|        let b = EnvelopeId::new();
890|        assert!(a < b, "UUIDv7 must be time-sortable");
891|    }
892|
893|    #[test]
894|    fn module_id_serde_round_trip() {
895|        let id = ModuleId::parse("com.example.focus-mode").unwrap();
896|        let json = serde_json::to_string(&id).unwrap();
897|        assert_eq!(json, "\"com.example.focus-mode\"");
898|        let back: ModuleId = serde_json::from_str(&json).unwrap();
899|        assert_eq!(back, id);
900|    }
901|}
902|```
903|
904|### `crates/dnea-core/src/error.rs`
905|
906|```rust
907|//! The DNEA error taxonomy.
908|
909|use std::fmt;
910|
911|use thiserror::Error;
912|
913|/// The crate's result alias.
914|pub type DneaResult<T, E = DneaError> = std::result::Result<T, E>;
915|
916|/// A DNEA error.
917|#[derive(Debug, Error)]
918|#[non_exhaustive]
919|pub enum DneaError {
920|    #[error("invalid module id: {0}")]
921|    InvalidModuleId(String),
922|
923|    #[error("invalid envelope: {0}")]
924|    InvalidEnvelope(String),
925|
926|    #[error("handshake rejected: {reason}")]
927|    HandshakeRejected { reason: String },
928|
929|    #[error("unsupported protocol version {got}; supported: {supported}")]
930|    UnsupportedVersion { got: u32, supported: u32 },
931|
932|    #[error("framing error: {0}")]
933|    Framing(#[from] FramingError),
934|
935|    #[error("correlation error: unknown reply_to {0}")]
936|    UnknownCorrelation(String),
937|
938|    #[error("capability denied: {capability} ({reason:?})")]
939|    CapabilityDenied { capability: String, reason: DenyReason },
940|
941|    #[error("constraint violated for {capability}: {detail}")]
942|    ConstraintViolation { capability: String, detail: String },
943|
944|    #[error("policy required: {policy_key}")]
945|    AdminRequired { policy_key: String },
946|
947|    #[error("rate limited; retry after {retry_after_ms}ms")]
948|    RateLimited { retry_after_ms: u64 },
949|
950|    #[error("module error: {module} {code}: {message}")]
951|    Module { module: String, code: String, message: String },
952|
953|    #[error("module unavailable: {0}")]
954|    ModuleUnavailable(String),
955|
956|    #[error("module quarantined: {0}")]
957|    ModuleQuarantined(String),
958|
959|    #[error("storage error: {0}")]
960|    Storage(String),
961|
962|    #[error("manifest error: {0}")]
963|    Manifest(String),
964|
965|    #[error("signature verification failed: {0}")]
966|    Signature(String),
967|
968|    #[error("wasm error: {0}")]
969|    Wasm(String),
970|
971|    #[error("transport disconnected")]
972|    Transport,
973|
974|    #[error("timeout after {0}ms")]
975|    Timeout(u64),
976|
977|    #[error("unsupported feature: {0}")]
978|    UnsupportedFeature(String),
979|
980|    #[error("internal error: {0}")]
981|    Internal(String),
982|}
983|
984|#[derive(Debug, Clone, Copy, PartialEq, Eq)]
985|pub enum DenyReason {
986|    NotGranted,
987|    ModuleDenied,
988|    ConsentExpired,
989|    PolicyForbidden,
990|    PolicyAdminRequired,
991|}
992|
993|#[derive(Debug, Clone, Error)]
994|pub enum FramingError {
995|    #[error("message too large ({actual} > {limit})")]
996|    TooLarge { actual: usize, limit: usize },
997|    #[error("chunk overflow for {cid} ({total})")]
998|    ChunkOverflow { cid: String, total: u32 },
999|    #[error("digest mismatch for {cid}")]
1000|    DigestMismatch { cid: String },
1001|    #[error("invalid JSON: {0}")]
1002|    InvalidJson(String),
1003|    #[error("missing envelope")]
1004|    MissingEnvelope,
1005|}
1006|
1007|impl DneaError {
1008|    /// The stable wire code.
1009|    #[must_use]
1010|    pub fn code(&self) -> &'static str {
1011|        use DneaError::*;
1012|        match self {
1013|            Transport => "E_TRANSPORT_DISCONNECTED",
1014|            UnsupportedVersion { .. } => "E_UNSUPPORTED_VERSION",
1015|            HandshakeRejected { .. } => "E_HANDSHAKE_REJECTED",
1016|            InvalidEnvelope(_) => "E_INVALID_ENVELOPE",
1017|            UnknownCorrelation(_) => "E_UNKNOWN_CORRELATION",
1018|            CapabilityDenied { reason: DenyReason::ModuleDenied, .. }
1019|            | CapabilityDenied { reason: DenyReason::PolicyForbidden, .. } => "E_DENIED",
1020|            CapabilityDenied { .. } => "E_NOT_GRANTED",
1021|            ConstraintViolation { .. } => "E_CONSTRAINT_VIOLATION",
1022|            AdminRequired { .. } => "E_ADMIN_REQUIRED",
1023|            RateLimited { .. } => "E_RATE_LIMITED",
1024|            Module { .. } => "E_MODULE_ERROR",
1025|            ModuleUnavailable(_) => "E_MODULE_UNAVAILABLE",
1026|            ModuleQuarantined(_) => "E_MODULE_QUARANTINED",
1027|            Timeout(_) => "E_TIMEOUT",
1028|            UnsupportedFeature(_) => "E_UNSUPPORTED_FEATURE",
1029|            Framing(FramingError::TooLarge { .. }) => "E_FRAMING_TOO_LARGE",
1030|            Framing(FramingError::ChunkOverflow { .. }) => "E_FRAMING_CHUNK_OVERFLOW",
1031|            Framing(FramingError::DigestMismatch { .. }) => "E_FRAMING_DIGEST_MISMATCH",
1032|            Framing(FramingError::InvalidJson(_)) => "E_FRAMING_INVALID_JSON",
1033|            Framing(FramingError::MissingEnvelope) => "E_INVALID_ENVELOPE",
1034|            Storage(_) | Manifest(_) | Signature(_) | Wasm(_) | Internal(_) => "E_INTERNAL",
1035|            InvalidModuleId(_) => "E_INVALID_ENVELOPE",
1036|        }
1037|    }
1038|
1039|    #[must_use]
1040|    pub fn retryable(&self) -> bool {
1041|        use DneaError::*;
1042|        matches!(
1043|            self,
1044|            Transport | Timeout(_) | RateLimited { .. } | ModuleUnavailable(_)
1045|        )
1046|    }
1047|}
1048|
1049|impl fmt::Display for FramingError {
1050|    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { Ok(()) }
1051|}
1052|
1053|#[cfg(test)]
1054|mod tests {
1055|    use super::*;
1056|
1057|    #[test]
1058|    fn all_variants_have_a_code() {
1059|        let variants: &[DneaError] = &[
1060|            DneaError::InvalidModuleId("x".into()),
1061|            DneaError::InvalidEnvelope("x".into()),
1062|            DneaError::HandshakeRejected { reason: "x".into() },
1063|            DneaError::UnsupportedVersion { got: 2, supported: 1 },
1064|            DneaError::Framing(FramingError::MissingEnvelope),
1065|            DneaError::UnknownCorrelation("x".into()),
1066|            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted },
1067|            DneaError::ConstraintViolation { capability: "x".into(), detail: "y".into() },
1068|            DneaError::AdminRequired { policy_key: "x".into() },
1069|            DneaError::RateLimited { retry_after_ms: 100 },
1070|            DneaError::Module { module: "x".into(), code: "y".into(), message: "z".into() },
1071|            DneaError::ModuleUnavailable("x".into()),
1072|            DneaError::ModuleQuarantined("x".into()),
1073|            DneaError::Storage("x".into()),
1074|            DneaError::Manifest("x".into()),
1075|            DneaError::Signature("x".into()),
1076|            DneaError::Wasm("x".into()),
1077|            DneaError::Transport,
1078|            DneaError::Timeout(100),
1079|            DneaError::UnsupportedFeature("x".into()),
1080|            DneaError::Internal("x".into()),
1081|        ];
1082|        for v in variants {
1083|            assert!(v.code().starts_with("E_"), "{v:?} has no code");
1084|        }
1085|    }
1086|
1087|    #[test]
1088|    fn code_denied_variants_map_correctly() {
1089|        assert_eq!(
1090|            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.code(),
1091|            "E_NOT_GRANTED",
1092|        );
1093|        assert_eq!(
1094|            DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::ModuleDenied }.code(),
1095|            "E_DENIED",
1096|        );
1097|    }
1098|
1099|    #[test]
1100|    fn retryable_is_conservative() {
1101|        assert!(DneaError::Transport.retryable());
1102|        assert!(DneaError::RateLimited { retry_after_ms: 100 }.retryable());
1103|        assert!(!DneaError::CapabilityDenied { capability: "x".into(), reason: DenyReason::NotGranted }.retryable());
1104|        assert!(!DneaError::Internal("x".into()).retryable());
1105|    }
1106|}
1107|```
1108|
1109|### `crates/dnea-core/src/envelope.rs`
1110|
1111|```rust
1112|//! The wire envelope.
1113|
1114|use std::fmt;
1115|
1116|use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
1117|use serde_json::Value;
1118|use time::OffsetDateTime;
1119|
1120|use crate::{
1121|    error::DneaError,
1122|    ids::{EnvelopeId, ModuleId, PortId},
1123|    DneaResult,
1124|};
1125|
1126|pub const PROTOCOL_VERSION: u32 = 1;
1127|
1128|#[derive(Clone, PartialEq, Eq, Hash)]
1129|pub enum Party {
1130|    Runtime,
1131|    Client,
1132|    ContentScript(PortId),
1133|    Module(ModuleId),
1134|}
1135|
1136|impl fmt::Debug for Party {
1137|    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
1138|        match self {
1139|            Party::Runtime => f.write_str("Runtime"),
1140|            Party::Client => f.write_str("Client"),
1141|            Party::ContentScript(id) => write!(f, "ContentScript({id})"),
1142|            Party::Module(id) => write!(f, "Module({id})"),
1143|        }
1144|    }
1145|}
1146|
1147|impl Serialize for Party {
1148|    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
1149|        match self {
1150|            Party::Runtime => s.serialize_str("runtime"),
1151|            Party::Client => s.serialize_str("client"),
1152|            Party::ContentScript(id) => s.serialize_str(&format!("cs:{id}")),
1153|            Party::Module(id) => s.serialize_str(&format!("mod:{id}")),
1154|        }
1155|    }
1156|}
1157|
1158|impl<'de> Deserialize<'de> for Party {
1159|    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
1160|        let s = String::deserialize(d)?;
1161|        if s == "runtime" { return Ok(Party::Runtime); }
1162|        if s == "client" { return Ok(Party::Client); }
1163|        if let Some(rest) = s.strip_prefix("cs:") {
1164|            let id: PortId = rest.parse().map_err(de::Error::custom)?;
1165|            return Ok(Party::ContentScript(id));
1166|        }
1167|        if let Some(rest) = s.strip_prefix("mod:") {
1168|            let id = ModuleId::parse(rest).map_err(de::Error::custom)?;
1169|            return Ok(Party::Module(id));
1170|        }
1171|        Err(de::Error::custom(format!("unknown party: {s}")))
1172|    }
1173|}
1174|
1175|#[derive(Clone, PartialEq, Eq, Hash)]
1176|pub enum MessageType {
1177|    Hello, Welcome, Ping, Pong, Goodbye, Error,
1178|    Invoke, Result, Event, Subscribe, Unsubscribe, Chunk,
1179|    CsRegister, CsUnregister, CsForward,
1180|    CapabilityList, CapabilityListResult, CapabilityChanged,
1181|    ConsentRequest, ConsentResponse,
1182|    UiPanel, UiPopup, UiNotify, UiInject, UiClear,
1183|    Module(String),
1184|}
1185|
1186|impl MessageType {
1187|    #[must_use]
1188|    pub fn as_str(&self) -> &str {
1189|        use MessageType::*;
1190|        match self {
1191|            Hello => "dnea/hello",
1192|            Welcome => "dnea/welcome",
1193|            Ping => "dnea/ping",
1194|            Pong => "dnea/pong",
1195|            Goodbye => "dnea/goodbye",
1196|            Error => "dnea/error",
1197|            Invoke => "dnea/invoke",
1198|            Result => "dnea/result",
1199|            Event => "dnea/event",
1200|            Subscribe => "dnea/subscribe",
1201|            Unsubscribe => "dnea/unsubscribe",
1202|            Chunk => "dnea/chunk",
1203|            CsRegister => "dnea/cs/register",
1204|            CsUnregister => "dnea/cs/unregister",
1205|            CsForward => "dnea/cs/forward",
1206|            CapabilityList => "dnea/capability/list",
1207|            CapabilityListResult => "dnea/capability/list/result",
1208|            CapabilityChanged => "dnea/capability/changed",
1209|            ConsentRequest => "dnea/consent/request",
1210|            ConsentResponse => "dnea/consent/response",
1211|            UiPanel => "dnea/ui/panel",
1212|            UiPopup => "dnea/ui/popup",
1213|            UiNotify => "dnea/ui/notify",
1214|            UiInject => "dnea/ui/inject",
1215|            UiClear => "dnea/ui/clear",
1216|            Module(s) => s,
1217|        }
1218|    }
1219|
1220|    pub fn parse(s: &str) -> DneaResult<Self> {
1221|        use MessageType::*;
1222|        Ok(match s {
1223|            "dnea/hello" => Hello,
1224|            "dnea/welcome" => Welcome,
1225|            "dnea/ping" => Ping,
1226|            "dnea/pong" => Pong,
1227|            "dnea/goodbye" => Goodbye,
1228|            "dnea/error" => Error,
1229|            "dnea/invoke" => Invoke,
1230|            "dnea/result" => Result,
1231|            "dnea/event" => Event,
1232|            "dnea/subscribe" => Subscribe,
1233|            "dnea/unsubscribe" => Unsubscribe,
1234|            "dnea/chunk" => Chunk,
1235|            "dnea/cs/register" => CsRegister,
1236|            "dnea/cs/unregister" => CsUnregister,
1237|            "dnea/cs/forward" => CsForward,
1238|            "dnea/capability/list" => CapabilityList,
1239|            "dnea/capability/list/result" => CapabilityListResult,
1240|            "dnea/capability/changed" => CapabilityChanged,
1241|            "dnea/consent/request" => ConsentRequest,
1242|            "dnea/consent/response" => ConsentResponse,
1243|            "dnea/ui/panel" => UiPanel,
1244|            "dnea/ui/popup" => UiPopup,
1245|            "dnea/ui/notify" => UiNotify,
1246|            "dnea/ui/inject" => UiInject,
1247|            "dnea/ui/clear" => UiClear,
1248|            other if other.starts_with("mod/") => Module(other.to_string()),
1249|            other => return Err(DneaError::InvalidEnvelope(format!("unknown message type: {other}"))),
1250|        })
1251|    }
1252|}
1253|
1254|impl fmt::Debug for MessageType {
1255|    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
1256|        write!(f, "MessageType({})", self.as_str())
1257|    }
1258|}
1259|
1260|impl Serialize for MessageType {
1261|    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
1262|        s.serialize_str(self.as_str())
1263|    }
1264|}
1265|
1266|impl<'de> Deserialize<'de> for MessageType {
1267|    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
1268|        let s = String::deserialize(d)?;
1269|        MessageType::parse(&s).map_err(de::Error::custom)
1270|    }
1271|}
1272|
1273|#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
1274|pub enum Encoding { #[default] Identity, Zstd, Br }
1275|
1276|impl Serialize for Encoding {
1277|    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
1278|        s.serialize_str(match self { Encoding::Identity => "identity", Encoding::Zstd => "zstd", Encoding::Br => "br" })
1279|    }
1280|}
1281|
1282|impl<'de> Deserialize<'de> for Encoding {
1283|    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
1284|        let s = String::deserialize(d)?;
1285|        match s.as_str() {
1286|            "identity" => Ok(Encoding::Identity),
1287|            "zstd" => Ok(Encoding::Zstd),
1288|            "br" => Ok(Encoding::Br),
1289|            other => Err(de::Error::custom(format!("unknown encoding: {other}"))),
1290|        }
1291|    }
1292|}
1293|
1294|#[derive(Clone, Debug, Serialize, Deserialize)]
1295|pub struct EnvelopeError {
1296|    pub code: String,
1297|    pub message: String,
1298|    #[serde(default, skip_serializing_if = "is_false")]
1299|    pub retryable: bool,
1300|    #[serde(skip_serializing_if = "Option::is_none")]
1301|    pub retry_after_ms: Option<u64>,
1302|    #[serde(skip_serializing_if = "Option::is_none")]
1303|    pub details: Option<Value>,
1304|    #[serde(skip_serializing_if = "Option::is_none")]
1305|    pub cause: Option<String>,
1306|}
1307|
1308|#[derive(Clone, Debug, Serialize, Deserialize)]
1309|pub struct Envelope {
1310|    pub v: u32,
1311|    pub id: EnvelopeId,
1312|    #[serde(rename = "type")]
1313|    pub kind: MessageType,
1314|    pub ts: OffsetDateTime,
1315|    pub src: Party,
1316|    pub dst: Party,
1317|    #[serde(skip_serializing_if = "Option::is_none")]
1318|    pub reply_to: Option<EnvelopeId>,
1319|    #[serde(skip_serializing_if = "Option::is_none")]
1320|    pub cap: Option<String>,
1321|    #[serde(skip_serializing_if = "Option::is_none")]
1322|    pub module: Option<ModuleId>,
1323|    #[serde(default, skip_serializing_if = "is_identity")]
1324|    pub enc: Encoding,
1325|    #[serde(skip_serializing_if = "Option::is_none")]
1326|    pub ttl_ms: Option<u64>,
1327|    #[serde(skip_serializing_if = "Option::is_none")]
1328|    pub payload: Option<Value>,
1329|    #[serde(skip_serializing_if = "Option::is_none")]
1330|    pub err: Option<EnvelopeError>,
1331|    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
1332|    pub meta: serde_json::Map<String, Value>,
1333|}
1334|
1335|#[derive(Clone, Copy, Debug, PartialEq, Eq)]
1336|pub enum TransportDirection { Client, Runtime }
1337|
1338|fn is_identity(e: &Encoding) -> bool { matches!(e, Encoding::Identity) }
1339|fn is_false(b: &bool) -> bool { !*b }
1340|
1341|impl Envelope {
1342|    #[must_use]
1343|    pub fn new(kind: MessageType, src: Party, dst: Party) -> Self {
1344|        Self {
1345|            v: PROTOCOL_VERSION,
1346|            id: EnvelopeId::new(),
1347|            kind,
1348|            ts: OffsetDateTime::now_utc(),
1349|            src,
1350|            dst,
1351|            reply_to: None,
1352|            cap: None,
1353|            module: None,
1354|            enc: Encoding::Identity,
1355|            ttl_ms: None,
1356|            payload: None,
1357|            err: None,
1358|            meta: Default::default(),
1359|        }
1360|    }
1361|
1362|    #[must_use]
1363|    pub fn reply(&self, kind: MessageType) -> Self {
1364|        Self {
1365|            v: self.v,
1366|            id: EnvelopeId::new(),
1367|            kind,
1368|            ts: OffsetDateTime::now_utc(),
1369|            src: self.dst.clone(),
1370|            dst: self.src.clone(),
1371|            reply_to: Some(self.id),
1372|            cap: None,
1373|            module: None,
1374|            enc: Encoding::Identity,
1375|            ttl_ms: None,
1376|            payload: None,
1377|            err: None,
1378|            meta: Default::default(),
1379|        }
1380|    }
1381|
1382|    #[must_use]
1383|    pub fn from_error(req: &Envelope, err: &DneaError) -> Self {
1384|        let mut out = req.reply(MessageType::Error);
1385|        out.err = Some(EnvelopeError {
1386|            code: err.code().into(),
1387|            message: err.to_string(),
1388|            retryable: err.retryable(),
1389|            retry_after_ms: match err {
1390|                DneaError::RateLimited { retry_after_ms } => Some(*retry_after_ms),
1391|                _ => None,
1392|            },
1393|            details: None,
1394|            cause: None,
1395|        });
1396|        out
1397|    }
1398|
1399|    pub fn validate_direction(&self, direction: TransportDirection) -> DneaResult<()> {
1400|        let ok = matches!(
1401|            (direction, &self.src),
1402|            (TransportDirection::Client, Party::Client)
1403|                | (TransportDirection::Client, Party::ContentScript(_))
1404|                | (TransportDirection::Runtime, Party::Runtime)
1405|                | (TransportDirection::Runtime, Party::Module(_))
1406|        );
1407|        if ok { Ok(()) } else {
1408|            Err(DneaError::InvalidEnvelope(format!(
1409|                "envelope src={:?} is inconsistent with {direction:?} transport", self.src
1410|            )))
1411|        }
1412|    }
1413|}
1414|
1415|impl Default for Envelope {
1416|    fn default() -> Self { Self::new(MessageType::Ping, Party::Runtime, Party::Client) }
1417|}
1418|
1419|#[cfg(test)]
1420|mod tests {
1421|    use super::*;
1422|    use crate::ids::PortId;
1423|
1424|    #[test]
1425|    fn message_type_round_trips() {
1426|        for t in [
1427|            MessageType::Hello, MessageType::Welcome, MessageType::Ping,
1428|            MessageType::Pong, MessageType::Goodbye, MessageType::Error,
1429|            MessageType::ConsentRequest, MessageType::UiInject,
1430|            MessageType::Module("mod/com.example.focus-mode/getState".into()),
1431|        ] {
1432|            assert_eq!(MessageType::parse(t.as_str()).unwrap(), t);
1433|        }
1434|    }
1435|
1436|    #[test]
1437|    fn party_round_trips() {
1438|        let parties = [
1439|            Party::Runtime, Party::Client,
1440|            Party::ContentScript(PortId::new()),
1441|            Party::Module(ModuleId::parse("com.example.focus-mode").unwrap()),
1442|        ];
1443|        for p in parties {
1444|            let json = serde_json::to_string(&p).unwrap();
1445|            let back: Party = serde_json::from_str(&json).unwrap();
1446|            assert_eq!(back, p);
1447|        }
1448|    }
1449|
1450|    #[test]
1451|    fn reply_swaps_src_and_dst() {
1452|        let req = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
1453|        let rep = req.reply(MessageType::Pong);
1454|        assert_eq!(rep.src, Party::Runtime);
1455|        assert_eq!(rep.dst, Party::Client);
1456|        assert_eq!(rep.reply_to, Some(req.id));
1457|    }
1458|
1459|    #[test]
1460|    fn direction_validation_enforced() {
1461|        let client_env = Envelope::new(MessageType::Ping, Party::Client, Party::Runtime);
1462|        assert!(client_env.validate_direction(TransportDirection::Client).is_ok());
1463|        assert!(client_env.validate_direction(TransportDirection::Runtime).is_err());
1464|    }
1465|}
1466|```
1467|
1468|### `crates/dnea-core/src/codec.rs`
1469|
1470|```rust
1471|//! The wire codec.
1472|
1473|use std::{
1474|    sync::Arc,
1475|    time::{Duration, Instant},
1476|};
1477|
1478|use bytes::Bytes;
1479|use dashmap::DashMap;
1480|use serde::{Deserialize, Serialize};
1481|use sha2::{Digest, Sha256};
1482|
1483|use crate::{
1484|    envelope::{Envelope, MessageType},
1485|    error::FramingError,
1486|    DneaError, DneaResult,
1487|};
1488|
1489|pub const DEFAULT_MTU: usize = 1_048_576;
1490|pub const OVERHEAD_BUDGET: usize = 4_096;
1491|pub const MAX_CHUNKS: u32 = 256;
1492|pub const CHUNK_TIMEOUT: Duration = Duration::from_secs(30);
1493|
1494|#[derive(Clone, Debug, Serialize, Deserialize)]
1495|struct ChunkFrame {
1496|    v: u32,
1497|    #[serde(rename = "type")]
1498|    kind: MessageType,
1499|    cid: String,
1500|    seq: u32,
1501|    total: u32,
1502|    part: String,
1503|    digest: String,
1504|}
1505|
1506|struct Inflight {
1507|    parts: Vec<Bytes>,
1508|    total: u32,
1509|    hasher: Sha256,
1510|    started: Instant,
1511|}
1512|
1513|#[derive(Debug)]
1514|pub enum Decoded {
1515|    Envelope(Box<Envelope>),
1516|    Incomplete,
1517|}
1518|
1519|pub struct Codec {
1520|    mtu: usize,
1521|    inflight: Arc<DashMap<String, Inflight>>,
1522|}
1523|
1524|impl std::fmt::Debug for Codec {
1525|    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
1526|        f.debug_struct("Codec").field("mtu", &self.mtu).finish_non_exhaustive()
1527|    }
1528|}
1529|
1530|impl Codec {
1531|    #[must_use]
1532|    pub fn new(mtu: usize) -> Self {
1533|        assert!(mtu > OVERHEAD_BUDGET + 512, "MTU {mtu} is too small");
1534|        Self { mtu, inflight: Arc::new(DashMap::new()) }
1535|    }
1536|
1537|    #[must_use]
1538|    pub fn default_mtu() -> Self { Self::new(DEFAULT_MTU) }
1539|
1540|    #[must_use]
1541|    pub fn mtu(&self) -> usize { self.mtu }
1542|
1543|    pub fn encode(&self, env: &Envelope) -> DneaResult<Vec<Vec<u8>>> {
1544|        let body = serde_json::to_vec(env)
1545|            .map_err(|e| DneaError::Internal(format!("envelope serialization: {e}")))?;
1546|        if body.len() <= self.mtu - OVERHEAD_BUDGET {
1547|            return Ok(vec![body]);
1548|        }
1549|        self.encode_chunked(env, &body)
1550|    }
1551|
1552|    fn encode_chunked(&self, env: &Envelope, body: &[u8]) -> DneaResult<Vec<Vec<u8>>> {
1553|        let digest = {
1554|            let mut h = Sha256::new();
1555|            h.update(body);
1556|            format!("sha-256:{}", base64_url_encode(&h.finalize()))
1557|        };
1558|        let payload_budget = self.mtu - OVERHEAD_BUDGET - 512;
1559|        let total = body.len().div_ceil(payload_budget) as u32;
1560|        if total > MAX_CHUNKS {
1561|            return Err(DneaError::Framing(FramingError::ChunkOverflow {
1562|                cid: env.id.to_string(), total,
1563|            }));
1564|        }
1565|        let mut frames = Vec::with_capacity(total as usize);
1566|        for seq in 0..total {
1567|            let start = seq as usize * payload_budget;
1568|            let end = (start + payload_budget).min(body.len());
1569|            let frame = ChunkFrame {
1570|                v: env.v, kind: MessageType::Chunk, cid: env.id.to_string(),
1571|                seq, total, part: base64_url_encode(&body[start..end]), digest: digest.clone(),
1572|            };
1573|            let bytes = serde_json::to_vec(&frame)
1574|                .map_err(|e| DneaError::Internal(format!("chunk serialization: {e}")))?;
1575|            frames.push(bytes);
1576|        }
1577|        Ok(frames)
1578|    }
1579|
1580|    pub fn decode(&self, raw: &[u8]) -> DneaResult<Decoded> {
1581|        if raw.len() > self.mtu + OVERHEAD_BUDGET {
1582|            return Err(DneaError::Framing(FramingError::TooLarge {
1583|                actual: raw.len(), limit: self.mtu,
1584|            }));
1585|        }
1586|        let value: serde_json::Value = serde_json::from_slice(raw)
1587|            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
1588|        let is_chunk = value.get("type").and_then(|v| v.as_str()) == Some("dnea/chunk");
1589|        if is_chunk {
1590|            return self.decode_chunk(value);
1591|        }
1592|        let env: Envelope = serde_json::from_value(value)
1593|            .map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
1594|        Ok(Decoded::Envelope(Box::new(env)))
1595|    }
1596|
1597|    fn decode_chunk(&self, value: serde_json::Value) -> DneaResult<Decoded> {
1598|        let frame: ChunkFrame = serde_json::from_value(value)
1599|            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
1600|        if frame.total == 0 || frame.total > MAX_CHUNKS || frame.seq >= frame.total {
1601|            return Err(DneaError::Framing(FramingError::ChunkOverflow {
1602|                cid: frame.cid, total: frame.total,
1603|            }));
1604|        }
1605|        let part = base64_url_decode(&frame.part)
1606|            .map_err(|e| DneaError::Framing(FramingError::InvalidJson(e.to_string())))?;
1607|        self.sweep_stale();
1608|        let mut entry = self.inflight.entry(frame.cid.clone()).or_insert_with(|| Inflight {
1609|            parts: Vec::with_capacity(frame.total as usize),
1610|            total: frame.total, hasher: Sha256::new(), started: Instant::now(),
1611|        });
1612|        if entry.total != frame.total || frame.seq as usize != entry.parts.len() {
1613|            drop(entry);
1614|            self.inflight.remove(&frame.cid);
1615|            return Err(DneaError::Framing(FramingError::ChunkOverflow {
1616|                cid: frame.cid, total: frame.total,
1617|            }));
1618|        }
1619|        entry.hasher.update(&part);
1620|        entry.parts.push(Bytes::from(part));
1621|        if entry.parts.len() as u32 != entry.total {
1622|            return Ok(Decoded::Incomplete);
1623|        }
1624|        drop(entry);
1625|        let entry = self.inflight.remove(&frame.cid)
1626|            .ok_or_else(|| DneaError::Internal("inflight vanished".into()))?.1;
1627|        let actual = format!("sha-256:{}", base64_url_encode(&entry.hasher.finalize()));
1628|        if actual != frame.digest {
1629|            return Err(DneaError::Framing(FramingError::DigestMismatch { cid: frame.cid }));
1630|        }
1631|        let full: Vec<u8> = entry.parts.into_iter().flat_map(|b| b.to_vec()).collect();
1632|        let env: Envelope = serde_json::from_slice(&full)
1633|            .map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
1634|        Ok(Decoded::Envelope(Box::new(env)))
1635|    }
1636|
1637|    fn sweep_stale(&self) {
1638|        let now = Instant::now();
1639|        self.inflight.retain(|_, e| now.duration_since(e.started) < CHUNK_TIMEOUT);
1640|    }
1641|}
1642|
1643|fn base64_url_encode(input: &[u8]) -> String {
1644|    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
1645|    URL_SAFE_NO_PAD.encode(input)
1646|}
1647|
1648|fn base64_url_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
1649|    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
1650|    URL_SAFE_NO_PAD.decode(input)
1651|}
1652|
1653|#[cfg(test)]
1654|mod tests {
1655|    use super::*;
1656|    use crate::envelope::Party;
1657|    use serde_json::json;
1658|
1659|    fn large(bytes: usize) -> Envelope {
1660|        let mut e = Envelope::new(MessageType::Event, Party::Runtime, Party::Client);
1661|        e.payload = Some(json!({ "data": "x".repeat(bytes) }));
1662|        e
1663|    }
1664|
1665|    #[test]
1666|    fn large_message_chunks_and_round_trips() {
1667|        let codec = Codec::default_mtu();
1668|        let env = large(3 * 1024 * 1024);
1669|        let frames = codec.encode(&env).unwrap();
1670|        assert!(frames.len() > 1);
1671|        let mut decoded = None;
1672|        for frame in &frames {
1673|            match codec.decode(frame).unwrap() {
1674|                Decoded::Envelope(e) => decoded = Some(e),
1675|                Decoded::Incomplete => {}
1676|            }
1677|        }
1678|        let decoded = decoded.expect("reassembly");
1679|        assert_eq!(decoded.id, env.id);
1680|        assert_eq!(decoded.payload, env.payload);
1681|    }
1682|
1683|    #[test]
1684|    fn out_of_order_rejected() {
1685|        let codec = Codec::default_mtu();
1686|        let frames = codec.encode(&large(2 * 1024 * 1024)).unwrap();
1687|        assert!(frames.len() >= 2);
1688|        let err = codec.decode(&frames[1]).unwrap_err();
1689|        assert!(matches!(err, DneaError::Framing(FramingError::ChunkOverflow { .. })));
1690|    }
1691|
1692|    #[test]
1693|    fn digest_mismatch_rejected() {
1694|        let codec = Codec::default_mtu();
1695|        let mut frames = codec.encode(&large(2 * 1024 * 1024)).unwrap();
1696|        let last = frames.last_mut().unwrap();
1697|        let s = std::str::from_utf8(last).unwrap();
1698|        *last = s.replace("\"digest\":\"sha-256:", "\"digest\":\"sha-256:AAAA").into_bytes();
1699|        let mut err = None;
1700|        for frame in &frames {
1701|            match codec.decode(frame) {
1702|                Ok(Decoded::Envelope(_)) => panic!("corruption must not decode"),
1703|                Ok(Decoded::Incomplete) => {}
1704|                Err(e) => err = Some(e),
1705|            }
1706|        }
1707|        assert!(matches!(err, Some(DneaError::Framing(FramingError::DigestMismatch { .. }))));
1708|    }
1709|
1710|    #[test]
1711|    fn oversize_rejected() {
1712|        let codec = Codec::new(1_048_576);
1713|        let oversize = vec![b'{'; 2_000_000];
1714|        assert!(matches!(
1715|            codec.decode(&oversize),
1716|            Err(DneaError::Framing(FramingError::TooLarge { .. }))
1717|        ));
1718|    }
1719|
1720|    #[test]
1721|    fn chunk_count_exceeds_limit() {
1722|        let codec = Codec::new(4_096);
1723|        let env = large(2 * 1024 * 1024);
1724|        assert!(matches!(
1725|            codec.encode(&env),
1726|            Err(DneaError::Framing(FramingError::ChunkOverflow { .. }))
1727|        ));
1728|    }
1729|}
1730|```
1731|
1732|### `crates/dnea-core/tests/property.rs`
1733|
1734|```rust
1735|//! Property tests for the wire codec.
1736|
1737|use dnea_core::{
1738|    codec::{Codec, Decoded, DEFAULT_MTU, MAX_CHUNKS},
1739|    envelope::{Envelope, MessageType, Party},
1740|};
1741|use proptest::prelude::*;
1742|use serde_json::json;
1743|
1744|fn arb_envelope() -> impl Strategy<Value = Envelope> {
1745|    (0usize..200_000).prop_map(|n| {
1746|        let mut e = Envelope::new(MessageType::Event, Party::Runtime, Party::Client);
1747|        e.payload = Some(json!({ "n": n, "s": "y".repeat(n % 8192) }));
1748|        e
1749|    })
1750|}
1751|
1752|proptest! {
1753|    #![proptest_config(ProptestConfig::with_cases(500))]
1754|
1755|    #[test]
1756|    fn round_trip(env in arb_envelope()) {
1757|        let codec = Codec::new(DEFAULT_MTU);
1758|        let frames = codec.encode(&env).unwrap();
1759|        let mut decoded = None;
1760|        for frame in &frames {
1761|            match codec.decode(frame).unwrap() {
1762|                Decoded::Envelope(e) => decoded = Some(*e),
1763|                Decoded::Incomplete => {}
1764|            }
1765|        }
1766|        let decoded = decoded.expect("reassembly");
1767|        prop_assert_eq!(decoded.id, env.id);
1768|        prop_assert_eq!(decoded.payload, env.payload);
1769|    }
1770|
1771|    #[test]
1772|    fn frames_fit_mtu(env in arb_envelope()) {
1773|        let codec = Codec::new(DEFAULT_MTU);
1774|        for f in codec.encode(&env).unwrap() {
1775|            prop_assert!(f.len() <= DEFAULT_MTU);
1776|        }
1777|    }
1778|
1779|    #[test]
1780|    fn chunk_count_within_limit(env in arb_envelope()) {
1781|        let codec = Codec::new(DEFAULT_MTU);
1782|        if let Ok(frames) = codec.encode(&env) {
1783|            prop_assert!(frames.len() as u32 <= MAX_CHUNKS);
1784|        }
1785|    }
1786|}
1787|```
1788|
1789|## 3.2 `crates/dnea-config`
1790|
1791|### `crates/dnea-config/Cargo.toml`
1792|
1793|```toml
1794|[package]
1795|name        = "dnea-config"
1796|version.workspace = true
1797|description = "DNEA configuration loading."
1798|edition.workspace     = true
1799|rust-version.workspace = true
1800|license.workspace     = true
1801|
1802|[dependencies]
1803|dnea-core   = { path = "../dnea-core" }
1804|anyhow      = { workspace = true }
1805|directories = { workspace = true }
1806|serde       = { workspace = true }
1807|serde_json  = { workspace = true }
1808|thiserror   = { workspace = true }
1809|toml        = { workspace = true }
1810|```
1811|
1812|### `crates/dnea-config/src/lib.rs`
1813|
1814|```rust
1815|//! DNEA configuration.
1816|
1817|#![forbid(unsafe_code)]
1818|#![deny(rust_2018_idioms, unreachable_pub)]
1819|#![warn(missing_docs)]
1820|
1821|use std::path::{Path, PathBuf};
1822|use std::time::Duration;
1823|
1824|use serde::{Deserialize, Serialize};
1825|use thiserror::Error;
1826|
1827|#[derive(Debug, Error)]
1828|pub enum ConfigError {
1829|    #[error("read config: {0}")]
1830|    Io(#[from] std::io::Error),
1831|    #[error("parse config: {0}")]
1832|    Parse(#[from] toml::de::Error),
1833|    #[error("no home directory")]
1834|    NoHome,
1835|}
1836|
1837|#[derive(Clone, Debug, Serialize, Deserialize)]
1838|#[serde(default)]
1839|pub struct Config {
1840|    pub paths: Paths,
1841|    pub modules: ModulesConfig,
1842|    pub nm: NmConfig,
1843|    pub api: ApiConfig,
1844|    pub store: StoreConfig,
1845|    pub audit: AuditConfig,
1846|    pub policy: PolicyConfig,
1847|    pub watchdog: WatchdogConfig,
1848|}
1849|
1850|impl Default for Config {
1851|    fn default() -> Self {
1852|        Self {
1853|            paths: Paths::default(), modules: ModulesConfig::default(),
1854|            nm: NmConfig::default(), api: ApiConfig::default(),
1855|            store: StoreConfig::default(), audit: AuditConfig::default(),
1856|            policy: PolicyConfig::default(), watchdog: WatchdogConfig::default(),
1857|        }
1858|    }
1859|}
1860|
1861|#[derive(Clone, Debug, Serialize, Deserialize)]
1862|#[serde(default)]
1863|pub struct Paths {
1864|    pub state_dir: PathBuf,
1865|    pub run_dir: PathBuf,
1866|    pub log_dir: PathBuf,
1867|    pub modules_dir: PathBuf,
1868|    pub pid_file: PathBuf,
1869|}
1870|
1871|impl Default for Paths {
1872|    fn default() -> Self {
1873|        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
1874|            .expect("no home directory");
1875|        let state_dir = proj.state_dir().map(Path::to_path_buf)
1876|            .unwrap_or_else(|| PathBuf::from("/var/lib/dnea"));
1877|        let run_dir = proj.runtime_dir().map(Path::to_path_buf)
1878|            .unwrap_or_else(std::env::temp_dir);
1879|        let log_dir = proj.state_dir().map(|p| p.join("log"))
1880|            .unwrap_or_else(|| PathBuf::from("/var/log/dnea"));
1881|        Self {
1882|            modules_dir: state_dir.join("modules"),
1883|            pid_file: run_dir.join("dnea.pid"),
1884|            state_dir, run_dir, log_dir,
1885|        }
1886|    }
1887|}
1888|
1889|#[derive(Clone, Debug, Serialize, Deserialize)]
1890|#[serde(default)]
1891|pub struct ModulesConfig {
1892|    pub allow_dev: bool,
1893|    pub trust_store: Option<PathBuf>,
1894|    pub max_loaded: usize,
1895|    pub pooling_slots: u32,
1896|}
1897|
1898|impl Default for ModulesConfig {
1899|    fn default() -> Self {
1900|        Self { allow_dev: false, trust_store: None, max_loaded: 32, pooling_slots: 16 }
1901|    }
1902|}
1903|
1904|#[derive(Clone, Debug, Serialize, Deserialize)]
1905|#[serde(default)]
1906|pub struct NmConfig {
1907|    pub endpoint: Option<PathBuf>,
1908|    pub max_connections: usize,
1909|    pub mtu: usize,
1910|}
1911|
1912|impl Default for NmConfig {
1913|    fn default() -> Self {
1914|        Self { endpoint: None, max_connections: 64, mtu: 1_048_576 }
1915|    }
1916|}
1917|
1918|#[derive(Clone, Debug, Serialize, Deserialize)]
1919|#[serde(default)]
1920|pub struct ApiConfig {
1921|    pub bind: String,
1922|    pub discovery_path: Option<PathBuf>,
1923|    pub token_ttl_secs: u64,
1924|    pub install_roots: Vec<PathBuf>,
1925|}
1926|
1927|impl Default for ApiConfig {
1928|    fn default() -> Self {
1929|        Self {
1930|            bind: "127.0.0.1:0".into(),
1931|            discovery_path: None,
1932|            token_ttl_secs: 600,
1933|            install_roots: Vec::new(),
1934|        }
1935|    }
1936|}
1937|
1938|#[derive(Clone, Debug, Serialize, Deserialize)]
1939|#[serde(default)]
1940|pub struct StoreConfig {
1941|    pub db_path: Option<PathBuf>,
1942|    pub master_key: MasterKeySource,
1943|    pub default_quota_bytes: u64,
1944|}
1945|
1946|impl Default for StoreConfig {
1947|    fn default() -> Self {
1948|        Self {
1949|            db_path: None,
1950|            master_key: MasterKeySource::Auto,
1951|            default_quota_bytes: 10 * 1024 * 1024,
1952|        }
1953|    }
1954|}
1955|
1956|#[derive(Clone, Debug, Serialize, Deserialize)]
1957|#[serde(rename_all = "kebab-case", tag = "kind")]
1958|pub enum MasterKeySource {
1959|    Auto,
1960|    OsKeystore,
1961|    File { path: PathBuf },
1962|    Passphrase,
1963|}
1964|
1965|#[derive(Clone, Debug, Serialize, Deserialize)]
1966|#[serde(default)]
1967|pub struct AuditConfig {
1968|    pub retention_days: u32,
1969|    pub max_bytes_per_module: u64,
1970|    pub sinks: Vec<AuditSinkKind>,
1971|}
1972|
1973|impl Default for AuditConfig {
1974|    fn default() -> Self {
1975|        Self {
1976|            retention_days: 30,
1977|            max_bytes_per_module: 64 * 1024 * 1024,
1978|            sinks: vec![AuditSinkKind::Sqlite, AuditSinkKind::File],
1979|        }
1980|    }
1981|}
1982|
1983|#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
1984|#[serde(rename_all = "kebab-case")]
1985|pub enum AuditSinkKind { Sqlite, File, OsLog }
1986|
1987|#[derive(Clone, Debug, Serialize, Deserialize)]
1988|#[serde(default)]
1989|pub struct PolicyConfig {
1990|    pub extra_sources: Vec<PathBuf>,
1991|    pub poll_interval_secs: u64,
1992|}
1993|
1994|impl Default for PolicyConfig {
1995|    fn default() -> Self {
1996|        Self { extra_sources: Vec::new(), poll_interval_secs: 60 }
1997|    }
1998|}
1999|
2000|#[derive(Clone, Debug, Serialize, Deserialize)]
2001|#[serde(default)]
2002|pub struct WatchdogConfig {
2003|    pub epoch_interval_ms: u64,
2004|    pub default_fuel: u64,
2005|    pub default_timeout_ms: u64,
2006|}
2007|
2008|impl Default for WatchdogConfig {
2009|    fn default() -> Self {
2010|        Self { epoch_interval_ms: 100, default_fuel: 10_000_000, default_timeout_ms: 30_000 }
2011|    }
2012|}
2013|
2014|impl Config {
2015|    pub fn load() -> Result<Self, ConfigError> {
2016|        let path = Self::default_path();
2017|        let mut cfg = if path.exists() {
2018|            let text = std::fs::read_to_string(&path)?;
2019|            toml::from_str(&text)?
2020|        } else {
2021|            Self::default()
2022|        };
2023|        cfg.apply_env();
2024|        cfg.validate()?;
2025|        Ok(cfg)
2026|    }
2027|
2028|    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
2029|        let text = std::fs::read_to_string(path)?;
2030|        let mut cfg: Self = toml::from_str(&text)?;
2031|        cfg.apply_env();
2032|        cfg.validate()?;
2033|        Ok(cfg)
2034|    }
2035|
2036|    /// Validates configuration invariants that MUST hold at startup.
2037|    pub fn validate(&self) -> Result<(), ConfigError> {
2038|        // Local API MUST bind to loopback only.
2039|        let bind = &self.api.bind;
2040|        let is_loopback = bind.starts_with("127.")
2041|            || bind.starts_with("[::1]")
2042|            || bind.starts_with("localhost");
2043|        if !is_loopback {
2044|            return Err(ConfigError::Io(std::io::Error::new(
2045|                std::io::ErrorKind::InvalidInput,
2046|                format!("api.bind must be a loopback address, got {bind}"),
2047|            )));
2048|        }
2049|        Ok(())
2050|    }
2051|
2052|    #[must_use]
2053|    pub fn default_path() -> PathBuf {
2054|        if let Ok(p) = std::env::var("DNEA_CONFIG") {
2055|            return PathBuf::from(p);
2056|        }
2057|        directories::ProjectDirs::from("org", "dnea", "runtime")
2058|            .map(|p| p.config_dir().join("dnea.toml"))
2059|            .unwrap_or_else(|| PathBuf::from("/etc/dnea/dnea.toml"))
2060|    }
2061|
2062|    fn apply_env(&mut self) {
2063|        if let Ok(v) = std::env::var("DNEA_STATE_DIR") { self.paths.state_dir = PathBuf::from(v); }
2064|        if let Ok(v) = std::env::var("DNEA_RUN_DIR") { self.paths.run_dir = PathBuf::from(v); }
2065|        if let Ok(v) = std::env::var("DNEA_LOG_DIR") { self.paths.log_dir = PathBuf::from(v); }
2066|        if let Ok(v) = std::env::var("DNEA_ALLOW_DEV") {
2067|            self.modules.allow_dev = v == "1" || v.eq_ignore_ascii_case("true");
2068|        }
2069|        if let Ok(v) = std::env::var("DNEA_EPOCH_INTERVAL_MS") {
2070|            if let Ok(n) = v.parse() { self.watchdog.epoch_interval_ms = n; }
2071|        }
2072|    }
2073|
2074|    #[must_use]
2075|    pub fn epoch_interval(&self) -> Duration {
2076|        Duration::from_millis(self.watchdog.epoch_interval_ms)
2077|    }
2078|
2079|    #[must_use]
2080|    pub fn token_ttl(&self) -> Duration {
2081|        Duration::from_secs(self.api.token_ttl_secs)
2082|    }
2083|
2084|    #[must_use]
2085|    pub fn nm_endpoint(&self) -> PathBuf {
2086|        if let Some(p) = &self.nm.endpoint { return p.clone(); }
2087|        #[cfg(unix)]
2088|        { self.paths.run_dir.join("nm.sock") }
2089|        #[cfg(windows)]
2090|        {
2091|            let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
2092|            PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}"))
2093|        }
2094|    }
2095|
2096|    #[must_use]
2097|    pub fn db_path(&self) -> PathBuf {
2098|        self.store.db_path.clone().unwrap_or_else(|| self.paths.state_dir.join("dnea.db"))
2099|    }
2100|}
2101|```
2102|
2103|## 3.3 `crates/dnea-abi`
2104|
2105|### `crates/dnea-abi/Cargo.toml`
2106|
2107|```toml
2108|[package]
2109|name        = "dnea-abi"
2110|version.workspace = true
2111|description = "Generated host bindings for the DNEA module world."
2112|edition.workspace     = true
2113|rust-version.workspace = true
2114|license.workspace     = true
2115|
2116|[dependencies]
2117|wasmtime = { workspace = true }
2118|anyhow   = { workspace = true }
2119|```
2120|
2121|### `crates/dnea-abi/src/lib.rs`
2122|
2123|```rust
2124|//! Host bindings for the DNEA module world.
2125|
2126|#![forbid(unsafe_code)]
2127|#![allow(clippy::all, missing_docs)]
2128|
2129|wasmtime::component::bindgen!({
2130|    path: "../wit",
2131|    world: "module",
2132|    async: true,
2133|    tracing: true,
2134|});
2135|```
2136|
2137|## 3.4 `crates/dnea-manifest`
2138|
2139|### `crates/dnea-manifest/Cargo.toml`
2140|
2141|```toml
2142|[package]
2143|name        = "dnea-manifest"
2144|version.workspace = true
2145|description = "DNEA module manifest: schema, canonicalization, signing, validation."
2146|edition.workspace     = true
2147|rust-version.workspace = true
2148|license.workspace     = true
2149|
2150|[dependencies]
2151|dnea-core     = { path = "../dnea-core" }
2152|base64        = { workspace = true }
2153|ed25519-dalek = { workspace = true }
2154|jsonschema    = { workspace = true }
2155|once_cell     = { workspace = true }
2156|regex         = { workspace = true }
2157|serde         = { workspace = true }
2158|serde_json    = { workspace = true }
2159|sha2          = { workspace = true }
2160|thiserror     = { workspace = true }
2161|time          = { workspace = true }
2162|uuid          = { workspace = true }
2163|
2164|[dev-dependencies]
2165|ed25519-dalek = { workspace = true }
2166|rand          = { workspace = true }
2167|tempfile      = { workspace = true }
2168|```
2169|
2170|### `crates/dnea-manifest/src/lib.rs`
2171|
2172|```rust
2173|//! DNEA module manifests.
2174|
2175|#![forbid(unsafe_code)]
2176|#![deny(rust_2018_idioms, unreachable_pub)]
2177|#![warn(missing_docs)]
2178|
2179|pub mod canonical;
2180|pub mod capability;
2181|pub mod manifest;
2182|pub mod signature;
2183|pub mod trust;
2184|pub mod validate;
2185|
2186|pub use canonical::{canonicalize, CanonicalError};
2187|pub use capability::{is_valid_capability_id, CapabilityRegistry, ConsentTier};
2188|pub use manifest::{
2189|    Author, Capabilities, CapabilityRequest, Conflict, Dependency, DynamicOriginGrants,
2190|    Entrypoint, HookRef, Lifecycle, Manifest, ManifestError, Migration, PayloadDigest,
2191|    Signature, Signer, Storage, UiSurface,
2192|};
2193|pub use signature::{sign_manifest, verify_manifest, SignatureInfo, VerifiedSigner};
2194|pub use trust::{ResolvedKey, SigAlg, StaticTrustStore, TrustStore};
2195|pub use validate::{ValidationError, ValidationReport, Validator};
2196|```
2197|
2198|### `crates/dnea-manifest/src/canonical.rs`
2199|
2200|```rust
2201|//! RFC 8785 (JCS) canonicalization.
2202|
2203|use serde_json::Value;
2204|use thiserror::Error;
2205|
2206|#[derive(Debug, Error)]
2207|pub enum CanonicalError {
2208|    #[error("number is not finite")]
2209|    NonFinite,
2210|    #[error("number cannot be represented in JCS: {0}")]
2211|    UnrepresentableNumber(String),
2212|}
2213|
2214|pub fn canonicalize(value: &Value) -> Result<String, CanonicalError> {
2215|    let mut out = String::with_capacity(256);
2216|    write_value(value, &mut out)?;
2217|    Ok(out)
2218|}
2219|
2220|fn write_value(v: &Value, out: &mut String) -> Result<(), CanonicalError> {
2221|    match v {
2222|        Value::Null => out.push_str("null"),
2223|        Value::Bool(true) => out.push_str("true"),
2224|        Value::Bool(false) => out.push_str("false"),
2225|        Value::Number(n) => write_number(n, out)?,
2226|        Value::String(s) => write_string(s, out),
2227|        Value::Array(a) => {
2228|            out.push('[');
2229|            for (i, item) in a.iter().enumerate() {
2230|                if i > 0 { out.push(','); }
2231|                write_value(item, out)?;
2232|            }
2233|            out.push(']');
2234|        }
2235|        Value::Object(o) => {
2236|            out.push('{');
2237|            let mut keys: Vec<&String> = o.keys().collect();
2238|            keys.sort_by(|a, b| cmp_utf16(a, b));
2239|            for (i, k) in keys.iter().enumerate() {
2240|                if i > 0 { out.push(','); }
2241|                write_string(k, out);
2242|                out.push(':');
2243|                write_value(&o[*k], out)?;
2244|            }
2245|            out.push('}');
2246|        }
2247|    }
2248|    Ok(())
2249|}
2250|
2251|fn cmp_utf16(a: &str, b: &str) -> std::cmp::Ordering {
2252|    let mut ai = a.encode_utf16();
2253|    let mut bi = b.encode_utf16();
2254|    loop {
2255|        match (ai.next(), bi.next()) {
2256|            (None, None) => return std::cmp::Ordering::Equal,
2257|            (None, Some(_)) => return std::cmp::Ordering::Less,
2258|            (Some(_), None) => return std::cmp::Ordering::Greater,
2259|            (Some(x), Some(y)) if x == y => continue,
2260|            (Some(x), Some(y)) => return x.cmp(&y),
2261|        }
2262|    }
2263|}
2264|
2265|fn write_string(s: &str, out: &mut String) {
2266|    out.push('"');
2267|    for c in s.chars() {
2268|        match c {
2269|            '"' => out.push_str("\\\""),
2270|            '\\' => out.push_str("\\\\"),
2271|            '\u{08}' => out.push_str("\\b"),
2272|            '\u{0C}' => out.push_str("\\f"),
2273|            '\n' => out.push_str("\\n"),
2274|            '\r' => out.push_str("\\r"),
2275|            '\t' => out.push_str("\\t"),
2276|            c if (c as u32) < 0x20 => {
2277|                use std::fmt::Write;
2278|                let _ = write!(out, "\\u{:04x}", c as u32);
2279|            }
2280|            c => out.push(c),
2281|        }
2282|    }
2283|    out.push('"');
2284|}
2285|
2286|fn write_number(n: &serde_json::Number, out: &mut String) -> Result<(), CanonicalError> {
2287|    if let Some(i) = n.as_i64() { out.push_str(&i.to_string()); return Ok(()); }
2288|    if let Some(u) = n.as_u64() { out.push_str(&u.to_string()); return Ok(()); }
2289|    if let Some(f) = n.as_f64() { return write_f64(f, out); }
2290|    Err(CanonicalError::UnrepresentableNumber(n.to_string()))
2291|}
2292|
2293|fn write_f64(f: f64, out: &mut String) -> Result<(), CanonicalError> {
2294|    if !f.is_finite() { return Err(CanonicalError::NonFinite); }
2295|    if f == 0.0 { out.push('0'); return Ok(()); }
2296|    if f.fract() == 0.0 && f.abs() < 9.0e15 {
2297|        use std::fmt::Write;
2298|        let _ = write!(out, "{}", f as i64);
2299|        return Ok(());
2300|    }
2301|    use std::fmt::Write;
2302|    let _ = write!(out, "{}", f);
2303|    Ok(())
2304|}
2305|
2306|#[cfg(test)]
2307|mod tests {
2308|    use super::*;
2309|    use serde_json::json;
2310|
2311|    #[test]
2312|    fn sorts_object_keys() {
2313|        assert_eq!(canonicalize(&json!({"b": 1, "a": 2, "c": 3})).unwrap(), r#"{"a":2,"b":1,"c":3}"#);
2314|    }
2315|
2316|    #[test]
2317|    fn removes_whitespace() {
2318|        let v: Value = serde_json::from_str(r#"{ "a" : 1 , "b" : [ 1 , 2 ] }"#).unwrap();
2319|        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":1,"b":[1,2]}"#);
2320|    }
2321|
2322|    #[test]
2323|    fn nested_structures_sort_recursively() {
2324|        let v = json!({"z": {"y": 1, "x": 2}, "a": [{"b": 3, "a": 4}]});
2325|        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":[{"a":4,"b":3}],"z":{"x":2,"y":1}}"#);
2326|    }
2327|
2328|    #[test]
2329|    fn determinism() {
2330|        let v1: Value = serde_json::from_str(r#"{"a":1,"b":[2,3],"c":{"d":4}}"#).unwrap();
2331|        let v2: Value = serde_json::from_str(r#"{"c":{"d":4},"b":[2,3],"a":1}"#).unwrap();
2332|        assert_eq!(canonicalize(&v1).unwrap(), canonicalize(&v2).unwrap());
2333|    }
2334|}
2335|```
2336|
2337|### `crates/dnea-manifest/src/capability.rs`
2338|
2339|```rust
2340|//! The capability vocabulary.
2341|
2342|use std::collections::HashMap;
2343|
2344|use once_cell::sync::Lazy;
2345|use regex::Regex;
2346|
2347|static CAP_RE: Lazy<Regex> = Lazy::new(|| {
2348|    Regex::new(r"^[a-z][a-z0-9]*(:[a-z0-9*_-]+)+$").expect("capability regex is a constant")
2349|});
2350|
2351|#[must_use]
2352|pub fn is_valid_capability_id(s: &str) -> bool {
2353|    !s.is_empty() && s.len() <= 128 && CAP_RE.is_match(s)
2354|}
2355|
2356|#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
2357|pub enum ConsentTier { Auto, Prompt, PerUse, Admin, Forbidden }
2358|
2359|impl ConsentTier {
2360|    #[must_use]
2361|    pub fn parse(s: &str) -> Option<Self> {
2362|        Some(match s {
2363|            "auto" => Self::Auto,
2364|            "prompt" => Self::Prompt,
2365|            "per-use" => Self::PerUse,
2366|            "admin" => Self::Admin,
2367|            "forbidden" => Self::Forbidden,
2368|            _ => return None,
2369|        })
2370|    }
2371|}
2372|
2373|pub struct CapabilityRegistry {
2374|    tiers: HashMap<&'static str, ConsentTier>,
2375|}
2376|
2377|impl CapabilityRegistry {
2378|    #[must_use]
2379|    pub fn v1() -> Self {
2380|        use ConsentTier::*;
2381|        let entries: &[(&str, ConsentTier)] = &[
2382|            ("fs:read", Prompt), ("fs:write", Prompt), ("fs:append", Prompt),
2383|            ("fs:list", Auto), ("fs:watch", Prompt), ("fs:exec", Admin), ("fs:mount", Admin),
2384|            ("net:connect", Prompt), ("net:listen", Admin), ("net:proxy", Admin),
2385|            ("net:dns:resolve", Auto), ("net:websocket", Prompt), ("net:udp", Admin),
2386|            ("browser:identity", Auto),
2387|            ("browser:tabs:query", Prompt), ("browser:tabs:create", Prompt),
2388|            ("browser:tabs:update", Prompt), ("browser:tabs:close", Prompt),
2389|            ("browser:windows:query", Prompt),
2390|            ("browser:history:read", Prompt), ("browser:history:delete", PerUse),
2391|            ("browser:bookmarks:read", Auto), ("browser:bookmarks:write", Prompt),
2392|            ("browser:cookies:read", Prompt), ("browser:cookies:write", PerUse),
2393|            ("browser:downloads:read", Auto), ("browser:downloads:manage", Prompt),
2394|            ("browser:content:read", Prompt), ("browser:content:inject", Prompt),
2395|            ("browser:content:mutate", PerUse),
2396|            ("browser:forms:read", Prompt), ("browser:forms:values", PerUse),
2397|            ("browser:forms:fill", PerUse),
2398|            ("browser:navigation:intercept", Admin),
2399|            ("browser:clipboard:read", PerUse), ("browser:clipboard:write", Prompt),
2400|            ("browser:a11y:tree:read", Admin), ("browser:a11y:input:inject", Admin),
2401|            ("ui:companion:panel", Auto), ("ui:companion:notify", Prompt),
2402|            ("ui:webext:popup", Auto), ("ui:webext:content", Prompt),
2403|            ("ui:overlay:fullscreen", PerUse),
2404|            ("storage:module", Auto), ("storage:shared", Prompt), ("storage:secure", Admin),
2405|            ("runtime:log", Auto), ("runtime:modules:list", Auto),
2406|            ("runtime:modules:control", Admin),
2407|            ("runtime:config:read", Auto), ("runtime:config:write", Admin),
2408|            ("runtime:identity:read", Prompt), ("runtime:spawn", Admin), ("runtime:schedule", Auto),
2409|            ("notifications:send", Prompt),
2410|            ("identity:crypto:sign", Admin), ("identity:crypto:verify", Auto),
2411|            ("system:autostart", Admin), ("system:tray", Auto), ("system:power", Admin),
2412|            ("media:capture", Admin),
2413|            ("policy:read", Auto), ("audit:read", Admin), ("telemetry:emit", Prompt),
2414|        ];
2415|        Self { tiers: entries.iter().copied().collect() }
2416|    }
2417|
2418|    #[must_use]
2419|    pub fn is_known(&self, id: &str) -> bool { self.tiers.contains_key(id) }
2420|
2421|    #[must_use]
2422|    pub fn default_tier(&self, id: &str) -> ConsentTier {
2423|        self.tiers.get(id).copied().unwrap_or(ConsentTier::Prompt)
2424|    }
2425|}
2426|
2427|#[cfg(test)]
2428|mod tests {
2429|    use super::*;
2430|
2431|    #[test]
2432|    fn capability_grammar() {
2433|        assert!(is_valid_capability_id("fs:read"));
2434|        assert!(is_valid_capability_id("browser:tabs:query"));
2435|        assert!(!is_valid_capability_id(""));
2436|        assert!(!is_valid_capability_id("nocolon"));
2437|        assert!(!is_valid_capability_id("Upper:Case"));
2438|    }
2439|}
2440|```
2441|
2442|### `crates/dnea-manifest/src/manifest.rs`
2443|
2444|```rust
2445|//! The typed manifest.
2446|
2447|use std::collections::BTreeMap;
2448|
2449|use serde::{Deserialize, Serialize};
2450|use serde_json::Value;
2451|use thiserror::Error;
2452|
2453|use crate::canonical::CanonicalError;
2454|
2455|#[derive(Debug, Error)]
2456|pub enum ManifestError {
2457|    #[error("invalid JSON: {0}")]
2458|    Json(#[from] serde_json::Error),
2459|    #[error("missing required field: {0}")]
2460|    MissingField(String),
2461|    #[error("invalid module ID: {0}")]
2462|    InvalidId(String),
2463|    #[error("manifest is not signed")]
2464|    MissingSignature,
2465|    #[error("payload digest mismatch: expected {expected}, actual {actual}")]
2466|    DigestMismatch { expected: String, actual: String },
2467|    #[error("unknown signing key: {0}")]
2468|    UnknownKey(String),
2469|    #[error("signing key revoked: {0}")]
2470|    RevokedKey(String),
2471|    #[error("signature verification failed for key {key_id}: {reason}")]
2472|    BadSignature { key_id: String, reason: String },
2473|    #[error("manifest has no author or publisher signature")]
2474|    MissingTrustedSigner,
2475|    #[error("unsupported signature algorithm: {0}")]
2476|    UnsupportedAlgorithm(String),
2477|    #[error("canonicalization: {0}")]
2478|    Canonical(#[from] CanonicalError),
2479|}
2480|
2481|#[derive(Clone, Debug, Serialize, Deserialize)]
2482|pub struct Manifest {
2483|    #[serde(rename = "manifestVersion")]
2484|    pub manifest_version: u32,
2485|    pub id: String,
2486|    pub version: String,
2487|    pub runtime: String,
2488|    pub name: String,
2489|    #[serde(default)]
2490|    pub description: Option<String>,
2491|    #[serde(default)]
2492|    pub homepage: Option<String>,
2493|    #[serde(default)]
2494|    pub license: Option<String>,
2495|    #[serde(default)]
2496|    pub authors: Vec<Author>,
2497|    #[serde(default)]
2498|    pub dev: bool,
2499|    #[serde(default)]
2500|    pub signature: Option<Signature>,
2501|    pub entrypoints: BTreeMap<String, Entrypoint>,
2502|    pub capabilities: Capabilities,
2503|    #[serde(default)]
2504|    pub dependencies: BTreeMap<String, Dependency>,
2505|    #[serde(default)]
2506|    pub conflicts: Vec<Conflict>,
2507|    #[serde(default)]
2508|    pub lifecycle: Lifecycle,
2509|    #[serde(default)]
2510|    pub migrations: Vec<Migration>,
2511|    #[serde(default)]
2512|    pub storage: Storage,
2513|    #[serde(default)]
2514|    pub ui: Vec<UiSurface>,
2515|    #[serde(default)]
2516|    pub settings: Option<Value>,
2517|    #[serde(default)]
2518|    pub sandbox: Option<Value>,
2519|    #[serde(default)]
2520|    pub resources: Option<Value>,
2521|    #[serde(default)]
2522|    pub audit: Option<Value>,
2523|    #[serde(default)]
2524|    pub update: Option<Value>,
2525|    #[serde(default)]
2526|    pub i18n: Option<Value>,
2527|    #[serde(default, rename = "dynamicOriginGrants")]
2528|    pub dynamic_origin_grants: Option<DynamicOriginGrants>,
2529|    #[serde(default, rename = "signaturePolicy")]
2530|    pub signature_policy: Option<Value>,
2531|    #[serde(flatten)]
2532|    pub extra: BTreeMap<String, Value>,
2533|}
2534|
2535|#[derive(Clone, Debug, Serialize, Deserialize)]
2536|pub struct Author {
2537|    pub name: String,
2538|    #[serde(default)]
2539|    pub email: Option<String>,
2540|    #[serde(default)]
2541|    pub url: Option<String>,
2542|}
2543|
2544|#[derive(Clone, Debug, Serialize, Deserialize)]
2545|pub struct Signature {
2546|    #[serde(rename = "payloadDigest")]
2547|    pub payload_digest: PayloadDigest,
2548|    pub signers: Vec<Signer>,
2549|    #[serde(default)]
2550|    pub transparency: Option<Value>,
2551|}
2552|
2553|#[derive(Clone, Debug, Serialize, Deserialize)]
2554|pub struct PayloadDigest {
2555|    pub alg: String,
2556|    pub value: String,
2557|}
2558|
2559|#[derive(Clone, Debug, Serialize, Deserialize)]
2560|pub struct Signer {
2561|    #[serde(rename = "keyId")]
2562|    pub key_id: String,
2563|    pub alg: String,
2564|    pub value: String,
2565|    #[serde(default = "default_role")]
2566|    pub role: String,
2567|    #[serde(default, rename = "signedAt")]
2568|    pub signed_at: Option<String>,
2569|    #[serde(default, rename = "expiresAt")]
2570|    pub expires_at: Option<String>,
2571|}
2572|
2573|fn default_role() -> String { "author".into() }
2574|
2575|#[derive(Clone, Debug, Serialize, Deserialize)]
2576|pub struct Entrypoint {
2577|    pub kind: String,
2578|    pub path: String,
2579|    pub digest: PayloadDigest,
2580|    #[serde(default = "default_abi")]
2581|    pub abi: String,
2582|    #[serde(default)]
2583|    pub exports: Vec<String>,
2584|    #[serde(default, rename = "sandboxProfile")]
2585|    pub sandbox_profile: Option<String>,
2586|}
2587|
2588|fn default_abi() -> String { "1.0.0".into() }
2589|
2590|#[derive(Clone, Debug, Serialize, Deserialize)]
2591|pub struct Capabilities {
2592|    pub request: Vec<CapabilityRequest>,
2593|    #[serde(default)]
2594|    pub deny: Vec<String>,
2595|}
2596|
2597|#[derive(Clone, Debug, Serialize, Deserialize)]
2598|pub struct CapabilityRequest {
2599|    pub id: String,
2600|    #[serde(default)]
2601|    pub reason: Option<String>,
2602|    #[serde(default)]
2603|    pub consent: Option<String>,
2604|    #[serde(default)]
2605|    pub constraints: Value,
2606|    #[serde(default = "default_true")]
2607|    pub required: bool,
2608|    #[serde(default)]
2609|    pub audit: Option<String>,
2610|    #[serde(default, rename = "policyKey")]
2611|    pub policy_key: Option<String>,
2612|}
2613|
2614|fn default_true() -> bool { true }
2615|
2616|#[derive(Clone, Debug, Serialize, Deserialize)]
2617|pub struct Dependency {
2618|    pub version: String,
2619|    #[serde(default)]
2620|    pub optional: bool,
2621|    #[serde(default)]
2622|    pub capabilities: Vec<String>,
2623|}
2624|
2625|#[derive(Clone, Debug, Serialize, Deserialize)]
2626|pub struct Conflict {
2627|    pub id: String,
2628|    #[serde(default)]
2629|    pub version: Option<String>,
2630|    #[serde(default)]
2631|    pub reason: Option<String>,
2632|}
2633|
2634|#[derive(Clone, Debug, Default, Serialize, Deserialize)]
2635|pub struct Lifecycle {
2636|    #[serde(default)] pub install: Option<HookRef>,
2637|    #[serde(default)] pub activate: Option<HookRef>,
2638|    #[serde(default)] pub deactivate: Option<HookRef>,
2639|    #[serde(default)] pub upgrade: Option<HookRef>,
2640|    #[serde(default)] pub downgrade: Option<HookRef>,
2641|    #[serde(default)] pub uninstall: Option<HookRef>,
2642|    #[serde(default, rename = "healthCheck")] pub health_check: Option<HookRef>,
2643|    #[serde(default, rename = "onCapabilityChange")] pub on_capability_change: Option<HookRef>,
2644|}
2645|
2646|#[derive(Clone, Debug, Serialize, Deserialize)]
2647|pub struct HookRef {
2648|    pub entrypoint: String,
2649|    pub export: String,
2650|    #[serde(default = "default_timeout", rename = "timeoutMs")]
2651|    pub timeout_ms: u64,
2652|    #[serde(default)]
2653|    pub fatal: bool,
2654|}
2655|
2656|fn default_timeout() -> u64 { 10_000 }
2657|
2658|#[derive(Clone, Debug, Serialize, Deserialize)]
2659|pub struct Migration {
2660|    pub from: u32,
2661|    pub to: u32,
2662|    pub entrypoint: String,
2663|    pub export: String,
2664|    #[serde(default)]
2665|    pub reversible: bool,
2666|    #[serde(default = "default_migration_timeout", rename = "timeoutMs")]
2667|    pub timeout_ms: u64,
2668|}
2669|
2670|fn default_migration_timeout() -> u64 { 60_000 }
2671|
2672|#[derive(Clone, Debug, Serialize, Deserialize)]
2673|pub struct Storage {
2674|    #[serde(default = "default_storage_schema", rename = "storageSchemaVersion")]
2675|    pub storage_schema_version: u32,
2676|    #[serde(default = "default_true")]
2677|    pub encrypted: bool,
2678|    #[serde(default = "default_quota", rename = "quotaBytes")]
2679|    pub quota_bytes: u64,
2680|    #[serde(default, rename = "sharedNamespaces")]
2681|    pub shared_namespaces: Vec<String>,
2682|    #[serde(default)]
2683|    pub exportable: bool,
2684|}
2685|
2686|fn default_storage_schema() -> u32 { 1 }
2687|fn default_quota() -> u64 { 10 * 1024 * 1024 }
2688|
2689|impl Default for Storage {
2690|    fn default() -> Self {
2691|        Self {
2692|            storage_schema_version: 1,
2693|            encrypted: true,
2694|            quota_bytes: default_quota(),
2695|            shared_namespaces: Vec::new(),
2696|            exportable: false,
2697|        }
2698|    }
2699|}
2700|
2701|#[derive(Clone, Debug, Serialize, Deserialize)]
2702|pub struct UiSurface {
2703|    pub id: String,
2704|    pub kind: String,
2705|    pub entrypoint: String,
2706|    #[serde(default)]
2707|    pub surfaces: Vec<String>,
2708|    #[serde(default)]
2709|    pub title: Option<String>,
2710|    #[serde(default)]
2711|    pub icon: Option<String>,
2712|    #[serde(default)]
2713|    pub origins: Vec<String>,
2714|    #[serde(default)]
2715|    pub keybinding: Option<Value>,
2716|    #[serde(default)]
2717|    pub order: Option<i64>,
2718|}
2719|
2720|#[derive(Clone, Debug, Serialize, Deserialize)]
2721|pub struct DynamicOriginGrants {
2722|    pub enabled: bool,
2723|    #[serde(default = "default_max_origins", rename = "maxOrigins")]
2724|    pub max_origins: u32,
2725|    #[serde(default = "default_dynamic_consent")]
2726|    pub consent: String,
2727|    #[serde(default)]
2728|    pub reason: Option<String>,
2729|}
2730|
2731|fn default_max_origins() -> u32 { 256 }
2732|fn default_dynamic_consent() -> String { "per-use".into() }
2733|
2734|impl Manifest {
2735|    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ManifestError> {
2736|        Ok(serde_json::from_slice(bytes)?)
2737|    }
2738|
2739|    #[must_use]
2740|    pub fn entrypoint(&self, name: &str) -> Option<&Entrypoint> {
2741|        self.entrypoints.get(name)
2742|    }
2743|}
2744|```
2745|
2746|### `crates/dnea-manifest/src/trust.rs`
2747|
2748|```rust
2749|//! The trust store.
2750|
2751|use std::collections::HashMap;
2752|
2753|#[derive(Clone, Copy, Debug, PartialEq, Eq)]
2754|pub enum SigAlg { EdDSA, Es256, Es384, Ps256 }
2755|
2756|impl SigAlg {
2757|    #[must_use]
2758|    pub fn parse(s: &str) -> Option<Self> {
2759|        Some(match s {
2760|            "EdDSA" => Self::EdDSA,
2761|            "ES256" => Self::Es256,
2762|            "ES384" => Self::Es384,
2763|            "PS256" => Self::Ps256,
2764|            _ => return None,
2765|        })
2766|    }
2767|}
2768|
2769|#[derive(Clone, Debug)]
2770|pub struct ResolvedKey {
2771|    pub alg: SigAlg,
2772|    pub public_key: Vec<u8>,
2773|    pub revoked: bool,
2774|}
2775|
2776|pub trait TrustStore: Send + Sync + 'static {
2777|    fn resolve(&self, key_id: &str) -> Option<ResolvedKey>;
2778|}
2779|
2780|pub struct StaticTrustStore {
2781|    keys: HashMap<String, ResolvedKey>,
2782|}
2783|
2784|impl StaticTrustStore {
2785|    #[must_use]
2786|    pub fn new() -> Self { Self { keys: HashMap::new() } }
2787|
2788|    pub fn insert(&mut self, key_id: impl Into<String>, key: ResolvedKey) {
2789|        self.keys.insert(key_id.into(), key);
2790|    }
2791|}
2792|
2793|impl Default for StaticTrustStore {
2794|    fn default() -> Self { Self::new() }
2795|}
2796|
2797|impl TrustStore for StaticTrustStore {
2798|    fn resolve(&self, key_id: &str) -> Option<ResolvedKey> {
2799|        self.keys.get(key_id).cloned()
2800|    }
2801|}
2802|```
2803|
2804|### `crates/dnea-manifest/src/signature.rs`
2805|
2806|```rust
2807|//! Signing and verification.
2808|
2809|use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
2810|use ed25519_dalek::{Signature as Ed25519Signature, Signer as _, SigningKey, Verifier, VerifyingKey};
2811|use serde_json::Value;
2812|use sha2::{Digest, Sha256};
2813|
2814|use crate::{
2815|    canonical::canonicalize,
2816|    manifest::{ManifestError, Signature},
2817|    trust::{SigAlg, TrustStore},
2818|};
2819|
2820|#[derive(Clone, Debug)]
2821|pub struct VerifiedSigner {
2822|    pub key_id: String,
2823|    pub role: String,
2824|    pub alg: SigAlg,
2825|}
2826|
2827|#[derive(Clone, Debug)]
2828|pub struct SignatureInfo {
2829|    pub signers: Vec<VerifiedSigner>,
2830|}
2831|
2832|pub fn sign_manifest(
2833|    manifest: &mut Value,
2834|    signing_key: &SigningKey,
2835|    key_id: &str,
2836|    role: &str,
2837|) -> Result<(), ManifestError> {
2838|    let obj = manifest
2839|        .as_object_mut()
2840|        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?;
2841|    obj.remove("signature");
2842|    let canonical = canonicalize(manifest)?;
2843|    let digest_bytes = Sha256::digest(canonical.as_bytes());
2844|    let payload_digest = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
2845|    let signature = signing_key.sign(&digest_bytes);
2846|    let sig_value = serde_json::json!({
2847|        "payloadDigest": { "alg": "sha-256", "value": payload_digest },
2848|        "signers": [{
2849|            "keyId": key_id,
2850|            "alg": "EdDSA",
2851|            "role": role,
2852|            "value": URL_SAFE_NO_PAD.encode(signature.to_bytes()),
2853|        }],
2854|    });
2855|    manifest
2856|        .as_object_mut()
2857|        .unwrap()
2858|        .insert("signature".to_string(), sig_value);
2859|    Ok(())
2860|}
2861|
2862|pub fn verify_manifest(
2863|    manifest: &Value,
2864|    trust: &dyn TrustStore,
2865|) -> Result<SignatureInfo, ManifestError> {
2866|    let sig_value = manifest
2867|        .get("signature")
2868|        .ok_or(ManifestError::MissingSignature)?;
2869|    let sig: Signature = serde_json::from_value(sig_value.clone())?;
2870|
2871|    let mut without_sig = manifest.clone();
2872|    without_sig
2873|        .as_object_mut()
2874|        .ok_or_else(|| ManifestError::MissingField("manifest must be an object".into()))?
2875|        .remove("signature");
2876|
2877|    let canonical = canonicalize(&without_sig)?;
2878|    let digest_bytes = Sha256::digest(canonical.as_bytes());
2879|    let actual = format!("sha-256:{}", URL_SAFE_NO_PAD.encode(digest_bytes));
2880|    if actual != sig.payload_digest.value {
2881|        return Err(ManifestError::DigestMismatch {
2882|            expected: sig.payload_digest.value.clone(),
2883|            actual,
2884|        });
2885|    }
2886|
2887|    let mut verified = Vec::with_capacity(sig.signers.len());
2888|    for signer in &sig.signers {
2889|        let alg = SigAlg::parse(&signer.alg)
2890|            .ok_or_else(|| ManifestError::UnsupportedAlgorithm(signer.alg.clone()))?;
2891|        let key = trust
2892|            .resolve(&signer.key_id)
2893|            .ok_or_else(|| ManifestError::UnknownKey(signer.key_id.clone()))?;
2894|        if key.revoked {
2895|            return Err(ManifestError::RevokedKey(signer.key_id.clone()));
2896|        }
2897|        if key.alg != alg {
2898|            return Err(ManifestError::BadSignature {
2899|                key_id: signer.key_id.clone(),
2900|                reason: format!("key algorithm mismatch: manifest={alg:?} trust={:?}", key.alg),
2901|            });
2902|        }
2903|        let sig_bytes = URL_SAFE_NO_PAD
2904|            .decode(&signer.value)
2905|            .map_err(|e| ManifestError::BadSignature {
2906|                key_id: signer.key_id.clone(),
2907|                reason: format!("bad base64: {e}"),
2908|            })?;
2909|        verify_signature(alg, &key.public_key, &digest_bytes, &sig_bytes).map_err(|reason| {
2910|            ManifestError::BadSignature {
2911|                key_id: signer.key_id.clone(),
2912|                reason,
2913|            }
2914|        })?;
2915|        verified.push(VerifiedSigner {
2916|            key_id: signer.key_id.clone(),
2917|            role: signer.role.clone(),
2918|            alg,
2919|        });
2920|    }
2921|
2922|    let has_trusted = verified
2923|        .iter()
2924|        .any(|s| matches!(s.role.as_str(), "author" | "publisher"));
2925|    if !has_trusted {
2926|        return Err(ManifestError::MissingTrustedSigner);
2927|    }
2928|
2929|    Ok(SignatureInfo { signers: verified })
2930|}
2931|
2932|fn verify_signature(
2933|    alg: SigAlg,
2934|    public_key: &[u8],
2935|    digest: &[u8],
2936|    signature: &[u8],
2937|) -> Result<(), String> {
2938|    match alg {
2939|        SigAlg::EdDSA => {
2940|            let vk = VerifyingKey::from_bytes(
2941|                public_key.try_into().map_err(|_| "Ed25519 public key must be 32 bytes".to_string())?,
2942|            )
2943|            .map_err(|e| format!("invalid Ed25519 key: {e}"))?;
2944|            let sig = Ed25519Signature::from_slice(signature)
2945|                .map_err(|e| format!("invalid Ed25519 signature: {e}"))?;
2946|            vk.verify(digest, &sig).map_err(|e| format!("Ed25519 verify: {e}"))
2947|        }
2948|        SigAlg::Es256 | SigAlg::Es384 | SigAlg::Ps256 => {
2949|            Err("only Ed25519 is implemented in this build".to_string())
2950|        }
2951|    }
2952|}
2953|
2954|#[cfg(test)]
2955|mod tests {
2956|    use super::*;
2957|    use crate::trust::StaticTrustStore;
2958|    use ed25519_dalek::SigningKey;
2959|    use rand::rngs::OsRng;
2960|    use serde_json::json;
2961|
2962|    fn key_pair() -> (SigningKey, StaticTrustStore) {
2963|        let signing = SigningKey::generate(&mut OsRng);
2964|        let mut trust = StaticTrustStore::new();
2965|        trust.insert("test-key", crate::trust::ResolvedKey {
2966|            alg: SigAlg::EdDSA,
2967|            public_key: signing.verifying_key().to_bytes().to_vec(),
2968|            revoked: false,
2969|        });
2970|        (signing, trust)
2971|    }
2972|
2973|    fn base_manifest() -> Value {
2974|        json!({
2975|            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
2976|            "runtime": "*", "name": "X",
2977|            "entrypoints": { "core": { "kind": "wasm", "path": "x.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
2978|            "capabilities": { "request": [] }
2979|        })
2980|    }
2981|
2982|    #[test]
2983|    fn sign_then_verify() {
2984|        let (signing, trust) = key_pair();
2985|        let mut m = base_manifest();
2986|        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
2987|        let info = verify_manifest(&m, &trust).unwrap();
2988|        assert_eq!(info.signers[0].key_id, "test-key");
2989|    }
2990|
2991|    #[test]
2992|    fn tampering_invalidates() {
2993|        let (signing, trust) = key_pair();
2994|        let mut m = base_manifest();
2995|        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
2996|        m["name"] = json!("Tampered");
2997|        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::DigestMismatch { .. })));
2998|    }
2999|
3000|    #[test]
3001|    fn unknown_key_rejected() {
3002|        let (signing, _) = key_pair();
3003|        let mut m = base_manifest();
3004|        sign_manifest(&mut m, &signing, "test-key", "author").unwrap();
3005|        let trust = StaticTrustStore::new();
3006|        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::UnknownKey(_))));
3007|    }
3008|
3009|    #[test]
3010|    fn only_reviewer_role_rejected() {
3011|        let (signing, trust) = key_pair();
3012|        let mut m = base_manifest();
3013|        sign_manifest(&mut m, &signing, "test-key", "reviewer").unwrap();
3014|        assert!(matches!(verify_manifest(&m, &trust), Err(ManifestError::MissingTrustedSigner)));
3015|    }
3016|}
3017|```
3018|
3019|### `crates/dnea-manifest/src/validate.rs`
3020|
3021|```rust
3022|//! Semantic validation.
3023|
3024|use std::collections::HashSet;
3025|
3026|use jsonschema::{Draft, JSONSchema};
3027|use serde_json::Value;
3028|use thiserror::Error;
3029|
3030|use crate::{
3031|    capability::{is_valid_capability_id, CapabilityRegistry},
3032|    manifest::{Manifest, ManifestError},
3033|};
3034|
3035|const SCHEMA_JSON: &str = include_str!("../../schemas/module-manifest-v1.json");
3036|
3037|#[derive(Debug, Error)]
3038|pub enum ValidationError {
3039|    #[error("schema validation failed: {0:?}")]
3040|    Schema(Vec<String>),
3041|    #[error("manifest parse: {0}")]
3042|    Manifest(#[from] ManifestError),
3043|    #[error("invalid capability ID: {0}")]
3044|    InvalidCapabilityId(String),
3045|    #[error("unknown capability: {0}")]
3046|    UnknownCapability(String),
3047|    #[error("migration chain broken: missing migration from version {0}")]
3048|    BrokenMigrationChain(u32),
3049|    #[error("UI surface {surface} declares origin {origin} not granted")]
3050|    UiOriginNotGranted { surface: String, origin: String },
3051|    #[error("module declares a conflict with itself")]
3052|    SelfConflict,
3053|    #[error("invalid entrypoint kind: {0}")]
3054|    InvalidEntrypointKind(String),
3055|    #[error("entrypoint path escapes module root: {0}")]
3056|    PathTraversal(String),
3057|    #[error("invalid runtime range: {0}")]
3058|    InvalidRuntimeRange(String),
3059|    #[error("invalid module ID: {0}")]
3060|    InvalidModuleId(String),
3061|    #[error("invalid version: {0}")]
3062|    InvalidVersion(String),
3063|    #[error("dynamicOriginGrants declared but no origin-scoped capability is requested")]
3064|    DynamicOriginsWithoutOriginCapability,
3065|}
3066|
3067|#[derive(Debug, Default)]
3068|pub struct ValidationReport {
3069|    pub warnings: Vec<String>,
3070|}
3071|
3072|pub struct Validator {
3073|    schema: JSONSchema,
3074|    registry: CapabilityRegistry,
3075|}
3076|
3077|impl Validator {
3078|    pub fn new() -> Result<Self, ValidationError> {
3079|        let schema_value: Value = serde_json::from_str(SCHEMA_JSON)
3080|            .map_err(|e| ValidationError::Manifest(ManifestError::Json(e)))?;
3081|        let schema = JSONSchema::options()
3082|            .with_draft(Draft::Draft202012)
3083|            .compile(&schema_value)
3084|            .map_err(|e| ValidationError::Schema(vec![format!("schema compile: {e}")]))?;
3085|        Ok(Self { schema, registry: CapabilityRegistry::v1() })
3086|    }
3087|
3088|    pub fn validate(&self, value: &Value) -> Result<ValidationReport, ValidationError> {
3089|        if let Err(errors) = self.schema.validate(value) {
3090|            let msgs: Vec<String> = errors
3091|                .map(|e| format!("{} at {}", e, e.instance_path))
3092|                .collect();
3093|            return Err(ValidationError::Schema(msgs));
3094|        }
3095|        let manifest: Manifest = serde_json::from_value(value.clone())
3096|            .map_err(ValidationError::Manifest)?;
3097|
3098|        if dnea_core::ModuleId::parse(&manifest.id).is_err() {
3099|            return Err(ValidationError::InvalidModuleId(manifest.id.clone()));
3100|        }
3101|        if semver_check(&manifest.version).is_err() {
3102|            return Err(ValidationError::InvalidVersion(manifest.version.clone()));
3103|        }
3104|        if semver_range_check(&manifest.runtime).is_err() {
3105|            return Err(ValidationError::InvalidRuntimeRange(manifest.runtime.clone()));
3106|        }
3107|
3108|        for (_, ep) in &manifest.entrypoints {
3109|            if !matches!(ep.kind.as_str(), "wasm" | "js" | "native") {
3110|                return Err(ValidationError::InvalidEntrypointKind(ep.kind.clone()));
3111|            }
3112|            if ep.path.starts_with('/') || ep.path.contains("..") {
3113|                return Err(ValidationError::PathTraversal(ep.path.clone()));
3114|            }
3115|        }
3116|
3117|        let mut has_origin_capability = false;
3118|        for req in &manifest.capabilities.request {
3119|            if !is_valid_capability_id(&req.id) {
3120|                return Err(ValidationError::InvalidCapabilityId(req.id.clone()));
3121|            }
3122|            if !self.registry.is_known(&req.id) {
3123|                return Err(ValidationError::UnknownCapability(req.id.clone()));
3124|            }
3125|            let c = &req.constraints;
3126|            if c.get("origins").is_some() || c.get("hosts").is_some() {
3127|                has_origin_capability = true;
3128|            }
3129|        }
3130|
3131|        if manifest.dynamic_origin_grants.is_some() && !has_origin_capability {
3132|            return Err(ValidationError::DynamicOriginsWithoutOriginCapability);
3133|        }
3134|
3135|        self.check_migration_chain(&manifest)?;
3136|
3137|        let granted_origins: HashSet<&str> = manifest
3138|            .capabilities
3139|            .request
3140|            .iter()
3141|            .filter(|r| r.id == "browser:content:inject")
3142|            .filter_map(|r| r.constraints.get("origins"))
3143|            .filter_map(|v| v.as_array())
3144|            .flatten()
3145|            .filter_map(|v| v.as_str())
3146|            .collect();
3147|        for surface in &manifest.ui {
3148|            if surface.kind == "content-overlay" {
3149|                for origin in &surface.origins {
3150|                    if !granted_origins.contains(origin.as_str()) {
3151|                        return Err(ValidationError::UiOriginNotGranted {
3152|                            surface: surface.id.clone(),
3153|                            origin: origin.clone(),
3154|                        });
3155|                    }
3156|                }
3157|            }
3158|        }
3159|
3160|        for c in &manifest.conflicts {
3161|            if c.id == manifest.id {
3162|                return Err(ValidationError::SelfConflict);
3163|            }
3164|        }
3165|
3166|        Ok(ValidationReport::default())
3167|    }
3168|
3169|    fn check_migration_chain(&self, manifest: &Manifest) -> Result<(), ValidationError> {
3170|        let target = manifest.storage.storage_schema_version;
3171|        if target <= 1 { return Ok(()); }
3172|        let has: HashSet<u32> = manifest.migrations.iter().map(|m| m.from).collect();
3173|        for v in 1..target {
3174|            if !has.contains(&v) {
3175|                return Err(ValidationError::BrokenMigrationChain(v));
3176|            }
3177|        }
3178|        Ok(())
3179|    }
3180|}
3181|
3182|fn semver_check(s: &str) -> Result<(), ()> {
3183|    let bytes = s.as_bytes();
3184|    if bytes.is_empty() { return Err(()); }
3185|    let mut dot_count = 0;
3186|    let mut segment_digits = 0;
3187|    for (i, &b) in bytes.iter().enumerate() {
3188|        match b {
3189|            b'0'..=b'9' => segment_digits += 1,
3190|            b'.' => {
3191|                if segment_digits == 0 { return Err(()); }
3192|                dot_count += 1;
3193|                segment_digits = 0;
3194|            }
3195|            b'-' | b'+' => {
3196|                if dot_count != 2 || segment_digits == 0 { return Err(()); }
3197|                return if i == bytes.len() - 1 { Err(()) } else { Ok(()) };
3198|            }
3199|            _ => return Err(()),
3200|        }
3201|    }
3202|    if dot_count == 2 && segment_digits > 0 { Ok(()) } else { Err(()) }
3203|}
3204|
3205|fn semver_range_check(s: &str) -> Result<(), ()> {
3206|    if s.is_empty() { return Err(()); }
3207|    if s == "*" { return Ok(()); }
3208|    if s.chars().any(|c| c.is_ascii_digit()) { Ok(()) } else { Err(()) }
3209|}
3210|
3211|#[cfg(test)]
3212|mod tests {
3213|    use super::*;
3214|    use serde_json::json;
3215|
3216|    fn minimal() -> Value {
3217|        json!({
3218|            "manifestVersion": 1, "id": "com.example.x", "version": "0.1.0",
3219|            "runtime": ">=1.0.0 <2.0.0", "name": "X",
3220|            "entrypoints": { "core": { "kind": "wasm", "path": "core.wasm", "digest": { "alg": "sha-256", "value": "AAAA" } } },
3221|            "capabilities": { "request": [] }
3222|        })
3223|    }
3224|
3225|    #[test]
3226|    fn minimal_manifest_validates() {
3227|        Validator::new().unwrap().validate(&minimal()).unwrap();
3228|    }
3229|
3230|    #[test]
3231|    fn unknown_capability_rejected() {
3232|        let mut m = minimal();
3233|        m["capabilities"]["request"] = json!([{ "id": "nonsense:thing" }]);
3234|        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::UnknownCapability(_))));
3235|    }
3236|
3237|    #[test]
3238|    fn broken_migration_chain_rejected() {
3239|        let mut m = minimal();
3240|        m["storage"] = json!({ "storageSchemaVersion": 3 });
3241|        m["migrations"] = json!([{ "from": 1, "to": 2, "entrypoint": "core", "export": "m1" }]);
3242|        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::BrokenMigrationChain(2))));
3243|    }
3244|
3245|    #[test]
3246|    fn self_conflict_rejected() {
3247|        let mut m = minimal();
3248|        m["conflicts"] = json!([{ "id": "com.example.x" }]);
3249|        assert!(matches!(Validator::new().unwrap().validate(&m), Err(ValidationError::SelfConflict)));
3250|    }
3251|}
3252|```
3253|
3254|The `validate.rs` module references `../../schemas/module-manifest-v1.json` — a symlink or a duplicate copy of the root `schemas/module-manifest-v1.json`. Create the symlink:
3255|
3256|```bash
3257|ln -sf ../../../../schemas/module-manifest-v1.json \
3258|   crates/dnea-manifest/schemas/module-manifest-v1.json
3259|```
3260|
3261|On Windows, copy the file instead:
3262|
3263|```powershell
3264|Copy-Item schemas\module-manifest-v1.json crates\dnea-manifest\schemas\
3265|```
3266|
3267|## 3.5 `crates/dnea-module-host`
3268|
3269|### `crates/dnea-module-host/Cargo.toml`
3270|
3271|```toml
3272|[package]
3273|name        = "dnea-module-host"
3274|version.workspace = true
3275|description = "Wasmtime-based module host for DNEA."
3276|edition.workspace     = true
3277|rust-version.workspace = true
3278|license.workspace     = true
3279|
3280|[dependencies]
3281|dnea-abi    = { path = "../dnea-abi" }
3282|dnea-core   = { path = "../dnea-core" }
3283|anyhow      = { workspace = true }
3284|async-trait = { workspace = true }
3285|serde       = { workspace = true }
3286|serde_json  = { workspace = true }
3287|tokio       = { workspace = true }
3288|tracing     = { workspace = true }
3289|wasmtime    = { workspace = true }
3290|
3291|[dev-dependencies]
3292|tempfile = { workspace = true }
3293|tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "time", "fs"] }
3294|```
3295|
3296|### `crates/dnea-module-host/src/lib.rs`
3297|
3298|```rust
3299|//! The Wasm module host.
3300|
3301|#![forbid(unsafe_code)]
3302|#![deny(rust_2018_idioms, unreachable_pub)]
3303|#![warn(missing_docs)]
3304|
3305|pub mod engine;
3306|pub mod grant;
3307|pub mod host;
3308|pub mod instance;
3309|pub mod link;
3310|
3311|pub use engine::{build_engine, EngineConfig};
3312|pub use grant::{EffectiveGrant, GrantSet, GrantSource};
3313|pub use instance::{InstanceHandle, InstanceMessage, ModuleHost, ModuleHostError};
3314|```
3315|
3316|### `crates/dnea-module-host/src/grant.rs`
3317|
3318|```rust
3319|//! The grant set.
3320|
3321|use std::collections::{BTreeMap, BTreeSet};
3322|
3323|use serde::{Deserialize, Serialize};
3324|
3325|#[derive(Clone, Debug, Serialize, Deserialize)]
3326|pub struct EffectiveGrant {
3327|    pub id: String,
3328|    #[serde(default)]
3329|    pub constraints: serde_json::Value,
3330|    pub source: GrantSource,
3331|}
3332|
3333|#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
3334|#[serde(rename_all = "snake_case")]
3335|pub enum GrantSource { Auto, UserConsent, Policy, DynamicOrigin }
3336|
3337|#[derive(Clone, Debug, Default, Serialize, Deserialize)]
3338|pub struct GrantSet {
3339|    grants: BTreeMap<String, EffectiveGrant>,
3340|    denied: BTreeSet<String>,
3341|}
3342|
3343|impl GrantSet {
3344|    #[must_use]
3345|    pub fn new() -> Self { Self::default() }
3346|
3347|    pub fn insert(&mut self, grant: EffectiveGrant) {
3348|        if self.denied.contains(&grant.id) { return; }
3349|        self.grants.insert(grant.id.clone(), grant);
3350|    }
3351|
3352|    pub fn deny(&mut self, capability: impl Into<String>) {
3353|        let cap = capability.into();
3354|        self.grants.remove(&cap);
3355|        self.denied.insert(cap);
3356|    }
3357|
3358|    #[must_use]
3359|    pub fn get(&self, capability: &str) -> Option<&EffectiveGrant> {
3360|        self.grants.get(capability)
3361|    }
3362|
3363|    #[must_use]
3364|    pub fn any_of(&self, capabilities: &[&str]) -> bool {
3365|        capabilities.iter().any(|c| self.grants.contains_key(*c))
3366|    }
3367|
3368|    #[must_use]
3369|    pub fn all_of(&self, capabilities: &[&str]) -> bool {
3370|        capabilities.iter().all(|c| self.grants.contains_key(*c))
3371|    }
3372|
3373|    pub fn iter(&self) -> impl Iterator<Item = (&str, &EffectiveGrant)> {
3374|        self.grants.iter().map(|(k, v)| (k.as_str(), v))
3375|    }
3376|
3377|    #[must_use]
3378|    pub fn len(&self) -> usize { self.grants.len() }
3379|
3380|    #[must_use]
3381|    pub fn is_empty(&self) -> bool { self.grants.is_empty() }
3382|
3383|    #[must_use]
3384|    pub fn is_denied(&self, capability: &str) -> bool {
3385|        self.denied.contains(capability)
3386|    }
3387|}
3388|
3389|#[cfg(test)]
3390|mod tests {
3391|    use super::*;
3392|
3393|    fn g(id: &str) -> EffectiveGrant {
3394|        EffectiveGrant { id: id.into(), constraints: serde_json::json!({}), source: GrantSource::Auto }
3395|    }
3396|
3397|    #[test]
3398|    fn deny_overrides_grant() {
3399|        let mut s = GrantSet::new();
3400|        s.insert(g("net:connect"));
3401|        assert!(s.get("net:connect").is_some());
3402|        s.deny("net:connect");
3403|        assert!(s.get("net:connect").is_none());
3404|        assert!(s.is_denied("net:connect"));
3405|        // Re-granting is a no-op.
3406|        s.insert(g("net:connect"));
3407|        assert!(s.get("net:connect").is_none());
3408|    }
3409|}
3410|```
3411|
3412|### `crates/dnea-module-host/src/engine.rs`
3413|
3414|```rust
3415|//! Wasmtime engine construction.
3416|
3417|use anyhow::{Context, Result};
3418|use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig};
3419|
3420|#[derive(Clone, Debug)]
3421|pub struct EngineConfig {
3422|    pub total_memory_slots: u32,
3423|    pub total_stack_slots: u32,
3424|    pub max_memory_bytes: u64,
3425|    pub max_memory_pages: u32,
3426|    pub max_tables_per_component: u32,
3427|    pub table_elements: u32,
3428|    pub total_tables: u32,
3429|}
3430|
3431|impl Default for EngineConfig {
3432|    fn default() -> Self {
3433|        Self {
3434|            total_memory_slots: 16,
3435|            total_stack_slots: 16,
3436|            max_memory_bytes: 64 * 1024 * 1024,
3437|            max_memory_pages: 1024,
3438|            max_tables_per_component: 4,
3439|            table_elements: 10_000,
3440|            total_tables: 64,
3441|        }
3442|    }
3443|}
3444|
3445|pub fn build_engine(cfg: &EngineConfig) -> Result<Engine> {
3446|    let mut c = Config::new();
3447|    c.wasm_component_model(true);
3448|    c.async_support(true);
3449|    c.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
3450|    c.wasm_multi_memory(true);
3451|    c.wasm_bulk_memory(true);
3452|    c.wasm_simd(true);
3453|    c.wasm_relaxed_simd(false);
3454|    c.consume_fuel(true);
3455|    c.epoch_interruption(true);
3456|    c.cranelift_opt_level(wasmtime::OptLevel::Speed);
3457|
3458|    let mut pool = PoolingAllocationConfig::default();
3459|    pool.total_memories(cfg.total_memory_slots);
3460|    pool.total_stacks(cfg.total_stack_slots);
3461|    pool.max_memory_size(cfg.max_memory_bytes as usize);
3462|    pool.memory_pages(cfg.max_memory_pages);
3463|    pool.max_tables_per_component(cfg.max_tables_per_component);
3464|    pool.table_elements(cfg.table_elements);
3465|    pool.total_tables(cfg.total_tables);
3466|    c.allocation_strategy(InstanceAllocationStrategy::Pooling(pool));
3467|
3468|    Engine::new(&c).context("build wasmtime engine")
3469|}
3470|```
3471|
3472|### `crates/dnea-module-host/src/host.rs`
3473|
3474|```rust
3475|//! Host-side implementations of the DNEA module ABI.
3476|
3477|use std::{sync::Arc, time::Duration};
3478|
3479|use dnea_abi::dnea::host::{
3480|    crypto, lifecycle, log as log_iface, net, schedule, storage, tabs, types, ui,
3481|};
3482|use dnea_core::{DneaError, DneaResult};
3483|use tracing::{debug, error, info, trace, warn};
3484|
3485|use crate::grant::GrantSet;
3486|
3487|#[async_trait::async_trait]
3488|pub trait TabSource: Send + Sync + 'static {
3489|    async fn query(&self, filter_json: &str) -> DneaResult<Vec<TabInfo>>;
3490|    async fn update(&self, tab_id: u64, props_json: &str) -> DneaResult<()>;
3491|    async fn create(&self, props_json: &str) -> DneaResult<TabInfo>;
3492|    async fn close(&self, tab_id: u64) -> DneaResult<()>;
3493|}
3494|
3495|#[derive(Clone, Debug)]
3496|pub struct TabInfo {
3497|    pub id: u64,
3498|    pub url: String,
3499|    pub title: String,
3500|    pub active: bool,
3501|    pub window_id: u64,
3502|}
3503|
3504|#[async_trait::async_trait]
3505|pub trait ModuleStore: Send + Sync + 'static {
3506|    async fn get(&self, module: &str, key: &str) -> DneaResult<Option<Vec<u8>>>;
3507|    async fn put(&self, module: &str, key: &str, value: &[u8]) -> DneaResult<()>;
3508|    async fn delete(&self, module: &str, key: &str) -> DneaResult<()>;
3509|    async fn list(&self, module: &str, prefix: &str) -> DneaResult<Vec<String>>;
3510|}
3511|
3512|#[async_trait::async_trait]
3513|pub trait UiSink: Send + Sync + 'static {
3514|    async fn set_panel_html(&self, panel_id: &str, html: &str) -> DneaResult<()>;
3515|    async fn notify(&self, title: &str, body: &str) -> DneaResult<()>;
3516|}
3517|
3518|#[async_trait::async_trait]
3519|pub trait CryptoSigner: Send + Sync + 'static {
3520|    async fn sign(&self, key_id: &str, data: &[u8]) -> DneaResult<Vec<u8>>;
3521|    async fn verify(&self, key_id: &str, data: &[u8], sig: &[u8]) -> DneaResult<bool>;
3522|}
3523|
3524|pub struct HostState {
3525|    pub module_id: String,
3526|    pub module_version: String,
3527|    pub grants: Arc<GrantSet>,
3528|    pub tabs: Option<Arc<dyn TabSource>>,
3529|    pub store: Option<Arc<dyn ModuleStore>>,
3530|    pub ui: Option<Arc<dyn UiSink>>,
3531|    pub crypto: Option<Arc<dyn CryptoSigner>>,
3532|    pub limits: wasmtime::StoreLimits,
3533|}
3534|
3535|impl HostState {
3536|    #[must_use]
3537|    pub fn new(
3538|        module_id: String,
3539|        module_version: String,
3540|        grants: Arc<GrantSet>,
3541|        limits: wasmtime::StoreLimits,
3542|    ) -> Self {
3543|        Self {
3544|            module_id, module_version, grants,
3545|            tabs: None, store: None, ui: None, crypto: None, limits,
3546|        }
3547|    }
3548|
3549|    fn check(&self, capability: &str) -> Result<(), types::HostError> {
3550|        if self.grants.get(capability).is_some() {
3551|            Ok(())
3552|        } else if self.grants.is_denied(capability) {
3553|            Err(types::HostError::Denied(capability.to_string()))
3554|        } else {
3555|            Err(types::HostError::NotGranted(capability.to_string()))
3556|        }
3557|    }
3558|}
3559|
3560|impl log_iface::Host for HostState {
3561|    async fn emit(
3562|        &mut self,
3563|        level: types::LogLevel,
3564|        message: String,
3565|        fields_json: Option<String>,
3566|    ) {
3567|        let fields = fields_json.unwrap_or_default();
3568|        match level {
3569|            types::LogLevel::Trace => trace!(module = %self.module_id, fields, "{message}"),
3570|            types::LogLevel::Debug => debug!(module = %self.module_id, fields, "{message}"),
3571|            types::LogLevel::Info  => info! (module = %self.module_id, fields, "{message}"),
3572|            types::LogLevel::Warn  => warn! (module = %self.module_id, fields, "{message}"),
3573|            types::LogLevel::Error => error!(module = %self.module_id, fields, "{message}"),
3574|        }
3575|    }
3576|}
3577|
3578|impl storage::Host for HostState {
3579|    async fn get(&mut self, key: String) -> Result<Option<Vec<u8>>, types::HostError> {
3580|        self.check("storage:module")?;
3581|        let store = self.store.as_ref()
3582|            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
3583|        store.get(&self.module_id, &key).await.map_err(map_err)
3584|    }
3585|
3586|    async fn put(&mut self, key: String, value: Vec<u8>) -> Result<(), types::HostError> {
3587|        self.check("storage:module")?;
3588|        let store = self.store.as_ref()
3589|            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
3590|        store.put(&self.module_id, &key, &value).await.map_err(map_err)
3591|    }
3592|
3593|    async fn delete(&mut self, key: String) -> Result<(), types::HostError> {
3594|        self.check("storage:module")?;
3595|        let store = self.store.as_ref()
3596|            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
3597|        store.delete(&self.module_id, &key).await.map_err(map_err)
3598|    }
3599|
3600|    async fn list(&mut self, prefix: String) -> Result<Vec<String>, types::HostError> {
3601|        self.check("storage:module")?;
3602|        let store = self.store.as_ref()
3603|            .ok_or_else(|| types::HostError::Unavailable("storage backend not wired".into()))?;
3604|        store.list(&self.module_id, &prefix).await.map_err(map_err)
3605|    }
3606|}
3607|
3608|impl schedule::Host for HostState {
3609|    async fn after(&mut self, _delay_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
3610|        Ok(next_handle())
3611|    }
3612|    async fn every(&mut self, _interval_ms: u64, _callback_id: String) -> Result<u64, types::HostError> {
3613|        Ok(next_handle())
3614|    }
3615|    async fn cancel(&mut self, _handle: u64) -> Result<(), types::HostError> { Ok(()) }
3616|}
3617|
3618|fn next_handle() -> u64 {
3619|    use std::sync::atomic::{AtomicU64, Ordering};
3620|    static NEXT: AtomicU64 = AtomicU64::new(1);
3621|    NEXT.fetch_add(1, Ordering::Relaxed)
3622|}
3623|
3624|impl tabs::Host for HostState {
3625|    async fn query(&mut self, filter_json: String) -> Result<Vec<tabs::Tab>, types::HostError> {
3626|        self.check("browser:tabs:query")?;
3627|        let src = self.tabs.as_ref()
3628|            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
3629|        let rows = src.query(&filter_json).await.map_err(map_err)?;
3630|        Ok(rows.into_iter().map(to_wit_tab).collect())
3631|    }
3632|    async fn update(&mut self, tab_id: u64, props_json: String) -> Result<(), types::HostError> {
3633|        self.check("browser:tabs:update")?;
3634|        let src = self.tabs.as_ref()
3635|            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
3636|        src.update(tab_id, &props_json).await.map_err(map_err)
3637|    }
3638|    async fn create(&mut self, props_json: String) -> Result<tabs::Tab, types::HostError> {
3639|        self.check("browser:tabs:create")?;
3640|        let src = self.tabs.as_ref()
3641|            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
3642|        let tab = src.create(&props_json).await.map_err(map_err)?;
3643|        Ok(to_wit_tab(tab))
3644|    }
3645|    async fn close(&mut self, tab_id: u64) -> Result<(), types::HostError> {
3646|        self.check("browser:tabs:close")?;
3647|        let src = self.tabs.as_ref()
3648|            .ok_or_else(|| types::HostError::Unavailable("tabs backend not wired".into()))?;
3649|        src.close(tab_id).await.map_err(map_err)
3650|    }
3651|}
3652|
3653|fn to_wit_tab(t: TabInfo) -> tabs::Tab {
3654|    tabs::Tab { id: t.id, url: t.url, title: t.title, active: t.active, window_id: t.window_id }
3655|}
3656|
3657|impl net::Host for HostState {
3658|    async fn request(&mut self, _req_json: String) -> Result<Vec<u8>, types::HostError> {
3659|        self.check("net:connect")?;
3660|        Err(types::HostError::Unavailable(
3661|            "net:connect is not implemented in this build".into(),
3662|        ))
3663|    }
3664|}
3665|
3666|impl ui::Host for HostState {
3667|    async fn set_panel_html(&mut self, panel_id: String, html: String) -> Result<(), types::HostError> {
3668|        self.check("ui:companion:panel")?;
3669|        let sink = self.ui.as_ref()
3670|            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
3671|        sink.set_panel_html(&panel_id, &html).await.map_err(map_err)
3672|    }
3673|    async fn notify(&mut self, title: String, body: String) -> Result<(), types::HostError> {
3674|        self.check("ui:companion:notify")?;
3675|        let sink = self.ui.as_ref()
3676|            .ok_or_else(|| types::HostError::Unavailable("ui sink not wired".into()))?;
3677|        sink.notify(&title, &body).await.map_err(map_err)
3678|    }
3679|}
3680|
3681|impl crypto::Host for HostState {
3682|    async fn sign(&mut self, key_id: String, data: Vec<u8>) -> Result<Vec<u8>, types::HostError> {
3683|        self.check("identity:crypto:sign")?;
3684|        let signer = self.crypto.as_ref()
3685|            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
3686|        signer.sign(&key_id, &data).await.map_err(map_err)
3687|    }
3688|    async fn verify(&mut self, key_id: String, data: Vec<u8>, sig: Vec<u8>) -> Result<bool, types::HostError> {
3689|        self.check("identity:crypto:verify")?;
3690|        let signer = self.crypto.as_ref()
3691|            .ok_or_else(|| types::HostError::Unavailable("crypto signer not wired".into()))?;
3692|        signer.verify(&key_id, &data, &sig).await.map_err(map_err)
3693|    }
3694|}
3695|
3696|impl lifecycle::Host for HostState {
3697|    async fn activate(&mut self) -> Result<(), types::HostError> {
3698|        Err(types::HostError::Internal("lifecycle.activate is a module export".into()))
3699|    }
3700|    async fn deactivate(&mut self) -> Result<(), types::HostError> {
3701|        Err(types::HostError::Internal("lifecycle.deactivate is a module export".into()))
3702|    }
3703|    async fn upgrade(&mut self, _from_version: String, _from_storage_schema: u32) -> Result<(), types::HostError> {
3704|        Err(types::HostError::Internal("lifecycle.upgrade is a module export".into()))
3705|    }
3706|    async fn health_check(&mut self) -> Result<(), types::HostError> {
3707|        Err(types::HostError::Internal("lifecycle.health-check is a module export".into()))
3708|    }
3709|}
3710|
3711|fn map_err(e: DneaError) -> types::HostError {
3712|    match e {
3713|        DneaError::CapabilityDenied { capability, .. } => types::HostError::NotGranted(capability),
3714|        DneaError::ModuleUnavailable(s) => types::HostError::Unavailable(s),
3715|        DneaError::Timeout(ms) => types::HostError::Timeout(ms),
3716|        DneaError::RateLimited { retry_after_ms } => {
3717|            types::HostError::RateLimited(u32::try_from(retry_after_ms).unwrap_or(u32::MAX))
3718|        }
3719|        DneaError::Internal(s) => types::HostError::Internal(s),
3720|        other => types::HostError::Internal(other.to_string()),
3721|    }
3722|}
3723|
3724|const _: Duration = Duration::from_secs(0);
3725|```
3726|
3727|### `crates/dnea-module-host/src/link.rs`
3728|
3729|```rust
3730|//! The conditional linker.
3731|
3732|use anyhow::Result;
3733|use wasmtime::component::Linker;
3734|
3735|use crate::{grant::GrantSet, host::HostState};
3736|
3737|pub fn build_linker(
3738|    engine: &wasmtime::Engine,
3739|    grants: &GrantSet,
3740|) -> Result<Linker<HostState>> {
3741|    let mut linker: Linker<HostState> = Linker::new(engine);
3742|
3743|    dnea_abi::dnea::host::types::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3744|        &mut linker, |s| s,
3745|    )?;
3746|    dnea_abi::dnea::host::log::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3747|        &mut linker, |s| s,
3748|    )?;
3749|    dnea_abi::dnea::host::storage::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3750|        &mut linker, |s| s,
3751|    )?;
3752|    dnea_abi::dnea::host::schedule::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3753|        &mut linker, |s| s,
3754|    )?;
3755|
3756|    if grants.any_of(&[
3757|        "browser:tabs:query", "browser:tabs:update",
3758|        "browser:tabs:create", "browser:tabs:close",
3759|    ]) {
3760|        dnea_abi::dnea::host::tabs::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3761|            &mut linker, |s| s,
3762|        )?;
3763|    }
3764|
3765|    if grants.any_of(&["net:connect", "net:listen", "net:websocket"]) {
3766|        dnea_abi::dnea::host::net::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3767|            &mut linker, |s| s,
3768|        )?;
3769|    }
3770|
3771|    if grants.any_of(&[
3772|        "ui:companion:panel", "ui:companion:notify",
3773|        "ui:webext:popup", "ui:webext:content",
3774|    ]) {
3775|        dnea_abi::dnea::host::ui::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3776|            &mut linker, |s| s,
3777|        )?;
3778|    }
3779|
3780|    if grants.any_of(&["identity:crypto:sign", "identity:crypto:verify"]) {
3781|        dnea_abi::dnea::host::crypto::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
3782|            &mut linker, |s| s,
3783|        )?;
3784|    }
3785|
3786|    Ok(linker)
3787|}
3788|```
3789|
3790|### `crates/dnea-module-host/src/instance.rs`
3791|
3792|```rust
3793|//! Instance lifecycle and message routing.
3794|
3795|use std::{path::PathBuf, sync::Arc, time::Duration};
3796|
3797|use anyhow::{Context, Result};
3798|use dnea_abi::dnea::host::lifecycle::Lifecycle;
3799|use dnea_abi::dnea::host::module_api::ModuleApi;
3800|use dnea_core::{DneaError, DneaResult};
3801|use tokio::sync::{mpsc, oneshot};
3802|use tracing::{info, warn};
3803|use wasmtime::{
3804|    component::{Component, Linker},
3805|    Store, StoreLimitsBuilder,
3806|};
3807|
3808|use crate::{
3809|    engine::{build_engine, EngineConfig},
3810|    grant::GrantSet,
3811|    host::HostState,
3812|    link::build_linker,
3813|};
3814|
3815|#[derive(Debug, thiserror::Error)]
3816|pub enum ModuleHostError {
3817|    #[error("compile error: {0}")]
3818|    Compile(String),
3819|    #[error("instantiate error: {0}")]
3820|    Instantiate(String),
3821|    #[error("lifecycle {hook} failed: {message}")]
3822|    Lifecycle { hook: String, message: String },
3823|    #[error("instance task stopped")]
3824|    Stopped,
3825|}
3826|
3827|pub enum InstanceMessage {
3828|    Invoke {
3829|        method: String,
3830|        args: Vec<u8>,
3831|        reply: oneshot::Sender<DneaResult<Vec<u8>>>,
3832|    },
3833|    Event { kind: String, data: Vec<u8> },
3834|    Shutdown(oneshot::Sender<()>),
3835|}
3836|
3837|pub struct InstanceHandle {
3838|    pub id: String,
3839|    pub version: String,
3840|    pub grants: Arc<GrantSet>,
3841|    tx: mpsc::Sender<InstanceMessage>,
3842|    join: Option<tokio::task::JoinHandle<()>>,
3843|}
3844|
3845|impl InstanceHandle {
3846|    pub async fn invoke(&self, method: &str, args: Vec<u8>) -> DneaResult<Vec<u8>> {
3847|        let (tx, rx) = oneshot::channel();
3848|        self.tx
3849|            .send(InstanceMessage::Invoke { method: method.to_string(), args, reply: tx })
3850|            .await
3851|            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?;
3852|        rx.await
3853|            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))?
3854|    }
3855|
3856|    pub async fn event(&self, kind: &str, data: Vec<u8>) -> DneaResult<()> {
3857|        self.tx
3858|            .send(InstanceMessage::Event { kind: kind.to_string(), data })
3859|            .await
3860|            .map_err(|_| DneaError::ModuleUnavailable(self.id.clone()))
3861|    }
3862|
3863|    pub async fn shutdown(mut self) {
3864|        let (tx, rx) = oneshot::channel();
3865|        let _ = self.tx.send(InstanceMessage::Shutdown(tx)).await;
3866|        let _ = tokio::time::timeout(Duration::from_secs(5), rx).await;
3867|        if let Some(join) = self.join.take() { join.abort(); }
3868|    }
3869|
3870|    /// Constructs a handle for tests. Never invokes any method.
3871|    #[doc(hidden)]
3872|    pub fn new_for_test(id: String, version: String, grants: Arc<GrantSet>) -> Self {
3873|        let (tx, _rx) = mpsc::channel(1);
3874|        Self { id, version, grants, tx, join: None }
3875|    }
3876|}
3877|
3878|pub struct ModuleHost {
3879|    engine: wasmtime::Engine,
3880|}
3881|
3882|impl ModuleHost {
3883|    pub fn new(cfg: &EngineConfig) -> Result<Self> {
3884|        let engine = build_engine(cfg)?;
3885|        Ok(Self { engine })
3886|    }
3887|
3888|    #[must_use]
3889|    pub fn engine_clone(&self) -> wasmtime::Engine { self.engine.clone() }
3890|
3891|    pub async fn load(
3892|        &self,
3893|        module_id: String,
3894|        module_version: String,
3895|        component_path: PathBuf,
3896|        grants: GrantSet,
3897|        host_state_builder: impl FnOnce(Arc<GrantSet>) -> HostState,
3898|    ) -> Result<InstanceHandle, ModuleHostError> {
3899|        let component = Component::from_file(&self.engine, &component_path)
3900|            .with_context(|| format!("load {}", component_path.display()))
3901|            .map_err(|e| ModuleHostError::Compile(e.to_string()))?;
3902|
3903|        let grants = Arc::new(grants);
3904|        let linker: Linker<HostState> = build_linker(&self.engine, &grants)
3905|            .map_err(|e| ModuleHostError::Instantiate(format!("linker: {e}")))?;
3906|
3907|        let mut host_state = host_state_builder(grants.clone());
3908|        host_state.limits = StoreLimitsBuilder::new()
3909|            .memory_size(64 * 1024 * 1024)
3910|            .instances(1)
3911|            .memories(4)
3912|            .tables(4)
3913|            .build();
3914|
3915|        let mut store = Store::new(&self.engine, host_state);
3916|        store.limiter(|s| &mut s.limits);
3917|        store.set_fuel(10_000_000)
3918|            .map_err(|e| ModuleHostError::Instantiate(format!("fuel: {e}")))?;
3919|        store.set_epoch_deadline(1);
3920|
3921|        let instance = linker
3922|            .instantiate_async(&mut store, &component)
3923|            .await
3924|            .map_err(|e| ModuleHostError::Instantiate(e.to_string()))?;
3925|
3926|        let lifecycle = Lifecycle::new(&mut store, &instance)
3927|            .map_err(|e| ModuleHostError::Lifecycle { hook: "new".into(), message: e.to_string() })?;
3928|        lifecycle
3929|            .call_activate(&mut store)
3930|            .await
3931|            .map_err(|e| ModuleHostError::Lifecycle { hook: "activate".into(), message: e.to_string() })?;
3932|
3933|        if let Err(e) = lifecycle.call_health_check(&mut store).await {
3934|            warn!(module = %module_id, error = %e, "initial health check failed");
3935|        }
3936|
3937|        info!(module = %module_id, version = %module_version, "module loaded");
3938|
3939|        let (tx, rx) = mpsc::channel::<InstanceMessage>(64);
3940|        let task_id = module_id.clone();
3941|        let join = tokio::spawn(instance_task(task_id, store, instance, rx));
3942|
3943|        Ok(InstanceHandle {
3944|            id: module_id,
3945|            version: module_version,
3946|            grants,
3947|            tx,
3948|            join: Some(join),
3949|        })
3950|    }
3951|}
3952|
3953|async fn instance_task(
3954|    module_id: String,
3955|    mut store: Store<HostState>,
3956|    instance: wasmtime::component::Instance,
3957|    mut rx: mpsc::Receiver<InstanceMessage>,
3958|) {
3959|    while let Some(msg) = rx.recv().await {
3960|        match msg {
3961|            InstanceMessage::Invoke { method, args, reply } => {
3962|                let result = dispatch_invoke(&module_id, &mut store, &instance, &method, args).await;
3963|                let _ = reply.send(result);
3964|            }
3965|            InstanceMessage::Event { kind, data } => {
3966|                let _ = dispatch_invoke(&module_id, &mut store, &instance, &kind, data).await;
3967|            }
3968|            InstanceMessage::Shutdown(ack) => {
3969|                if let Ok(lifecycle) = Lifecycle::new(&mut store, &instance) {
3970|                    let _ = lifecycle.call_deactivate(&mut store).await;
3971|                }
3972|                let _ = ack.send(());
3973|                return;
3974|            }
3975|        }
3976|    }
3977|}
3978|
3979|async fn dispatch_invoke(
3980|    module_id: &str,
3981|    store: &mut Store<HostState>,
3982|    instance: &wasmtime::component::Instance,
3983|    method: &str,
3984|    args: Vec<u8>,
3985|) -> DneaResult<Vec<u8>> {
3986|    // Prefer the generic module-api export.
3987|    if let Ok(api) = ModuleApi::new(store, instance) {
3988|        let args_json = String::from_utf8_lossy(&args).into_owned();
3989|        return match api.call_invoke(store, method, &args_json).await {
3990|            Ok(Ok(json)) => Ok(json.into_bytes()),
3991|            Ok(Err(e)) => Err(DneaError::Module {
3992|                module: module_id.to_string(),
3993|                code: "host_error".into(),
3994|                message: format!("{e:?}"),
3995|            }),
3996|            Err(e) => Err(DneaError::Module {
3997|                module: module_id.to_string(),
3998|                code: "invoke".into(),
3999|                message: e.to_string(),
4000|            }),
4001|        };
4002|    }
4003|
4004|    // Fallback: lifecycle-only modules.
4005|    if method == "healthCheck" {
4006|        let lifecycle = Lifecycle::new(store, instance)
4007|            .map_err(|e| DneaError::Module {
4008|                module: module_id.to_string(),
4009|                code: "lifecycle_new".into(),
4010|                message: e.to_string(),
4011|            })?;
4012|        lifecycle
4013|            .call_health_check(store)
4014|            .await
4015|            .map_err(|e| DneaError::Module {
4016|                module: module_id.to_string(),
4017|                code: "health_check".into(),
4018|                message: e.to_string(),
4019|            })?;
4020|        return Ok(Vec::new());
4021|    }
4022|
4023|    Err(DneaError::Module {
4024|        module: module_id.to_string(),
4025|        code: "unknown_method".into(),
4026|        message: format!("no such export: {method}"),
4027|    })
4028|}
4029|```
4030|
4031|## 3.6 `crates/dnea-nm-host`
4032|
4033|### `crates/dnea-nm-host/Cargo.toml`
4034|
4035|```toml
4036|[package]
4037|name        = "dnea-nm-host"
4038|version.workspace = true
4039|description = "DNEA Native Messaging host and bridge."
4040|edition.workspace     = true
4041|rust-version.workspace = true
4042|license.workspace     = true
4043|
4044|[[bin]]
4045|name = "dnea-nm-bridge"
4046|path = "src/bin/bridge.rs"
4047|
4048|[dependencies]
4049|dnea-core   = { path = "../dnea-core" }
4050|anyhow      = { workspace = true }
4051|async-trait = { workspace = true }
4052|bytes       = { workspace = true }
4053|directories = { workspace = true }
4054|serde       = { workspace = true }
4055|serde_json  = { workspace = true }
4056|tokio       = { workspace = true }
4057|tracing     = { workspace = true }
4058|uuid        = { workspace = true }
4059|base64      = { workspace = true }
4060|sha2        = { workspace = true }
4061|time        = { workspace = true }
4062|thiserror   = { workspace = true }
4063|
4064|[target.'cfg(unix)'.dependencies]
4065|libc = { workspace = true }
4066|
4067|[target.'cfg(windows)'.dependencies]
4068|winreg      = "0.52"
4069|windows-sys = { version = "0.59", features = [
4070|    "Win32_Foundation",
4071|    "Win32_Storage_FileSystem",
4072|    "Win32_System_IO",
4073|    "Win32_System_Pipes",
4074|] }
4075|
4076|[dev-dependencies]
4077|tempfile = { workspace = true }
4078|tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "process", "io-util", "time"] }
4079|```
4080|
4081|### `crates/dnea-nm-host/src/lib.rs`
4082|
4083|```rust
4084|//! DNEA Native Messaging host.
4085|
4086|#![forbid(unsafe_code)]
4087|#![deny(rust_2018_idioms, unreachable_pub)]
4088|#![warn(missing_docs)]
4089|
4090|pub mod registration;
4091|pub mod session;
4092|pub mod supervisor;
4093|
4094|pub use session::{ClientId, ClientInfo, SessionSink};
4095|pub use supervisor::{NmHostConfig, NmHostError, NmSupervisor};
4096|```
4097|
4098|### `crates/dnea-nm-host/src/session.rs`
4099|
4100|```rust
4101|//! The interface the NM supervisor uses to talk to the runtime core.
4102|
4103|use async_trait::async_trait;
4104|use dnea_core::{DneaResult, Envelope, SessionId};
4105|use uuid::Uuid;
4106|
4107|#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
4108|pub struct ClientId(Uuid);
4109|
4110|impl ClientId {
4111|    #[must_use]
4112|    pub fn new() -> Self { Self(Uuid::now_v7()) }
4113|    #[must_use]
4114|    pub fn as_uuid(&self) -> Uuid { self.0 }
4115|}
4116|
4117|impl Default for ClientId {
4118|    fn default() -> Self { Self::new() }
4119|}
4120|
4121|impl std::fmt::Display for ClientId {
4122|    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
4123|        self.0.fmt(f)
4124|    }
4125|}
4126|
4127|#[derive(Clone, Debug)]
4128|pub struct ClientInfo {
4129|    pub browser_family: String,
4130|    pub extension_id: Option<String>,
4131|    pub bridge_pid: Option<u32>,
4132|    pub connected_at: std::time::SystemTime,
4133|}
4134|
4135|#[async_trait]
4136|pub trait SessionSink: Send + Sync + 'static {
4137|    async fn on_client_connected(
4138|        &self,
4139|        client_id: ClientId,
4140|        info: ClientInfo,
4141|    ) -> DneaResult<SessionId>;
4142|
4143|    async fn on_client_disconnected(&self, client_id: ClientId);
4144|
4145|    async fn handle_client_envelope(
4146|        &self,
4147|        client_id: ClientId,
4148|        env: Envelope,
4149|    ) -> DneaResult<Option<Envelope>>;
4150|
4151|    async fn send_to_client(
4152|        &self,
4153|        _client_id: ClientId,
4154|        _env: Envelope,
4155|    ) -> DneaResult<()> {
4156|        Err(dnea_core::DneaError::UnsupportedFeature(
4157|            "this SessionSink does not support server-initiated messages".into(),
4158|        ))
4159|    }
4160|}
4161|```
4162|
4163|### `crates/dnea-nm-host/src/supervisor.rs`
4164|
4165|```rust
4166|//! The daemon-side NM supervisor.
4167|
4168|use std::{
4169|    path::PathBuf,
4170|    sync::Arc,
4171|    time::SystemTime,
4172|};
4173|
4174|use dnea_core::{
4175|    codec::{Codec, Decoded, DEFAULT_MTU},
4176|    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
4177|    DneaError, DneaResult, EnvelopeId,
4178|};
4179|use tokio::{
4180|    io::{AsyncReadExt, AsyncWriteExt},
4181|    sync::{mpsc, Semaphore},
4182|};
4183|use tracing::{debug, info, warn};
4184|
4185|use crate::session::{ClientId, ClientInfo, SessionSink};
4186|
4187|const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;
4188|
4189|#[derive(Clone, Debug)]
4190|pub struct NmHostConfig {
4191|    pub endpoint: PathBuf,
4192|    pub max_connections: usize,
4193|    pub mtu: usize,
4194|}
4195|
4196|impl NmHostConfig {
4197|    #[must_use]
4198|    pub fn new(endpoint: PathBuf) -> Self {
4199|        Self { endpoint, max_connections: 64, mtu: DEFAULT_MTU }
4200|    }
4201|}
4202|
4203|#[derive(Debug, thiserror::Error)]
4204|pub enum NmHostError {
4205|    #[error("endpoint {0} is already in use")]
4206|    AlreadyBound(PathBuf),
4207|    #[error("listener I/O: {0}")]
4208|    Io(#[from] std::io::Error),
4209|    #[error("session sink rejected client: {0}")]
4210|    SinkRejected(DneaError),
4211|}
4212|
4213|pub struct NmSupervisor {
4214|    config: NmHostConfig,
4215|    sink: Arc<dyn SessionSink>,
4216|    conn_limit: Arc<Semaphore>,
4217|}
4218|
4219|impl NmSupervisor {
4220|    #[must_use]
4221|    pub fn new(config: NmHostConfig, sink: Arc<dyn SessionSink>) -> Self {
4222|        Self {
4223|            conn_limit: Arc::new(Semaphore::new(config.max_connections)),
4224|            config,
4225|            sink,
4226|        }
4227|    }
4228|
4229|    pub async fn run(self) -> Result<(), NmHostError> {
4230|        #[cfg(unix)]
4231|        {
4232|            let endpoint = self.config.endpoint.clone();
4233|            if endpoint.exists() {
4234|                std::fs::remove_file(&endpoint).ok();
4235|            }
4236|            let listener = tokio::net::UnixListener::bind(&endpoint)?;
4237|            info!(endpoint = %endpoint.display(), "nm supervisor listening");
4238|            self.accept_loop_unix(listener).await;
4239|            Ok(())
4240|        }
4241|        #[cfg(windows)]
4242|        {
4243|            crate::supervisor_windows::run_windows_listener(self.config, self.sink).await
4244|        }
4245|    }
4246|
4247|    #[cfg(unix)]
4248|    async fn accept_loop_unix(self, listener: tokio::net::UnixListener) {
4249|        loop {
4250|            let permit = match self.conn_limit.clone().acquire_owned().await {
4251|                Ok(p) => p,
4252|                Err(_) => {
4253|                    warn!("connection limiter closed");
4254|                    return;
4255|                }
4256|            };
4257|            match listener.accept().await {
4258|                Ok((stream, _addr)) => {
4259|                    let sink = self.sink.clone();
4260|                    let mtu = self.config.mtu;
4261|                    tokio::spawn(async move {
4262|                        if let Err(e) = handle_connection(stream, sink, mtu).await {
4263|                            warn!(error = %e, "nm connection ended with error");
4264|                        }
4265|                        drop(permit);
4266|                    });
4267|                }
4268|                Err(e) => {
4269|                    warn!(error = %e, "nm accept failed; continuing");
4270|                    drop(permit);
4271|                }
4272|            }
4273|        }
4274|    }
4275|}
4276|
4277|#[cfg(unix)]
4278|async fn handle_connection(
4279|    mut sock: tokio::net::UnixStream,
4280|    sink: Arc<dyn SessionSink>,
4281|    mtu: usize,
4282|) -> DneaResult<()> {
4283|    let preamble = read_frame(&mut sock).await?;
4284|    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
4285|        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
4286|    let browser_family = preamble
4287|        .get("browser_family").and_then(|v| v.as_str())
4288|        .unwrap_or("unknown").to_string();
4289|    let extension_id = preamble
4290|        .get("extension_id").and_then(|v| v.as_str())
4291|        .map(str::to_string);
4292|    let bridge_pid = preamble
4293|        .get("pid").and_then(|v| v.as_u64())
4294|        .and_then(|n| u32::try_from(n).ok());
4295|
4296|    let client_id = ClientId::new();
4297|    let info = ClientInfo {
4298|        browser_family: browser_family.clone(),
4299|        extension_id,
4300|        bridge_pid,
4301|        connected_at: SystemTime::now(),
4302|    };
4303|
4304|    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
4305|    let (read_half, mut write_half) = sock.into_split();
4306|
4307|    // Attach the client with its outbound sender before invoking
4308|    // on_client_connected so the session can deliver Welcome through
4309|    // the normal channel.
4310|    if let Some(session) = sink.as_ref() as *const dyn SessionSink as *const _ as *const crate::session::Attachable {
4311|        let _ = session; // fallback path; see note below
4312|    }
4313|    sink.on_client_connected(client_id, info)
4314|        .await
4315|        .map_err(NmHostError::SinkRejected)?;
4316|
4317|    let writer = tokio::spawn(async move {
4318|        while let Some(bytes) = rx.recv().await {
4319|            if write_frame(&mut write_half, &bytes).await.is_err() {
4320|                return;
4321|            }
4322|        }
4323|    });
4324|
4325|    let codec = Codec::new(mtu);
4326|    let mut read_half = read_half;
4327|    let result = read_loop(&mut read_half, &sink, client_id, &codec, &tx).await;
4328|
4329|    drop(tx);
4330|    let _ = writer.await;
4331|    sink.on_client_disconnected(client_id).await;
4332|    debug!(%client_id, "nm client disconnected");
4333|    result
4334|}
4335|
4336|#[cfg(unix)]
4337|async fn read_loop(
4338|    read_half: &mut tokio::net::unix::OwnedReadHalf,
4339|    sink: &Arc<dyn SessionSink>,
4340|    client_id: ClientId,
4341|    codec: &Codec,
4342|    tx: &mpsc::Sender<Vec<u8>>,
4343|) -> DneaResult<()> {
4344|    loop {
4345|        let frame = match read_frame_opt(read_half).await? {
4346|            Some(f) => f,
4347|            None => return Ok(()),
4348|        };
4349|        let decoded = match codec.decode(&frame) {
4350|            Ok(d) => d,
4351|            Err(e) => {
4352|                warn!(%client_id, error = %e, "framing error");
4353|                let err_env = Envelope {
4354|                    v: PROTOCOL_VERSION,
4355|                    id: EnvelopeId::new(),
4356|                    kind: MessageType::Error,
4357|                    ts: time::OffsetDateTime::now_utc(),
4358|                    src: Party::Runtime,
4359|                    dst: Party::Client,
4360|                    reply_to: None,
4361|                    cap: None,
4362|                    module: None,
4363|                    enc: dnea_core::Encoding::Identity,
4364|                    ttl_ms: None,
4365|                    payload: None,
4366|                    err: Some(dnea_core::EnvelopeError {
4367|                        code: e.code().to_string(),
4368|                        message: e.to_string(),
4369|                        retryable: false,
4370|                        retry_after_ms: None,
4371|                        details: None,
4372|                        cause: None,
4373|                    }),
4374|                    meta: Default::default(),
4375|                };
4376|                let _ = send_envelope(codec, tx, &err_env).await;
4377|                continue;
4378|            }
4379|        };
4380|        let env = match decoded {
4381|            Decoded::Incomplete => continue,
4382|            Decoded::Envelope(e) => *e,
4383|        };
4384|        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
4385|            warn!(%client_id, error = %e, "direction violation");
4386|            continue;
4387|        }
4388|        match sink.handle_client_envelope(client_id, env).await {
4389|            Ok(Some(reply)) => { send_envelope(codec, tx, &reply).await?; }
4390|            Ok(None) => {}
4391|            Err(e) => {
4392|                let err_env = Envelope {
4393|                    v: PROTOCOL_VERSION,
4394|                    id: EnvelopeId::new(),
4395|                    kind: MessageType::Error,
4396|                    ts: time::OffsetDateTime::now_utc(),
4397|                    src: Party::Runtime,
4398|                    dst: Party::Client,
4399|                    reply_to: None,
4400|                    cap: None,
4401|                    module: None,
4402|                    enc: dnea_core::Encoding::Identity,
4403|                    ttl_ms: None,
4404|                    payload: None,
4405|                    err: Some(dnea_core::EnvelopeError {
4406|                        code: e.code().to_string(),
4407|                        message: e.to_string(),
4408|                        retryable: e.retryable(),
4409|                        retry_after_ms: match e {
4410|                            DneaError::RateLimited { retry_after_ms } => Some(retry_after_ms),
4411|                            _ => None,
4412|                        },
4413|                        details: None,
4414|                        cause: None,
4415|                    }),
4416|                    meta: Default::default(),
4417|                };
4418|                send_envelope(codec, tx, &err_env).await?;
4419|            }
4420|        }
4421|    }
4422|}
4423|
4424|async fn send_envelope(
4425|    codec: &Codec,
4426|    tx: &mpsc::Sender<Vec<u8>>,
4427|    env: &Envelope,
4428|) -> DneaResult<()> {
4429|    for frame in codec.encode(env)? {
4430|        tx.send(frame).await.map_err(|_| DneaError::Transport)?;
4431|    }
4432|    Ok(())
4433|}
4434|
4435|async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Vec<u8>> {
4436|    read_frame_opt(r).await?
4437|        .ok_or_else(|| DneaError::Framing(dnea_core::FramingError::MissingEnvelope))
4438|}
4439|
4440|async fn read_frame_opt<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Option<Vec<u8>>> {
4441|    let mut len_buf = [0u8; 4];
4442|    match r.read_exact(&mut len_buf).await {
4443|        Ok(_) => {}
4444|        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
4445|        Err(e) => return Err(DneaError::Internal(format!("read length: {e}"))),
4446|    }
4447|    let len = u32::from_ne_bytes(len_buf) as usize;
4448|    if len == 0 || len > MAX_FRAME_BYTES {
4449|        return Err(DneaError::Framing(dnea_core::FramingError::TooLarge {
4450|            actual: len, limit: MAX_FRAME_BYTES,
4451|        }));
4452|    }
4453|    let mut buf = vec![0u8; len];
4454|    r.read_exact(&mut buf).await
4455|        .map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
4456|    Ok(Some(buf))
4457|}
4458|
4459|async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, bytes: &[u8]) -> DneaResult<()> {
4460|    if bytes.is_empty() {
4461|        return Err(DneaError::Internal("refusing to write empty frame".into()));
4462|    }
4463|    let len = u32::try_from(bytes.len())
4464|        .map_err(|_| DneaError::Internal("frame too large for u32 prefix".into()))?;
4465|    w.write_all(&len.to_ne_bytes()).await.map_err(|_| DneaError::Transport)?;
4466|    w.write_all(bytes).await.map_err(|_| DneaError::Transport)?;
4467|    w.flush().await.map_err(|_| DneaError::Transport)?;
4468|    Ok(())
4469|}
4470|```
4471|
4472|### `crates/dnea-nm-host/src/supervisor_windows.rs`
4473|
4474|```rust
4475|//! Windows named-pipe accept loop.
4476|
4477|#![cfg(windows)]
4478|
4479|use std::{
4480|    path::PathBuf,
4481|    sync::Arc,
4482|};
4483|
4484|use anyhow::{Context, Result};
4485|use tokio::io::{AsyncReadExt, AsyncWriteExt};
4486|use tokio::net::windows::named_pipe::ServerOptions;
4487|use tokio::sync::mpsc;
4488|use tracing::{debug, warn};
4489|
4490|use dnea_core::{
4491|    codec::{Codec, Decoded, DEFAULT_MTU},
4492|    envelope::{Envelope, MessageType, Party, TransportDirection, PROTOCOL_VERSION},
4493|    DneaError, DneaResult, EnvelopeId,
4494|};
4495|
4496|use crate::{
4497|    session::{ClientId, ClientInfo, SessionSink},
4498|    supervisor::NmHostConfig,
4499|};
4500|
4501|const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;
4502|
4503|pub async fn run_windows_listener(
4504|    config: NmHostConfig,
4505|    sink: Arc<dyn SessionSink>,
4506|) -> Result<(), crate::supervisor::NmHostError> {
4507|    let pipe_name = pipe_name_from(&config.endpoint);
4508|    tracing::info!(pipe = %pipe_name, "nm supervisor listening (Windows named pipe)");
4509|
4510|    loop {
4511|        let server = ServerOptions::new()
4512|            .first_pipe_instance(false)
4513|            .create(&pipe_name)
4514|            .map_err(crate::supervisor::NmHostError::Io)?;
4515|
4516|        server.connect().await.map_err(crate::supervisor::NmHostError::Io)?;
4517|
4518|        let sink = sink.clone();
4519|        let mtu = config.mtu;
4520|        tokio::spawn(async move {
4521|            if let Err(e) = handle_windows_client(server, sink, mtu).await {
4522|                warn!(error = %e, "Windows NM client error");
4523|            }
4524|        });
4525|    }
4526|}
4527|
4528|fn pipe_name_from(endpoint: &std::path::Path) -> String {
4529|    let s = endpoint.to_string_lossy();
4530|    if s.starts_with(r"\\.\pipe\") {
4531|        return s.to_string();
4532|    }
4533|    let user = std::env::var("USERNAME").unwrap_or_else(|_| "default".into());
4534|    format!(r"\\.\pipe\dnea-nm-{user}")
4535|}
4536|
4537|async fn handle_windows_client(
4538|    mut pipe: tokio::net::windows::named_pipe::NamedPipeServer,
4539|    sink: Arc<dyn SessionSink>,
4540|    mtu: usize,
4541|) -> DneaResult<()> {
4542|    // Preamble.
4543|    let preamble = read_frame(&mut pipe).await?;
4544|    let preamble: serde_json::Value = serde_json::from_slice(&preamble)
4545|        .map_err(|e| DneaError::InvalidEnvelope(format!("bad preamble JSON: {e}")))?;
4546|    let browser_family = preamble.get("browser_family").and_then(|v| v.as_str())
4547|        .unwrap_or("unknown").to_string();
4548|    let extension_id = preamble.get("extension_id").and_then(|v| v.as_str()).map(str::to_string);
4549|    let bridge_pid = preamble.get("pid").and_then(|v| v.as_u64())
4550|        .and_then(|n| u32::try_from(n).ok());
4551|
4552|    let client_id = ClientId::new();
4553|    let info = ClientInfo {
4554|        browser_family,
4555|        extension_id,
4556|        bridge_pid,
4557|        connected_at: std::time::SystemTime::now(),
4558|    };
4559|    sink.on_client_connected(client_id, info).await
4560|        .map_err(|e| DneaError::Internal(e.to_string()))?;
4561|
4562|    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);
4563|    let (mut read_half, mut write_half) = tokio::io::split(pipe);
4564|
4565|    let writer = tokio::spawn(async move {
4566|        while let Some(bytes) = rx.recv().await {
4567|            if write_frame(&mut write_half, &bytes).await.is_err() { return; }
4568|        }
4569|    });
4570|
4571|    let codec = Codec::new(mtu);
4572|    loop {
4573|        let frame = match read_frame_opt(&mut read_half).await? {
4574|            Some(f) => f,
4575|            None => break,
4576|        };
4577|        let decoded = match codec.decode(&frame) {
4578|            Ok(d) => d,
4579|            Err(e) => {
4580|                warn!(%client_id, error = %e, "framing error");
4581|                continue;
4582|            }
4583|        };
4584|        let env = match decoded {
4585|            Decoded::Incomplete => continue,
4586|            Decoded::Envelope(e) => *e,
4587|        };
4588|        if let Err(e) = env.validate_direction(TransportDirection::Runtime) {
4589|            debug!(%client_id, error = %e, "direction violation");
4590|            continue;
4591|        }
4592|        match sink.handle_client_envelope(client_id, env).await {
4593|            Ok(Some(reply)) => { send_envelope(&codec, &tx, &reply).await?; }
4594|            Ok(None) => {}
4595|            Err(e) => {
4596|                let err_env = Envelope {
4597|                    v: PROTOCOL_VERSION,
4598|                    id: EnvelopeId::new(),
4599|                    kind: MessageType::Error,
4600|                    ts: time::OffsetDateTime::now_utc(),
4601|                    src: Party::Runtime,
4602|                    dst: Party::Client,
4603|                    reply_to: None,
4604|                    cap: None,
4605|                    module: None,
4606|                    enc: dnea_core::Encoding::Identity,
4607|                    ttl_ms: None,
4608|                    payload: None,
4609|                    err: Some(dnea_core::EnvelopeError {
4610|                        code: e.code().to_string(),
4611|                        message: e.to_string(),
4612|                        retryable: e.retryable(),
4613|                        retry_after_ms: None,
4614|                        details: None,
4615|                        cause: None,
4616|                    }),
4617|                    meta: Default::default(),
4618|                };
4619|                let _ = send_envelope(&codec, &tx, &err_env).await;
4620|            }
4621|        }
4622|    }
4623|
4624|    drop(tx);
4625|    let _ = writer.await;
4626|    sink.on_client_disconnected(client_id).await;
4627|    Ok(())
4628|}
4629|
4630|async fn send_envelope(
4631|    codec: &Codec,
4632|    tx: &mpsc::Sender<Vec<u8>>,
4633|    env: &Envelope,
4634|) -> DneaResult<()> {
4635|    for frame in codec.encode(env)? {
4636|        tx.send(frame).await.map_err(|_| DneaError::Transport)?;
4637|    }
4638|    Ok(())
4639|}
4640|
4641|async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Vec<u8>> {
4642|    read_frame_opt(r).await?
4643|        .ok_or_else(|| DneaError::Framing(dnea_core::FramingError::MissingEnvelope))
4644|}
4645|
4646|async fn read_frame_opt<R: AsyncReadExt + Unpin>(r: &mut R) -> DneaResult<Option<Vec<u8>>> {
4647|    let mut len_buf = [0u8; 4];
4648|    match r.read_exact(&mut len_buf).await {
4649|        Ok(_) => {}
4650|        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
4651|        Err(e) => return Err(DneaError::Internal(format!("read length: {e}"))),
4652|    }
4653|    let len = u32::from_ne_bytes(len_buf) as usize;
4654|    if len == 0 || len > MAX_FRAME_BYTES {
4655|        return Err(DneaError::Framing(dnea_core::FramingError::TooLarge {
4656|            actual: len, limit: MAX_FRAME_BYTES,
4657|        }));
4658|    }
4659|    let mut buf = vec![0u8; len];
4660|    r.read_exact(&mut buf).await.map_err(|e| DneaError::Internal(format!("read body: {e}")))?;
4661|    Ok(Some(buf))
4662|}
4663|
4664|async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, bytes: &[u8]) -> DneaResult<()> {
4665|    if bytes.is_empty() {
4666|        return Err(DneaError::Internal("refusing to write empty frame".into()));
4667|    }
4668|    let len = u32::try_from(bytes.len())
4669|        .map_err(|_| DneaError::Internal("frame too large for u32 prefix".into()))?;
4670|    w.write_all(&len.to_ne_bytes()).await.map_err(|_| DneaError::Transport)?;
4671|    w.write_all(bytes).await.map_err(|_| DneaError::Transport)?;
4672|    w.flush().await.map_err(|_| DneaError::Transport)?;
4673|    Ok(())
4674|}
4675|```
4676|
4677|Add `mod supervisor_windows;` guarded by `#[cfg(windows)]` in `lib.rs`:
4678|
4679|```rust
4680|#[cfg(windows)]
4681|mod supervisor_windows;
4682|```
4683|
4684|### `crates/dnea-nm-host/src/bin/bridge.rs`
4685|
4686|```rust
4687|//! DNEA Native Messaging bridge.
4688|
4689|use std::{
4690|    env,
4691|    io::{self, Read, Write},
4692|    process::ExitCode,
4693|};
4694|
4695|use anyhow::{bail, Context, Result};
4696|
4697|const MAX_FRAME: usize = 64 * 1024 * 1024;
4698|
4699|#[cfg(unix)]
4700|mod transport {
4701|    use super::*;
4702|    use std::os::unix::net::UnixStream;
4703|    use std::path::PathBuf;
4704|
4705|    pub fn connect() -> Result<UnixStream> {
4706|        let path = socket_path()?;
4707|        UnixStream::connect(&path)
4708|            .with_context(|| format!("connect to runtime at {}", path.display()))
4709|    }
4710|
4711|    fn socket_path() -> Result<PathBuf> {
4712|        if let Ok(p) = env::var("DNEA_SOCKET") {
4713|            return Ok(PathBuf::from(p));
4714|        }
4715|        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
4716|            .context("no home directory for runtime dir")?;
4717|        let dir = proj.runtime_dir().context("no runtime dir")?;
4718|        Ok(dir.join("nm.sock"))
4719|    }
4720|}
4721|
4722|#[cfg(windows)]
4723|mod transport {
4724|    use super::*;
4725|    use std::fs::OpenOptions;
4726|    use std::path::PathBuf;
4727|
4728|    pub fn connect() -> Result<std::fs::File> {
4729|        let path = pipe_path()?;
4730|        OpenOptions::new()
4731|            .read(true)
4732|            .write(true)
4733|            .open(&path)
4734|            .with_context(|| format!("open named pipe {}", path.display()))
4735|    }
4736|
4737|    fn pipe_path() -> Result<PathBuf> {
4738|        if let Ok(p) = env::var("DNEA_PIPE") {
4739|            return Ok(PathBuf::from(p));
4740|        }
4741|        let user = env::var("USERNAME").unwrap_or_else(|_| "default".into());
4742|        Ok(PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}")))
4743|    }
4744|}
4745|
4746|fn main() -> ExitCode {
4747|    match run() {
4748|        Ok(()) => ExitCode::SUCCESS,
4749|        Err(e) => {
4750|            eprintln!("dnea-nm-bridge: {e:#}");
4751|            ExitCode::FAILURE
4752|        }
4753|    }
4754|}
4755|
4756|fn run() -> Result<()> {
4757|    let mut upstream = transport::connect()?;
4758|    let preamble = serde_json::json!({
4759|        "browser_family": detect_browser_family(),
4760|        "extension_id":   env::var("DNEA_EXTENSION_ID").ok(),
4761|        "pid":            std::process::id(),
4762|        "protocol":       "dnea-nm-v1",
4763|    });
4764|    let preamble_bytes = serde_json::to_vec(&preamble)?;
4765|    write_frame(&mut upstream, &preamble_bytes)?;
4766|
4767|    let stdin = io::stdin();
4768|    let stdout = io::stdout();
4769|    let mut upstream_r = upstream.try_clone()?;
4770|    let mut upstream_w = upstream;
4771|
4772|    let in_thread = std::thread::Builder::new()
4773|        .name("browser->runtime".into())
4774|        .spawn(move || pump_framed(stdin.lock(), upstream_w))?;
4775|    let out_result = pump_framed(upstream_r, stdout.lock());
4776|    let _ = in_thread.join();
4777|    out_result
4778|}
4779|
4780|fn pump_framed<R, W>(mut r: R, mut w: W) -> Result<()>
4781|where R: Read, W: Write {
4782|    loop {
4783|        match read_frame(&mut r)? {
4784|            None => return Ok(()),
4785|            Some(bytes) => write_frame(&mut w, &bytes)?,
4786|        }
4787|    }
4788|}
4789|
4790|fn read_frame<R: Read>(r: &mut R) -> Result<Option<Vec<u8>>> {
4791|    let mut len_buf = [0u8; 4];
4792|    match r.read_exact(&mut len_buf) {
4793|        Ok(()) => {}
4794|        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
4795|        Err(e) => return Err(e).context("read length prefix"),
4796|    }
4797|    let len = u32::from_ne_bytes(len_buf) as usize;
4798|    if len == 0 { bail!("zero-length frame"); }
4799|    if len > MAX_FRAME { bail!("frame size {len} exceeds maximum {MAX_FRAME}"); }
4800|    let mut buf = vec![0u8; len];
4801|    r.read_exact(&mut buf).context("read frame body")?;
4802|    Ok(Some(buf))
4803|}
4804|
4805|fn write_frame<W: Write>(w: &mut W, bytes: &[u8]) -> Result<()> {
4806|    if bytes.is_empty() { bail!("refusing to write empty frame"); }
4807|    let len = u32::try_from(bytes.len()).context("frame too large for u32 prefix")?;
4808|    w.write_all(&len.to_ne_bytes())?;
4809|    w.write_all(bytes)?;
4810|    w.flush()?;
4811|    Ok(())
4812|}
4813|
4814|fn detect_browser_family() -> &'static str {
4815|    if env::var_os("MOZ_CRASHREPORTER").is_some() { return "firefox"; }
4816|    if let Ok(v) = env::var("DNEA_BROWSER_FAMILY") {
4817|        return match v.as_str() {
4818|            "chrome" => "chromium", "edge" => "chromium",
4819|            "brave" => "chromium", "vivaldi" => "chromium",
4820|            "opera" => "chromium", "safari" => "safari",
4821|            "firefox" => "firefox", _ => "unknown",
4822|        };
4823|    }
4824|    "unknown"
4825|}
4826|```
4827|
4828|### `crates/dnea-nm-host/src/registration.rs`
4829|
4830|```rust
4831|//! Per-browser Native Messaging host manifest writers.
4832|
4833|use std::fs;
4834|use std::path::{Path, PathBuf};
4835|
4836|use anyhow::{bail, Context, Result};
4837|use serde::Serialize;
4838|
4839|#[derive(Clone, Copy, Debug, PartialEq, Eq)]
4840|pub enum Browser { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }
4841|
4842|impl Browser {
4843|    #[must_use]
4844|    pub fn all() -> &'static [Browser] {
4845|        &[Browser::Chrome, Browser::Chromium, Browser::Edge,
4846|          Browser::Brave, Browser::Vivaldi, Browser::Opera, Browser::Firefox]
4847|    }
4848|
4849|    #[must_use]
4850|    pub fn host_name(self) -> &'static str { "org.dnea.runtime" }
4851|
4852|    #[must_use]
4853|    pub fn allowed_origin(self, extension_id: &str) -> String {
4854|        match self {
4855|            Browser::Firefox => format!("moz-extension://{extension_id}/"),
4856|            _ => format!("chrome-extension://{extension_id}/"),
4857|        }
4858|    }
4859|
4860|    #[must_use]
4861|    pub fn manifest_dirs(self) -> Vec<PathBuf> {
4862|        #[cfg(target_os = "linux")]
4863|        {
4864|            match self {
4865|                Browser::Chrome => vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
4866|                Browser::Chromium => vec![PathBuf::from("/etc/chromium/native-messaging-hosts")],
4867|                Browser::Edge => vec![PathBuf::from("/etc/opt/edge/native-messaging-hosts")],
4868|                Browser::Firefox => vec![PathBuf::from("/usr/lib/mozilla/native-messaging-hosts")],
4869|                Browser::Brave | Browser::Vivaldi | Browser::Opera =>
4870|                    vec![PathBuf::from("/etc/opt/chrome/native-messaging-hosts")],
4871|            }
4872|        }
4873|        #[cfg(target_os = "macos")]
4874|        {
4875|            match self {
4876|                Browser::Chrome | Browser::Chromium | Browser::Brave
4877|                | Browser::Vivaldi | Browser::Opera =>
4878|                    vec![PathBuf::from("/Library/Google/Chrome/NativeMessagingHosts")],
4879|                Browser::Edge => vec![PathBuf::from("/Library/Microsoft/Edge/NativeMessagingHosts")],
4880|                Browser::Firefox => vec![PathBuf::from(
4881|                    "/Library/Application Support/Mozilla/NativeMessagingHosts")],
4882|            }
4883|        }
4884|        #[cfg(windows)]
4885|        {
4886|            let _ = self;
4887|            vec![PathBuf::from(r"C:\ProgramData\DNEA\nm")]
4888|        }
4889|    }
4890|}
4891|
4892|#[derive(Serialize)]
4893|struct HostManifest {
4894|    name: String,
4895|    description: String,
4896|    path: PathBuf,
4897|    #[serde(rename = "type")]
4898|    kind: &'static str,
4899|    allowed_origins: Vec<String>,
4900|    #[serde(rename = "x-dnea-installed")]
4901|    marker: XDneaInstalled,
4902|}
4903|
4904|#[derive(Serialize)]
4905|struct XDneaInstalled {
4906|    version: String,
4907|    installed_at: String,
4908|    content_digest: String,
4909|}
4910|
4911|pub fn register(
4912|    browser: Browser,
4913|    extension_ids: &[&str],
4914|    bridge_path: &Path,
4915|    runtime_version: &str,
4916|) -> Result<Vec<PathBuf>> {
4917|    if extension_ids.is_empty() {
4918|        bail!("at least one extension ID is required");
4919|    }
4920|    let allowed: Vec<String> = extension_ids.iter()
4921|        .map(|id| browser.allowed_origin(id))
4922|        .collect();
4923|    let mut manifest = HostManifest {
4924|        name: browser.host_name().into(),
4925|        description: "DNEA runtime bridge".into(),
4926|        path: bridge_path.to_path_buf(),
4927|        kind: "stdio",
4928|        allowed_origins: allowed,
4929|        marker: XDneaInstalled {
4930|            version: runtime_version.into(),
4931|            installed_at: time::OffsetDateTime::now_utc()
4932|                .format(&time::format_description::well_known::Rfc3339)
4933|                .unwrap_or_else(|_| "unknown".into()),
4934|            content_digest: String::new(),
4935|        },
4936|    };
4937|
4938|    let pre_digest = serde_json::to_vec(&serde_json::json!({
4939|        "name": manifest.name,
4940|        "description": manifest.description,
4941|        "path": manifest.path,
4942|        "type": manifest.kind,
4943|        "allowed_origins": manifest.allowed_origins,
4944|        "x-dnea-installed": {
4945|            "version": manifest.marker.version,
4946|            "installed_at": manifest.marker.installed_at,
4947|        }
4948|    }))?;
4949|    use sha2::{Digest, Sha256};
4950|    let mut h = Sha256::new();
4951|    h.update(&pre_digest);
4952|    manifest.marker.content_digest = format!(
4953|        "sha-256:{}",
4954|        base64::Engine::encode(
4955|            &base64::engine::general_purpose::URL_SAFE_NO_PAD, h.finalize()),
4956|    );
4957|
4958|    let bytes = serde_json::to_vec_pretty(&manifest)?;
4959|    let mut written = Vec::new();
4960|    for dir in browser.manifest_dirs() {
4961|        fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
4962|        let path = dir.join(format!("{}.json", browser.host_name()));
4963|        if path.exists() {
4964|            let existing = fs::read_to_string(&path).unwrap_or_default();
4965|            if !existing.contains("x-dnea-installed") {
4966|                let backup = path.with_extension(format!(
4967|                    "json.bak-{}",
4968|                    time::OffsetDateTime::now_utc()
4969|                        .format(&time::format_description::well_known::Rfc3339)
4970|                        .unwrap_or_else(|_| "unknown".into())
4971|                        .replace(':', "-"),
4972|                ));
4973|                fs::rename(&path, &backup)?;
4974|            }
4975|        }
4976|        fs::write(&path, &bytes)?;
4977|        written.push(path);
4978|    }
4979|
4980|    #[cfg(windows)]
4981|    {
4982|        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
4983|        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
4984|        let key_path = browser.registry_key();
4985|        let (key, _) = hklm
4986|            .create_subkey(key_path)
4987|            .with_context(|| format!("create registry key {key_path}"))?;
4988|        let manifest_path = written
4989|            .first()
4990|            .ok_or_else(|| anyhow::anyhow!("no manifest written"))?;
4991|        key.set_value(browser.host_name(), &manifest_path.to_string_lossy().to_string())?;
4992|    }
4993|
4994|    Ok(written)
4995|}
4996|
4997|#[cfg(windows)]
4998|impl Browser {
4999|    fn registry_key(self) -> &'static str {
5000|        match self {
5001|            Browser::Chrome | Browser::Chromium | Browser::Brave
5002|            | Browser::Vivaldi | Browser::Opera =>
5003|                r"SOFTWARE\Google\Chrome\NativeMessagingHosts",
5004|            Browser::Edge => r"SOFTWARE\Microsoft\Edge\NativeMessagingHosts",
5005|            Browser::Firefox => r"SOFTWARE\Mozilla\NativeMessagingHosts",
5006|        }
5007|    }
5008|}
5009|
5010|use base64::Engine as _;
5011|```
5012|
5013|### `crates/dnea-nm-host/tests/mock_browser.rs`
5014|
5015|```rust
5016|//! End-to-end test against the real bridge binary and a real supervisor.
5017|
5018|use std::{path::PathBuf, sync::Arc, time::Duration};
5019|
5020|use async_trait::async_trait;
5021|use dnea_core::{
5022|    envelope::{Envelope, MessageType, Party, PROTOCOL_VERSION},
5023|    DneaResult, SessionId,
5024|};
5025|use dnea_nm_host::{
5026|    session::{ClientId, ClientInfo, SessionSink},
5027|    supervisor::{NmHostConfig, NmSupervisor},
5028|};
5029|use serde_json::json;
5030|use tempfile::TempDir;
5031|use tokio::{
5032|    io::{AsyncReadExt, AsyncWriteExt},
5033|    process::{Child, Command},
5034|};
5035|
5036|struct EchoSink { session_id: SessionId }
5037|
5038|impl EchoSink {
5039|    fn new() -> Self { Self { session_id: SessionId::new() } }
5040|}
5041|
5042|#[async_trait]
5043|impl SessionSink for EchoSink {
5044|    async fn on_client_connected(&self, _: ClientId, _: ClientInfo) -> DneaResult<SessionId> {
5045|        Ok(self.session_id)
5046|    }
5047|    async fn on_client_disconnected(&self, _: ClientId) {}
5048|    async fn handle_client_envelope(
5049|        &self, _: ClientId, env: Envelope,
5050|    ) -> DneaResult<Option<Envelope>> {
5051|        match env.kind {
5052|            MessageType::Hello => {
5053|                let mut w = env.reply(MessageType::Welcome);
5054|                w.payload = Some(json!({
5055|                    "runtimeVersion": "test",
5056|                    "protocolVersion": PROTOCOL_VERSION,
5057|                    "sessionId": self.session_id.to_string(),
5058|                    "mtu": 1_048_576,
5059|                    "encodings": ["identity"],
5060|                    "features": { "chunking": true },
5061|                }));
5062|                let ext_id = env.payload.as_ref()
5063|                    .and_then(|p| p.get("extensionId"))
5064|                    .and_then(|v| v.as_str())
5065|                    .unwrap_or("")
5066|                    .to_string();
5067|                w.meta.insert("client_extension_id".into(), json!(ext_id));
5068|                Ok(Some(w))
5069|            }
5070|            _ => {
5071|                let mut r = env.reply(MessageType::Result);
5072|                r.payload = env.payload;
5073|                Ok(Some(r))
5074|            }
5075|        }
5076|    }
5077|}
5078|
5079|struct MockBrowser {
5080|    child: Child,
5081|    stdin: tokio::process::ChildStdin,
5082|    stdout: tokio::process::ChildStdout,
5083|}
5084|
5085|impl MockBrowser {
5086|    async fn spawn(bridge_path: &PathBuf, socket: &PathBuf) -> std::io::Result<Self> {
5087|        let mut child = Command::new(bridge_path)
5088|            .env("DNEA_SOCKET", socket)
5089|            .env("DNEA_BROWSER_FAMILY", "chrome")
5090|            .env("DNEA_EXTENSION_ID", "abcdefghijklmnopabcdefghijklmnop")
5091|            .stdin(std::process::Stdio::piped())
5092|            .stdout(std::process::Stdio::piped())
5093|            .stderr(std::process::Stdio::piped())
5094|            .kill_on_drop(true)
5095|            .spawn()?;
5096|        let stdin = child.stdin.take().expect("piped stdin");
5097|        let stdout = child.stdout.take().expect("piped stdout");
5098|        Ok(Self { child, stdin, stdout })
5099|    }
5100|
5101|    async fn send(&mut self, env: &Envelope) -> std::io::Result<()> {
5102|        let bytes = serde_json::to_vec(env).unwrap();
5103|        let len = u32::try_from(bytes.len()).unwrap();
5104|        self.stdin.write_all(&len.to_ne_bytes()).await?;
5105|        self.stdin.write_all(&bytes).await?;
5106|        self.stdin.flush().await?;
5107|        Ok(())
5108|    }
5109|
5110|    async fn recv(&mut self) -> std::io::Result<Envelope> {
5111|        let mut len_buf = [0u8; 4];
5112|        self.stdout.read_exact(&mut len_buf).await?;
5113|        let len = u32::from_ne_bytes(len_buf) as usize;
5114|        let mut buf = vec![0u8; len];
5115|        self.stdout.read_exact(&mut buf).await?;
5116|        let env: Envelope = serde_json::from_slice(&buf).unwrap();
5117|        Ok(env)
5118|    }
5119|
5120|    async fn kill(&mut self) { let _ = self.child.kill().await; }
5121|}
5122|
5123|fn bridge_binary() -> PathBuf {
5124|    PathBuf::from(env!("CARGO_BIN_EXE_dnea-nm-bridge"))
5125|}
5126|
5127|#[tokio::test(flavor = "multi_thread")]
5128|async fn t01_hello_welcome() {
5129|    let tmp = TempDir::new().unwrap();
5130|    let socket = tmp.path().join("nm.sock");
5131|
5132|    let sink = Arc::new(EchoSink::new());
5133|    let config = NmHostConfig::new(socket.clone());
5134|    let supervisor = NmSupervisor::new(config, sink.clone());
5135|    let sup_task = tokio::spawn(supervisor.run());
5136|    tokio::time::sleep(Duration::from_millis(100)).await;
5137|
5138|    let bridge = bridge_binary();
5139|    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();
5140|
5141|    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
5142|    hello.payload = Some(json!({
5143|        "clientVersion": "1.0.0",
5144|        "browserFamily": "chromium",
5145|        "browserVersion": "test",
5146|        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
5147|        "mtu": 1_048_576,
5148|        "encodings": ["identity"],
5149|        "features": { "chunking": true },
5150|    }));
5151|    browser.send(&hello).await.unwrap();
5152|
5153|    let welcome = tokio::time::timeout(Duration::from_secs(5), browser.recv())
5154|        .await.expect("welcome within 5s").unwrap();
5155|    assert_eq!(welcome.kind, MessageType::Welcome);
5156|    assert_eq!(welcome.reply_to, Some(hello.id));
5157|    assert_eq!(
5158|        welcome.meta.get("client_extension_id").and_then(|v| v.as_str()),
5159|        Some("abcdefghijklmnopabcdefghijklmnop"),
5160|    );
5161|
5162|    browser.kill().await;
5163|    sup_task.abort();
5164|}
5165|
5166|#[tokio::test(flavor = "multi_thread")]
5167|async fn t12_reconnect_after_restart() {
5168|    let tmp = TempDir::new().unwrap();
5169|    let socket = tmp.path().join("nm.sock");
5170|
5171|    let sink = Arc::new(EchoSink::new());
5172|    let config = NmHostConfig::new(socket.clone());
5173|    let supervisor = NmSupervisor::new(config.clone(), sink.clone());
5174|    let sup_task = tokio::spawn(supervisor.run());
5175|    tokio::time::sleep(Duration::from_millis(100)).await;
5176|
5177|    let bridge = bridge_binary();
5178|    let mut browser = MockBrowser::spawn(&bridge, &socket).await.unwrap();
5179|    let mut hello = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
5180|    hello.payload = Some(json!({
5181|        "clientVersion": "1.0.0",
5182|        "browserFamily": "chromium",
5183|        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
5184|        "mtu": 1_048_576,
5185|    }));
5186|    browser.send(&hello).await.unwrap();
5187|    let _welcome = browser.recv().await.unwrap();
5188|
5189|    sup_task.abort();
5190|    tokio::time::sleep(Duration::from_millis(100)).await;
5191|    browser.kill().await;
5192|    let _ = browser.child.wait().await;
5193|
5194|    let supervisor2 = NmSupervisor::new(config, sink.clone());
5195|    let sup_task2 = tokio::spawn(supervisor2.run());
5196|    tokio::time::sleep(Duration::from_millis(100)).await;
5197|
5198|    let mut browser2 = MockBrowser::spawn(&bridge, &socket).await.unwrap();
5199|    let mut hello2 = Envelope::new(MessageType::Hello, Party::Client, Party::Runtime);
5200|    hello2.payload = Some(json!({
5201|        "clientVersion": "1.0.0",
5202|        "browserFamily": "chromium",
5203|        "extensionId": "abcdefghijklmnopabcdefghijklmnop",
5204|        "mtu": 1_048_576,
5205|    }));
5206|    browser2.send(&hello2).await.unwrap();
5207|    let welcome2 = tokio::time::timeout(Duration::from_secs(5), browser2.recv())
5208|        .await.expect("reconnect welcome").unwrap();
5209|    assert_eq!(welcome2.kind, MessageType::Welcome);
5210|    assert_eq!(welcome2.reply_to, Some(hello2.id));
5211|
5212|    browser2.kill().await;
5213|    sup_task2.abort();
5214|}
5215|```
5216|
5217|## 3.7 `crates/dnea-session`
5218|
5219|### `crates/dnea-session/Cargo.toml`
5220|
5221|```toml
5222|[package]
5223|name        = "dnea-session"
5224|version.workspace = true
5225|description = "The DNEA runtime hub."
5226|edition.workspace     = true
5227|rust-version.workspace = true
5228|license.workspace     = true
5229|
5230|[dependencies]
5231|dnea-core        = { path = "../dnea-core" }
5232|dnea-manifest    = { path = "../dnea-manifest" }
5233|dnea-module-host = { path = "../dnea-module-host" }
5234|dnea-nm-host     = { path = "../dnea-nm-host" }
5235|anyhow           = { workspace = true }
5236|async-trait      = { workspace = true }
5237|base64           = { workspace = true }
5238|ed25519-dalek    = { workspace = true }
5239|parking_lot      = { workspace = true }
5240|serde            = { workspace = true }
5241|serde_json       = { workspace = true }
5242|sha2             = { workspace = true }
5243|thiserror        = { workspace = true }
5244|tokio            = { workspace = true }
5245|tracing          = { workspace = true }
5246|uuid             = { workspace = true }
5247|
5248|[dev-dependencies]
5249|rand          = { workspace = true }
5250|tempfile      = { workspace = true }
5251|wasmtime      = { workspace = true }
5252|```
5253|
5254|### `crates/dnea-session/src/lib.rs`
5255|
5256|```rust
5257|//! The DNEA runtime hub.
5258|
5259|#![forbid(unsafe_code)]
5260|#![deny(rust_2018_idioms, unreachable_pub)]
5261|#![warn(missing_docs)]
5262|
5263|mod consent;
5264|mod grants;
5265|mod hub;
5266|pub mod install;
5267|mod registry;
5268|
5269|pub use consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor};
5270|pub use grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant};
5271|pub use hub::{Session, SessionError, SessionEvent, SessionHandle};
5272|pub use install::{
5273|    HostStateFactory, InstallError, InstallPipeline, InstallReport, InstallSource, InstallStatus,
5274|    TranscriptStep,
5275|};
5276|pub use registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry};
5277|```
5278|
5279|### `crates/dnea-session/src/registry.rs`
5280|
5281|```rust
5282|//! Client and module registries.
5283|
5284|use std::collections::HashMap;
5285|use std::sync::Arc;
5286|
5287|use dnea_core::{ids::ModuleId, SessionId};
5288|use dnea_manifest::Manifest;
5289|use dnea_module_host::InstanceHandle;
5290|use dnea_nm_host::session::ClientId;
5291|use parking_lot::RwLock;
5292|use tokio::sync::mpsc;
5293|
5294|pub struct ClientRecord {
5295|    pub id: ClientId,
5296|    pub browser_family: String,
5297|    pub extension_id: Option<String>,
5298|    pub bridge_pid: Option<u32>,
5299|    pub connected_at: std::time::SystemTime,
5300|    pub tx: mpsc::Sender<Vec<u8>>,
5301|}
5302|
5303|pub struct ModuleRecord {
5304|    pub id: ModuleId,
5305|    pub version: String,
5306|    pub handle: Arc<InstanceHandle>,
5307|    pub manifest: Option<Arc<Manifest>>,
5308|    pub loaded_at: std::time::SystemTime,
5309|}
5310|
5311|impl ModuleRecord {
5312|    #[must_use]
5313|    pub fn manifest(&self) -> Option<&Manifest> {
5314|        self.manifest.as_deref()
5315|    }
5316|}
5317|
5318|#[derive(Default)]
5319|pub struct ClientRegistry {
5320|    inner: RwLock<HashMap<ClientId, Arc<ClientRecord>>>,
5321|    session_id: RwLock<Option<SessionId>>,
5322|}
5323|
5324|impl ClientRegistry {
5325|    pub fn insert(&self, rec: Arc<ClientRecord>) {
5326|        self.inner.write().insert(rec.id, rec);
5327|    }
5328|
5329|    pub fn remove(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
5330|        self.inner.write().remove(&id)
5331|    }
5332|
5333|    pub fn get(&self, id: ClientId) -> Option<Arc<ClientRecord>> {
5334|        self.inner.read().get(&id).cloned()
5335|    }
5336|
5337|    pub fn all(&self) -> Vec<Arc<ClientRecord>> {
5338|        self.inner.read().values().cloned().collect()
5339|    }
5340|
5341|    pub fn len(&self) -> usize { self.inner.read().len() }
5342|
5343|    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }
5344|
5345|    pub fn session_id(&self) -> SessionId {
5346|        let mut g = self.session_id.write();
5347|        *g.get_or_insert_with(SessionId::new)
5348|    }
5349|}
5350|
5351|#[derive(Default)]
5352|pub struct ModuleRegistry {
5353|    inner: RwLock<HashMap<ModuleId, Arc<ModuleRecord>>>,
5354|}
5355|
5356|impl ModuleRegistry {
5357|    pub fn insert(&self, rec: Arc<ModuleRecord>) {
5358|        self.inner.write().insert(rec.id.clone(), rec);
5359|    }
5360|
5361|    pub fn remove(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
5362|        self.inner.write().remove(id)
5363|    }
5364|
5365|    pub fn get(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> {
5366|        self.inner.read().get(id).cloned()
5367|    }
5368|
5369|    pub fn all(&self) -> Vec<Arc<ModuleRecord>> {
5370|        self.inner.read().values().cloned().collect()
5371|    }
5372|
5373|    pub fn len(&self) -> usize { self.inner.read().len() }
5374|
5375|    pub fn is_empty(&self) -> bool { self.inner.read().is_empty() }
5376|}
5377|```
5378|
5379|### `crates/dnea-session/src/grants.rs`
5380|
5381|```rust
5382|//! The grant store.
5383|
5384|use std::collections::HashMap;
5385|use std::time::SystemTime;
5386|
5387|use dnea_core::ids::ModuleId;
5388|use dnea_module_host::grant::{EffectiveGrant, GrantSet, GrantSource};
5389|use parking_lot::RwLock;
5390|use serde::{Deserialize, Serialize};
5391|
5392|#[derive(Clone, Debug, Serialize, Deserialize)]
5393|pub struct GrantUpdate {
5394|    pub module: String,
5395|    pub capability: String,
5396|    pub change: GrantChange,
5397|    pub effective_constraints: serde_json::Value,
5398|}
5399|
5400|#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
5401|#[serde(rename_all = "snake_case")]
5402|pub enum GrantChange { Granted, Revoked, Widened, Narrowed }
5403|
5404|#[derive(Clone, Debug, Serialize, Deserialize)]
5405|pub struct OriginGrant {
5406|    pub module: String,
5407|    pub capability: String,
5408|    pub origin: String,
5409|    pub granted_at: SystemTime,
5410|    pub expires_at: Option<SystemTime>,
5411|}
5412|
5413|#[derive(Default)]
5414|pub struct GrantStore {
5415|    inner: RwLock<HashMap<ModuleId, GrantSet>>,
5416|    origins: RwLock<HashMap<(String, String, String), OriginGrant>>,
5417|}
5418|
5419|impl GrantStore {
5420|    pub fn set(&self, module: ModuleId, grants: GrantSet) {
5421|        self.inner.write().insert(module, grants);
5422|    }
5423|
5424|    pub fn get(&self, module: &ModuleId) -> Option<GrantSet> {
5425|        self.inner.read().get(module).cloned()
5426|    }
5427|
5428|    pub fn grant(
5429|        &self,
5430|        module: &ModuleId,
5431|        capability: &str,
5432|        constraints: serde_json::Value,
5433|        source: GrantSource,
5434|    ) -> Option<GrantUpdate> {
5435|        let mut guard = self.inner.write();
5436|        let set = guard.entry(module.clone()).or_default();
5437|        let existed = set.get(capability).is_some();
5438|        set.insert(EffectiveGrant {
5439|            id: capability.to_string(),
5440|            constraints: constraints.clone(),
5441|            source,
5442|        });
5443|        if existed { None } else {
5444|            Some(GrantUpdate {
5445|                module: module.to_string(),
5446|                capability: capability.to_string(),
5447|                change: GrantChange::Granted,
5448|                effective_constraints: constraints,
5449|            })
5450|        }
5451|    }
5452|
5453|    pub fn revoke(&self, module: &ModuleId, capability: &str) -> Option<GrantUpdate> {
5454|        let mut guard = self.inner.write();
5455|        let set = guard.get_mut(module)?;
5456|        if set.get(capability).is_none() { return None; }
5457|        let mut new_set = GrantSet::new();
5458|        for (id, g) in set.iter() {
5459|            if id != capability { new_set.insert(g.clone()); }
5460|        }
5461|        *set = new_set;
5462|        Some(GrantUpdate {
5463|            module: module.to_string(),
5464|            capability: capability.to_string(),
5465|            change: GrantChange::Revoked,
5466|            effective_constraints: serde_json::Value::Null,
5467|        })
5468|    }
5469|
5470|    pub fn modules(&self) -> Vec<ModuleId> {
5471|        self.inner.read().keys().cloned().collect()
5472|    }
5473|
5474|    pub fn add_origin(
5475|        &self,
5476|        module: &ModuleId,
5477|        capability: &str,
5478|        origin: &str,
5479|        expires_at: Option<SystemTime>,
5480|    ) -> OriginGrant {
5481|        let rec = OriginGrant {
5482|            module: module.to_string(),
5483|            capability: capability.to_string(),
5484|            origin: origin.to_string(),
5485|            granted_at: SystemTime::now(),
5486|            expires_at,
5487|        };
5488|        self.origins.write().insert(
5489|            (module.to_string(), capability.to_string(), origin.to_string()),
5490|            rec.clone(),
5491|        );
5492|        rec
5493|    }
5494|
5495|    pub fn remove_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
5496|        self.origins
5497|            .write()
5498|            .remove(&(module.to_string(), capability.to_string(), origin.to_string()))
5499|            .is_some()
5500|    }
5501|
5502|    pub fn origins_for(&self, module: &ModuleId, capability: &str) -> Vec<OriginGrant> {
5503|        self.origins
5504|            .read()
5505|            .iter()
5506|            .filter(|((m, c, _), _)| m == module.as_str() && c == capability)
5507|            .map(|(_, v)| v.clone())
5508|            .collect()
5509|    }
5510|
5511|    #[must_use]
5512|    pub fn has_origin(&self, module: &ModuleId, capability: &str, origin: &str) -> bool {
5513|        let key = (module.to_string(), capability.to_string(), origin.to_string());
5514|        let guard = self.origins.read();
5515|        match guard.get(&key) {
5516|            Some(g) => g.expires_at.map(|e| e > SystemTime::now()).unwrap_or(true),
5517|            None => false,
5518|        }
5519|    }
5520|}
5521|```
5522|
5523|### `crates/dnea-session/src/consent.rs`
5524|
5525|```rust
5526|//! Consent state.
5527|
5528|use std::collections::HashMap;
5529|use std::time::{Duration, SystemTime};
5530|
5531|use parking_lot::RwLock;
5532|use serde::{Deserialize, Serialize};
5533|use tokio::sync::oneshot;
5534|use uuid::Uuid;
5535|
5536|#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
5537|#[serde(rename_all = "kebab-case")]
5538|pub enum RememberFor { Session, ModuleVersion, Always }
5539|
5540|#[derive(Clone, Debug, Serialize, Deserialize)]
5541|pub struct ConsentRequest {
5542|    pub prompt_id: Uuid,
5543|    pub module: String,
5544|    pub module_name: String,
5545|    pub capability: String,
5546|    pub tier: String,
5547|    pub reason: String,
5548|    pub constraints: serde_json::Value,
5549|    pub target_description: Option<String>,
5550|    pub expires_at: SystemTime,
5551|    pub can_remember: Vec<RememberFor>,
5552|}
5553|
5554|#[derive(Clone, Debug, Serialize, Deserialize)]
5555|pub struct ConsentDecision {
5556|    pub prompt_id: Uuid,
5557|    pub granted: bool,
5558|    #[serde(default)]
5559|    pub scope: Option<serde_json::Value>,
5560|    #[serde(default)]
5561|    pub remember_for: Option<RememberFor>,
5562|}
5563|
5564|#[derive(Default)]
5565|pub struct ConsentState {
5566|    open: RwLock<HashMap<Uuid, OpenPrompt>>,
5567|}
5568|
5569|struct OpenPrompt {
5570|    request: ConsentRequest,
5571|    created_at: SystemTime,
5572|    waiter: Option<oneshot::Sender<ConsentDecision>>,
5573|}
5574|
5575|impl ConsentState {
5576|    pub fn open(&self, request: ConsentRequest) -> oneshot::Receiver<ConsentDecision> {
5577|        let (tx, rx) = oneshot::channel();
5578|        let mut g = self.open.write();
5579|        g.insert(request.prompt_id, OpenPrompt {
5580|            request,
5581|            created_at: SystemTime::now(),
5582|            waiter: Some(tx),
5583|        });
5584|        rx
5585|    }
5586|
5587|    pub fn resolve(&self, decision: ConsentDecision) -> Option<ConsentRequest> {
5588|        let mut g = self.open.write();
5589|        let mut entry = g.remove(&decision.prompt_id)?;
5590|        if let Some(tx) = entry.waiter.take() {
5591|            let _ = tx.send(decision);
5592|        }
5593|        Some(entry.request)
5594|    }
5595|
5596|    pub fn list(&self) -> Vec<ConsentRequest> {
5597|        self.open.read().values().map(|p| p.request.clone()).collect()
5598|    }
5599|
5600|    pub fn get(&self, id: &Uuid) -> Option<ConsentRequest> {
5601|        self.open.read().get(id).map(|p| p.request.clone())
5602|    }
5603|
5604|    pub fn sweep(&self, ttl: Duration) {
5605|        let now = SystemTime::now();
5606|        let mut g = self.open.write();
5607|        let expired: Vec<Uuid> = g.iter()
5608|            .filter(|(_, p)| {
5609|                now.duration_since(p.created_at).map(|d| d > ttl).unwrap_or(false)
5610|            })
5611|            .map(|(id, _)| *id)
5612|            .collect();
5613|        for id in expired {
5614|            if let Some(mut p) = g.remove(&id) {
5615|                if let Some(tx) = p.waiter.take() {
5616|                    let _ = tx.send(ConsentDecision {
5617|                        prompt_id: id,
5618|                        granted: false,
5619|                        scope: None,
5620|                        remember_for: None,
5621|                    });
5622|                }
5623|            }
5624|        }
5625|    }
5626|}
5627|```
5628|
5629|### `crates/dnea-session/src/hub.rs`
5630|
5631|```rust
5632|//! The central hub.
5633|
5634|use std::sync::Arc;
5635|use std::time::Duration;
5636|
5637|use dnea_core::{
5638|    envelope::{Envelope, MessageType, Party},
5639|    ids::ModuleId,
5640|    DneaError, DneaResult, SessionId,
5641|};
5642|use dnea_module_host::grant::GrantSet;
5643|use dnea_nm_host::session::{ClientId, ClientInfo, SessionSink};
5644|use serde::{Deserialize, Serialize};
5645|use tokio::sync::{broadcast, mpsc};
5646|use tracing::{debug, info, warn};
5647|use uuid::Uuid;
5648|
5649|use crate::{
5650|    consent::{ConsentDecision, ConsentRequest, ConsentState, RememberFor},
5651|    grants::{GrantChange, GrantStore, GrantUpdate, OriginGrant},
5652|    registry::{ClientRecord, ClientRegistry, ModuleRecord, ModuleRegistry},
5653|};
5654|
5655|#[derive(Clone, Debug, Serialize, Deserialize)]
5656|#[serde(tag = "topic", content = "payload", rename_all = "snake_case")]
5657|pub enum SessionEvent {
5658|    ModuleLoaded { module: String, version: String },
5659|    ModuleUnloaded { module: String },
5660|    GrantChanged(GrantUpdate),
5661|    ConsentOpened(ConsentRequest),
5662|    ConsentResolved { prompt_id: Uuid, granted: bool },
5663|    ClientConnected { client_id: String, browser_family: String, extension_id: Option<String> },
5664|    ClientDisconnected { client_id: String },
5665|    OriginGranted { module: String, capability: String, origin: String },
5666|    OriginRevoked { module: String, capability: String, origin: String },
5667|}
5668|
5669|#[derive(Debug, thiserror::Error)]
5670|pub enum SessionError {
5671|    #[error("module not loaded: {0}")]
5672|    ModuleNotLoaded(String),
5673|    #[error("session shutting down")]
5674|    ShuttingDown,
5675|    #[error("internal: {0}")]
5676|    Internal(String),
5677|}
5678|
5679|#[derive(Clone)]
5680|pub struct Session { inner: Arc<SessionInner> }
5681|
5682|struct SessionInner {
5683|    clients: ClientRegistry,
5684|    modules: ModuleRegistry,
5685|    grants: GrantStore,
5686|    consent: ConsentState,
5687|    events: broadcast::Sender<SessionEvent>,
5688|}
5689|
5690|pub type SessionHandle = Session;
5691|
5692|impl Session {
5693|    #[must_use]
5694|    pub fn new() -> Self {
5695|        let (events, _) = broadcast::channel(1024);
5696|        Self {
5697|            inner: Arc::new(SessionInner {
5698|                clients: ClientRegistry::default(),
5699|                modules: ModuleRegistry::default(),
5700|                grants: GrantStore::default(),
5701|                consent: ConsentState::default(),
5702|                events,
5703|            }),
5704|        }
5705|    }
5706|
5707|    pub fn clients(&self) -> Vec<Arc<ClientRecord>> { self.inner.clients.all() }
5708|    pub fn modules(&self) -> Vec<Arc<ModuleRecord>> { self.inner.modules.all() }
5709|    pub fn module(&self, id: &ModuleId) -> Option<Arc<ModuleRecord>> { self.inner.modules.get(id) }
5710|    pub fn grants(&self, id: &ModuleId) -> Option<GrantSet> { self.inner.grants.get(id) }
5711|    pub fn grant_store(&self) -> &GrantStore { &self.inner.grants }
5712|    pub fn session_id(&self) -> SessionId { self.inner.clients.session_id() }
5713|    pub fn subscribe(&self) -> broadcast::Receiver<SessionEvent> { self.inner.events.subscribe() }
5714|    pub fn open_consents(&self) -> Vec<ConsentRequest> { self.inner.consent.list() }
5715|    pub fn consent(&self, id: &Uuid) -> Option<ConsentRequest> { self.inner.consent.get(id) }
5716|
5717|    pub fn set_grants(&self, module: &ModuleId, grants: Arc<GrantSet>) {
5718|        self.inner.grants.set(module.clone(), (*grants).clone());
5719|    }
5720|
5721|    pub fn grants_arc(&self, module: &ModuleId) -> Option<Arc<GrantSet>> {
5722|        self.inner.grants.get(module).map(Arc::new)
5723|    }
5724|
5725|    pub async fn request_consent(&self, request: ConsentRequest) -> DneaResult<ConsentDecision> {
5726|        let rx = self.inner.consent.open(request.clone());
5727|        self.broadcast(SessionEvent::ConsentOpened(request));
5728|        rx.await.map_err(|_| DneaError::Internal("consent waiter dropped".into()))
5729|    }
5730|
5731|    pub fn resolve_consent(&self, decision: ConsentDecision) -> DneaResult<()> {
5732|        let req = self.inner.consent.resolve(decision.clone())
5733|            .ok_or_else(|| DneaError::Internal(format!("no such prompt: {}", decision.prompt_id)))?;
5734|        self.broadcast(SessionEvent::ConsentResolved {
5735|            prompt_id: decision.prompt_id,
5736|            granted: decision.granted,
5737|        });
5738|        if decision.granted {
5739|            let module = ModuleId::parse(&req.module)
5740|                .map_err(|e| DneaError::Internal(e.to_string()))?;
5741|            if let Some(update) = self.inner.grants.grant(
5742|                &module, &req.capability,
5743|                decision.scope.unwrap_or(req.constraints),
5744|                dnea_module_host::grant::GrantSource::UserConsent,
5745|            ) {
5746|                self.broadcast(SessionEvent::GrantChanged(update));
5747|            }
5748|        }
5749|        Ok(())
5750|    }
5751|
5752|    pub fn register_module(&self, record: Arc<ModuleRecord>) {
5753|        let id = record.id.clone();
5754|        let version = record.version.clone();
5755|        if self.inner.grants.get(&id).is_none() {
5756|            self.inner.grants.set(id.clone(), GrantSet::new());
5757|        }
5758|        self.inner.modules.insert(record);
5759|        self.broadcast(SessionEvent::ModuleLoaded { module: id.to_string(), version });
5760|    }
5761|
5762|    pub fn unregister_module(&self, id: &ModuleId) {
5763|        if self.inner.modules.remove(id).is_some() {
5764|            self.broadcast(SessionEvent::ModuleUnloaded { module: id.to_string() });
5765|        }
5766|    }
5767|
5768|    pub async fn send_to_client_by_id(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
5769|        let rec = self.inner.clients.get(client_id).ok_or(DneaError::Transport)?;
5770|        let codec = dnea_core::Codec::default_mtu();
5771|        for f in codec.encode(&env)? {
5772|            rec.tx.send(f).await.map_err(|_| DneaError::Transport)?;
5773|        }
5774|        Ok(())
5775|    }
5776|
5777|    pub async fn broadcast_to_clients(&self, env: Envelope) -> DneaResult<()> {
5778|        let codec = dnea_core::Codec::default_mtu();
5779|        for rec in self.inner.clients.all() {
5780|            if let Ok(frames) = codec.encode(&env) {
5781|                for f in frames {
5782|                    let _ = rec.tx.send(f).await;
5783|                }
5784|            }
5785|        }
5786|        Ok(())
5787|    }
5788|
5789|    pub fn broadcast_origin_granted(&self, grant: &OriginGrant) {
5790|        self.broadcast(SessionEvent::OriginGranted {
5791|            module: grant.module.clone(),
5792|            capability: grant.capability.clone(),
5793|            origin: grant.origin.clone(),
5794|        });
5795|    }
5796|
5797|    pub fn broadcast_origin_revoked(&self, module: &str, capability: &str, origin: &str) {
5798|        self.broadcast(SessionEvent::OriginRevoked {
5799|            module: module.to_string(),
5800|            capability: capability.to_string(),
5801|            origin: origin.to_string(),
5802|        });
5803|    }
5804|
5805|    pub async fn sweep_consents(&self, ttl: Duration) {
5806|        self.inner.consent.sweep(ttl);
5807|    }
5808|
5809|    pub async fn shutdown(&self) {
5810|        let env = Envelope::new(MessageType::Goodbye, Party::Runtime, Party::Client);
5811|        self.broadcast_to_clients(env).await.ok();
5812|    }
5813|
5814|    pub fn attach_client(
5815|        &self,
5816|        client_id: ClientId,
5817|        info: ClientInfo,
5818|        tx: mpsc::Sender<Vec<u8>>,
5819|    ) {
5820|        self.inner.clients.insert(Arc::new(ClientRecord {
5821|            id: client_id,
5822|            browser_family: info.browser_family,
5823|            extension_id: info.extension_id,
5824|            bridge_pid: info.bridge_pid,
5825|            connected_at: info.connected_at,
5826|            tx,
5827|        }));
5828|    }
5829|
5830|    fn broadcast(&self, event: SessionEvent) {
5831|        let _ = self.inner.events.send(event);
5832|    }
5833|}
5834|
5835|impl Default for Session {
5836|    fn default() -> Self { Self::new() }
5837|}
5838|
5839|#[async_trait::async_trait]
5840|impl SessionSink for Session {
5841|    async fn on_client_connected(
5842|        &self,
5843|        client_id: ClientId,
5844|        info: ClientInfo,
5845|    ) -> DneaResult<SessionId> {
5846|        if self.inner.clients.get(client_id).is_none() {
5847|            return Err(DneaError::Internal(
5848|                "client was not attached before on_client_connected".into(),
5849|            ));
5850|        }
5851|        info!(%client_id, browser = %info.browser_family, "client connected");
5852|        self.broadcast(SessionEvent::ClientConnected {
5853|            client_id: client_id.to_string(),
5854|            browser_family: info.browser_family.clone(),
5855|            extension_id: info.extension_id.clone(),
5856|        });
5857|        Ok(self.inner.clients.session_id())
5858|    }
5859|
5860|    async fn on_client_disconnected(&self, client_id: ClientId) {
5861|        self.inner.clients.remove(client_id);
5862|        self.broadcast(SessionEvent::ClientDisconnected {
5863|            client_id: client_id.to_string(),
5864|        });
5865|    }
5866|
5867|    async fn handle_client_envelope(
5868|        &self,
5869|        client_id: ClientId,
5870|        env: Envelope,
5871|    ) -> DneaResult<Option<Envelope>> {
5872|        debug!(%client_id, kind = %env.kind.as_str(), "client envelope");
5873|        match env.kind {
5874|            MessageType::Hello => {
5875|                let mut w = env.reply(MessageType::Welcome);
5876|                w.payload = Some(serde_json::json!({
5877|                    "runtimeVersion": env!("CARGO_PKG_VERSION"),
5878|                    "protocolVersion": dnea_core::PROTOCOL_VERSION,
5879|                    "sessionId": self.inner.clients.session_id().to_string(),
5880|                    "mtu": dnea_core::DEFAULT_MTU,
5881|                    "encodings": ["identity"],
5882|                    "features": { "chunking": true, "consentUI": true },
5883|                }));
5884|                let ext_id = env.payload.as_ref()
5885|                    .and_then(|p| p.get("extensionId"))
5886|                    .and_then(|v| v.as_str())
5887|                    .unwrap_or("")
5888|                    .to_string();
5889|                w.meta.insert("client_extension_id".into(), serde_json::json!(ext_id));
5890|                Ok(Some(w))
5891|            }
5892|            MessageType::Ping => Ok(Some(env.reply(MessageType::Pong))),
5893|            MessageType::Goodbye => Ok(None),
5894|            MessageType::Invoke => {
5895|                let module_id = match &env.module {
5896|                    Some(m) => m.clone(),
5897|                    None => return Ok(Some(Envelope::from_error(
5898|                        &env, &DneaError::InvalidEnvelope("invoke without module".into())))),
5899|                };
5900|                let Some(record) = self.inner.modules.get(&module_id) else {
5901|                    return Ok(Some(Envelope::from_error(
5902|                        &env, &DneaError::ModuleUnavailable(module_id.to_string()))));
5903|                };
5904|                let method = env.meta.get("method").and_then(|v| v.as_str())
5905|                    .unwrap_or("").to_string();
5906|                let args = env.payload.as_ref()
5907|                    .map(|v| serde_json::to_vec(v).unwrap_or_default())
5908|                    .unwrap_or_default();
5909|                let result = record.handle.invoke(&method, args).await;
5910|                let mut reply = env.reply(MessageType::Result);
5911|                match result {
5912|                    Ok(bytes) => reply.payload = serde_json::from_slice(&bytes).ok(),
5913|                    Err(e) => return Ok(Some(Envelope::from_error(&env, &e))),
5914|                }
5915|                Ok(Some(reply))
5916|            }
5917|            MessageType::ConsentResponse => {
5918|                let decision: ConsentDecision = serde_json::from_value(
5919|                    env.payload.clone().unwrap_or_default(),
5920|                ).map_err(|e| DneaError::InvalidEnvelope(e.to_string()))?;
5921|                self.resolve_consent(decision)?;
5922|                Ok(Some(env.reply(MessageType::Result)))
5923|            }
5924|            MessageType::CapabilityList => {
5925|                let mut out = env.reply(MessageType::CapabilityListResult);
5926|                let grants: Vec<_> = self.inner.modules.all().into_iter()
5927|                    .flat_map(|m| {
5928|                        let set = self.inner.grants.get(&m.id);
5929|                        let id = m.id.clone();
5930|                        set.into_iter().flat_map(move |s| {
5931|                            s.iter().map(|(cap, g)| serde_json::json!({
5932|                                "module": id.to_string(),
5933|                                "capability": cap,
5934|                                "constraints": g.constraints,
5935|                                "source": g.source,
5936|                            })).collect::<Vec<_>>()
5937|                        })
5938|                    })
5939|                    .collect();
5940|                out.payload = Some(serde_json::json!({ "grants": grants }));
5941|                Ok(Some(out))
5942|            }
5943|            _ => Ok(Some(env.reply(MessageType::Result))),
5944|        }
5945|    }
5946|
5947|    async fn send_to_client(&self, client_id: ClientId, env: Envelope) -> DneaResult<()> {
5948|        self.send_to_client_by_id(client_id, env).await
5949|    }
5950|}
5951|
5952|const _: Option<RememberFor> = None;
5953|const _: Option<GrantChange> = None;
5954|```
5955|
5956|### `crates/dnea-session/src/install.rs`
5957|
5958|```rust
5959|//! Module installation pipeline.
5960|
5961|use std::{collections::BTreeMap, path::PathBuf, sync::Arc, time::{Duration, SystemTime}};
5962|
5963|use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
5964|use dnea_core::ids::ModuleId;
5965|use dnea_manifest::{
5966|    capability::{CapabilityRegistry, ConsentTier},
5967|    manifest::Manifest,
5968|    signature::verify_manifest,
5969|    trust::TrustStore,
5970|    validate::Validator,
5971|};
5972|use dnea_module_host::{
5973|    grant::{EffectiveGrant, GrantSet, GrantSource},
5974|    host::HostState,
5975|    instance::ModuleHost,
5976|};
5977|use parking_lot::RwLock;
5978|use serde::{Deserialize, Serialize};
5979|use sha2::{Digest, Sha256};
5980|use thiserror::Error;
5981|use tracing::{info, warn};
5982|use uuid::Uuid;
5983|
5984|use crate::{
5985|    consent::{ConsentRequest, RememberFor},
5986|    hub::Session,
5987|    registry::ModuleRecord,
5988|};
5989|
5990|#[derive(Clone, Debug)]
5991|pub enum InstallSource {
5992|    Local { manifest_path: PathBuf, artifact_dir: PathBuf },
5993|}
5994|
5995|#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
5996|#[serde(rename_all = "kebab-case")]
5997|pub enum InstallStatus { Installed, PendingConsent, Failed }
5998|
5999|#[derive(Clone, Debug, Serialize, Deserialize)]
6000|pub struct TranscriptStep {
6001|    pub step: String,
6002|    pub ok: bool,
6003|    pub message: String,
6004|    #[serde(default, skip_serializing_if = "Option::is_none")]
6005|    pub details: Option<serde_json::Value>,
6006|}
6007|
6008|#[derive(Clone, Debug, Serialize, Deserialize)]
6009|pub struct InstallReport {
6010|    pub install_id: Uuid,
6011|    pub module_id: String,
6012|    pub version: String,
6013|    pub status: InstallStatus,
6014|    pub transcript: Vec<TranscriptStep>,
6015|    pub pending_consent: Vec<String>,
6016|}
6017|
6018|#[derive(Debug, Error)]
6019|pub enum InstallError {
6020|    #[error("io: {0}")]
6021|    Io(#[from] std::io::Error),
6022|    #[error("manifest parse: {0}")]
6023|    Parse(#[from] serde_json::Error),
6024|    #[error("validation: {0}")]
6025|    Validation(#[from] dnea_manifest::ValidationError),
6026|    #[error("signature: {0}")]
6027|    Signature(#[from] dnea_manifest::ManifestError),
6028|    #[error("artifact {name} digest mismatch: expected {expected}, got {actual}")]
6029|    ArtifactDigest { name: String, expected: String, actual: String },
6030|    #[error("missing dependency: {module}")]
6031|    MissingDependency { module: String },
6032|    #[error("conflict with {module}: {reason}")]
6033|    Conflict { module: String, reason: String },
6034|    #[error("consent denied for {capability}")]
6035|    ConsentDenied { capability: String },
6036|    #[error("consent timeout for {capability}")]
6037|    ConsentTimeout { capability: String },
6038|    #[error("capability is forbidden: {capability}")]
6039|    ForbiddenCapability { capability: String },
6040|    #[error("capability {capability} requires administrator approval")]
6041|    AdminRequired { capability: String },
6042|    #[error("module host: {0}")]
6043|    ModuleHost(String),
6044|    #[error("module ID: {0}")]
6045|    ModuleId(String),
6046|    #[error("unknown pending install: {0}")]
6047|    UnknownPending(Uuid),
6048|    #[error("install path outside permitted roots: {0}")]
6049|    PathOutsideRoots(PathBuf),
6050|}
6051|
6052|pub trait HostStateFactory: Send + Sync + 'static {
6053|    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState;
6054|}
6055|
6056|struct PendingInstall {
6057|    report: InstallReport,
6058|    manifest: Manifest,
6059|    artifact_dir: PathBuf,
6060|    auto_grants: GrantSet,
6061|    remaining_prompt: Vec<String>,
6062|    created_at: SystemTime,
6063|}
6064|
6065|pub struct InstallPipeline {
6066|    session: Session,
6067|    host: Arc<ModuleHost>,
6068|    validator: Arc<Validator>,
6069|    trust: Arc<dyn TrustStore>,
6070|    host_state_factory: Arc<dyn HostStateFactory>,
6071|    consent_timeout: Duration,
6072|    install_roots: Vec<PathBuf>,
6073|    pending: RwLock<BTreeMap<Uuid, PendingInstall>>,
6074|}
6075|
6076|impl InstallPipeline {
6077|    pub fn new(
6078|        session: Session,
6079|        host: Arc<ModuleHost>,
6080|        validator: Arc<Validator>,
6081|        trust: Arc<dyn TrustStore>,
6082|        host_state_factory: Arc<dyn HostStateFactory>,
6083|    ) -> Self {
6084|        Self {
6085|            session, host, validator, trust, host_state_factory,
6086|            consent_timeout: Duration::from_secs(300),
6087|            install_roots: Vec::new(),
6088|            pending: RwLock::new(BTreeMap::new()),
6089|        }
6090|    }
6091|
6092|    #[must_use]
6093|    pub fn with_consent_timeout(mut self, ttl: Duration) -> Self {
6094|        self.consent_timeout = ttl; self
6095|    }
6096|
6097|    #[must_use]
6098|    pub fn with_install_roots(mut self, roots: Vec<PathBuf>) -> Self {
6099|        self.install_roots = roots; self
6100|    }
6101|
6102|    pub async fn install(&self, source: InstallSource) -> Result<InstallReport, InstallError> {
6103|        let install_id = Uuid::now_v7();
6104|        let mut transcript = Vec::with_capacity(16);
6105|
6106|        let (manifest_bytes, artifact_dir) = match source {
6107|            InstallSource::Local { manifest_path, artifact_dir } => {
6108|                self.check_path(&manifest_path)?;
6109|                self.check_path(&artifact_dir)?;
6110|                let bytes = tokio::fs::read(&manifest_path).await?;
6111|                (bytes, artifact_dir)
6112|            }
6113|        };
6114|        transcript.push(step("load-source", true, "manifest and artifacts located"));
6115|
6116|        let manifest_value: serde_json::Value = serde_json::from_slice(&manifest_bytes)?;
6117|        let manifest: Manifest = serde_json::from_value(manifest_value.clone())?;
6118|        let module_id = ModuleId::parse(&manifest.id)
6119|            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
6120|        transcript.push(step("parse-manifest", true,
6121|            &format!("{} v{}", manifest.id, manifest.version)));
6122|
6123|        self.validator.validate(&manifest_value)?;
6124|        transcript.push(step("validate", true, "schema + semantic validation passed"));
6125|
6126|        if !manifest.dev || manifest.signature.is_some() {
6127|            let info = verify_manifest(&manifest_value, &*self.trust)?;
6128|            transcript.push(step_with_details("verify-signature", true,
6129|                "signature chain verified",
6130|                serde_json::json!({
6131|                    "signers": info.signers.iter().map(|s| serde_json::json!({
6132|                        "keyId": s.key_id, "role": s.role })).collect::<Vec<_>>()
6133|                })));
6134|        } else {
6135|            warn!(module = %manifest.id, "installing unsigned dev module");
6136|            transcript.push(step("verify-signature", true, "unsigned dev module accepted"));
6137|        }
6138|
6139|        for (name, ep) in &manifest.entrypoints {
6140|            let path = artifact_dir.join(&ep.path);
6141|            let bytes = tokio::fs::read(&path).await?;
6142|            let actual = format!("sha-256:{}",
6143|                URL_SAFE_NO_PAD.encode(Sha256::digest(&bytes)));
6144|            if actual != ep.digest.value {
6145|                return Err(InstallError::ArtifactDigest {
6146|                    name: name.clone(),
6147|                    expected: ep.digest.value.clone(),
6148|                    actual,
6149|                });
6150|            }
6151|        }
6152|        transcript.push(step("verify-artifacts", true, "all digests match"));
6153|
6154|        for (dep_id, dep) in &manifest.dependencies {
6155|            let dep_module = ModuleId::parse(dep_id)
6156|                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
6157|            if self.session.module(&dep_module).is_none() && !dep.optional {
6158|                return Err(InstallError::MissingDependency { module: dep_id.clone() });
6159|            }
6160|        }
6161|        transcript.push(step("resolve-deps", true, "dependencies satisfied"));
6162|
6163|        for c in &manifest.conflicts {
6164|            let cid = ModuleId::parse(&c.id)
6165|                .map_err(|e| InstallError::ModuleId(e.to_string()))?;
6166|            if self.session.module(&cid).is_some() {
6167|                return Err(InstallError::Conflict {
6168|                    module: c.id.clone(),
6169|                    reason: c.reason.clone().unwrap_or_else(|| "declared conflict".into()),
6170|                });
6171|            }
6172|        }
6173|        transcript.push(step("check-conflicts", true, "no conflicts"));
6174|
6175|        let (auto_grants, prompt_caps, admin_caps) = self.compute_grants(&manifest)?;
6176|        transcript.push(step("compute-grants", true,
6177|            &format!("{} auto, {} prompt, {} admin",
6178|                auto_grants.len(), prompt_caps.len(), admin_caps.len())));
6179|
6180|        if !admin_caps.is_empty() {
6181|            return Err(InstallError::AdminRequired { capability: admin_caps[0].clone() });
6182|        }
6183|
6184|        if prompt_caps.is_empty() {
6185|            return self.finish_load(
6186|                install_id, &manifest, &module_id, &artifact_dir, auto_grants, transcript,
6187|            ).await;
6188|        }
6189|
6190|        let first = &prompt_caps[0];
6191|        let request = self.build_consent_request(&manifest, first);
6192|        let _rx = self.session.request_consent(request).await;
6193|
6194|        let report = InstallReport {
6195|            install_id,
6196|            module_id: manifest.id.clone(),
6197|            version: manifest.version.clone(),
6198|            status: InstallStatus::PendingConsent,
6199|            transcript: {
6200|                let mut t = transcript;
6201|                t.push(step("consent", true,
6202|                    &format!("{} capabilities require consent", prompt_caps.len())));
6203|                t
6204|            },
6205|            pending_consent: prompt_caps.clone(),
6206|        };
6207|
6208|        self.pending.write().insert(install_id, PendingInstall {
6209|            report: report.clone(),
6210|            manifest, artifact_dir, auto_grants,
6211|            remaining_prompt: prompt_caps,
6212|            created_at: SystemTime::now(),
6213|        });
6214|
6215|        Ok(report)
6216|    }
6217|
6218|    pub async fn resume(&self, install_id: Uuid) -> Result<InstallReport, InstallError> {
6219|        let mut pending = {
6220|            let mut map = self.pending.write();
6221|            map.remove(&install_id).ok_or(InstallError::UnknownPending(install_id))?
6222|        };
6223|        if pending.created_at.elapsed().map(|d| d > self.consent_timeout).unwrap_or(true) {
6224|            return Err(InstallError::ConsentTimeout {
6225|                capability: pending.remaining_prompt.first().cloned().unwrap_or_default(),
6226|            });
6227|        }
6228|        let module_id = ModuleId::parse(&pending.manifest.id)
6229|            .map_err(|e| InstallError::ModuleId(e.to_string()))?;
6230|        let current = self.session.grants(&module_id).unwrap_or_default();
6231|        let mut still_needed = Vec::new();
6232|        for cap in &pending.remaining_prompt {
6233|            if current.get(cap).is_none() { still_needed.push(cap.clone()); }
6234|        }
6235|        if still_needed.is_empty() {
6236|            let mut grants = pending.auto_grants;
6237|            for (id, g) in current.iter() {
6238|                grants.insert(EffectiveGrant {
6239|                    id: id.to_string(),
6240|                    constraints: g.constraints.clone(),
6241|                    source: g.source,
6242|                });
6243|            }
6244|            return self.finish_load(
6245|                install_id, &pending.manifest, &module_id, &pending.artifact_dir,
6246|                grants, pending.report.transcript,
6247|            ).await;
6248|        }
6249|
6250|        let next = &still_needed[0];
6251|        let request = self.build_consent_request(&pending.manifest, next);
6252|        let _ = self.session.request_consent(request).await;
6253|        pending.remaining_prompt = still_needed.clone();
6254|        pending.report.pending_consent = still_needed;
6255|        pending.report.status = InstallStatus::PendingConsent;
6256|        let report = pending.report.clone();
6257|        self.pending.write().insert(install_id, pending);
6258|        Ok(report)
6259|    }
6260|
6261|    async fn finish_load(
6262|        &self,
6263|        install_id: Uuid,
6264|        manifest: &Manifest,
6265|        module_id: &ModuleId,
6266|        artifact_dir: &PathBuf,
6267|        grants: GrantSet,
6268|        mut transcript: Vec<TranscriptStep>,
6269|    ) -> Result<InstallReport, InstallError> {
6270|        let component_path = self.primary_component_path(manifest, artifact_dir)?;
6271|        let grants_arc = Arc::new(grants.clone());
6272|        let factory = self.host_state_factory.clone();
6273|        let mid = manifest.id.clone();
6274|        let ver = manifest.version.clone();
6275|
6276|        let handle = self.host.load(
6277|            mid.clone(), ver.clone(), component_path, grants,
6278|            move |g| factory.build(&mid, &ver, g),
6279|        ).await.map_err(|e| InstallError::ModuleHost(e.to_string()))?;
6280|
6281|        transcript.push(step("load", true, "Wasm component loaded and activated"));
6282|
6283|        let manifest_arc = Arc::new(manifest.clone());
6284|        self.session.register_module(Arc::new(ModuleRecord {
6285|            id: module_id.clone(),
6286|            version: manifest.version.clone(),
6287|            handle: Arc::new(handle),
6288|            manifest: Some(manifest_arc),
6289|            loaded_at: SystemTime::now(),
6290|        }));
6291|        self.session.set_grants(module_id, grants_arc);
6292|
6293|        transcript.push(step("register", true, "module registered"));
6294|        info!(module = %manifest.id, version = %manifest.version, "module installed");
6295|
6296|        Ok(InstallReport {
6297|            install_id,
6298|            module_id: manifest.id.clone(),
6299|            version: manifest.version.clone(),
6300|            status: InstallStatus::Installed,
6301|            transcript,
6302|            pending_consent: Vec::new(),
6303|        })
6304|    }
6305|
6306|    fn check_path(&self, path: &PathBuf) -> Result<(), InstallError> {
6307|        if self.install_roots.is_empty() { return Ok(()); }
6308|        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
6309|        for root in &self.install_roots {
6310|            let rc = root.canonicalize().unwrap_or_else(|_| root.clone());
6311|            if canonical.starts_with(&rc) { return Ok(()); }
6312|        }
6313|        Err(InstallError::PathOutsideRoots(path.clone()))
6314|    }
6315|
6316|    fn compute_grants(
6317|        &self, manifest: &Manifest,
6318|    ) -> Result<(GrantSet, Vec<String>, Vec<String>), InstallError> {
6319|        let registry = CapabilityRegistry::v1();
6320|        let mut auto = GrantSet::new();
6321|        let mut prompt = Vec::new();
6322|        let mut admin = Vec::new();
6323|        for req in &manifest.capabilities.request {
6324|            let tier = req.consent.as_deref().and_then(ConsentTier::parse)
6325|                .unwrap_or_else(|| registry.default_tier(&req.id));
6326|            match tier {
6327|                ConsentTier::Forbidden => return Err(InstallError::ForbiddenCapability {
6328|                    capability: req.id.clone() }),
6329|                ConsentTier::Admin => admin.push(req.id.clone()),
6330|                ConsentTier::Prompt | ConsentTier::PerUse => prompt.push(req.id.clone()),
6331|                ConsentTier::Auto => {
6332|                    auto.insert(EffectiveGrant {
6333|                        id: req.id.clone(),
6334|                        constraints: if req.constraints.is_null() {
6335|                            serde_json::json!({})
6336|                        } else { req.constraints.clone() },
6337|                        source: GrantSource::Auto,
6338|                    });
6339|                }
6340|            }
6341|        }
6342|        for d in &manifest.capabilities.deny { auto.deny(d); }
6343|        Ok((auto, prompt, admin))
6344|    }
6345|
6346|    fn build_consent_request(&self, manifest: &Manifest, capability: &str) -> ConsentRequest {
6347|        let req = manifest.capabilities.request.iter().find(|r| r.id == capability);
6348|        ConsentRequest {
6349|            prompt_id: Uuid::now_v7(),
6350|            module: manifest.id.clone(),
6351|            module_name: manifest.name.clone(),
6352|            capability: capability.to_string(),
6353|            tier: req.and_then(|r| r.consent.clone()).unwrap_or_else(|| "prompt".into()),
6354|            reason: req.and_then(|r| r.reason.clone())
6355|                .unwrap_or_else(|| format!("Module requests {capability}")),
6356|            constraints: req.map(|r| if r.constraints.is_null() { serde_json::json!({}) } else { r.constraints.clone() })
6357|                .unwrap_or(serde_json::json!({})),
6358|            target_description: None,
6359|            expires_at: SystemTime::now() + self.consent_timeout,
6360|            can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion, RememberFor::Always],
6361|        }
6362|    }
6363|
6364|    fn primary_component_path(
6365|        &self, manifest: &Manifest, artifact_dir: &PathBuf,
6366|    ) -> Result<PathBuf, InstallError> {
6367|        let ep = manifest.entrypoints.get("core")
6368|            .or_else(|| manifest.entrypoints.values().next())
6369|            .ok_or_else(|| InstallError::ModuleHost("manifest has no entrypoints".into()))?;
6370|        Ok(artifact_dir.join(&ep.path))
6371|    }
6372|}
6373|
6374|fn step(name: &str, ok: bool, msg: &str) -> TranscriptStep {
6375|    TranscriptStep { step: name.into(), ok, message: msg.into(), details: None }
6376|}
6377|
6378|fn step_with_details(name: &str, ok: bool, msg: &str, details: serde_json::Value) -> TranscriptStep {
6379|    TranscriptStep { step: name.into(), ok, message: msg.into(), details: Some(details) }
6380|}
6381|```
6382|
6383|### `crates/dnea-session/tests/install_signed_manifest.rs`
6384|
6385|```rust
6386|//! Signed-install end-to-end test.
6387|
6388|use std::{path::PathBuf, sync::Arc};
6389|
6390|use dnea_manifest::{
6391|    signature::sign_manifest,
6392|    trust::{ResolvedKey, SigAlg, StaticTrustStore},
6393|    validate::Validator,
6394|};
6395|use dnea_module_host::{engine::EngineConfig, grant::GrantSet, host::HostState, instance::ModuleHost};
6396|use dnea_session::{
6397|    install::{HostStateFactory, InstallPipeline, InstallSource, InstallStatus},
6398|    Session,
6399|};
6400|use ed25519_dalek::SigningKey;
6401|use rand::rngs::OsRng;
6402|use serde_json::json;
6403|use sha2::{Digest, Sha256};
6404|use tempfile::TempDir;
6405|
6406|struct NoopFactory;
6407|
6408|impl HostStateFactory for NoopFactory {
6409|    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
6410|        HostState::new(
6411|            module_id.to_string(), version.to_string(), grants,
6412|            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
6413|        )
6414|    }
6415|}
6416|
6417|fn make_manifest(artifact_digest: &str) -> serde_json::Value {
6418|    json!({
6419|        "manifestVersion": 1, "id": "com.example.focus-mode", "version": "1.2.0",
6420|        "runtime": ">=1.0.0 <2.0.0", "name": "Focus Mode",
6421|        "entrypoints": { "core": {
6422|            "kind": "wasm", "path": "core.wasm",
6423|            "digest": { "alg": "sha-256", "value": artifact_digest } } },
6424|        "capabilities": { "request": [
6425|            { "id": "storage:module" }, { "id": "runtime:log" },
6426|            { "id": "browser:bookmarks:read" } ] }
6427|    })
6428|}
6429|
6430|#[tokio::test(flavor = "multi_thread")]
6431|async fn install_signed_focus_mode_with_auto_grants() {
6432|    let tmp = TempDir::new().unwrap();
6433|    let artifact_bytes = b"\0asm\x01\0\0\0";
6434|    let artifact_digest = format!("sha-256:{}",
6435|        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
6436|            Sha256::digest(artifact_bytes)));
6437|    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();
6438|
6439|    let signing_key = SigningKey::generate(&mut OsRng);
6440|    let key_id = "https://test.example/keys/signing";
6441|    let mut manifest = make_manifest(&artifact_digest);
6442|    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
6443|    std::fs::write(tmp.path().join("manifest.json"),
6444|        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
6445|
6446|    let mut trust = StaticTrustStore::new();
6447|    trust.insert(key_id, ResolvedKey {
6448|        alg: SigAlg::EdDSA,
6449|        public_key: signing_key.verifying_key().to_bytes().to_vec(),
6450|        revoked: false,
6451|    });
6452|
6453|    let session = Session::new();
6454|    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
6455|    let validator = Arc::new(Validator::new().unwrap());
6456|    let pipeline = InstallPipeline::new(
6457|        session.clone(), host, validator, Arc::new(trust), Arc::new(NoopFactory));
6458|
6459|    let report = pipeline.install(InstallSource::Local {
6460|        manifest_path: tmp.path().join("manifest.json"),
6461|        artifact_dir: tmp.path().to_path_buf(),
6462|    }).await;
6463|
6464|    match report {
6465|        Ok(r) => {
6466|            assert_eq!(r.module_id, "com.example.focus-mode");
6467|            assert_eq!(r.status, InstallStatus::Installed);
6468|        }
6469|        Err(e) => {
6470|            let s = e.to_string();
6471|            assert!(s.contains("component") || s.contains("wasm") || s.contains("module host"),
6472|                "expected Wasm load error, got: {s}");
6473|        }
6474|    }
6475|}
6476|
6477|#[tokio::test(flavor = "multi_thread")]
6478|async fn install_rejects_tampered_manifest() {
6479|    let tmp = TempDir::new().unwrap();
6480|    let artifact_bytes = b"\0asm\x01\0\0\0";
6481|    let artifact_digest = format!("sha-256:{}",
6482|        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
6483|            Sha256::digest(artifact_bytes)));
6484|    std::fs::write(tmp.path().join("core.wasm"), artifact_bytes).unwrap();
6485|
6486|    let signing_key = SigningKey::generate(&mut OsRng);
6487|    let key_id = "https://test.example/keys/signing";
6488|    let mut manifest = make_manifest(&artifact_digest);
6489|    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
6490|    manifest["name"] = json!("Tampered Name");
6491|    std::fs::write(tmp.path().join("manifest.json"),
6492|        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
6493|
6494|    let mut trust = StaticTrustStore::new();
6495|    trust.insert(key_id, ResolvedKey {
6496|        alg: SigAlg::EdDSA,
6497|        public_key: signing_key.verifying_key().to_bytes().to_vec(),
6498|        revoked: false,
6499|    });
6500|
6501|    let session = Session::new();
6502|    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
6503|    let validator = Arc::new(Validator::new().unwrap());
6504|    let pipeline = InstallPipeline::new(
6505|        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));
6506|
6507|    let err = pipeline.install(InstallSource::Local {
6508|        manifest_path: tmp.path().join("manifest.json"),
6509|        artifact_dir: tmp.path().to_path_buf(),
6510|    }).await.unwrap_err();
6511|    assert!(err.to_string().contains("digest") || err.to_string().contains("signature"));
6512|}
6513|
6514|#[tokio::test(flavor = "multi_thread")]
6515|async fn install_rejects_tampered_artifact() {
6516|    let tmp = TempDir::new().unwrap();
6517|    let real = b"\0asm\x01\0\0\0";
6518|    let claimed = b"different bytes";
6519|    let claimed_digest = format!("sha-256:{}",
6520|        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD,
6521|            Sha256::digest(claimed)));
6522|    std::fs::write(tmp.path().join("core.wasm"), real.as_slice()).unwrap();
6523|
6524|    let signing_key = SigningKey::generate(&mut OsRng);
6525|    let key_id = "https://test.example/keys/signing";
6526|    let mut manifest = make_manifest(&claimed_digest);
6527|    sign_manifest(&mut manifest, &signing_key, key_id, "author").unwrap();
6528|    std::fs::write(tmp.path().join("manifest.json"),
6529|        serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
6530|
6531|    let mut trust = StaticTrustStore::new();
6532|    trust.insert(key_id, ResolvedKey {
6533|        alg: SigAlg::EdDSA,
6534|        public_key: signing_key.verifying_key().to_bytes().to_vec(),
6535|        revoked: false,
6536|    });
6537|
6538|    let session = Session::new();
6539|    let host = Arc::new(ModuleHost::new(&EngineConfig::default()).unwrap());
6540|    let validator = Arc::new(Validator::new().unwrap());
6541|    let pipeline = InstallPipeline::new(
6542|        session, host, validator, Arc::new(trust), Arc::new(NoopFactory));
6543|
6544|    let err = pipeline.install(InstallSource::Local {
6545|        manifest_path: tmp.path().join("manifest.json"),
6546|        artifact_dir: tmp.path().to_path_buf(),
6547|    }).await.unwrap_err();
6548|    assert!(matches!(err, dnea_session::install::InstallError::ArtifactDigest { .. }));
6549|}
6550|```
6551|
6552|## 3.8 `crates/dnea-local-api`
6553|
6554|### `crates/dnea-local-api/Cargo.toml`
6555|
6556|```toml
6557|[package]
6558|name        = "dnea-local-api"
6559|version.workspace = true
6560|description = "DNEA Local API and embedded companion UI."
6561|edition.workspace     = true
6562|rust-version.workspace = true
6563|license.workspace     = true
6564|
6565|[dependencies]
6566|dnea-core          = { path = "../dnea-core" }
6567|dnea-session       = { path = "../dnea-session" }
6568|dnea-companion-web = { path = "../dnea-companion-web" }
6569|anyhow             = { workspace = true }
6570|async-trait        = { workspace = true }
6571|axum               = { workspace = true }
6572|hmac               = { workspace = true }
6573|jsonwebtoken       = { workspace = true }
6574|serde              = { workspace = true }
6575|serde_json         = { workspace = true }
6576|tokio              = { workspace = true }
6577|tower              = { workspace = true }
6578|tower-http         = { workspace = true }
6579|tracing            = { workspace = true }
6580|uuid               = { workspace = true }
6581|time               = { workspace = true }
6582|rand               = { workspace = true }
6583|base64             = { workspace = true }
6584|directories        = { workspace = true }
6585|
6586|[dev-dependencies]
6587|reqwest  = { workspace = true }
6588|tempfile = { workspace = true }
6589|tokio    = { workspace = true, features = ["macros", "rt-multi-thread", "time"] }
6590|```
6591|
6592|### `crates/dnea-local-api/src/lib.rs`
6593|
6594|```rust
6595|//! The DNEA Local API and embedded companion UI.
6596|
6597|#![forbid(unsafe_code)]
6598|#![deny(rust_2018_idioms, unreachable_pub)]
6599|#![warn(missing_docs)]
6600|
6601|mod auth;
6602|mod module_ui;
6603|mod routes;
6604|mod state;
6605|mod static_assets;
6606|mod ws;
6607|
6608|pub use auth::{mint_token, verify_token, Claims, Scope};
6609|pub use state::{ApiConfig, ApiState, LocalApi, LocalApiError};
6610|```
6611|
6612|### `crates/dnea-local-api/src/auth.rs`
6613|
6614|```rust
6615|//! JWT minting and verification.
6616|
6617|use std::time::{Duration, SystemTime};
6618|
6619|use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
6620|use serde::{Deserialize, Serialize};
6621|use uuid::Uuid;
6622|
6623|#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
6624|#[serde(rename_all = "snake_case")]
6625|pub enum Scope {
6626|    UiRead, UiConsent, UiModules, UiSettings, UiAudit, UiAdmin,
6627|}
6628|
6629|impl Scope {
6630|    #[must_use]
6631|    pub fn as_str(&self) -> &'static str {
6632|        match self {
6633|            Self::UiRead => "ui.read",
6634|            Self::UiConsent => "ui.consent",
6635|            Self::UiModules => "ui.modules",
6636|            Self::UiSettings => "ui.settings",
6637|            Self::UiAudit => "ui.audit",
6638|            Self::UiAdmin => "ui.admin",
6639|        }
6640|    }
6641|
6642|    #[must_use]
6643|    pub fn companion_set() -> Vec<Self> {
6644|        vec![Self::UiRead, Self::UiConsent, Self::UiModules, Self::UiSettings, Self::UiAudit]
6645|    }
6646|}
6647|
6648|#[derive(Clone, Debug, Serialize, Deserialize)]
6649|pub struct Claims {
6650|    pub sub: String,
6651|    pub iat: u64,
6652|    pub exp: u64,
6653|    pub jti: Uuid,
6654|    pub scopes: Vec<Scope>,
6655|    pub session: String,
6656|}
6657|
6658|pub fn mint_token(
6659|    secret: &[u8],
6660|    subject: &str,
6661|    session: &str,
6662|    scopes: Vec<Scope>,
6663|    ttl: Duration,
6664|) -> anyhow::Result<String> {
6665|    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
6666|    let claims = Claims {
6667|        sub: subject.to_string(),
6668|        iat: now,
6669|        exp: now + ttl.as_secs(),
6670|        jti: Uuid::now_v7(),
6671|        scopes,
6672|        session: session.to_string(),
6673|    };
6674|    let header = Header::new(Algorithm::HS256);
6675|    Ok(encode(&header, &claims, &EncodingKey::from_secret(secret))?)
6676|}
6677|
6678|pub fn verify_token(secret: &[u8], token: &str) -> anyhow::Result<Claims> {
6679|    let mut validation = Validation::new(Algorithm::HS256);
6680|    validation.validate_exp = true;
6681|    validation.leeway = 5;
6682|    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
6683|    Ok(data.claims)
6684|}
6685|```
6686|
6687|### `crates/dnea-local-api/src/state.rs`
6688|
6689|```rust
6690|//! API state and configuration.
6691|
6692|use std::net::SocketAddr;
6693|use std::path::PathBuf;
6694|use std::sync::Arc;
6695|use std::time::Duration;
6696|
6697|use dnea_session::{install::InstallPipeline, Session};
6698|use rand::RngCore;
6699|use serde::Serialize;
6700|use tokio::net::TcpListener;
6701|use tracing::info;
6702|
6703|#[derive(Clone, Debug)]
6704|pub struct ApiConfig {
6705|    pub bind: SocketAddr,
6706|    pub discovery_path: PathBuf,
6707|    pub token_ttl: Duration,
6708|}
6709|
6710|impl ApiConfig {
6711|    #[must_use]
6712|    pub fn default_config() -> Self {
6713|        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
6714|            .expect("no home directory");
6715|        let discovery_path = proj.runtime_dir()
6716|            .unwrap_or_else(std::env::temp_dir).join("api.json");
6717|        Self {
6718|            bind: "127.0.0.1:0".parse().expect("valid loopback"),
6719|            discovery_path,
6720|            token_ttl: Duration::from_secs(600),
6721|        }
6722|    }
6723|}
6724|
6725|pub struct ApiState {
6726|    pub session: Session,
6727|    pub secret: Vec<u8>,
6728|    pub token_ttl: Duration,
6729|    pub install_pipeline: Option<Arc<InstallPipeline>>,
6730|}
6731|
6732|impl ApiState {
6733|    #[must_use]
6734|    pub fn new(session: Session, config: &ApiConfig) -> Self {
6735|        let mut secret = vec![0u8; 32];
6736|        rand::thread_rng().fill_bytes(&mut secret);
6737|        Self {
6738|            session,
6739|            secret,
6740|            token_ttl: config.token_ttl,
6741|            install_pipeline: None,
6742|        }
6743|    }
6744|
6745|    #[must_use]
6746|    pub fn with_pipeline(mut self, pipeline: Arc<InstallPipeline>) -> Self {
6747|        self.install_pipeline = Some(pipeline);
6748|        self
6749|    }
6750|}
6751|
6752|#[derive(Debug, thiserror::Error)]
6753|pub enum LocalApiError {
6754|    #[error("bind {addr}: {source}")]
6755|    Bind { addr: SocketAddr, source: std::io::Error },
6756|    #[error("write discovery file: {0}")]
6757|    Discovery(std::io::Error),
6758|}
6759|
6760|#[derive(Serialize)]
6761|struct Discovery<'a> {
6762|    v: u32,
6763|    url: String,
6764|    ws: String,
6765|    pid: u32,
6766|    started_at: String,
6767|    session_id: &'a str,
6768|    scopes: Vec<&'static str>,
6769|}
6770|
6771|pub struct LocalApi {
6772|    state: Arc<ApiState>,
6773|    listener: TcpListener,
6774|    config: ApiConfig,
6775|}
6776|
6777|impl LocalApi {
6778|    pub async fn bind(session: Session, config: ApiConfig) -> Result<Self, LocalApiError> {
6779|        let listener = TcpListener::bind(config.bind).await
6780|            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
6781|        let state = Arc::new(ApiState::new(session, &config));
6782|        Ok(Self { state, listener, config })
6783|    }
6784|
6785|    pub async fn bind_with_pipeline(
6786|        session: Session,
6787|        config: ApiConfig,
6788|        pipeline: Arc<InstallPipeline>,
6789|    ) -> Result<Self, LocalApiError> {
6790|        let listener = TcpListener::bind(config.bind).await
6791|            .map_err(|source| LocalApiError::Bind { addr: config.bind, source })?;
6792|        let state = Arc::new(ApiState::new(session, &config).with_pipeline(pipeline));
6793|        Ok(Self { state, listener, config })
6794|    }
6795|
6796|    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
6797|        self.listener.local_addr()
6798|    }
6799|
6800|    pub async fn serve(self) -> anyhow::Result<()> {
6801|        let addr = self.listener.local_addr()?;
6802|        let session_id = self.state.session.session_id().to_string();
6803|        let discovery = Discovery {
6804|            v: 1,
6805|            url: format!("http://{addr}"),
6806|            ws: format!("ws://{addr}/ws"),
6807|            pid: std::process::id(),
6808|            started_at: time::OffsetDateTime::now_utc()
6809|                .format(&time::format_description::well_known::Rfc3339)
6810|                .unwrap_or_else(|_| "unknown".into()),
6811|            session_id: &session_id,
6812|            scopes: crate::auth::Scope::companion_set().iter()
6813|                .map(|s| s.as_str()).collect(),
6814|        };
6815|        let bytes = serde_json::to_vec_pretty(&discovery)?;
6816|        if let Some(parent) = self.config.discovery_path.parent() {
6817|            std::fs::create_dir_all(parent).ok();
6818|        }
6819|        std::fs::write(&self.config.discovery_path, &bytes)
6820|            .map_err(LocalApiError::Discovery)?;
6821|        #[cfg(unix)]
6822|        {
6823|            use std::os::unix::fs::PermissionsExt;
6824|            if let Ok(meta) = std::fs::metadata(&self.config.discovery_path) {
6825|                let mut perms = meta.permissions();
6826|                perms.set_mode(0o600);
6827|                let _ = std::fs::set_permissions(&self.config.discovery_path, perms);
6828|            }
6829|        }
6830|
6831|        info!(%addr, "local api listening");
6832|        let router = routes::router(self.state.clone());
6833|        axum::serve(self.listener, router).await?;
6834|        Ok(())
6835|    }
6836|}
6837|```
6838|
6839|### `crates/dnea-local-api/src/routes.rs`
6840|
6841|```rust
6842|//! HTTP routes.
6843|
6844|use std::sync::Arc;
6845|
6846|use axum::{
6847|    extract::{Path, State},
6848|    http::StatusCode,
6849|    response::{IntoResponse, Response},
6850|    routing::{delete, get, post},
6851|    Json, Router,
6852|};
6853|use dnea_core::ids::ModuleId;
6854|use dnea_session::consent::{ConsentDecision, ConsentRequest};
6855|use dnea_session::install::InstallSource;
6856|use serde::Deserialize;
6857|use serde_json::json;
6858|use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer};
6859|use uuid::Uuid;
6860|
6861|use crate::{
6862|    auth::{mint_token, verify_token, Claims, Scope},
6863|    state::ApiState,
6864|    ws,
6865|};
6866|
6867|pub fn router(state: Arc<ApiState>) -> Router {
6868|    Router::new()
6869|        .route("/healthz", get(|| async { "ok" }))
6870|        .route("/v1/version", get(version))
6871|        .route("/v1/auth/token", post(mint))
6872|        .route("/v1/auth/refresh", post(refresh))
6873|        .route("/", get(crate::static_assets::index))
6874|        .route("/app.js", get(crate::static_assets::app_js))
6875|        .route("/style.css", get(crate::static_assets::style_css))
6876|        .route("/v1/status", get(status))
6877|        .route("/v1/modules", get(list_modules).post(install_module))
6878|        .route("/v1/modules/:id", get(get_module))
6879|        .route("/v1/modules/:id/capabilities", get(get_capabilities))
6880|        .route("/v1/modules/:id/capabilities/:cap/origins",
6881|            get(list_origins).post(add_origin))
6882|        .route("/v1/modules/:id/capabilities/:cap/origins/:origin",
6883|            delete(remove_origin))
6884|        .route("/v1/modules/installs/:install_id/resume", post(resume_install))
6885|        .route("/v1/consent/pending", get(list_consent))
6886|        .route("/v1/consent/:id/respond", post(respond_consent))
6887|        .route("/modules/:id/ui/:panel", get(crate::module_ui::serve_panel))
6888|        .route("/modules/:id/ui/:panel/invoke", post(crate::module_ui::invoke))
6889|        .route("/ws", get(ws::handler))
6890|        .layer(RequestBodyLimitLayer::new(1 << 20))
6891|        .layer(CompressionLayer::new())
6892|        .layer(TraceLayer::new_for_http())
6893|        .with_state(state)
6894|}
6895|
6896|struct Authed(Claims);
6897|
6898|#[axum::async_trait]
6899|impl axum::extract::FromRequestParts<Arc<ApiState>> for Authed {
6900|    type Rejection = Response;
6901|    async fn from_request_parts(
6902|        parts: &mut axum::http::request::Parts,
6903|        state: &Arc<ApiState>,
6904|    ) -> Result<Self, Self::Rejection> {
6905|        let token = parts.headers
6906|            .get("authorization")
6907|            .and_then(|v| v.to_str().ok())
6908|            .and_then(|v| v.strip_prefix("Bearer "))
6909|            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "missing bearer token").into_response())?;
6910|        let claims = verify_token(&state.secret, token)
6911|            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid token").into_response())?;
6912|        Ok(Authed(claims))
6913|    }
6914|}
6915|
6916|impl Authed {
6917|    fn require(&self, scope: Scope) -> Result<(), Response> {
6918|        if self.0.scopes.contains(&scope) { Ok(()) } else {
6919|            Err((StatusCode::FORBIDDEN, Json(json!({ "error": {
6920|                "code": "E_NOT_GRANTED",
6921|                "message": format!("missing scope {}", scope.as_str())
6922|            }}))).into_response())
6923|        }
6924|    }
6925|}
6926|
6927|async fn version() -> Json<serde_json::Value> {
6928|    Json(json!({
6929|        "runtimeVersion": env!("CARGO_PKG_VERSION"),
6930|        "protocolVersion": dnea_core::PROTOCOL_VERSION,
6931|    }))
6932|}
6933|
6934|#[derive(Deserialize)]
6935|struct MintRequest {
6936|    #[serde(default)] grant_type: Option<String>,
6937|    #[serde(default)] device_id: Option<String>,
6938|    #[serde(default)] requested_scopes: Option<Vec<String>>,
6939|}
6940|
6941|async fn mint(State(state): State<Arc<ApiState>>, Json(req): Json<MintRequest>) -> Response {
6942|    let subject = req.device_id.clone().unwrap_or_else(|| "dnea-client".to_string());
6943|    let scopes: Vec<Scope> = req.requested_scopes.as_ref()
6944|        .map(|s| s.iter().filter_map(|s| parse_scope(s)).collect())
6945|        .unwrap_or_else(Scope::companion_set);
6946|    let session = state.session.session_id().to_string();
6947|    match mint_token(&state.secret, &subject, &session, scopes, state.token_ttl) {
6948|        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
6949|            .into_response(),
6950|        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
6951|    }
6952|}
6953|
6954|async fn refresh(State(state): State<Arc<ApiState>>, headers: axum::http::HeaderMap) -> Response {
6955|    let Some(token) = headers.get("authorization")
6956|        .and_then(|v| v.to_str().ok())
6957|        .and_then(|v| v.strip_prefix("Bearer ")) else {
6958|        return (StatusCode::UNAUTHORIZED, "missing bearer token").into_response();
6959|    };
6960|    let Ok(claims) = verify_token(&state.secret, token) else {
6961|        return (StatusCode::UNAUTHORIZED, "invalid token").into_response();
6962|    };
6963|    match mint_token(&state.secret, &claims.sub, &claims.session, claims.scopes, state.token_ttl) {
6964|        Ok(token) => Json(json!({ "token": token, "expiresIn": state.token_ttl.as_secs() }))
6965|            .into_response(),
6966|        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
6967|    }
6968|}
6969|
6970|async fn status(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
6971|    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
6972|    let clients: Vec<_> = state.session.clients().into_iter()
6973|        .map(|c| json!({
6974|            "clientId": c.id.to_string(),
6975|            "browserFamily": c.browser_family,
6976|            "extensionId": c.extension_id,
6977|            "bridgePid": c.bridge_pid,
6978|        })).collect();
6979|    let modules: Vec<_> = state.session.modules().into_iter()
6980|        .map(|m| json!({ "id": m.id.to_string(), "version": m.version })).collect();
6981|    Json(json!({
6982|        "runtimeVersion": env!("CARGO_PKG_VERSION"),
6983|        "protocolVersion": dnea_core::PROTOCOL_VERSION,
6984|        "sessionId": state.session.session_id().to_string(),
6985|        "clients": clients,
6986|        "modules": modules,
6987|    })).into_response()
6988|}
6989|
6990|async fn list_modules(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
6991|    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
6992|    let list: Vec<_> = state.session.modules().into_iter()
6993|        .map(|m| {
6994|            let grants = state.session.grants(&m.id);
6995|            json!({
6996|                "id": m.id.to_string(),
6997|                "version": m.version,
6998|                "capabilityCount": grants.as_ref().map(|g| g.len()).unwrap_or(0),
6999|            })
7000|        }).collect();
7001|    Json(json!({ "modules": list })).into_response()
7002|}
7003|
7004|async fn get_module(
7005|    State(state): State<Arc<ApiState>>,
7006|    Authed(claims): Authed,
7007|    Path(id): Path<String>,
7008|) -> Response {
7009|    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
7010|    let Ok(module_id) = ModuleId::parse(&id) else {
7011|        return (StatusCode::BAD_REQUEST, "invalid module ID").into_response();
7012|    };
7013|    let Some(m) = state.session.module(&module_id) else {
7014|        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
7015|    };
7016|    let grants: Vec<_> = state.session.grants(&module_id).map(|g| {
7017|        g.iter().map(|(cap, grant)| json!({
7018|            "capability": cap,
7019|            "constraints": grant.constraints,
7020|            "source": grant.source,
7021|        })).collect()
7022|    }).unwrap_or_default();
7023|    Json(json!({ "id": m.id.to_string(), "version": m.version, "grants": grants }))
7024|        .into_response()
7025|}
7026|
7027|async fn get_capabilities(
7028|    State(state): State<Arc<ApiState>>,
7029|    Authed(claims): Authed,
7030|    Path(id): Path<String>,
7031|) -> Response {
7032|    get_module(State(state), Authed(claims), Path(id)).await
7033|}
7034|
7035|async fn list_origins(
7036|    State(state): State<Arc<ApiState>>,
7037|    Authed(claims): Authed,
7038|    Path((id, cap)): Path<(String, String)>,
7039|) -> Response {
7040|    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
7041|    let Ok(module_id) = ModuleId::parse(&id) else {
7042|        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
7043|    };
7044|    let origins = state.session.grant_store().origins_for(&module_id, &cap);
7045|    Json(json!({ "origins": origins })).into_response()
7046|}
7047|
7048|#[derive(Deserialize)]
7049|struct AddOriginRequest {
7050|    origin: String,
7051|    #[serde(default)] remember_for: Option<String>,
7052|}
7053|
7054|async fn add_origin(
7055|    State(state): State<Arc<ApiState>>,
7056|    Authed(claims): Authed,
7057|    Path((id, cap)): Path<(String, String)>,
7058|    Json(req): Json<AddOriginRequest>,
7059|) -> Response {
7060|    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
7061|    let Ok(module_id) = ModuleId::parse(&id) else {
7062|        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
7063|    };
7064|    let expires_at = match req.remember_for.as_deref() {
7065|        Some("always") => None,
7066|        Some("session") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(86400)),
7067|        Some("module-version") => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(90 * 86400)),
7068|        _ => Some(std::time::SystemTime::now() + std::time::Duration::from_secs(3600)),
7069|    };
7070|    let grant = state.session.grant_store()
7071|        .add_origin(&module_id, &cap, &req.origin, expires_at);
7072|    state.session.broadcast_origin_granted(&grant);
7073|    Json(grant).into_response()
7074|}
7075|
7076|async fn remove_origin(
7077|    State(state): State<Arc<ApiState>>,
7078|    Authed(claims): Authed,
7079|    Path((id, cap, origin)): Path<(String, String, String)>,
7080|) -> Response {
7081|    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
7082|    let Ok(module_id) = ModuleId::parse(&id) else {
7083|        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
7084|    };
7085|    if state.session.grant_store().remove_origin(&module_id, &cap, &origin) {
7086|        state.session.broadcast_origin_revoked(&module_id.to_string(), &cap, &origin);
7087|        StatusCode::NO_CONTENT.into_response()
7088|    } else {
7089|        StatusCode::NOT_FOUND.into_response()
7090|    }
7091|}
7092|
7093|#[derive(Deserialize)]
7094|struct InstallRequest {
7095|    source: String,
7096|    #[serde(default, rename = "manifestPath")] manifest_path: Option<String>,
7097|    #[serde(default, rename = "artifactDir")] artifact_dir: Option<String>,
7098|}
7099|
7100|async fn install_module(
7101|    State(state): State<Arc<ApiState>>,
7102|    Authed(claims): Authed,
7103|    Json(req): Json<InstallRequest>,
7104|) -> Response {
7105|    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
7106|    let Some(pipeline) = state.install_pipeline.clone() else {
7107|        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
7108|            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
7109|            .into_response();
7110|    };
7111|    let source = match req.source.as_str() {
7112|        "local" => {
7113|            let manifest_path = match req.manifest_path {
7114|                Some(p) => p.into(),
7115|                None => return (StatusCode::BAD_REQUEST, "manifestPath required").into_response(),
7116|            };
7117|            let artifact_dir = req.artifact_dir.map(Into::into)
7118|                .unwrap_or_else(|| std::path::PathBuf::from("."));
7119|            InstallSource::Local { manifest_path, artifact_dir }
7120|        }
7121|        other => return (StatusCode::BAD_REQUEST, format!("unsupported source: {other}")).into_response(),
7122|    };
7123|    match pipeline.install(source).await {
7124|        Ok(report) => Json(report).into_response(),
7125|        Err(e) => {
7126|            let status = match &e {
7127|                dnea_session::install::InstallError::Signature(_) => StatusCode::UNAUTHORIZED,
7128|                dnea_session::install::InstallError::ConsentDenied { .. } => StatusCode::FORBIDDEN,
7129|                dnea_session::install::InstallError::AdminRequired { .. } => StatusCode::FORBIDDEN,
7130|                dnea_session::install::InstallError::Conflict { .. } => StatusCode::CONFLICT,
7131|                dnea_session::install::InstallError::MissingDependency { .. } => StatusCode::FAILED_DEPENDENCY,
7132|                _ => StatusCode::BAD_REQUEST,
7133|            };
7134|            (status, Json(json!({ "error": {
7135|                "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
7136|                .into_response()
7137|        }
7138|    }
7139|}
7140|
7141|async fn resume_install(
7142|    State(state): State<Arc<ApiState>>,
7143|    Authed(claims): Authed,
7144|    Path(install_id): Path<Uuid>,
7145|) -> Response {
7146|    if let Err(r) = Authed(claims).require(Scope::UiModules) { return r; }
7147|    let Some(pipeline) = state.install_pipeline.clone() else {
7148|        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "error": {
7149|            "code": "E_INTERNAL", "message": "install pipeline not configured" } })))
7150|            .into_response();
7151|    };
7152|    match pipeline.resume(install_id).await {
7153|        Ok(report) => Json(report).into_response(),
7154|        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "error": {
7155|            "code": "E_INSTALL_FAILED", "message": e.to_string() } })))
7156|            .into_response(),
7157|    }
7158|}
7159|
7160|async fn list_consent(State(state): State<Arc<ApiState>>, Authed(claims): Authed) -> Response {
7161|    if let Err(r) = Authed(claims).require(Scope::UiRead) { return r; }
7162|    let list: Vec<ConsentRequest> = state.session.open_consents();
7163|    Json(json!({ "prompts": list })).into_response()
7164|}
7165|
7166|#[derive(Deserialize)]
7167|struct ConsentResponseBody {
7168|    granted: bool,
7169|    #[serde(default)] scope: Option<serde_json::Value>,
7170|    #[serde(default)] remember_for: Option<String>,
7171|}
7172|
7173|async fn respond_consent(
7174|    State(state): State<Arc<ApiState>>,
7175|    Authed(claims): Authed,
7176|    Path(id): Path<Uuid>,
7177|    Json(body): Json<ConsentResponseBody>,
7178|) -> Response {
7179|    if let Err(r) = Authed(claims).require(Scope::UiConsent) { return r; }
7180|    let decision = ConsentDecision {
7181|        prompt_id: id,
7182|        granted: body.granted,
7183|        scope: body.scope,
7184|        remember_for: body.remember_for.and_then(|s| match s.as_str() {
7185|            "session" => Some(dnea_session::consent::RememberFor::Session),
7186|            "module-version" => Some(dnea_session::consent::RememberFor::ModuleVersion),
7187|            "always" => Some(dnea_session::consent::RememberFor::Always),
7188|            _ => None,
7189|        }),
7190|    };
7191|    match state.session.resolve_consent(decision) {
7192|        Ok(()) => StatusCode::NO_CONTENT.into_response(),
7193|        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
7194|    }
7195|}
7196|
7197|fn parse_scope(s: &str) -> Option<Scope> {
7198|    Some(match s {
7199|        "ui.read" => Scope::UiRead,
7200|        "ui.consent" => Scope::UiConsent,
7201|        "ui.modules" => Scope::UiModules,
7202|        "ui.settings" => Scope::UiSettings,
7203|        "ui.audit" => Scope::UiAudit,
7204|        "ui.admin" => Scope::UiAdmin,
7205|        _ => return None,
7206|    })
7207|}
7208|```
7209|
7210|### `crates/dnea-local-api/src/ws.rs`
7211|
7212|```rust
7213|//! WebSocket event stream.
7214|
7215|use std::sync::Arc;
7216|
7217|use axum::{
7218|    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
7219|    response::Response,
7220|};
7221|use dnea_session::SessionEvent;
7222|use serde_json::json;
7223|use tokio::sync::broadcast::error::RecvError;
7224|use tracing::debug;
7225|use uuid::Uuid;
7226|
7227|use crate::state::ApiState;
7228|
7229|pub async fn handler(State(state): State<Arc<ApiState>>, ws: WebSocketUpgrade) -> Response {
7230|    ws.on_upgrade(move |socket| run(socket, state))
7231|}
7232|
7233|async fn run(mut socket: WebSocket, state: Arc<ApiState>) {
7234|    let mut rx = state.session.subscribe();
7235|    let mut heartbeat = tokio::time::interval(std::time::Duration::from_secs(20));
7236|
7237|    loop {
7238|        tokio::select! {
7239|            event = rx.recv() => {
7240|                match event {
7241|                    Ok(ev) => {
7242|                        let frame = json!({
7243|                            "v": 1,
7244|                            "id": Uuid::now_v7().to_string(),
7245|                            "ts": time::OffsetDateTime::now_utc()
7246|                                .format(&time::format_description::well_known::Rfc3339)
7247|                                .unwrap_or_default(),
7248|                            "kind": "event",
7249|                            "topic": topic_of(&ev),
7250|                            "payload": ev,
7251|                        });
7252|                        if socket.send(Message::Text(frame.to_string().into())).await.is_err() {
7253|                            return;
7254|                        }
7255|                    }
7256|                    Err(RecvError::Lagged(n)) => { debug!(missed = n, "ws subscriber lagged"); }
7257|                    Err(RecvError::Closed) => return,
7258|                }
7259|            }
7260|            _ = heartbeat.tick() => {
7261|                let ping = json!({ "v": 1, "id": Uuid::now_v7().to_string(), "kind": "ping" });
7262|                if socket.send(Message::Text(ping.to_string().into())).await.is_err() { return; }
7263|            }
7264|            msg = socket.recv() => {
7265|                match msg {
7266|                    Some(Ok(Message::Close(_))) | None => return,
7267|                    Some(Ok(Message::Ping(p))) => { let _ = socket.send(Message::Pong(p)).await; }
7268|                    Some(Ok(_)) => {}
7269|                    Some(Err(_)) => return,
7270|                }
7271|            }
7272|        }
7273|    }
7274|}
7275|
7276|fn topic_of(ev: &SessionEvent) -> &'static str {
7277|    match ev {
7278|        SessionEvent::ModuleLoaded { .. } => "module/loaded",
7279|        SessionEvent::ModuleUnloaded { .. } => "module/unloaded",
7280|        SessionEvent::GrantChanged(_) => "capability/changed",
7281|        SessionEvent::ConsentOpened(_) => "consent/opened",
7282|        SessionEvent::ConsentResolved { .. } => "consent/resolved",
7283|        SessionEvent::ClientConnected { .. } => "client/connected",
7284|        SessionEvent::ClientDisconnected { .. } => "client/disconnected",
7285|        SessionEvent::OriginGranted { .. } => "origin/granted",
7286|        SessionEvent::OriginRevoked { .. } => "origin/revoked",
7287|    }
7288|}
7289|```
7290|
7291|### `crates/dnea-local-api/src/module_ui.rs`
7292|
7293|```rust
7294|//! Module panel serving.
7295|
7296|use std::sync::Arc;
7297|
7298|use axum::{
7299|    extract::{Path, State},
7300|    http::StatusCode,
7301|    response::{Html, IntoResponse, Response},
7302|    Json,
7303|};
7304|use dnea_core::ids::ModuleId;
7305|use serde::Deserialize;
7306|use serde_json::json;
7307|use tracing::warn;
7308|
7309|use crate::state::ApiState;
7310|
7311|const BRIDGE_JS: &str = include_str!("../assets/dnea-bridge.js");
7312|
7313|pub async fn serve_panel(
7314|    State(state): State<Arc<ApiState>>,
7315|    Path((module_id, panel_id)): Path<(String, String)>,
7316|) -> Response {
7317|    let Ok(mid) = ModuleId::parse(&module_id) else {
7318|        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
7319|    };
7320|    let Some(record) = state.session.module(&mid) else {
7321|        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
7322|    };
7323|    let Some(manifest) = record.manifest() else {
7324|        return (StatusCode::NOT_FOUND, "module manifest unavailable").into_response();
7325|    };
7326|    let Some(surface) = manifest.ui.iter().find(|s| s.id == panel_id) else {
7327|        return (StatusCode::NOT_FOUND, "panel not declared").into_response();
7328|    };
7329|    if !surface.surfaces.iter().any(|s| s == "companion") {
7330|        return (StatusCode::FORBIDDEN, "panel is not a companion surface").into_response();
7331|    }
7332|
7333|    let title = surface.title.as_deref().unwrap_or("DNEA Module");
7334|    let html = format!(
7335|        r#"<!doctype html>
7336|<html>
7337|<head>
7338|<meta charset="utf-8">
7339|<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'wasm-unsafe-eval'; style-src 'unsafe-inline'; img-src data:;">
7340|<title>{title}</title>
7341|</head>
7342|<body>
7343|<div id="dnea-root"></div>
7344|<script>{bridge}</script>
7345|<script>
7346|window.dnea.onReady(function () {{
7347|  window.dnea.invoke("render", {{ panelId: "{panel_id}" }})
7348|    .then(function (html) {{ document.getElementById("dnea-root").innerHTML = html; }})
7349|    .catch(function (e) {{ document.getElementById("dnea-root").textContent =
7350|      "Panel failed to render: " + e.message; }});
7351|}});
7352|</script>
7353|</body>
7354|</html>"#,
7355|        title = title, panel_id = panel_id, bridge = BRIDGE_JS,
7356|    );
7357|    Html(html).into_response()
7358|}
7359|
7360|#[derive(Deserialize)]
7361|pub struct InvokeBody {
7362|    method: String,
7363|    #[serde(default)]
7364|    args: serde_json::Value,
7365|}
7366|
7367|pub async fn invoke(
7368|    State(state): State<Arc<ApiState>>,
7369|    Path((module_id, _panel_id)): Path<(String, String)>,
7370|    Json(body): Json<InvokeBody>,
7371|) -> Response {
7372|    let Ok(mid) = ModuleId::parse(&module_id) else {
7373|        return (StatusCode::BAD_REQUEST, "invalid module id").into_response();
7374|    };
7375|    let Some(record) = state.session.module(&mid) else {
7376|        return (StatusCode::NOT_FOUND, "module not loaded").into_response();
7377|    };
7378|    let args = serde_json::to_vec(&body.args).unwrap_or_default();
7379|    match record.handle.invoke(&body.method, args).await {
7380|        Ok(bytes) => {
7381|            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!(null));
7382|            Json(json!({ "ok": true, "value": value })).into_response()
7383|        }
7384|        Err(e) => {
7385|            warn!(module = %mid, method = %body.method, error = %e, "invoke failed");
7386|            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
7387|                "ok": false, "error": e.to_string()
7388|            }))).into_response()
7389|        }
7390|    }
7391|}
7392|```
7393|
7394|### `crates/dnea-local-api/src/static_assets.rs`
7395|
7396|```rust
7397|//! Embedded companion UI assets.
7398|
7399|use axum::http::header;
7400|use axum::response::{IntoResponse, Response};
7401|
7402|pub fn index() -> Response {
7403|    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")],
7404|     dnea_companion_web::INDEX_HTML).into_response()
7405|}
7406|
7407|pub fn app_js() -> Response {
7408|    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
7409|     dnea_companion_web::APP_JS).into_response()
7410|}
7411|
7412|pub fn style_css() -> Response {
7413|    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")],
7414|     dnea_companion_web::STYLE_CSS).into_response()
7415|}
7416|```
7417|
7418|### `crates/dnea-local-api/assets/dnea-bridge.js`
7419|
7420|```javascript
7421|(function () {
7422|  "use strict";
7423|
7424|  const readyHandlers = [];
7425|  let ready = false;
7426|
7427|  function fireReady() {
7428|    ready = true;
7429|    for (const h of readyHandlers) {
7430|      try { h(); } catch (e) { /* swallow */ }
7431|    }
7432|  }
7433|
7434|  async function invoke(method, args) {
7435|    const url = new URL(location.href);
7436|    const base = url.pathname.replace(/\/$/, "");
7437|    const res = await fetch(base + "/invoke", {
7438|      method: "POST",
7439|      headers: { "content-type": "application/json" },
7440|      body: JSON.stringify({ method, args: args ?? {} }),
7441|    });
7442|    if (!res.ok) {
7443|      const text = await res.text();
7444|      throw new Error(text || ("HTTP " + res.status));
7445|    }
7446|    const body = await res.json();
7447|    if (!body.ok) throw new Error(body.error || "invoke failed");
7448|    return body.value;
7449|  }
7450|
7451|  async function settingsGet() { return invoke("$settings.get", {}); }
7452|  async function settingsSet(patch) { return invoke("$settings.set", patch); }
7453|
7454|  function on(event, handler) {
7455|    const channel = "dnea-" + (location.pathname.split("/")[2] || "module");
7456|    const wrapped = (e) => {
7457|      if (e.source !== window.parent) return;
7458|      if (e.data?.channel !== channel) return;
7459|      if (e.data?.event !== event) return;
7460|      handler(e.data.data);
7461|    };
7462|    window.addEventListener("message", wrapped);
7463|    return () => window.removeEventListener("message", wrapped);
7464|  }
7465|
7466|  async function requestOrigin(capabilityId, origin) {
7467|    return invoke("$origin.request", { capabilityId, origin });
7468|  }
7469|
7470|  async function notify(input) { return invoke("$notify", input); }
7471|
7472|  window.dnea = {
7473|    invoke,
7474|    settings: { get: settingsGet, set: settingsSet },
7475|    on,
7476|    requestOrigin,
7477|    notify,
7478|    onReady(handler) {
7479|      if (ready) handler();
7480|      else readyHandlers.push(handler);
7481|    },
7482|    context: {
7483|      moduleId: location.pathname.split("/")[2] || "",
7484|      panelId: location.pathname.split("/")[4] || "",
7485|    },
7486|  };
7487|
7488|  if (document.readyState === "loading") {
7489|    document.addEventListener("DOMContentLoaded", fireReady);
7490|  } else {
7491|    fireReady();
7492|  }
7493|})();
7494|```
7495|
7496|### `crates/dnea-local-api/tests/e2e_companion.rs`
7497|
7498|```rust
7499|//! Companion end-to-end test.
7500|
7501|use std::sync::Arc;
7502|use std::time::Duration;
7503|
7504|use dnea_core::ids::ModuleId;
7505|use dnea_local_api::{ApiConfig, LocalApi};
7506|use dnea_module_host::{grant::GrantSet, InstanceHandle};
7507|use dnea_session::{
7508|    consent::{ConsentRequest, RememberFor},
7509|    registry::ModuleRecord,
7510|    Session,
7511|};
7512|use serde_json::json;
7513|use tempfile::TempDir;
7514|
7515|fn dummy_handle() -> Arc<InstanceHandle> {
7516|    InstanceHandle::new_for_test(
7517|        "test".into(), "0.0.0-test".into(), Arc::new(GrantSet::new()))
7518|}
7519|
7520|#[tokio::test(flavor = "multi_thread")]
7521|async fn companion_can_list_modules_and_respond_to_consent() {
7522|    let session = Session::new();
7523|    session.register_module(Arc::new(ModuleRecord {
7524|        id: ModuleId::parse("com.example.focus-mode").unwrap(),
7525|        version: "0.0.0-test".into(),
7526|        handle: dummy_handle(),
7527|        manifest: None,
7528|        loaded_at: std::time::SystemTime::now(),
7529|    }));
7530|
7531|    let tmp = TempDir::new().unwrap();
7532|    let config = ApiConfig {
7533|        bind: "127.0.0.1:0".parse().unwrap(),
7534|        discovery_path: tmp.path().join("api.json"),
7535|        token_ttl: Duration::from_secs(600),
7536|    };
7537|    let api = LocalApi::bind(session.clone(), config).await.unwrap();
7538|    let addr = api.local_addr().unwrap();
7539|    let server = tokio::spawn(api.serve());
7540|    tokio::time::sleep(Duration::from_millis(100)).await;
7541|
7542|    let client = reqwest::Client::new();
7543|    let token_res = client
7544|        .post(format!("http://{addr}/v1/auth/token"))
7545|        .json(&json!({
7546|            "grantType": "companion",
7547|            "requestedScopes": ["ui.read", "ui.consent", "ui.modules"]
7548|        }))
7549|        .send().await.unwrap();
7550|    assert_eq!(token_res.status(), 200);
7551|    let token_body: serde_json::Value = token_res.json().await.unwrap();
7552|    let token = token_body["token"].as_str().unwrap().to_string();
7553|
7554|    let modules_res = client
7555|        .get(format!("http://{addr}/v1/modules"))
7556|        .bearer_auth(&token)
7557|        .send().await.unwrap();
7558|    assert_eq!(modules_res.status(), 200);
7559|    let modules: serde_json::Value = modules_res.json().await.unwrap();
7560|    assert_eq!(modules["modules"].as_array().unwrap().len(), 1);
7561|
7562|    let prompt_id = uuid::Uuid::now_v7();
7563|    let request = ConsentRequest {
7564|        prompt_id,
7565|        module: "com.example.focus-mode".into(),
7566|        module_name: "Focus Mode".into(),
7567|        capability: "browser:tabs:query".into(),
7568|        tier: "prompt".into(),
7569|        reason: "Detect when you open a blocked site.".into(),
7570|        constraints: json!({ "tabScopes": ["all-windows"] }),
7571|        target_description: Some("Query open tabs".into()),
7572|        expires_at: std::time::SystemTime::now() + Duration::from_secs(300),
7573|        can_remember: vec![RememberFor::Session, RememberFor::ModuleVersion],
7574|    };
7575|    let session_clone = session.clone();
7576|    let consent_task = tokio::spawn(async move {
7577|        session_clone.request_consent(request).await
7578|    });
7579|
7580|    tokio::time::sleep(Duration::from_millis(50)).await;
7581|    let pending_res = client
7582|        .get(format!("http://{addr}/v1/consent/pending"))
7583|        .bearer_auth(&token)
7584|        .send().await.unwrap();
7585|    let pending: serde_json::Value = pending_res.json().await.unwrap();
7586|    assert_eq!(pending["prompts"].as_array().unwrap().len(), 1);
7587|
7588|    let respond_res = client
7589|        .post(format!("http://{addr}/v1/consent/{prompt_id}/respond"))
7590|        .bearer_auth(&token)
7591|        .json(&json!({ "granted": true, "remember_for": "module-version" }))
7592|        .send().await.unwrap();
7593|    assert_eq!(respond_res.status(), 204);
7594|
7595|    tokio::time::timeout(Duration::from_secs(2), consent_task)
7596|        .await.expect("consent decision").unwrap().unwrap();
7597|
7598|    let module_id = ModuleId::parse("com.example.focus-mode").unwrap();
7599|    let grants = session.grants(&module_id).expect("grants");
7600|    assert!(grants.get("browser:tabs:query").is_some());
7601|
7602|    server.abort();
7603|}
7604|```
7605|
7606|Add `uuid` to `dnea-local-api/Cargo.toml` dev-dependencies.
7607|
7608|## 3.9 `crates/dnea-companion-web`
7609|
7610|### `crates/dnea-companion-web/Cargo.toml`
7611|
7612|```toml
7613|[package]
7614|name        = "dnea-companion-web"
7615|version.workspace = true
7616|description = "Embedded companion UI assets."
7617|edition.workspace     = true
7618|rust-version.workspace = true
7619|license.workspace     = true
7620|```
7621|
7622|### `crates/dnea-companion-web/src/lib.rs`
7623|
7624|```rust
7625|//! Embedded companion UI.
7626|
7627|#![forbid(unsafe_code)]
7628|#![warn(missing_docs)]
7629|
7630|/// The companion UI HTML.
7631|pub const INDEX_HTML: &str = include_str!("../assets/index.html");
7632|/// The companion UI JavaScript.
7633|pub const APP_JS: &str = include_str!("../assets/app.js");
7634|/// The companion UI stylesheet.
7635|pub const STYLE_CSS: &str = include_str!("../assets/style.css");
7636|```
7637|
7638|### `crates/dnea-companion-web/assets/index.html`
7639|
7640|```html
7641|<!doctype html>
7642|<html lang="en">
7643|<head>
7644|<meta charset="utf-8">
7645|<meta name="viewport" content="width=device-width, initial-scale=1">
7646|<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self' ws: http://127.0.0.1:*; frame-ancestors 'none'; base-uri 'none'; form-action 'self';">
7647|<title>DNEA</title>
7648|<link rel="stylesheet" href="/style.css">
7649|</head>
7650|<body>
7651|<header>
7652|  <h1>DNEA</h1>
7653|  <span id="connection" class="badge">connecting…</span>
7654|</header>
7655|
7656|<main>
7657|  <section id="status-section">
7658|    <h2>Runtime</h2>
7659|    <dl id="status"></dl>
7660|  </section>
7661|
7662|  <section id="modules-section">
7663|    <h2>Modules</h2>
7664|    <ul id="modules" class="list"></ul>
7665|  </section>
7666|
7667|  <section id="consent-section" hidden>
7668|    <h2>Pending consent</h2>
7669|    <ul id="consent" class="list"></ul>
7670|  </section>
7671|
7672|  <section id="events-section">
7673|    <h2>Events</h2>
7674|    <ul id="events" class="list small"></ul>
7675|  </section>
7676|</main>
7677|
7678|<script src="/app.js"></script>
7679|</body>
7680|</html>
7681|```
7682|
7683|### `crates/dnea-companion-web/assets/style.css`
7684|
7685|```css
7686|:root {
7687|  --bg: #0f1115; --fg: #e6e8eb; --muted: #9aa0a6;
7688|  --panel: #171a21; --border: #2a2f38;
7689|  --accent: #6ea8fe; --ok: #7bd88f; --err: #ff6b6b;
7690|}
7691|* { box-sizing: border-box; }
7692|body {
7693|  margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
7694|  background: var(--bg); color: var(--fg); font-size: 14px; line-height: 1.5;
7695|}
7696|header {
7697|  display: flex; align-items: center; gap: 12px;
7698|  padding: 16px 24px; border-bottom: 1px solid var(--border);
7699|}
7700|header h1 { margin: 0; font-size: 16px; font-weight: 600; }
7701|.badge {
7702|  font-size: 11px; padding: 2px 8px; border-radius: 10px;
7703|  background: var(--panel); color: var(--muted); border: 1px solid var(--border);
7704|}
7705|.badge.ok  { color: var(--ok);  border-color: var(--ok); }
7706|.badge.err { color: var(--err); border-color: var(--err); }
7707|main { padding: 24px; max-width: 960px; margin: 0 auto; }
7708|section { margin-bottom: 32px; }
7709|section h2 {
7710|  font-size: 13px; text-transform: uppercase; color: var(--muted);
7711|  margin: 0 0 12px; letter-spacing: .05em;
7712|}
7713|dl#status { display: grid; grid-template-columns: 160px 1fr; gap: 4px 16px; margin: 0; }
7714|dl#status dt { color: var(--muted); }
7715|dl#status dd { margin: 0; }
7716|.list { list-style: none; padding: 0; margin: 0; }
7717|.list li {
7718|  background: var(--panel); border: 1px solid var(--border);
7719|  border-radius: 6px; padding: 12px 16px; margin-bottom: 8px;
7720|}
7721|.list.small li { padding: 6px 12px; font-size: 12px; color: var(--muted); }
7722|.card-title { font-weight: 600; margin-bottom: 4px; }
7723|.card-sub   { color: var(--muted); font-size: 12px; }
7724|.consent-actions { display: flex; gap: 8px; margin-top: 12px; }
7725|button {
7726|  font: inherit; padding: 6px 12px; border-radius: 4px;
7727|  border: 1px solid var(--border); background: var(--panel);
7728|  color: var(--fg); cursor: pointer;
7729|}
7730|button.primary { background: var(--accent); color: #000; border-color: var(--accent); }
7731|button.danger  { color: var(--err); border-color: var(--err); }
7732|button:hover { filter: brightness(1.1); }
7733|button:focus { outline: 2px solid var(--accent); outline-offset: 2px; }
7734|```
7735|
7736|### `crates/dnea-companion-web/assets/app.js`
7737|
7738|```javascript
7739|(() => {
7740|  "use strict";
7741|
7742|  const $ = (sel) => document.querySelector(sel);
7743|  const state = { token: null, tokenExpiresAt: 0, ws: null, reconnectDelay: 500, refreshFails: 0 };
7744|
7745|  async function ensureToken() {
7746|    if (state.token && Date.now() < state.tokenExpiresAt - 60_000) return;
7747|    const res = await fetch("/v1/auth/token", {
7748|      method: "POST",
7749|      headers: { "content-type": "application/json" },
7750|      body: JSON.stringify({
7751|        grantType: "companion",
7752|        requestedScopes: ["ui.read", "ui.consent", "ui.modules", "ui.settings", "ui.audit"],
7753|      }),
7754|    });
7755|    if (!res.ok) throw new Error("auth failed: " + res.status);
7756|    const body = await res.json();
7757|    state.token = body.token;
7758|    state.tokenExpiresAt = Date.now() + body.expiresIn * 1000;
7759|    state.refreshFails = 0;
7760|  }
7761|
7762|  async function api(path, init = {}) {
7763|    await ensureToken();
7764|    const headers = Object.assign(
7765|      { "authorization": "Bearer " + state.token, "content-type": "application/json" },
7766|      init.headers || {});
7767|    const res = await fetch(path, Object.assign({}, init, { headers }));
7768|    if (res.status === 401) {
7769|      state.token = null;
7770|      state.refreshFails++;
7771|      if (state.refreshFails >= 3) {
7772|        setConnection("auth failed", "err");
7773|        throw new Error("auth failed repeatedly");
7774|      }
7775|      return api(path, init);
7776|    }
7777|    return res;
7778|  }
7779|
7780|  function setConnection(text, cls) {
7781|    const el = $("#connection");
7782|    el.textContent = text;
7783|    el.className = "badge " + (cls || "");
7784|  }
7785|
7786|  async function refreshStatus() {
7787|    const res = await api("/v1/status");
7788|    if (!res.ok) return;
7789|    const s = await res.json();
7790|    const dl = $("#status");
7791|    dl.innerHTML = "";
7792|    for (const [k, v] of [
7793|      ["Version", s.runtimeVersion],
7794|      ["Protocol", "v" + s.protocolVersion],
7795|      ["Session", s.sessionId],
7796|      ["Clients", String(s.clients.length)],
7797|      ["Modules", String(s.modules.length)],
7798|    ]) {
7799|      const dt = document.createElement("dt"); dt.textContent = k;
7800|      const dd = document.createElement("dd"); dd.textContent = v;
7801|      dl.appendChild(dt); dl.appendChild(dd);
7802|    }
7803|  }
7804|
7805|  async function refreshModules() {
7806|    const res = await api("/v1/modules");
7807|    if (!res.ok) return;
7808|    const { modules } = await res.json();
7809|    const ul = $("#modules");
7810|    ul.innerHTML = "";
7811|    if (modules.length === 0) {
7812|      const li = document.createElement("li");
7813|      li.className = "card-sub";
7814|      li.textContent = "No modules loaded.";
7815|      ul.appendChild(li);
7816|      return;
7817|    }
7818|    for (const m of modules) {
7819|      const li = document.createElement("li");
7820|      const t = document.createElement("div");
7821|      t.className = "card-title";
7822|      t.textContent = m.id;
7823|      const s = document.createElement("div");
7824|      s.className = "card-sub";
7825|      s.textContent = "v" + m.version + " · " + m.capabilityCount + " grants";
7826|      li.appendChild(t); li.appendChild(s);
7827|      ul.appendChild(li);
7828|    }
7829|  }
7830|
7831|  async function refreshConsent() {
7832|    const res = await api("/v1/consent/pending");
7833|    if (!res.ok) return;
7834|    const { prompts } = await res.json();
7835|    const section = $("#consent-section");
7836|    const ul = $("#consent");
7837|    ul.innerHTML = "";
7838|    if (prompts.length === 0) { section.hidden = true; return; }
7839|    section.hidden = false;
7840|    for (const p of prompts) {
7841|      const li = document.createElement("li");
7842|      const t = document.createElement("div");
7843|      t.className = "card-title";
7844|      t.textContent = p.module_name + " wants " + p.capability;
7845|      const r = document.createElement("div");
7846|      r.className = "card-sub";
7847|      r.textContent = p.reason;
7848|      const actions = document.createElement("div");
7849|      actions.className = "consent-actions";
7850|
7851|      const allow = document.createElement("button");
7852|      allow.className = "primary";
7853|      allow.textContent = "Allow for this version";
7854|      allow.onclick = () => respond(p.prompt_id, true, "module-version");
7855|
7856|      const always = document.createElement("button");
7857|      always.textContent = "Always allow";
7858|      always.onclick = () => respond(p.prompt_id, true, "always");
7859|
7860|      const deny = document.createElement("button");
7861|      deny.className = "danger";
7862|      deny.textContent = "Deny";
7863|      deny.onclick = () => respond(p.prompt_id, false);
7864|
7865|      actions.appendChild(allow);
7866|      actions.appendChild(always);
7867|      actions.appendChild(deny);
7868|      li.appendChild(t); li.appendChild(r); li.appendChild(actions);
7869|      ul.appendChild(li);
7870|    }
7871|  }
7872|
7873|  async function respond(promptId, granted, rememberFor) {
7874|    await api("/v1/consent/" + encodeURIComponent(promptId) + "/respond", {
7875|      method: "POST",
7876|      body: JSON.stringify({ granted, remember_for: rememberFor || null }),
7877|    });
7878|    await refreshConsent();
7879|  }
7880|
7881|  function pushEvent(ev) {
7882|    const ul = $("#events");
7883|    const li = document.createElement("li");
7884|    const time = new Date().toLocaleTimeString();
7885|    li.textContent = time + " — " + (ev.topic || "event");
7886|    ul.prepend(li);
7887|    while (ul.children.length > 50) ul.removeChild(ul.lastChild);
7888|  }
7889|
7890|  function connectWs() {
7891|    const proto = location.protocol === "https:" ? "wss:" : "ws:";
7892|    const url = proto + "//" + location.host + "/ws";
7893|    const ws = new WebSocket(url);
7894|    state.ws = ws;
7895|    setConnection("connecting…", "");
7896|
7897|    ws.onopen = () => {
7898|      setConnection("connected", "ok");
7899|      state.reconnectDelay = 500;
7900|    };
7901|    ws.onmessage = (e) => {
7902|      try {
7903|        const frame = JSON.parse(e.data);
7904|        if (frame.kind === "event") {
7905|          pushEvent(frame);
7906|          if (frame.topic === "consent/opened" || frame.topic === "consent/resolved") {
7907|            refreshConsent();
7908|          }
7909|          if (frame.topic && frame.topic.startsWith("module/")) {
7910|            refreshModules();
7911|          }
7912|        }
7913|      } catch (_) { /* ignore */ }
7914|    };
7915|    ws.onclose = () => {
7916|      setConnection("disconnected", "err");
7917|      setTimeout(connectWs, state.reconnectDelay);
7918|      state.reconnectDelay = Math.min(state.reconnectDelay * 2, 30_000);
7919|    };
7920|    ws.onerror = () => ws.close();
7921|  }
7922|
7923|  async function main() {
7924|    try { await ensureToken(); }
7925|    catch (e) { setConnection("auth failed", "err"); return; }
7926|    await refreshStatus();
7927|    await refreshModules();
7928|    await refreshConsent();
7929|    connectWs();
7930|    setInterval(refreshStatus, 5000);
7931|    setInterval(refreshModules, 10_000);
7932|  }
7933|
7934|  main();
7935|})();
7936|```
7937|
7938|## 3.10 `crates/dnea-daemon`
7939|
7940|### `crates/dnea-daemon/Cargo.toml`
7941|
7942|```toml
7943|[package]
7944|name        = "dnea-daemon"
7945|version.workspace = true
7946|description = "DNEA runtime daemon."
7947|edition.workspace     = true
7948|rust-version.workspace = true
7949|license.workspace     = true
7950|
7951|[[bin]]
7952|name = "dnea-daemon"
7953|path = "src/main.rs"
7954|
7955|[dependencies]
7956|dnea-config        = { path = "../dnea-config" }
7957|dnea-core          = { path = "../dnea-core" }
7958|dnea-local-api     = { path = "../dnea-local-api" }
7959|dnea-manifest      = { path = "../dnea-manifest" }
7960|dnea-module-host   = { path = "../dnea-module-host" }
7961|dnea-nm-host       = { path = "../dnea-nm-host" }
7962|dnea-session       = { path = "../dnea-session" }
7963|anyhow             = { workspace = true }
7964|base64             = { workspace = true }
7965|clap               = { workspace = true }
7966|rand               = { workspace = true }
7967|serde_json         = { workspace = true }
7968|tokio              = { workspace = true }
7969|tracing            = { workspace = true }
7970|tracing-subscriber = { workspace = true }
7971|wasmtime           = { workspace = true }
7972|
7973|[target.'cfg(unix)'.dependencies]
7974|libc = { workspace = true }
7975|```
7976|
7977|### `crates/dnea-daemon/src/main.rs`
7978|
7979|```rust
7980|//! The DNEA runtime daemon.
7981|
7982|use std::path::PathBuf;
7983|use std::sync::Arc;
7984|use std::time::Duration;
7985|
7986|use anyhow::{Context, Result};
7987|use base64::Engine as _;
7988|use clap::Parser;
7989|use dnea_config::Config;
7990|use dnea_local_api::{ApiConfig, LocalApi};
7991|use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
7992|use dnea_manifest::validate::Validator;
7993|use dnea_module_host::engine::EngineConfig;
7994|use dnea_module_host::grant::GrantSet;
7995|use dnea_module_host::host::HostState;
7996|use dnea_module_host::instance::ModuleHost;
7997|use dnea_nm_host::supervisor::{NmHostConfig, NmSupervisor};
7998|use dnea_session::install::{HostStateFactory, InstallPipeline};
7999|use dnea_session::Session;
8000|use tokio::signal;
8001|use tracing::{error, info};
8002|use tracing_subscriber::{fmt, prelude::*, EnvFilter};
8003|
8004|#[derive(Parser, Debug)]
8005|#[command(name = "dnea-daemon", version, about = "DNEA runtime daemon")]
8006|struct Args {
8007|    #[arg(long, env = "DNEA_CONFIG", default_value_os_t = Config::default_path())]
8008|    config: PathBuf,
8009|    #[arg(long, env = "DNEA_LOG", default_value = "info")]
8010|    log: String,
8011|    #[arg(long)]
8012|    log_json: bool,
8013|    #[arg(long)]
8014|    init_key: bool,
8015|}
8016|
8017|fn main() -> Result<()> {
8018|    let args = Args::parse();
8019|
8020|    let filter = EnvFilter::try_new(&args.log).context("invalid log filter")?;
8021|    let registry = tracing_subscriber::registry().with(filter);
8022|    if args.log_json {
8023|        registry.with(fmt::layer().json()).init();
8024|    } else {
8025|        registry.with(fmt::layer().compact()).init();
8026|    }
8027|
8028|    let config = if args.config.exists() {
8029|        Config::load_from(&args.config)
8030|            .with_context(|| format!("load config from {}", args.config.display()))?
8031|    } else {
8032|        let mut c = Config::default();
8033|        c.validate().context("validate default config")?;
8034|        c
8035|    };
8036|
8037|    if args.init_key { return init_master_key(&config); }
8038|
8039|    let rt = tokio::runtime::Builder::new_multi_thread()
8040|        .enable_all()
8041|        .thread_name("dnea")
8042|        .build()?;
8043|    rt.block_on(run(config))
8044|}
8045|
8046|fn init_master_key(config: &Config) -> Result<()> {
8047|    let path = config.paths.state_dir.join("master.key");
8048|    std::fs::create_dir_all(&config.paths.state_dir)?;
8049|    if path.exists() {
8050|        anyhow::bail!("master key already exists at {}", path.display());
8051|    }
8052|    let mut key = [0u8; 32];
8053|    use rand::RngCore;
8054|    rand::thread_rng().fill_bytes(&mut key);
8055|    std::fs::write(&path, key)?;
8056|    #[cfg(unix)]
8057|    {
8058|        use std::os::unix::fs::PermissionsExt;
8059|        let mut perms = std::fs::metadata(&path)?.permissions();
8060|        perms.set_mode(0o600);
8061|        std::fs::set_permissions(&path, perms)?;
8062|    }
8063|    info!(path = %path.display(), "master key initialized");
8064|    Ok(())
8065|}
8066|
8067|struct DaemonHostStateFactory;
8068|
8069|impl HostStateFactory for DaemonHostStateFactory {
8070|    fn build(&self, module_id: &str, version: &str, grants: Arc<GrantSet>) -> HostState {
8071|        HostState::new(
8072|            module_id.to_string(),
8073|            version.to_string(),
8074|            grants,
8075|            wasmtime::StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).build(),
8076|        )
8077|    }
8078|}
8079|
8080|async fn run(config: Config) -> Result<()> {
8081|    info!(version = env!("CARGO_PKG_VERSION"), "dnea-daemon starting");
8082|
8083|    // Single-instance guard.
8084|    let lock_path = config.paths.pid_file.clone();
8085|    let _lock = SingleInstance::acquire(&lock_path)
8086|        .context("another runtime is already running")?;
8087|
8088|    for dir in [
8089|        &config.paths.state_dir, &config.paths.run_dir,
8090|        &config.paths.log_dir, &config.paths.modules_dir,
8091|    ] {
8092|        std::fs::create_dir_all(dir)?;
8093|    }
8094|
8095|    // 1. Module host.
8096|    let engine_cfg = EngineConfig {
8097|        total_memory_slots: config.modules.pooling_slots,
8098|        total_stack_slots: config.modules.pooling_slots,
8099|        ..EngineConfig::default()
8100|    };
8101|    let module_host = Arc::new(ModuleHost::new(&engine_cfg).context("build module host")?);
8102|
8103|    // 2. Session.
8104|    let session = Session::new();
8105|
8106|    // 3. Trust store.
8107|    let trust: Arc<dyn dnea_manifest::trust::TrustStore> =
8108|        Arc::new(build_trust_store(config.modules.trust_store.as_deref()));
8109|
8110|    // 4. Validator.
8111|    let validator = Arc::new(Validator::new().context("build manifest validator")?);
8112|
8113|    // 5. Install pipeline.
8114|    let pipeline = Arc::new(
8115|        InstallPipeline::new(
8116|            session.clone(),
8117|            module_host.clone(),
8118|            validator,
8119|            trust,
8120|            Arc::new(DaemonHostStateFactory),
8121|        )
8122|        .with_install_roots(config.api.install_roots.clone()),
8123|    );
8124|
8125|    // 6. NM supervisor.
8126|    let nm_config = NmHostConfig::new(config.nm_endpoint());
8127|    let nm_supervisor = NmSupervisor::new(nm_config, Arc::new(session.clone()));
8128|
8129|    // 7. Local API.
8130|    let api_config = ApiConfig {
8131|        bind: "127.0.0.1:0".parse().context("invalid API bind address")?,
8132|        discovery_path: config.api.discovery_path.clone()
8133|            .unwrap_or_else(|| config.paths.run_dir.join("api.json")),
8134|        token_ttl: config.token_ttl(),
8135|    };
8136|    let api = LocalApi::bind_with_pipeline(session.clone(), api_config, pipeline)
8137|        .await.context("bind local api")?;
8138|
8139|    // 8. Watchdog.
8140|    let engine_clone = module_host.engine_clone();
8141|    let epoch_interval = config.epoch_interval();
8142|    let watchdog = tokio::spawn(async move {
8143|        let mut ticker = tokio::time::interval(epoch_interval);
8144|        loop {
8145|            ticker.tick().await;
8146|            engine_clone.increment_epoch();
8147|        }
8148|    });
8149|
8150|    // 9. Consent sweeper.
8151|    let session_clone = session.clone();
8152|    let sweeper = tokio::spawn(async move {
8153|        let mut ticker = tokio::time::interval(Duration::from_secs(30));
8154|        loop {
8155|            ticker.tick().await;
8156|            session_clone.sweep_consents(Duration::from_secs(300)).await;
8157|        }
8158|    });
8159|
8160|    // 10. Run.
8161|    let nm_task = tokio::spawn(async move {
8162|        if let Err(e) = nm_supervisor.run().await {
8163|            error!(error = %e, "nm supervisor failed");
8164|        }
8165|    });
8166|    let api_task = tokio::spawn(async move {
8167|        if let Err(e) = api.serve().await {
8168|            error!(error = %e, "local api failed");
8169|        }
8170|    });
8171|
8172|    // 11. Shutdown.
8173|    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
8174|    tokio::select! {
8175|        _ = signal::ctrl_c() => info!("ctrl-c received"),
8176|        _ = sigterm.recv() => info!("SIGTERM received"),
8177|    }
8178|    info!("shutting down");
8179|    watchdog.abort();
8180|    sweeper.abort();
8181|    nm_task.abort();
8182|    api_task.abort();
8183|    let _ = tokio::time::timeout(Duration::from_secs(5), session.shutdown()).await;
8184|    Ok(())
8185|}
8186|
8187|fn build_trust_store(path: Option<&std::path::Path>) -> StaticTrustStore {
8188|    let mut store = StaticTrustStore::new();
8189|    if let Some(p) = path {
8190|        if let Ok(bytes) = std::fs::read(p) {
8191|            if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) {
8192|                if let Some(keys) = v.get("keys").and_then(|k| k.as_array()) {
8193|                    for k in keys {
8194|                        let Some(id) = k.get("keyId").and_then(|v| v.as_str()) else { continue };
8195|                        let Some(alg) = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse)
8196|                            else { continue };
8197|                        let Some(pk_b64) = k.get("publicKey").and_then(|v| v.as_str()) else { continue };
8198|                        let Ok(pk) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)
8199|                            else { continue };
8200|                        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
8201|                        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
8202|                    }
8203|                }
8204|            }
8205|        }
8206|    }
8207|    store
8208|}
8209|
8210|#[cfg(unix)]
8211|struct SingleInstance { _file: std::fs::File }
8212|
8213|#[cfg(unix)]
8214|impl SingleInstance {
8215|    fn acquire(path: &std::path::Path) -> Result<Self> {
8216|        use std::os::unix::io::AsRawFd;
8217|        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).ok(); }
8218|        let file = std::fs::OpenOptions::new()
8219|            .create(true).write(true).truncate(false).open(path)?;
8220|        let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
8221|        if rc != 0 {
8222|            anyhow::bail!("another runtime is already running (lock held on {})", path.display());
8223|        }
8224|        std::fs::write(path, std::process::id().to_string()).ok();
8225|        Ok(Self { _file: file })
8226|    }
8227|}
8228|
8229|#[cfg(windows)]
8230|struct SingleInstance { _handle: isize }
8231|
8232|#[cfg(windows)]
8233|impl SingleInstance {
8234|    fn acquire(_path: &std::path::Path) -> Result<Self> {
8235|        use windows_sys::Win32::System::Threading::CreateMutexW;
8236|        use windows_sys::Win32::Foundation::GetLastError;
8237|        use windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS;
8238|        let name: Vec<u16> = "Global\\DNEA-Runtime\0".encode_utf16().collect();
8239|        let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
8240|        if handle == 0 { anyhow::bail!("CreateMutexW failed"); }
8241|        let err = unsafe { GetLastError() };
8242|        if err == ERROR_ALREADY_EXISTS {
8243|            anyhow::bail!("another runtime is already running");
8244|        }
8245|        Ok(Self { _handle: handle as isize })
8246|    }
8247|}
8248|```
8249|
8250|Add to `dnea-daemon/Cargo.toml` on Windows:
8251|
8252|```toml
8253|[target.'cfg(windows)'.dependencies]
8254|windows-sys = { version = "0.59", features = ["Win32_Foundation", "Win32_System_Threading"] }
8255|```
8256|
8257|## 3.11 `crates/dnea-cli`
8258|
8259|### `crates/dnea-cli/Cargo.toml`
8260|
8261|```toml
8262|[package]
8263|name        = "dnea-cli"
8264|version.workspace = true
8265|description = "DNEA developer CLI."
8266|edition.workspace     = true
8267|rust-version.workspace = true
8268|license.workspace     = true
8269|
8270|[[bin]]
8271|name = "dnea"
8272|path = "src/main.rs"
8273|
8274|[dependencies]
8275|dnea-config     = { path = "../dnea-config" }
8276|dnea-core       = { path = "../dnea-core" }
8277|dnea-manifest   = { path = "../dnea-manifest" }
8278|dnea-nm-host    = { path = "../dnea-nm-host" }
8279|anyhow          = { workspace = true }
8280|base64          = { workspace = true }
8281|clap            = { workspace = true }
8282|ed25519-dalek   = { workspace = true }
8283|rand            = { workspace = true }
8284|reqwest         = { workspace = true }
8285|serde_json      = { workspace = true }
8286|sha2            = { workspace = true }
8287|tokio           = { workspace = true }
8288|```
8289|
8290|### `crates/dnea-cli/src/main.rs`
8291|
8292|```rust
8293|//! The `dnea` CLI.
8294|
8295|use std::path::{Path, PathBuf};
8296|use std::process::ExitCode;
8297|
8298|use anyhow::{Context, Result};
8299|use base64::Engine as _;
8300|use clap::{Parser, Subcommand};
8301|use dnea_manifest::signature::{sign_manifest, verify_manifest};
8302|use dnea_manifest::trust::{ResolvedKey, SigAlg, StaticTrustStore};
8303|use dnea_manifest::validate::Validator;
8304|use dnea_nm_host::registration::{register, Browser};
8305|
8306|#[derive(Parser)]
8307|#[command(name = "dnea", version, about = "DNEA developer CLI")]
8308|struct Cli {
8309|    #[command(subcommand)]
8310|    cmd: Cmd,
8311|}
8312|
8313|#[derive(Subcommand)]
8314|enum Cmd {
8315|    Validate {
8316|        manifest: PathBuf,
8317|        #[arg(long)] artifacts: Option<PathBuf>,
8318|        #[arg(long)] no_sig: bool,
8319|        #[arg(long)] trust: Option<PathBuf>,
8320|    },
8321|    Sign {
8322|        manifest: PathBuf,
8323|        #[arg(long)] key: PathBuf,
8324|        #[arg(long)] key_id: String,
8325|        #[arg(long, default_value = "author")] role: String,
8326|        #[arg(long)] out: Option<PathBuf>,
8327|    },
8328|    Verify {
8329|        manifest: PathBuf,
8330|        #[arg(long)] trust: PathBuf,
8331|    },
8332|    Keygen {
8333|        #[arg(long, default_value = "dnea-signing.key")] out: PathBuf,
8334|    },
8335|    RegisterHost {
8336|        #[arg(long, value_enum)] browser: BrowserArg,
8337|        #[arg(long)] extension_id: String,
8338|        #[arg(long)] bridge: PathBuf,
8339|    },
8340|    Status {
8341|        #[arg(long)] discovery: Option<PathBuf>,
8342|    },
8343|    InstallModule {
8344|        manifest: PathBuf,
8345|        #[arg(long)] artifacts: PathBuf,
8346|        #[arg(long)] discovery: Option<PathBuf>,
8347|    },
8348|    Doctor {
8349|        #[arg(long)] discovery: Option<PathBuf>,
8350|    },
8351|}
8352|
8353|#[derive(Clone, Copy, Debug, clap::ValueEnum)]
8354|enum BrowserArg { Chrome, Chromium, Edge, Brave, Vivaldi, Opera, Firefox }
8355|
8356|impl From<BrowserArg> for Browser {
8357|    fn from(a: BrowserArg) -> Self {
8358|        match a {
8359|            BrowserArg::Chrome => Browser::Chrome,
8360|            BrowserArg::Chromium => Browser::Chromium,
8361|            BrowserArg::Edge => Browser::Edge,
8362|            BrowserArg::Brave => Browser::Brave,
8363|            BrowserArg::Vivaldi => Browser::Vivaldi,
8364|            BrowserArg::Opera => Browser::Opera,
8365|            BrowserArg::Firefox => Browser::Firefox,
8366|        }
8367|    }
8368|}
8369|
8370|fn main() -> ExitCode {
8371|    let cli = Cli::parse();
8372|    match run(cli) {
8373|        Ok(code) => code,
8374|        Err(e) => { eprintln!("dnea: {e:#}"); ExitCode::from(1) }
8375|    }
8376|}
8377|
8378|fn run(cli: Cli) -> Result<ExitCode> {
8379|    match cli.cmd {
8380|        Cmd::Validate { manifest, artifacts, no_sig, trust } =>
8381|            validate(&manifest, artifacts.as_deref(), no_sig, trust.as_deref()),
8382|        Cmd::Sign { manifest, key, key_id, role, out } =>
8383|            sign(&manifest, &key, &key_id, &role, out.as_deref()),
8384|        Cmd::Verify { manifest, trust } => verify(&manifest, &trust),
8385|        Cmd::Keygen { out } => keygen(&out),
8386|        Cmd::RegisterHost { browser, extension_id, bridge } =>
8387|            register_host(browser.into(), &extension_id, &bridge),
8388|        Cmd::Status { discovery } =>
8389|            tokio::runtime::Runtime::new()?.block_on(status(discovery.as_deref())),
8390|        Cmd::InstallModule { manifest, artifacts, discovery } =>
8391|            tokio::runtime::Runtime::new()?.block_on(install_module(&manifest, &artifacts, discovery.as_deref())),
8392|        Cmd::Doctor { discovery } =>
8393|            tokio::runtime::Runtime::new()?.block_on(doctor(discovery.as_deref())),
8394|    }
8395|}
8396|
8397|fn validate(
8398|    manifest: &PathBuf,
8399|    artifacts: Option<&Path>,
8400|    no_sig: bool,
8401|    trust: Option<&Path>,
8402|) -> Result<ExitCode> {
8403|    let bytes = std::fs::read(manifest)?;
8404|    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
8405|
8406|    let validator = Validator::new()?;
8407|    validator.validate(&value).context("semantic validation failed")?;
8408|    println!("✓ schema and semantics");
8409|
8410|    if !no_sig {
8411|        let trust_path = trust.context("--trust required unless --no-sig")?;
8412|        let store = load_trust(trust_path)?;
8413|        let info = verify_manifest(&value, &store).context("signature verification failed")?;
8414|        println!("✓ signatures: {} signer(s)", info.signers.len());
8415|        for s in &info.signers {
8416|            println!("  - {} ({})", s.key_id, s.role);
8417|        }
8418|    }
8419|
8420|    if let Some(dir) = artifacts {
8421|        let manifest: dnea_manifest::Manifest = serde_json::from_value(value)?;
8422|        for (name, ep) in &manifest.entrypoints {
8423|            let path = dir.join(&ep.path);
8424|            let bytes = std::fs::read(&path)
8425|                .with_context(|| format!("read {}", path.display()))?;
8426|            let actual = format!("sha-256:{}",
8427|                base64::engine::general_purpose::URL_SAFE_NO_PAD
8428|                    .encode(sha2::Digest::digest(&mut sha2::Sha256::new(), &bytes)));
8429|            if actual != ep.digest.value {
8430|                anyhow::bail!("artifact {name} digest mismatch");
8431|            }
8432|            println!("✓ artifact {name}");
8433|        }
8434|    }
8435|    Ok(ExitCode::SUCCESS)
8436|}
8437|
8438|fn sign(
8439|    manifest: &PathBuf, key_path: &PathBuf, key_id: &str, role: &str, out: Option<&Path>,
8440|) -> Result<ExitCode> {
8441|    let key_bytes = std::fs::read(key_path)?;
8442|    let signing_key = ed25519_dalek::SigningKey::from_bytes(
8443|        key_bytes.as_slice().try_into().context("signing key must be 32 bytes")?,
8444|    );
8445|    let bytes = std::fs::read(manifest)?;
8446|    let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;
8447|    sign_manifest(&mut value, &signing_key, key_id, role)?;
8448|    let target = out.unwrap_or(manifest.as_path());
8449|    std::fs::write(target, serde_json::to_vec_pretty(&value)?)?;
8450|    println!("signed manifest written to {}", target.display());
8451|    Ok(ExitCode::SUCCESS)
8452|}
8453|
8454|fn verify(manifest: &PathBuf, trust: &PathBuf) -> Result<ExitCode> {
8455|    let bytes = std::fs::read(manifest)?;
8456|    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
8457|    let store = load_trust(trust)?;
8458|    let info = verify_manifest(&value, &store)?;
8459|    println!("verified: {} signer(s)", info.signers.len());
8460|    for s in &info.signers { println!("  - {} ({})", s.key_id, s.role); }
8461|    Ok(ExitCode::SUCCESS)
8462|}
8463|
8464|fn keygen(out: &PathBuf) -> Result<ExitCode> {
8465|    use rand::rngs::OsRng;
8466|    let sk = ed25519_dalek::SigningKey::generate(&mut OsRng);
8467|    std::fs::write(out, sk.to_bytes())?;
8468|    #[cfg(unix)]
8469|    {
8470|        use std::os::unix::fs::PermissionsExt;
8471|        let mut perms = std::fs::metadata(out)?.permissions();
8472|        perms.set_mode(0o600);
8473|        std::fs::set_permissions(out, perms)?;
8474|    }
8475|    let vk = sk.verifying_key();
8476|    let vk_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(vk.to_bytes());
8477|    println!("private key: {}", out.display());
8478|    println!("public key (base64url): {}", vk_b64);
8479|    println!();
8480|    println!("Add to your trust store:");
8481|    println!("  {{ \"keyId\": \"...\", \"alg\": \"EdDSA\", \"publicKey\": \"{vk_b64}\" }}");
8482|    Ok(ExitCode::SUCCESS)
8483|}
8484|
8485|fn register_host(browser: Browser, extension_id: &str, bridge: &PathBuf) -> Result<ExitCode> {
8486|    let written = register(browser, &[extension_id], bridge, env!("CARGO_PKG_VERSION"))?;
8487|    for p in written { println!("✓ {}", p.display()); }
8488|    Ok(ExitCode::SUCCESS)
8489|}
8490|
8491|async fn status(discovery: Option<&Path>) -> Result<ExitCode> {
8492|    let disc = load_discovery(discovery)?;
8493|    let client = reqwest::Client::new();
8494|    let token: serde_json::Value = client
8495|        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
8496|        .json(&serde_json::json!({ "grantType": "cli", "requestedScopes": ["ui.read"] }))
8497|        .send().await?.json().await?;
8498|    let token = token["token"].as_str().context("no token")?;
8499|    let status: serde_json::Value = client
8500|        .get(format!("{}/v1/status", disc["url"].as_str().unwrap()))
8501|        .bearer_auth(token).send().await?.json().await?;
8502|    println!("{}", serde_json::to_string_pretty(&status)?);
8503|    Ok(ExitCode::SUCCESS)
8504|}
8505|
8506|async fn install_module(
8507|    manifest: &PathBuf, artifacts: &PathBuf, discovery: Option<&Path>,
8508|) -> Result<ExitCode> {
8509|    let disc = load_discovery(discovery)?;
8510|    let client = reqwest::Client::new();
8511|    let token: serde_json::Value = client
8512|        .post(format!("{}/v1/auth/token", disc["url"].as_str().unwrap()))
8513|        .json(&serde_json::json!({
8514|            "grantType": "cli", "requestedScopes": ["ui.modules", "ui.read"] }))
8515|        .send().await?.json().await?;
8516|    let token = token["token"].as_str().context("no token")?;
8517|    let report: serde_json::Value = client
8518|        .post(format!("{}/v1/modules", disc["url"].as_str().unwrap()))
8519|        .bearer_auth(token)
8520|        .json(&serde_json::json!({
8521|            "source": "local",
8522|            "manifestPath": manifest,
8523|            "artifactDir": artifacts,
8524|        }))
8525|        .send().await?.json().await?;
8526|    println!("{}", serde_json::to_string_pretty(&report)?);
8527|    Ok(ExitCode::SUCCESS)
8528|}
8529|
8530|async fn doctor(discovery: Option<&Path>) -> Result<ExitCode> {
8531|    let disc = load_discovery(discovery)?;
8532|    println!("Runtime: {}", disc["url"]);
8533|    println!("PID:     {}", disc["pid"]);
8534|    println!("Session: {}", disc["session_id"]);
8535|    let cfg = dnea_config::Config::load()?;
8536|    println!("Config:  {}", dnea_config::Config::default_path().display());
8537|    println!("State:   {}", cfg.paths.state_dir.display());
8538|    println!("Run:     {}", cfg.paths.run_dir.display());
8539|    println!("Log:     {}", cfg.paths.log_dir.display());
8540|    for browser in Browser::all() {
8541|        let dirs = browser.manifest_dirs();
8542|        let mut found = false;
8543|        for dir in &dirs {
8544|            if dir.join(format!("{}.json", browser.host_name())).exists() { found = true; break; }
8545|        }
8546|        println!("Host manifest for {:?}: {}", browser, if found { "present" } else { "missing" });
8547|    }
8548|    Ok(ExitCode::SUCCESS)
8549|}
8550|
8551|fn load_trust(path: &Path) -> Result<StaticTrustStore> {
8552|    let bytes = std::fs::read(path)?;
8553|    let v: serde_json::Value = serde_json::from_slice(&bytes)?;
8554|    let mut store = StaticTrustStore::new();
8555|    let keys = v.get("keys").and_then(|k| k.as_array())
8556|        .context("trust store must have a `keys` array")?;
8557|    for k in keys {
8558|        let id = k.get("keyId").and_then(|v| v.as_str()).context("keyId")?;
8559|        let alg = k.get("alg").and_then(|v| v.as_str()).and_then(SigAlg::parse).context("alg")?;
8560|        let pk_b64 = k.get("publicKey").and_then(|v| v.as_str()).context("publicKey")?;
8561|        let pk = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(pk_b64)?;
8562|        let revoked = k.get("revoked").and_then(|v| v.as_bool()).unwrap_or(false);
8563|        store.insert(id, ResolvedKey { alg, public_key: pk, revoked });
8564|    }
8565|    Ok(store)
8566|}
8567|
8568|fn load_discovery(path: Option<&Path>) -> Result<serde_json::Value> {
8569|    let p = match path {
8570|        Some(p) => p.to_path_buf(),
8571|        None => {
8572|            let cfg = dnea_config::Config::load()?;
8573|            cfg.api.discovery_path.clone()
8574|                .unwrap_or_else(|| cfg.paths.run_dir.join("api.json"))
8575|        }
8576|    };
8577|    let bytes = std::fs::read(&p).with_context(|| format!("read {}", p.display()))?;
8578|    Ok(serde_json::from_slice(&bytes)?)
8579|}
8580|```
8581|
8582|Add `dnea-config` to CLI deps (already listed).
8583|
8584|## 3.12 `crates/focus-mode`
8585|
8586|### `crates/focus-mode/Cargo.toml`
8587|
8588|```toml
8589|[package]
8590|name = "focus-mode"
8591|version = "1.2.0"
8592|edition = "2021"
8593|publish = false
8594|
8595|[lib]
8596|crate-type = ["cdylib"]
8597|
8598|[dependencies]
8599|serde      = { version = "1", features = ["derive"] }
8600|serde_json = "1"
8601|wit-bindgen = "0.30"
8602|
8603|[package.metadata.component]
8604|package = "dnea:focus-mode"
8605|
8606|[package.metadata.component.target]
8607|path = "../../wit"
8608|world = "module"
8609|```
8610|
8611|### `crates/focus-mode/src/lib.rs`
8612|
8613|```rust
8614|//! Focus Mode — reference DNEA module.
8615|
8616|wit_bindgen::generate!({
8617|    path: "../../wit",
8618|    world: "module",
8619|    generate_all,
8620|});
8621|
8622|use dnea::host::types::HostError;
8623|use exports::dnea::host::{
8624|    lifecycle::Guest as LifecycleGuest,
8625|    module_api::Guest as ModuleApiGuest,
8626|};
8627|
8628|struct FocusMode;
8629|
8630|#[derive(serde::Serialize, serde::Deserialize, Default)]
8631|struct State {
8632|    #[serde(default)] block_list: Vec<String>,
8633|    #[serde(default)] allow_list: Vec<String>,
8634|    #[serde(default)] strict_mode: bool,
8635|    #[serde(default)] sessions_started: u64,
8636|}
8637|
8638|const STATE_KEY: &str = "state";
8639|
8640|fn load_state() -> State {
8641|    match dnea::host::storage::get(STATE_KEY) {
8642|        Ok(Some(bytes)) => serde_json::from_slice(&bytes).unwrap_or_default(),
8643|        _ => State::default(),
8644|    }
8645|}
8646|
8647|fn save_state(state: &State) -> Result<(), HostError> {
8648|    let bytes = serde_json::to_vec(state)
8649|        .map_err(|e| HostError::Internal(e.to_string()))?;
8650|    dnea::host::storage::put(STATE_KEY, &bytes)
8651|}
8652|
8653|impl LifecycleGuest for FocusMode {
8654|    fn activate() -> Result<(), HostError> {
8655|        let state = load_state();
8656|        dnea::host::log::emit(
8657|            dnea::host::types::LogLevel::Info,
8658|            "focus-mode activated",
8659|            Some(&format!(r#"{{"sessions":{}}}"#, state.sessions_started)),
8660|        );
8661|        Ok(())
8662|    }
8663|    fn deactivate() -> Result<(), HostError> {
8664|        dnea::host::log::emit(
8665|            dnea::host::types::LogLevel::Info,
8666|            "focus-mode deactivated", None);
8667|        Ok(())
8668|    }
8669|    fn upgrade(from_version: String, _from_storage_schema: u32) -> Result<(), HostError> {
8670|        dnea::host::log::emit(
8671|            dnea::host::types::LogLevel::Info,
8672|            "focus-mode upgraded",
8673|            Some(&format!(r#"{{"from":"{}"}}"#, from_version)),
8674|        );
8675|        Ok(())
8676|    }
8677|    fn health_check() -> Result<(), HostError> {
8678|        let _ = load_state();
8679|        Ok(())
8680|    }
8681|}
8682|
8683|impl ModuleApiGuest for FocusMode {
8684|    fn invoke(method: String, args_json: String) -> Result<String, HostError> {
8685|        match method.as_str() {
8686|            "getState" => {
8687|                let state = load_state();
8688|                Ok(serde_json::to_string(&state).unwrap_or_else(|_| "{}".into()))
8689|            }
8690|            "setBlockList" => {
8691|                let args: serde_json::Value = serde_json::from_str(&args_json)
8692|                    .map_err(|e| HostError::Internal(e.to_string()))?;
8693|                let mut state = load_state();
8694|                state.block_list = args.get("list").and_then(|v| v.as_array())
8695|                    .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
8696|                    .unwrap_or_default();
8697|                save_state(&state)?;
8698|                Ok(r#"{"ok":true}"#.into())
8699|            }
8700|            "startSession" => {
8701|                let mut state = load_state();
8702|                state.sessions_started += 1;
8703|                save_state(&state)?;
8704|                Ok(format!(r#"{{"ok":true,"session":{}}}"#, state.sessions_started))
8705|            }
8706|            "onTabNavigated" => {
8707|                let args: serde_json::Value = serde_json::from_str(&args_json)
8708|                    .map_err(|e| HostError::Internal(e.to_string()))?;
8709|                let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
8710|                let state = load_state();
8711|                let host = host_of(url);
8712|                let blocked = state.block_list.iter().any(|h| host.ends_with(h.as_str()))
8713|                    && !state.allow_list.iter().any(|h| host.ends_with(h.as_str()));
8714|                if blocked {
8715|                    dnea::host::log::emit(
8716|                        dnea::host::types::LogLevel::Info,
8717|                        "blocked host",
8718|                        Some(&format!(r#"{{"host":"{}"}}"#, host)));
8719|                }
8720|                Ok(format!(r#"{{"blocked":{}}}"#, blocked))
8721|            }
8722|            "render" => {
8723|                let state = load_state();
8724|                let blocked = state.block_list.iter()
8725|                    .map(|h| format!("<li>{}</li>", html_escape(h)))
8726|                    .collect::<Vec<_>>().join("");
8727|                Ok(format!(
8728|                    r#"<h2>Focus Mode</h2>
8729|<p>Strict mode: {strict}</p>
8730|<p>Sessions: {sessions}</p>
8731|<ul>{blocked}</ul>"#,
8732|                    strict = state.strict_mode,
8733|                    sessions = state.sessions_started,
8734|                    blocked = blocked))
8735|            }
8736|            other => Err(HostError::Unavailable(format!("unknown method: {other}"))),
8737|        }
8738|    }
8739|}
8740|
8741|fn host_of(url: &str) -> &str {
8742|    url.split("://").nth(1).and_then(|s| s.split('/').next()).unwrap_or("")
8743|}
8744|
8745|fn html_escape(s: &str) -> String {
8746|    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
8747|}
8748|
8749|export!(FocusMode);
8750|```
8751|
8752|---
8753|
8754|# PART 4 — THIN CLIENT
8755|
8756|## `thin-client/package.json`
8757|
8758|```json
8759|{
8760|  "name": "dnea-thin-client",
8761|  "version": "1.0.0",
8762|  "private": true,
8763|  "type": "module",
8764|  "scripts": {
8765|    "dev": "wxt",
8766|    "build": "wxt build",
8767|    "build:chrome": "wxt build -b chrome",
8768|    "build:firefox": "wxt build -b firefox --mv2",
8769|    "build:edge": "wxt build -b edge",
8770|    "build:safari": "wxt build -b safari",
8771|    "compile": "tsc --noEmit",
8772|    "test": "vitest run"
8773|  },
8774|  "dependencies": { "wxt": "^0.19.0" },
8775|  "devDependencies": {
8776|    "@types/chrome": "^0.0.280",
8777|    "typescript": "^5.6.0",
8778|    "vitest": "^2.1.0"
8779|  }
8780|}
8781|```
8782|
8783|## `thin-client/tsconfig.json`
8784|
8785|```json
8786|{
8787|  "extends": "./.wxt/tsconfig.json",
8788|  "compilerOptions": {
8789|    "strict": true,
8790|    "noUncheckedIndexedAccess": true,
8791|    "noImplicitOverride": true,
8792|    "paths": { "@/*": ["./src/*"] }
8793|  }
8794|}
8795|```
8796|
8797|## `thin-client/wxt.config.ts`
8798|
8799|```ts
8800|import { defineConfig } from "wxt";
8801|
8802|export default defineConfig({
8803|  manifestVersion: 3,
8804|  manifest: ({ browser, manifestVersion }) => ({
8805|    name: "DNEA",
8806|    description: "Device-Native Extension Architecture thin client.",
8807|    version: "1.0.0",
8808|    permissions: ["nativeMessaging", "storage", "alarms", "notifications", "scripting"],
8809|    host_permissions: ["http://*/*", "https://*/*"],
8810|    background: manifestVersion === 3
8811|      ? { service_worker: "background.js", type: "module" }
8812|      : { scripts: ["background.js"], type: "module" },
8813|    content_scripts: [{
8814|      matches: ["http://*/*", "https://*/*"],
8815|      js: ["content.js"],
8816|      run_at: "document_start",
8817|      all_frames: true,
8818|      match_about_blank: true,
8819|    }],
8820|    action: { default_title: "DNEA", default_popup: "popup.html" },
8821|    options_ui: { page: "options.html", open_in_tab: true },
8822|    web_accessible_resources: [{
8823|      resources: ["consent.html"],
8824|      matches: ["http://*/*", "https://*/*"],
8825|    }],
8826|    ...(browser === "firefox" ? {
8827|      browser_specific_settings: {
8828|        gecko: { id: "dnea@dnea.org", strict_min_version: "115.0" },
8829|      },
8830|    } : {}),
8831|  }),
8832|  vite: () => ({
8833|    build: { target: "es2022", minify: "esbuild", sourcemap: false },
8834|  }),
8835|});
8836|```
8837|
8838|## `thin-client/src/lib/protocol.ts`
8839|
8840|```typescript
8841|export type Envelope = {
8842|  v: 1;
8843|  id: string;
8844|  type: string;
8845|  ts: string;
8846|  src: Party;
8847|  dst: Party;
8848|  reply_to?: string;
8849|  cap?: string;
8850|  mod?: string;
8851|  enc?: "identity" | "zstd" | "br";
8852|  ttl_ms?: number;
8853|  payload?: unknown;
8854|  err?: EnvelopeError;
8855|  meta?: Record<string, unknown>;
8856|};
8857|
8858|export type Party = "runtime" | "client" | `cs:${string}` | `mod:${string}`;
8859|
8860|export type EnvelopeError = {
8861|  code: string;
8862|  message: string;
8863|  retryable?: boolean;
8864|  retry_after_ms?: number;
8865|  details?: unknown;
8866|  cause?: string;
8867|};
8868|
8869|export const PROTOCOL_VERSION = 1;
8870|export const DEFAULT_MTU = 1_048_576;
8871|export const OVERHEAD_BUDGET = 4_096;
8872|export const MAX_CHUNKS = 256;
8873|export const CHUNK_TIMEOUT_MS = 30_000;
8874|export const HANDSHAKE_TIMEOUT_MS = 5_000;
8875|export const HEARTBEAT_IDLE_MS = 20_000;
8876|export const HEARTBEAT_TIMEOUT_MS = 5_000;
8877|export const DEFAULT_TTL_MS = 30_000;
8878|```
8879|
8880|## `thin-client/src/lib/crypto.ts`
8881|
8882|```typescript
8883|export function base64UrlEncode(bytes: Uint8Array): string {
8884|  let bin = "";
8885|  for (const b of bytes) bin += String.fromCharCode(b);
8886|  return btoa(bin).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
8887|}
8888|
8889|export function base64UrlDecode(s: string): Uint8Array {
8890|  const pad = s.length % 4 === 0 ? "" : "=".repeat(4 - (s.length % 4));
8891|  const bin = atob(s.replace(/-/g, "+").replace(/_/g, "/") + pad);
8892|  const out = new Uint8Array(bin.length);
8893|  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
8894|  return out;
8895|}
8896|
8897|export async function sha256Hex(bytes: Uint8Array): Promise<string> {
8898|  const buf = bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
8899|  const digest = await crypto.subtle.digest("SHA-256", buf);
8900|  return [...new Uint8Array(digest)]
8901|    .map((b) => b.toString(16).padStart(2, "0"))
8902|    .join("");
8903|}
8904|
8905|export function uuidv7(): string {
8906|  const now = Date.now();
8907|  const bytes = new Uint8Array(16);
8908|  const dv = new DataView(bytes.buffer);
8909|  dv.setUint32(0, Math.floor(now / 0x100000000));
8910|  dv.setUint32(4, now >>> 0);
8911|  crypto.getRandomValues(bytes.subarray(6));
8912|  bytes[6] = (bytes[6]! & 0x0f) | 0x70;
8913|  bytes[8] = (bytes[8]! & 0x3f) | 0x80;
8914|  const hex = [...bytes].map((b) => b.toString(16).padStart(2, "0")).join("");
8915|  return `${hex.slice(0,8)}-${hex.slice(8,12)}-${hex.slice(12,16)}-${hex.slice(16,20)}-${hex.slice(20)}`;
8916|}
8917|```
8918|
8919|## `thin-client/src/lib/errors.ts`
8920|
8921|```typescript
8922|export class DneaError extends Error {
8923|  constructor(
8924|    public readonly code: string,
8925|    public readonly retryable: boolean = false,
8926|    public readonly retryAfterMs?: number,
8927|    public readonly details?: unknown,
8928|  ) {
8929|    super(code);
8930|    this.name = "DneaError";
8931|  }
8932|}
8933|
8934|export function fromEnvelopeError(err: {
8935|  code: string;
8936|  retryable?: boolean;
8937|  retry_after_ms?: number;
8938|  details?: unknown;
8939|}): DneaError {
8940|  return new DneaError(err.code, err.retryable ?? false, err.retry_after_ms, err.details);
8941|}
8942|```
8943|
8944|## `thin-client/src/lib/log.ts`
8945|
8946|```typescript
8947|const MAX_ENTRIES = 100;
8948|
8949|export class Logger {
8950|  private entries: { ts: number; level: string; msg: string; extra?: unknown }[] = [];
8951|  constructor(private scope: string) {}
8952|  info(msg: string, extra?: unknown): void { this.record("info", msg, extra); }
8953|  warn(msg: string, extra?: unknown): void { this.record("warn", msg, extra); }
8954|  error(msg: string, extra?: unknown): void { this.record("error", msg, extra); }
8955|  snapshot() { return this.entries.slice(); }
8956|  private record(level: string, msg: string, extra?: unknown): void {
8957|    this.entries.push({ ts: Date.now(), level, msg, extra });
8958|    if (this.entries.length > MAX_ENTRIES) this.entries.shift();
8959|    if (level === "error") console.error(`[${this.scope}]`, msg, extra);
8960|    else if (level === "warn") console.warn(`[${this.scope}]`, msg, extra);
8961|    else console.debug(`[${this.scope}]`, msg, extra);
8962|  }
8963|}
8964|```
8965|
8966|## `thin-client/src/lib/backoff.ts`
8967|
8968|```typescript
8969|const BASE_MS = 250;
8970|const CAP_MS = 30_000;
8971|
8972|export function backoffDelay(attempt: number): number {
8973|  const prev = Math.min(CAP_MS, BASE_MS * Math.pow(3, attempt));
8974|  return BASE_MS + Math.random() * (prev * 3 - BASE_MS);
8975|}
8976|```
8977|
8978|## `thin-client/src/lib/codec.ts`
8979|
8980|```typescript
8981|import { Envelope, DEFAULT_MTU, OVERHEAD_BUDGET, MAX_CHUNKS, CHUNK_TIMEOUT_MS } from "./protocol";
8982|import { base64UrlEncode, base64UrlDecode, sha256Hex } from "./crypto";
8983|
8984|export type Decoded = { kind: "envelope"; envelope: Envelope } | { kind: "incomplete" };
8985|type Inflight = { parts: (Uint8Array | undefined)[]; total: number; startedAt: number; digest: string };
8986|
8987|export class Codec {
8988|  private inflight = new Map<string, Inflight>();
8989|  constructor(private mtu: number = DEFAULT_MTU) {}
8990|
8991|  async encode(env: Envelope): Promise<unknown[]> {
8992|    const bytes = new TextEncoder().encode(JSON.stringify(env));
8993|    if (bytes.length <= this.mtu - OVERHEAD_BUDGET) return [env];
8994|    const chunkSize = this.mtu - OVERHEAD_BUDGET - 512;
8995|    const total = Math.ceil(bytes.length / chunkSize);
8996|    if (total > MAX_CHUNKS) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
8997|    const digest = `sha-256:${await sha256Hex(bytes)}`;
8998|    const frames: unknown[] = [];
8999|    for (let i = 0; i < total; i++) {
9000|      const part = bytes.slice(i * chunkSize, (i + 1) * chunkSize);
9001|      frames.push({
9002|        v: 1, type: "dnea/chunk", cid: env.id, seq: i, total,
9003|        part: base64UrlEncode(part), digest, final: i + 1 === total,
9004|      });
9005|    }
9006|    return frames;
9007|  }
9008|
9009|  async decode(raw: unknown): Promise<Decoded> {
9010|    const obj = raw as Record<string, unknown>;
9011|    if (obj.type !== "dnea/chunk") {
9012|      return { kind: "envelope", envelope: obj as unknown as Envelope };
9013|    }
9014|    return this.decodeChunk(obj);
9015|  }
9016|
9017|  private async decodeChunk(frame: Record<string, unknown>): Promise<Decoded> {
9018|    const cid = String(frame.cid);
9019|    const seq = Number(frame.seq);
9020|    const total = Number(frame.total);
9021|    if (total > MAX_CHUNKS || seq < 0 || seq >= total) throw new Error("E_FRAMING_CHUNK_OVERFLOW");
9022|    let entry = this.inflight.get(cid);
9023|    if (!entry) {
9024|      entry = { parts: new Array(total), total, startedAt: Date.now(), digest: String(frame.digest) };
9025|      this.inflight.set(cid, entry);
9026|    }
9027|    if (Date.now() - entry.startedAt > CHUNK_TIMEOUT_MS) {
9028|      this.inflight.delete(cid);
9029|      throw new Error("E_FRAMING_CHUNK_OVERFLOW");
9030|    }
9031|    entry.parts[seq] = base64UrlDecode(String(frame.part));
9032|    if (entry.parts.some((p) => p === undefined)) return { kind: "incomplete" };
9033|    const total_bytes = entry.parts.reduce((n, p) => n + (p?.byteLength ?? 0), 0);
9034|    const full = new Uint8Array(total_bytes);
9035|    let off = 0;
9036|    for (const p of entry.parts) { if (!p) continue; full.set(p, off); off += p.byteLength; }
9037|    const actual = `sha-256:${await sha256Hex(full)}`;
9038|    this.inflight.delete(cid);
9039|    if (actual !== entry.digest) throw new Error("E_FRAMING_DIGEST_MISMATCH");
9040|    const envelope = JSON.parse(new TextDecoder().decode(full)) as Envelope;
9041|    return { kind: "envelope", envelope };
9042|  }
9043|}
9044|```
9045|
9046|## `thin-client/src/lib/router.ts`
9047|
9048|```typescript
9049|import { Connection } from "./connection";
9050|import { Envelope, PROTOCOL_VERSION } from "./protocol";
9051|import { uuidv7 } from "./crypto";
9052|
9053|type Handler = (env: Envelope) => void;
9054|
9055|export class Router {
9056|  private handlers = new Map<string, Set<Handler>>();
9057|  constructor(private conn: Connection) { conn.on("*", (env) => this.dispatch(env)); }
9058|
9059|  on(type: string, handler: Handler): () => void {
9060|    const set = this.handlers.get(type) ?? new Set();
9061|    set.add(handler);
9062|    this.handlers.set(type, set);
9063|    return () => set.delete(handler);
9064|  }
9065|
9066|  private dispatch(env: Envelope): void {
9067|    const set = this.handlers.get(env.type);
9068|    if (set) for (const h of set) h(env);
9069|    const wildcard = this.handlers.get("*");
9070|    if (wildcard) for (const h of wildcard) h(env);
9071|  }
9072|
9073|  mintEnvelope(input: {
9074|    type: string; src: string; dst: string;
9075|    cap?: string; mod?: string; ttl_ms?: number; payload?: unknown;
9076|  }): Envelope {
9077|    return {
9078|      v: PROTOCOL_VERSION, id: uuidv7(), type: input.type,
9079|      ts: new Date().toISOString(),
9080|      src: input.src as never, dst: input.dst as never,
9081|      cap: input.cap, mod: input.mod, ttl_ms: input.ttl_ms, payload: input.payload,
9082|    };
9083|  }
9084|
9085|  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
9086|    return this.conn.request(env, opts);
9087|  }
9088|
9089|  send(env: Envelope): void { this.conn.send(env); }
9090|
9091|  sendRuntimeEnvelope(input: { type: string; payload?: unknown; reply_to?: string }): void {
9092|    const env = this.mintEnvelope({
9093|      type: input.type, src: "client", dst: "runtime", payload: input.payload,
9094|    });
9095|    if (input.reply_to) env.reply_to = input.reply_to as never;
9096|    this.send(env);
9097|  }
9098|
9099|  sendEnvelope(env: Envelope): void { this.conn.send(env); }
9100|}
9101|```
9102|
9103|## `thin-client/src/lib/connection.ts`
9104|
9105|```typescript
9106|import { browser } from "wxt/browser";
9107|import { Codec } from "./codec";
9108|import { Envelope, PROTOCOL_VERSION, HEARTBEAT_IDLE_MS, HANDSHAKE_TIMEOUT_MS, DEFAULT_TTL_MS } from "./protocol";
9109|import { DneaError } from "./errors";
9110|import { backoffDelay } from "./backoff";
9111|import { Logger } from "./log";
9112|import { uuidv7 } from "./crypto";
9113|
9114|const MAX_CONSECUTIVE_FAILURES = 8;
9115|const BREAKER_COOLDOWN_MS = 5 * 60_000;
9116|
9117|type State = "DISCONNECTED" | "CONNECTING" | "HANDSHAKING" | "READY" | "DRAINING" | "FAILED";
9118|
9119|type Pending = {
9120|  id: string;
9121|  resolve: (env: Envelope) => void;
9122|  reject: (err: DneaError) => void;
9123|  timer: ReturnType<typeof setTimeout>;
9124|  idempotent: boolean;
9125|};
9126|
9127|export class Connection {
9128|  private state: State = "DISCONNECTED";
9129|  private port?: browser.runtime.Port;
9130|  private codec = new Codec();
9131|  private attempt = 0;
9132|  private consecutiveFailures = 0;
9133|  private breakerOpenUntil = 0;
9134|  private outstanding = new Map<string, Pending>();
9135|  private lastRxAt = 0;
9136|  private lastTxAt = 0;
9137|  private log = new Logger("connection");
9138|  private listeners = new Map<string, Set<(env: Envelope) => void>>();
9139|
9140|  constructor(private opts: {
9141|    hostName: string;
9142|    clientVersion: string;
9143|    browserFamily: "chromium" | "firefox" | "safari";
9144|    onStateChange?: (s: State) => void;
9145|  }) {}
9146|
9147|  on(type: string, fn: (env: Envelope) => void): () => void {
9148|    const set = this.listeners.get(type) ?? new Set();
9149|    set.add(fn);
9150|    this.listeners.set(type, set);
9151|    return () => set.delete(fn);
9152|  }
9153|
9154|  private setState(s: State) {
9155|    if (this.state === s) return;
9156|    this.state = s;
9157|    this.opts.onStateChange?.(s);
9158|  }
9159|
9160|  private emit(env: Envelope) {
9161|    const set = this.listeners.get(env.type);
9162|    if (set) for (const fn of set) fn(env);
9163|    const wildcard = this.listeners.get("*");
9164|    if (wildcard) for (const fn of wildcard) fn(env);
9165|  }
9166|
9167|  connect(): void {
9168|    if (this.state === "READY" || this.state === "HANDSHAKING" || this.state === "CONNECTING") return;
9169|    if (Date.now() < this.breakerOpenUntil) return;
9170|
9171|    this.setState("CONNECTING");
9172|    try {
9173|      this.port = browser.runtime.connectNative(this.opts.hostName);
9174|    } catch (e) {
9175|      this.log.warn("connectNative threw", { error: String(e) });
9176|      return this.fail(new DneaError("E_TRANSPORT_NOT_INSTALLED", false));
9177|    }
9178|
9179|    this.port.onMessage.addListener((raw) => this.onRawMessage(raw));
9180|    this.port.onDisconnect.addListener(() => this.onDisconnect());
9181|    this.setState("HANDSHAKING");
9182|
9183|    this.sendRaw({
9184|      v: 1, id: uuidv7(), type: "dnea/hello",
9185|      ts: new Date().toISOString(), src: "client", dst: "runtime",
9186|      payload: {
9187|        clientVersion: this.opts.clientVersion,
9188|        browserFamily: this.opts.browserFamily,
9189|        browserVersion: navigator.userAgent,
9190|        extensionId: browser.runtime.id,
9191|        mtu: 1_048_576,
9192|        encodings: ["identity"],
9193|        features: {
9194|          chunking: true, contentOverlay: true, consentUI: true,
9195|          serviceWorkerBackground: this.opts.browserFamily === "chromium",
9196|        },
9197|      },
9198|    });
9199|
9200|    setTimeout(() => {
9201|      if (this.state === "HANDSHAKING") {
9202|        this.log.warn("handshake timeout");
9203|        this.fail(new DneaError("E_TRANSPORT_HANDSHAKE_TIMEOUT", true));
9204|      }
9205|    }, HANDSHAKE_TIMEOUT_MS);
9206|  }
9207|
9208|  private async onRawMessage(raw: unknown): Promise<void> {
9209|    this.lastRxAt = Date.now();
9210|    let decoded;
9211|    try { decoded = await this.codec.decode(raw); }
9212|    catch (e) {
9213|      const code = String(e).includes("CHUNK") ? "E_FRAMING_CHUNK_OVERFLOW"
9214|                 : String(e).includes("DIGEST") ? "E_FRAMING_DIGEST_MISMATCH"
9215|                 : "E_FRAMING_INVALID_JSON";
9216|      this.log.warn("framing error", { code });
9217|      return;
9218|    }
9219|    if (decoded.kind === "incomplete") return;
9220|    const env = decoded.envelope;
9221|    const srcOk = env.src === "runtime" || env.src.startsWith("mod:");
9222|    if (!srcOk) { this.log.warn("invalid src on runtime-side transport", { src: env.src }); return; }
9223|    if (env.type === "dnea/welcome") return this.onWelcome(env);
9224|    if (env.type === "dnea/pong") return;
9225|    if (env.type === "dnea/goodbye") return this.onGoodbye(env);
9226|    if (env.reply_to) return this.onReply(env);
9227|    this.emit(env);
9228|  }
9229|
9230|  private onReply(env: Envelope): void {
9231|    const pending = this.outstanding.get(env.reply_to!);
9232|    if (!pending) { this.log.warn("unknown correlation", { reply_to: env.reply_to }); return; }
9233|    clearTimeout(pending.timer);
9234|    this.outstanding.delete(env.reply_to!);
9235|    pending.resolve(env);
9236|  }
9237|
9238|  private onWelcome(env: Envelope): void {
9239|    const meta = env.meta as Record<string, unknown> | undefined;
9240|    if (meta?.client_extension_id !== browser.runtime.id) {
9241|      this.fail(new DneaError("E_HANDSHAKE_REJECTED", false, undefined, { reason: "extension_id_mismatch" }));
9242|      return;
9243|    }
9244|    this.consecutiveFailures = 0;
9245|    this.attempt = 0;
9246|    this.setState("READY");
9247|    void browser.storage.session.set({ dnea: { wasConnected: true } });
9248|    this.emit(env);
9249|    this.log.info("handshake complete");
9250|  }
9251|
9252|  private onGoodbye(env: Envelope): void {
9253|    const reason = (env.payload as { reason?: string } | undefined)?.reason ?? "shutdown";
9254|    this.setState("DRAINING");
9255|    setTimeout(() => this.close(), 5_000);
9256|    if (reason === "shutdown") {
9257|      void browser.storage.session.set({ dnea: { wasConnected: false } });
9258|    }
9259|  }
9260|
9261|  private onDisconnect(): void {
9262|    const stillPending = [...this.outstanding.values()];
9263|    this.outstanding.clear();
9264|    for (const p of stillPending) {
9265|      clearTimeout(p.timer);
9266|      p.reject(new DneaError("E_TRANSPORT_DISCONNECTED", true));
9267|    }
9268|    void browser.storage.session.set({ dnea: { wasConnected: false } });
9269|    this.port = undefined;
9270|    this.consecutiveFailures++;
9271|    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
9272|      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
9273|    }
9274|    this.setState("FAILED");
9275|    this.scheduleReconnect();
9276|  }
9277|
9278|  private fail(err: DneaError): void {
9279|    this.log.warn("connection failure", { code: err.code });
9280|    try { this.port?.disconnect(); } catch {}
9281|    this.port = undefined;
9282|    this.consecutiveFailures++;
9283|    if (this.consecutiveFailures >= MAX_CONSECUTIVE_FAILURES) {
9284|      this.breakerOpenUntil = Date.now() + BREAKER_COOLDOWN_MS;
9285|    }
9286|    this.setState("FAILED");
9287|    this.scheduleReconnect();
9288|  }
9289|
9290|  private scheduleReconnect(): void {
9291|    const delay = backoffDelay(this.attempt);
9292|    this.attempt = Math.min(this.attempt + 1, 12);
9293|    setTimeout(() => this.connect(), delay);
9294|  }
9295|
9296|  async request(env: Envelope, opts: { idempotent?: boolean } = {}): Promise<Envelope> {
9297|    if (this.state !== "READY") throw new DneaError("E_TRANSPORT_DISCONNECTED", true);
9298|    return new Promise<Envelope>((resolve, reject) => {
9299|      const ttl = env.ttl_ms ?? DEFAULT_TTL_MS;
9300|      const timer = setTimeout(() => {
9301|        this.outstanding.delete(env.id);
9302|        reject(new DneaError("E_TTL_EXPIRED", true));
9303|      }, ttl);
9304|      this.outstanding.set(env.id, { id: env.id, resolve, reject, timer, idempotent: !!opts.idempotent });
9305|      void this.sendEnvelope(env);
9306|    });
9307|  }
9308|
9309|  send(env: Envelope): void {
9310|    if (this.state !== "READY") { this.log.warn("send dropped: not READY", { type: env.type }); return; }
9311|    void this.sendEnvelope(env);
9312|  }
9313|
9314|  private async sendEnvelope(env: Envelope): Promise<void> {
9315|    const frames = await this.codec.encode(env);
9316|    for (const frame of frames) this.sendRaw(frame);
9317|    this.lastTxAt = Date.now();
9318|  }
9319|
9320|  private sendRaw(frame: unknown): void {
9321|    if (!this.port) return;
9322|    try { this.port.postMessage(frame); }
9323|    catch (e) { this.log.warn("postMessage failed", { error: String(e) }); this.fail(new DneaError("E_TRANSPORT_DISCONNECTED", true)); }
9324|  }
9325|
9326|  maybePing(): void {
9327|    if (this.state !== "READY") return;
9328|    if (Date.now() - this.lastRxAt < HEARTBEAT_IDLE_MS) return;
9329|    void this.sendEnvelope({
9330|      v: 1, id: uuidv7(), type: "dnea/ping", ts: new Date().toISOString(),
9331|      src: "client", dst: "runtime", payload: { seq: Date.now() },
9332|    });
9333|  }
9334|
9335|  probeIfIdle(): void {
9336|    if (this.state === "DISCONNECTED" || this.state === "FAILED") this.connect();
9337|  }
9338|
9339|  private close(): void {
9340|    try { this.port?.disconnect(); } catch {}
9341|    this.port = undefined;
9342|    this.setState("DISCONNECTED");
9343|  }
9344|}
9345|```
9346|
9347|## `thin-client/src/lib/cs-bridge.ts`
9348|
9349|```typescript
9350|import { browser } from "wxt/browser";
9351|import { Router } from "./router";
9352|import { Envelope } from "./protocol";
9353|import { Logger } from "./log";
9354|import { uuidv7 } from "./crypto";
9355|
9356|const MAX_PORTS = 512;
9357|const QUEUE_MAX = 128;
9358|const RATE_LIMIT_PER_MIN = 60;
9359|const RATE_BURST = 20;
9360|
9361|type CSMessage =
9362|  | { kind: "dnea.cs/req"; cid: string; type: string; mod?: string; cap?: string; ttl_ms?: number; payload?: unknown }
9363|  | { kind: "dnea.cs/evt"; cid: string; type: string; mod?: string; payload?: unknown };
9364|
9365|type BgMessage =
9366|  | { kind: "dnea.cs/res"; cid: string; payload?: unknown }
9367|  | { kind: "dnea.cs/err"; cid: string; err: { code: string; message: string; retryable?: boolean; retry_after_ms?: number } };
9368|
9369|type PortRecord = {
9370|  portId: string;
9371|  port: browser.runtime.Port;
9372|  tabId: number;
9373|  frameId: number;
9374|  url: string;
9375|  incognito: boolean;
9376|  registeredAt: number;
9377|  lastSeen: number;
9378|  queue: unknown[];
9379|  tokens: { count: number; resetAt: number };
9380|  pendingCids: Map<string, string>;
9381|};
9382|
9383|export class CSPortBridge {
9384|  private ports = new Map<string, PortRecord>();
9385|  private byTabFrame = new Map<string, string>();
9386|  private log = new Logger("cs-bridge");
9387|
9388|  constructor(private router: Router) {
9389|    router.on("dnea/result", (env) => this.deliverResult(env));
9390|    router.on("dnea/error", (env) => this.deliverResult(env));
9391|    router.on("dnea/event", (env) => this.deliverEvent(env));
9392|    browser.tabs.onRemoved.addListener((tabId) => this.unregisterTab(tabId));
9393|    browser.tabs.onUpdated.addListener((tabId, change) => {
9394|      if (change.url) this.unregisterTab(tabId, "url_changed");
9395|    });
9396|  }
9397|
9398|  accept(port: browser.runtime.Port): void {
9399|    const sender = port.sender;
9400|    if (!sender?.tab || sender.id !== browser.runtime.id) {
9401|      try { port.disconnect(); } catch {}
9402|      return;
9403|    }
9404|    const key = `${sender.tab.id}:${sender.frameId ?? 0}`;
9405|    const existing = this.byTabFrame.get(key);
9406|    if (existing) this.disconnectPort(existing, "superseded");
9407|    if (this.ports.size >= MAX_PORTS) {
9408|      const lru = [...this.ports.values()].sort((a, b) => a.lastSeen - b.lastSeen)[0];
9409|      if (lru) this.disconnectPort(lru.portId, "evicted");
9410|    }
9411|    const portId = uuidv7();
9412|    const rec: PortRecord = {
9413|      portId, port,
9414|      tabId: sender.tab.id!, frameId: sender.frameId ?? 0,
9415|      url: sender.url ?? "", incognito: sender.tab.incognito ?? false,
9416|      registeredAt: Date.now(), lastSeen: Date.now(),
9417|      queue: [], tokens: { count: RATE_BURST, resetAt: Date.now() + 60_000 },
9418|      pendingCids: new Map(),
9419|    };
9420|    this.ports.set(portId, rec);
9421|    this.byTabFrame.set(key, portId);
9422|    port.onMessage.addListener((msg) => this.onCSMessage(rec, msg as CSMessage));
9423|    port.onDisconnect.addListener(() => this.disconnectPort(portId, "disconnected"));
9424|    this.router.sendRuntimeEnvelope({
9425|      type: "dnea/cs/register",
9426|      payload: { portId, tabId: rec.tabId, frameId: rec.frameId, url: rec.url, incognito: rec.incognito },
9427|    });
9428|  }
9429|
9430|  private onCSMessage(rec: PortRecord, msg: CSMessage): void {
9431|    rec.lastSeen = Date.now();
9432|    if (!this.takeToken(rec)) {
9433|      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
9434|        err: { code: "E_RATE_LIMITED", message: "rate limit", retryable: true, retry_after_ms: 1000 } });
9435|      return;
9436|    }
9437|    if (rec.queue.length >= QUEUE_MAX) {
9438|      this.postSafe(rec, { kind: "dnea.cs/err", cid: msg.cid,
9439|        err: { code: "E_CS_BACKPRESSURE", message: "queue full", retryable: true, retry_after_ms: 250 } });
9440|      rec.queue.shift();
9441|    }
9442|    const env = this.router.mintEnvelope({
9443|      type: msg.type, src: `cs:${rec.portId}`,
9444|      dst: msg.mod ? `mod:${msg.mod}` : "runtime",
9445|      cap: msg.cap, mod: msg.mod, ttl_ms: msg.ttl_ms, payload: msg.payload,
9446|    });
9447|    if (msg.kind === "dnea.cs/req") rec.pendingCids.set(env.id, msg.cid);
9448|    rec.queue.push(env);
9449|    this.router.sendEnvelope(env);
9450|  }
9451|
9452|  private deliverResult(env: Envelope): void {
9453|    const portId = env.dst.startsWith("cs:") ? env.dst.slice(3) : undefined;
9454|    if (!portId) return;
9455|    const rec = this.ports.get(portId);
9456|    if (!rec) return;
9457|    const cid = rec.pendingCids.get(env.reply_to ?? env.id);
9458|    if (cid) {
9459|      rec.pendingCids.delete(env.reply_to ?? env.id);
9460|      const out: BgMessage = env.err
9461|        ? { kind: "dnea.cs/err", cid, err: {
9462|            code: env.err.code, message: env.err.message,
9463|            retryable: env.err.retryable, retry_after_ms: env.err.retry_after_ms } }
9464|        : { kind: "dnea.cs/res", cid, payload: env.payload };
9465|      this.postSafe(rec, out);
9466|    }
9467|    if (rec.queue.length) rec.queue.shift();
9468|  }
9469|
9470|  private deliverEvent(env: Envelope): void {
9471|    for (const rec of this.ports.values()) {
9472|      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
9473|    }
9474|  }
9475|
9476|  private postSafe(rec: PortRecord, msg: BgMessage): void {
9477|    try { rec.port.postMessage(msg); } catch { this.disconnectPort(rec.portId, "post_failed"); }
9478|  }
9479|
9480|  private takeToken(rec: PortRecord): boolean {
9481|    const now = Date.now();
9482|    if (now > rec.tokens.resetAt) rec.tokens = { count: RATE_BURST, resetAt: now + 60_000 };
9483|    if (rec.tokens.count <= 0) return false;
9484|    rec.tokens.count--;
9485|    return true;
9486|  }
9487|
9488|  private disconnectPort(portId: string, reason: string): void {
9489|    const rec = this.ports.get(portId);
9490|    if (!rec) return;
9491|    this.ports.delete(portId);
9492|    this.byTabFrame.delete(`${rec.tabId}:${rec.frameId}`);
9493|    try { rec.port.disconnect(); } catch {}
9494|    this.router.sendRuntimeEnvelope({
9495|      type: "dnea/cs/unregister",
9496|      payload: { portId, reason },
9497|    });
9498|  }
9499|
9500|  private unregisterTab(tabId: number, reason = "tab_removed"): void {
9501|    for (const rec of [...this.ports.values()]) {
9502|      if (rec.tabId === tabId) this.disconnectPort(rec.portId, reason);
9503|    }
9504|  }
9505|
9506|  broadcastCapabilityChange(env: Envelope): void {
9507|    for (const rec of this.ports.values()) {
9508|      this.postSafe(rec, { kind: "dnea.cs/evt", cid: env.id, payload: env.payload });
9509|    }
9510|  }
9511|}
9512|```
9513|
9514|## `thin-client/src/lib/consent.ts`
9515|
9516|```typescript
9517|import { browser } from "wxt/browser";
9518|import { Router } from "./router";
9519|import { Envelope } from "./protocol";
9520|
9521|export class ConsentController {
9522|  private pending = new Map<string, Envelope>();
9523|  private consentWindowId?: number;
9524|  constructor(private router: Router) {}
9525|
9526|  async handle(env: Envelope): Promise<void> {
9527|    const payload = env.payload as { promptId: string };
9528|    this.pending.set(payload.promptId, env);
9529|    try {
9530|      await browser.runtime.sendMessage({ kind: "dnea.consent.show", promptId: payload.promptId });
9531|      return;
9532|    } catch {}
9533|    await this.openConsentPage(payload.promptId);
9534|  }
9535|
9536|  private async openConsentPage(promptId: string): Promise<void> {
9537|    const url = browser.runtime.getURL(`/consent.html?promptId=${encodeURIComponent(promptId)}`);
9538|    const win = await browser.windows.create({ url, type: "popup", width: 520, height: 640 });
9539|    this.consentWindowId = win.id;
9540|  }
9541|
9542|  fetchPrompt(promptId: string): Envelope | undefined { return this.pending.get(promptId); }
9543|
9544|  async respond(promptId: string, response: { granted: boolean; scope?: unknown; rememberFor?: string }): Promise<void> {
9545|    const env = this.pending.get(promptId);
9546|    if (!env) throw new Error("prompt not found");
9547|    this.pending.delete(promptId);
9548|    this.router.sendRuntimeEnvelope({
9549|      type: "dnea/consent/response",
9550|      reply_to: env.id,
9551|      payload: {
9552|        prompt_id: promptId,
9553|        granted: response.granted,
9554|        scope: response.scope,
9555|        remember_for: response.rememberFor,
9556|      },
9557|    });
9558|  }
9559|}
9560|```
9561|
9562|## `thin-client/src/lib/ui.ts`
9563|
9564|```typescript
9565|import { Envelope } from "./protocol";
9566|
9567|const mounts = new Map<string, { host: HTMLElement; shadow: ShadowRoot }>();
9568|
9569|export class OverlayMount {
9570|  async mount(env: Envelope): Promise<void> {
9571|    const p = env.payload as { origin: string; mountId: string; css?: string; html?: string };
9572|    if (!matchesOrigin(location.origin, p.origin)) return;
9573|    const existing = mounts.get(p.mountId);
9574|    if (existing) existing.host.remove();
9575|
9576|    const host = document.createElement("div");
9577|    host.id = `dnea-mount-${p.mountId}`;
9578|    host.style.position = "fixed";
9579|    host.style.zIndex = "2147483647";
9580|
9581|    if (!("attachShadow" in host)) {
9582|      throw new Error("E_UNSUPPORTED_FEATURE");
9583|    }
9584|    const shadow = host.attachShadow({ mode: "closed" });
9585|    if (!shadow) throw new Error("E_UNSUPPORTED_FEATURE");
9586|
9587|    if (p.css) {
9588|      const style = document.createElement("style");
9589|      style.textContent = p.css;
9590|      shadow.appendChild(style);
9591|    }
9592|    if (p.html) {
9593|      const container = document.createElement("div");
9594|      container.innerHTML = sanitize(p.html);
9595|      shadow.appendChild(container);
9596|    }
9597|    document.documentElement.appendChild(host);
9598|    mounts.set(p.mountId, { host, shadow });
9599|  }
9600|
9601|  clear(env: Envelope): void {
9602|    const p = env.payload as { targetId: string };
9603|    const m = mounts.get(p.targetId);
9604|    if (m) { m.host.remove(); mounts.delete(p.targetId); }
9605|  }
9606|}
9607|
9608|function matchesOrigin(current: string, pattern: string): boolean {
9609|  if (pattern.endsWith("/*")) return current.startsWith(pattern.slice(0, -1));
9610|  return current === pattern;
9611|}
9612|
9613|function sanitize(html: string): string {
9614|  return html
9615|    .replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi, "")
9616|    .replace(/\son\w+\s*=\s*"[^"]*"/gi, "")
9617|    .replace(/\son\w+\s*=\s*'[^']*'/gi, "");
9618|}
9619|```
9620|
9621|## `thin-client/src/lib/crypto.ts` (already above)
9622|
9623|## `thin-client/src/entrypoints/background.ts`
9624|
9625|```typescript
9626|import { browser } from "wxt/browser";
9627|import { Connection } from "@/lib/connection";
9628|import { Router } from "@/lib/router";
9629|import { CSPortBridge } from "@/lib/cs-bridge";
9630|import { ConsentController } from "@/lib/consent";
9631|import { OverlayMount } from "@/lib/ui";
9632|
9633|const HOST_NAME = "org.dnea.runtime";
9634|
9635|const conn = new Connection({
9636|  hostName: HOST_NAME,
9637|  clientVersion: browser.runtime.getManifest().version,
9638|  browserFamily: detectFamily(),
9639|});
9640|
9641|const router = new Router(conn);
9642|const cs = new CSPortBridge(router);
9643|const consent = new ConsentController(router);
9644|const overlays = new OverlayMount();
9645|
9646|router.on("dnea/consent/request", (env) => consent.handle(env));
9647|router.on("dnea/capability/changed", (env) => cs.broadcastCapabilityChange(env));
9648|router.on("dnea/ui/inject", (env) => overlays.mount(env));
9649|router.on("dnea/ui/clear", (env) => overlays.clear(env));
9650|
9651|browser.runtime.onConnect.addListener((port) => {
9652|  if (port.name === "dnea.cs.v1") cs.accept(port);
9653|});
9654|browser.runtime.onStartup.addListener(() => conn.connect());
9655|browser.runtime.onInstalled.addListener(() => conn.connect());
9656|
9657|browser.alarms.create("dnea.heartbeat", { periodInMinutes: 0.25 });
9658|browser.alarms.create("dnea.probe", { periodInMinutes: 1 });
9659|browser.alarms.onAlarm.addListener((a) => {
9660|  if (a.name === "dnea.heartbeat") conn.maybePing();
9661|  if (a.name === "dnea.probe") conn.probeIfIdle();
9662|});
9663|
9664|void (async () => {
9665|  const snap = await browser.storage.session.get("dnea");
9666|  if (snap?.dnea?.wasConnected) conn.connect();
9667|})();
9668|
9669|function detectFamily(): "chromium" | "firefox" | "safari" {
9670|  const ua = navigator.userAgent;
9671|  if (ua.includes("Firefox/")) return "firefox";
9672|  if (ua.includes("Safari/") && !ua.includes("Chrome/")) return "safari";
9673|  return "chromium";
9674|}
9675|
9676|browser.runtime.onMessage.addListener((msg, _sender, sendResponse) => {
9677|  if (msg?.kind === "dnea.consent.fetch") {
9678|    sendResponse(consent.fetchPrompt(msg.promptId));
9679|    return true;
9680|  }
9681|  if (msg?.kind === "dnea.consent.respond") {
9682|    consent.respond(msg.promptId, { granted: msg.granted, rememberFor: msg.rememberFor })
9683|      .then(() => sendResponse({ ok: true }))
9684|      .catch((e) => sendResponse({ ok: false, error: String(e) }));
9685|    return true;
9686|  }
9687|  return false;
9688|});
9689|```
9690|
9691|## `thin-client/src/entrypoints/content.ts`
9692|
9693|```typescript
9694|import { browser } from "wxt/browser";
9695|
9696|const port = browser.runtime.connect({ name: "dnea.cs.v1" });
9697|
9698|const pending = new Map<string, (msg: unknown) => void>();
9699|const listeners = new Set<(data: unknown) => void>();
9700|
9701|port.onMessage.addListener((raw: unknown) => {
9702|  const msg = raw as { kind: string; cid: string; payload?: unknown; err?: unknown };
9703|  if (msg.kind === "dnea.cs/res" || msg.kind === "dnea.cs/err") {
9704|    const resolver = pending.get(msg.cid);
9705|    if (resolver) { pending.delete(msg.cid); resolver(msg); }
9706|    return;
9707|  }
9708|  if (msg.kind === "dnea.cs/evt") {
9709|    for (const fn of listeners) fn(msg.payload);
9710|  }
9711|});
9712|
9713|const bridge = {
9714|  invoke(type: string, payload: unknown, opts: { mod?: string; cap?: string; ttl_ms?: number } = {}): Promise<unknown> {
9715|    const cid = crypto.randomUUID();
9716|    return new Promise((resolve, reject) => {
9717|      pending.set(cid, (msg) => {
9718|        const m = msg as { kind: string; payload?: unknown; err?: { code: string; message: string } };
9719|        if (m.kind === "dnea.cs/err") reject(Object.assign(new Error(m.err!.message), { code: m.err!.code }));
9720|        else resolve(m.payload);
9721|      });
9722|      port.postMessage({ kind: "dnea.cs/req", cid, type, ...opts, payload });
9723|    });
9724|  },
9725|  on(_event: string, fn: (data: unknown) => void): () => void {
9726|    listeners.add(fn);
9727|    return () => listeners.delete(fn);
9728|  },
9729|};
9730|
9731|(window as unknown as { __dnea: typeof bridge }).__dnea = bridge;
9732|
9733|const ALLOWED_PAGE_TYPES = new Set(["focus.getState", "focus.toggle"]);
9734|
9735|window.addEventListener("message", (ev) => {
9736|  if (ev.source !== window) return;
9737|  if (ev.origin !== location.origin) return;
9738|  const data = ev.data as { __dnea?: true; channel?: string; cid?: string; type?: string; payload?: unknown };
9739|  if (!data?.__dnea || !data.cid || !data.type) return;
9740|  if (!ALLOWED_PAGE_TYPES.has(data.type)) return;
9741|  bridge.invoke(data.type, data.payload)
9742|    .then((payload) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: true, payload }, location.origin))
9743|    .catch((err) => window.postMessage({ __dnea: true, channel: data.channel, cid: data.cid, ok: false, error: String(err) }, location.origin));
9744|});
9745|```
9746|
9747|## `thin-client/src/entrypoints/consent/main.ts`
9748|
9749|```typescript
9750|import { browser } from "wxt/browser";
9751|
9752|const promptId = new URLSearchParams(location.search).get("promptId")!;
9753|
9754|(async () => {
9755|  const env = await browser.runtime.sendMessage({ kind: "dnea.consent.fetch", promptId });
9756|  if (!env) {
9757|    document.getElementById("title")!.textContent = "This consent request has expired.";
9758|    return;
9759|  }
9760|  const p = env.payload as { moduleName: string; capabilityId: string; reason: string; constraints: unknown };
9761|  document.getElementById("title")!.textContent = `${p.moduleName} wants ${p.capabilityId}`;
9762|  document.getElementById("reason")!.textContent = p.reason;
9763|  document.getElementById("constraints")!.textContent = JSON.stringify(p.constraints, null, 2);
9764|  const respond = async (granted: boolean, rememberFor?: string) => {
9765|    await browser.runtime.sendMessage({ kind: "dnea.consent.respond", promptId, granted, rememberFor });
9766|    window.close();
9767|  };
9768|  document.getElementById("allow")!.onclick = () => respond(true, "module-version");
9769|  document.getElementById("always")!.onclick = () => respond(true, "always");
9770|  document.getElementById("deny")!.onclick = () => respond(false);
9771|})();
9772|```
9773|
9774|## `thin-client/src/entrypoints/consent/index.html`
9775|
9776|```html
9777|<!doctype html>
9778|<html>
9779|<head>
9780|<meta charset="utf-8">
9781|<title>DNEA consent</title>
9782|<style>
9783|body { font: 14px system-ui; margin: 24px; max-width: 480px; }
9784|h1 { font-size: 16px; margin: 0 0 12px; }
9785|.reason { color: #555; margin: 12px 0; }
9786|button { padding: 6px 12px; margin-right: 8px; }
9787|</style>
9788|</head>
9789|<body>
9790|<h1 id="title">Loading…</h1>
9791|<div class="reason" id="reason"></div>
9792|<pre id="constraints"></pre>
9793|<div>
9794|<button id="allow">Allow for this version</button>
9795|<button id="always">Always allow</button>
9796|<button id="deny">Deny</button>
9797|</div>
9798|<script type="module" src="./main.ts"></script>
9799|</body>
9800|</html>
9801|```
9802|
9803|## `thin-client/src/entrypoints/popup/index.html`
9804|
9805|```html
9806|<!doctype html>
9807|<html>
9808|<head><meta charset="utf-8"><title>DNEA</title></head>
9809|<body style="font:14px system-ui;padding:12px;min-width:220px">
9810|<h1 style="font-size:14px;margin:0 0 8px">DNEA</h1>
9811|<div id="state">connecting…</div>
9812|</body>
9813|</html>
9814|```
9815|
9816|## `thin-client/src/entrypoints/options/index.html` + `main.ts`
9817|
9818|```html
9819|<!doctype html>
9820|<html>
9821|<head><meta charset="utf-8"><title>DNEA diagnostics</title></head>
9822|<body style="font:14px system-ui;padding:16px">
9823|<h1>Diagnostics</h1>
9824|<pre id="log"></pre>
9825|</body>
9826|</html>
9827|```
9828|
9829|```typescript
9830|const el = document.getElementById("log");
9831|if (el) el.textContent = "Runtime connection state is managed by the background service worker.";
9832|```
9833|
9834|---
9835|
9836|# PART 5 — INSTALLERS
9837|
9838|## 5.1 Windows
9839|
9840|### `installer/windows/dnea.wxs`
9841|
9842|```xml
9843|<?xml version="1.0" encoding="UTF-8"?>
9844|<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
9845|  <Package Name="DNEA Runtime" Manufacturer="DNEA Project"
9846|    Version="1.0.0.0" UpgradeCode="A2B4C6D8-E0F2-4051-8263-A4B6C8D0E2F4"
9847|    Scope="perMachine">
9848|
9849|    <MajorUpgrade DowngradeErrorMessage="A newer version of DNEA is already installed." />
9850|    <MediaTemplate EmbedCab="yes" />
9851|
9852|    <StandardDirectory Id="ProgramFiles64Folder">
9853|      <Directory Id="INSTALLFOLDER" Name="DNEA">
9854|        <Component Id="DaemonExe" Guid="*">
9855|          <File Id="dnea-daemon.exe" Source="$(var.StageDir)\dnea-daemon.exe" KeyPath="yes" />
9856|          <ServiceInstall Id="DNEAService" Name="DNEA" DisplayName="DNEA Runtime"
9857|            Description="Device-Native Extension Architecture runtime"
9858|            Start="auto" Type="ownProcess" ErrorControl="normal" Account="LocalSystem">
9859|            <ServiceConfig FailureActions="restart/60000/restart/60000/restart/60000"
9860|              FailureResetPeriod="86400" OnInstall="yes" OnReinstall="yes" />
9861|          </ServiceInstall>
9862|          <ServiceControl Id="DNEAServiceControl" Name="DNEA"
9863|            Start="install" Stop="both" Remove="uninstall" Wait="yes" />
9864|        </Component>
9865|        <Component Id="BridgeExe" Guid="*">
9866|          <File Id="dnea-nm-bridge.exe" Source="$(var.StageDir)\dnea-nm-bridge.exe" KeyPath="yes" />
9867|        </Component>
9868|        <Component Id="CliExe" Guid="*">
9869|          <File Id="dnea.exe" Source="$(var.StageDir)\dnea.exe" KeyPath="yes" />
9870|          <Environment Id="PATH" Name="PATH" Value="[INSTALLFOLDER]" Part="last" Action="set" Permanent="no" System="yes" />
9871|        </Component>
9872|      </Directory>
9873|    </StandardDirectory>
9874|
9875|    <StandardDirectory Id="CommonAppDataFolder">
9876|      <Directory Id="DNEA_DATA" Name="DNEA">
9877|        <Directory Id="DNEA_NM" Name="nm" />
9878|        <Directory Id="DNEA_STATE" Name="state" />
9879|      </Directory>
9880|    </StandardDirectory>
9881|
9882|    <Component Id="NMChrome" Guid="*" Directory="DNEA_NM">
9883|      <File Id="nmChrome.json" Source="$(var.StageDir)\nm\org.dnea.runtime.chrome.json" />
9884|      <RegistryKey Root="HKLM" Key="SOFTWARE\Google\Chrome\NativeMessagingHosts\org.dnea.runtime">
9885|        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.chrome.json" />
9886|      </RegistryKey>
9887|    </Component>
9888|    <Component Id="NMEdge" Guid="*" Directory="DNEA_NM">
9889|      <File Id="nmEdge.json" Source="$(var.StageDir)\nm\org.dnea.runtime.edge.json" />
9890|      <RegistryKey Root="HKLM" Key="SOFTWARE\Microsoft\Edge\NativeMessagingHosts\org.dnea.runtime">
9891|        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.edge.json" />
9892|      </RegistryKey>
9893|    </Component>
9894|    <Component Id="NMFirefox" Guid="*" Directory="DNEA_NM">
9895|      <File Id="nmFirefox.json" Source="$(var.StageDir)\nm\org.dnea.runtime.firefox.json" />
9896|      <RegistryKey Root="HKLM" Key="SOFTWARE\Mozilla\NativeMessagingHosts\org.dnea.runtime">
9897|        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.firefox.json" />
9898|      </RegistryKey>
9899|    </Component>
9900|    <Component Id="NMBrave" Guid="*" Directory="DNEA_NM">
9901|      <File Id="nmBrave.json" Source="$(var.StageDir)\nm\org.dnea.runtime.brave.json" />
9902|      <RegistryKey Root="HKLM" Key="SOFTWARE\BraveSoftware\Brave-Browser\NativeMessagingHosts\org.dnea.runtime">
9903|        <RegistryValue Type="string" Value="[DNEA_NM]org.dnea.runtime.brave.json" />
9904|      </RegistryKey>
9905|    </Component>
9906|
9907|    <Component Id="ForcePolicyChrome" Guid="*" Condition="POLICY=1">
9908|      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Google\Chrome\ExtensionInstallForcelist">
9909|        <RegistryValue Name="100" Type="string"
9910|          Value="abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx" />
9911|      </RegistryKey>
9912|    </Component>
9913|    <Component Id="ForcePolicyEdge" Guid="*" Condition="POLICY=1">
9914|      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Microsoft\Edge\ExtensionInstallForcelist">
9915|        <RegistryValue Name="100" Type="string"
9916|          Value="bcdefghijklmnopabcdefghijklmno;https://edge.microsoft.com/extensionwebstorebase/v1/crx" />
9917|      </RegistryKey>
9918|    </Component>
9919|    <Component Id="ForcePolicyFirefox" Guid="*" Condition="POLICY=1">
9920|      <RegistryKey Root="HKLM" Key="SOFTWARE\Policies\Mozilla\Firefox\Extensions\Install">
9921|        <RegistryValue Name="100" Type="string" Value="dnea@dnea.org" />
9922|      </RegistryKey>
9923|    </Component>
9924|
9925|    <Feature Id="Main" Title="DNEA Runtime" Level="1">
9926|      <ComponentRef Id="DaemonExe" />
9927|      <ComponentRef Id="BridgeExe" />
9928|      <ComponentRef Id="CliExe" />
9929|    </Feature>
9930|    <Feature Id="BrowserIntegration" Title="Browser Integration" Level="1">
9931|      <ComponentRef Id="NMChrome" />
9932|      <ComponentRef Id="NMEdge" />
9933|      <ComponentRef Id="NMFirefox" />
9934|      <ComponentRef Id="NMBrave" />
9935|    </Feature>
9936|    <Feature Id="EnterprisePolicy" Title="Force-install thin client" Level="1">
9937|      <ComponentRef Id="ForcePolicyChrome" />
9938|      <ComponentRef Id="ForcePolicyEdge" />
9939|      <ComponentRef Id="ForcePolicyFirefox" />
9940|    </Feature>
9941|
9942|    <CustomAction Id="GenerateMasterKey"
9943|      ExeCommand="--init-key --config &quot;[DNEA_STATE]dnea.toml&quot;"
9944|      FileRef="dnea-daemon.exe" Execute="deferred" Impersonate="no" Return="ignore" />
9945|    <InstallExecuteSequence>
9946|      <Custom Action="GenerateMasterKey" After="InstallFiles" Condition="NOT Installed" />
9947|    </InstallExecuteSequence>
9948|
9949|    <Property Id="POLICY" Value="0" />
9950|  </Package>
9951|</Wix>
9952|```
9953|
9954|### `installer/windows/build.ps1`
9955|
9956|```powershell
9957|param(
9958|    [string]$StageDir = ".\stage",
9959|    [string]$OutDir = ".\out",
9960|    [string]$Configuration = "release"
9961|)
9962|$ErrorActionPreference = "Stop"
9963|
9964|cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
9965|if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
9966|
9967|New-Item -ItemType Directory -Force -Path $StageDir | Out-Null
9968|Copy-Item target\release\dnea-daemon.exe $StageDir
9969|Copy-Item target\release\dnea-nm-bridge.exe $StageDir
9970|Copy-Item target\release\dnea.exe $StageDir
9971|
9972|New-Item -ItemType Directory -Force -Path "$StageDir\nm" | Out-Null
9973|$bridgePath = (Resolve-Path "$StageDir\dnea-nm-bridge.exe").Path
9974|foreach ($b in @("chrome","edge","firefox","brave")) {
9975|    $origin = if ($b -eq "firefox") { "moz-extension://dnea@dnea.org/" }
9976|              else { "chrome-extension://abcdefghijklmnopabcdefghijklmnop/" }
9977|    $manifest = @{
9978|        name = "org.dnea.runtime"
9979|        description = "DNEA runtime bridge"
9980|        path = $bridgePath
9981|        type = "stdio"
9982|        allowed_origins = @($origin)
9983|        "x-dnea-installed" = @{ version = "1.0.0"; installed_at = (Get-Date -Format "o") }
9984|    }
9985|    $manifest | ConvertTo-Json -Depth 4 | Out-File -Encoding utf8 "$StageDir\nm\org.dnea.runtime.$b.json"
9986|}
9987|
9988|New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
9989|wix build -arch x64 -d StageDir=$StageDir -o "$OutDir\dnea-1.0.0.msi" dnea.wxs
9990|if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
9991|Write-Host "Built $OutDir\dnea-1.0.0.msi"
9992|```
9993|
9994|## 5.2 macOS
9995|
9996|### `installer/macos/org.dnea.runtime.plist`
9997|
9998|```xml
9999|<?xml version="1.0" encoding="UTF-8"?>
10000|<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
10001|<plist version="1.0">
10002|<dict>
10003|  <key>Label</key>            <string>org.dnea.runtime</string>
10004|  <key>ProgramArguments</key> <array>
10005|    <string>/Library/Application Support/DNEA/bin/dnea-daemon</string>
10006|  </array>
10007|  <key>RunAtLoad</key>        <true/>
10008|  <key>KeepAlive</key>        <true/>
10009|  <key>ProcessType</key>      <string>Interactive</string>
10010|  <key>StandardOutPath</key>  <string>/Library/Logs/DNEA/daemon.out</string>
10011|  <key>StandardErrorPath</key><string>/Library/Logs/DNEA/daemon.err</string>
10012|</dict>
10013|</plist>
10014|```
10015|
10016|### `installer/macos/scripts/postinstall`
10017|
10018|```bash
10019|#!/bin/bash
10020|set -euo pipefail
10021|
10022|/usr/local/bin/dnea-daemon --init-key --config /Library/Application\ Support/DNEA/dnea.toml || true
10023|
10024|for d in \
10025|  "/Library/Google/Chrome/NativeMessagingHosts" \
10026|  "/Library/Microsoft/Edge/NativeMessagingHosts" \
10027|  "/Library/Application Support/Mozilla/NativeMessagingHosts"; do
10028|  mkdir -p "$d"
10029|done
10030|
10031|BRIDGE="/Library/Application Support/DNEA/bin/dnea-nm-bridge"
10032|for spec in \
10033|  "chrome:/Library/Google/Chrome/NativeMessagingHosts:chrome-extension" \
10034|  "edge:/Library/Microsoft/Edge/NativeMessagingHosts:chrome-extension" \
10035|  "firefox:/Library/Application Support/Mozilla/NativeMessagingHosts:moz-extension"; do
10036|  IFS=":" read -r name dir prefix <<< "$spec"
10037|  ext_id="abcdefghijklmnopabcdefghijklmnop"
10038|  [ "$name" = "firefox" ] && ext_id="dnea@dnea.org"
10039|  cat > "$dir/org.dnea.runtime.json" <<EOF
10040|{
10041|  "name": "org.dnea.runtime",
10042|  "description": "DNEA runtime bridge",
10043|  "path": "$BRIDGE",
10044|  "type": "stdio",
10045|  "allowed_origins": ["$prefix://$ext_id/"],
10046|  "x-dnea-installed": { "version": "1.0.0", "installed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)" }
10047|}
10048|EOF
10049|done
10050|
10051|launchctl load -w /Library/LaunchDaemons/org.dnea.runtime.plist
10052|
10053|if profiles status -type enrollment 2>/dev/null | grep -q "Enrolled"; then
10054|  cat > /Library/Managed\ Preferences/com.google.Chrome.plist <<EOF
10055|<?xml version="1.0" encoding="UTF-8"?>
10056|<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
10057|<plist version="1.0"><dict>
10058|<key>ExtensionInstallForcelist</key>
10059|<array><string>abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx</string></array>
10060|</dict></plist>
10061|EOF
10062|fi
10063|exit 0
10064|```
10065|
10066|### `installer/macos/build.sh`
10067|
10068|```bash
10069|#!/bin/bash
10070|set -euo pipefail
10071|VERSION="1.0.0"
10072|STAGE="build/stage"
10073|OUT="build/out"
10074|rm -rf "$STAGE" "$OUT"
10075|mkdir -p "$STAGE/Library/Application Support/DNEA/bin"
10076|mkdir -p "$STAGE/Library/LaunchDaemons"
10077|mkdir -p "$OUT"
10078|
10079|cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
10080|cp target/release/dnea-daemon "$STAGE/Library/Application Support/DNEA/bin/"
10081|cp target/release/dnea-nm-bridge "$STAGE/Library/Application Support/DNEA/bin/"
10082|cp target/release/dnea "$STAGE/Library/Application Support/DNEA/bin/"
10083|cp installer/macos/org.dnea.runtime.plist "$STAGE/Library/LaunchDaemons/"
10084|
10085|pkgbuild --root "$STAGE" --scripts installer/macos/scripts \
10086|  --identifier org.dnea.runtime --version "$VERSION" \
10087|  --install-location / "$OUT/dnea-component.pkg"
10088|
10089|productbuild --package "$OUT/dnea-component.pkg" \
10090|  --distribution installer/macos/distribution.xml \
10091|  --resources installer/macos/resources "$OUT/dnea-$VERSION.pkg"
10092|
10093|productsign --sign "Developer ID Installer: DNEA Project" \
10094|  "$OUT/dnea-$VERSION.pkg" "$OUT/dnea-$VERSION-signed.pkg"
10095|```
10096|
10097|### `installer/macos/distribution.xml`
10098|
10099|```xml
10100|<?xml version="1.0" encoding="utf-8"?>
10101|<installer-gui-script minSpecVersion="2">
10102|  <title>DNEA Runtime</title>
10103|  <organization>org.dnea</organization>
10104|  <domains enable_localSystem="true" />
10105|  <options customize="allow" require-scripts="false" />
10106|  <choices-outline>
10107|    <line choice="default"><line choice="org.dnea.runtime" /></line>
10108|  </choices-outline>
10109|  <choice id="default" />
10110|  <choice id="org.dnea.runtime" visible="true" title="DNEA Runtime">
10111|    <pkg-ref id="org.dnea.runtime" />
10112|  </choice>
10113|  <pkg-ref id="org.dnea.runtime" version="1.0.0">dnea-component.pkg</pkg-ref>
10114|</installer-gui-script>
10115|```
10116|
10117|## 5.3 Linux
10118|
10119|### `installer/linux/nfpm.yaml`
10120|
10121|```yaml
10122|name: dnea
10123|arch: amd64
10124|platform: linux
10125|version: 1.0.0
10126|section: utils
10127|priority: optional
10128|maintainer: DNEA Project <maintainers@dnea.org>
10129|description: |
10130|  Device-Native Extension Architecture runtime.
10131|  Installs the runtime daemon, the native messaging bridge, and the CLI.
10132|homepage: https://dnea.org
10133|license: Apache-2.0
10134|depends:
10135|  - libc6 (>= 2.31)
10136|contents:
10137|  - src: target/release/dnea-daemon
10138|    dst: /usr/lib/dnea/dnea-daemon
10139|    file_info: { mode: 0755 }
10140|  - src: target/release/dnea-nm-bridge
10141|    dst: /usr/lib/dnea/dnea-nm-bridge
10142|    file_info: { mode: 0755 }
10143|  - src: target/release/dnea
10144|    dst: /usr/bin/dnea
10145|    file_info: { mode: 0755 }
10146|  - src: installer/linux/dnea.service
10147|    dst: /usr/lib/systemd/system/dnea.service
10148|  - src: installer/linux/dnea.toml
10149|    dst: /etc/dnea/dnea.toml
10150|    type: config|noreplace
10151|scripts:
10152|  preinstall: installer/linux/scripts/preinstall.sh
10153|  postinstall: installer/linux/scripts/postinstall.sh
10154|  preremove: installer/linux/scripts/preremove.sh
10155|  postremove: installer/linux/scripts/postremove.sh
10156|```
10157|
10158|### `installer/linux/dnea.service`
10159|
10160|```ini
10161|[Unit]
10162|Description=DNEA runtime
10163|After=network.target
10164|Wants=network.target
10165|
10166|[Service]
10167|Type=notify
10168|ExecStart=/usr/lib/dnea/dnea-daemon --foreground
10169|Restart=on-failure
10170|RestartSec=2s
10171|User=dnea
10172|Group=dnea
10173|StateDirectory=dnea
10174|RuntimeDirectory=dnea
10175|RuntimeDirectoryMode=0750
10176|NoNewPrivileges=true
10177|ProtectSystem=strict
10178|ProtectHome=read-only
10179|PrivateTmp=true
10180|PrivateDevices=true
10181|ProtectKernelTunables=true
10182|ProtectKernelModules=true
10183|ProtectControlGroups=true
10184|RestrictNamespaces=true
10185|RestrictSUIDSGID=true
10186|SystemCallArchitectures=native
10187|LockPersonality=true
10188|
10189|[Install]
10190|WantedBy=multi-user.target
10191|```
10192|
10193|### `installer/linux/dnea.toml`
10194|
10195|```toml
10196|[paths]
10197|state_dir = "/var/lib/dnea"
10198|run_dir = "/run/dnea"
10199|log_dir = "/var/log/dnea"
10200|
10201|[modules]
10202|allow_dev = false
10203|pooling_slots = 16
10204|
10205|[nm]
10206|max_connections = 64
10207|mtu = 1048576
10208|
10209|[api]
10210|bind = "127.0.0.1:0"
10211|token_ttl_secs = 600
10212|
10213|[audit]
10214|retention_days = 30
10215|max_bytes_per_module = 67108864
10216|sinks = ["sqlite", "file"]
10217|
10218|[watchdog]
10219|epoch_interval_ms = 100
10220|default_fuel = 10000000
10221|default_timeout_ms = 30000
10222|```
10223|
10224|### `installer/linux/scripts/preinstall.sh`
10225|
10226|```bash
10227|#!/bin/bash
10228|set -e
10229|for dir in \
10230|  /etc/opt/chrome/native-messaging-hosts \
10231|  /etc/chromium/native-messaging-hosts \
10232|  /etc/opt/edge/native-messaging-hosts \
10233|  /usr/lib/mozilla/native-messaging-hosts; do
10234|  manifest="$dir/org.dnea.runtime.json"
10235|  if [ -f "$manifest" ] && ! grep -q "x-dnea-installed" "$manifest"; then
10236|    echo "Refusing to overwrite foreign host manifest at $manifest" >&2
10237|    exit 1
10238|  fi
10239|done
10240|exit 0
10241|```
10242|
10243|### `installer/linux/scripts/postinstall.sh`
10244|
10245|```bash
10246|#!/bin/bash
10247|set -e
10248|if ! id dnea >/dev/null 2>&1; then
10249|  adduser --system --group --home /var/lib/dnea --shell /usr/sbin/nologin dnea
10250|fi
10251|mkdir -p /var/lib/dnea
10252|chown dnea:dnea /var/lib/dnea
10253|chmod 750 /var/lib/dnea
10254|sudo -u dnea /usr/lib/dnea/dnea-daemon --init-key --config /etc/dnea/dnea.toml || true
10255|
10256|for dir in \
10257|  /etc/opt/chrome/native-messaging-hosts \
10258|  /etc/chromium/native-messaging-hosts \
10259|  /etc/opt/edge/native-messaging-hosts \
10260|  /usr/lib/mozilla/native-messaging-hosts; do
10261|  mkdir -p "$dir"
10262|  ext_prefix="chrome-extension"
10263|  ext_id="abcdefghijklmnopabcdefghijklmnop"
10264|  case "$dir" in
10265|    *mozilla*) ext_prefix="moz-extension"; ext_id="dnea@dnea.org" ;;
10266|  esac
10267|  cat > "$dir/org.dnea.runtime.json" <<EOF
10268|{
10269|  "name": "org.dnea.runtime",
10270|  "description": "DNEA runtime bridge",
10271|  "path": "/usr/lib/dnea/dnea-nm-bridge",
10272|  "type": "stdio",
10273|  "allowed_origins": ["$ext_prefix://$ext_id/"],
10274|  "x-dnea-installed": { "version": "1.0.0" }
10275|}
10276|EOF
10277|done
10278|
10279|systemctl daemon-reload
10280|systemctl enable --now dnea.service || true
10281|
10282|if [ -f /etc/default/dnea ] && grep -q "DNEA_POLICY=1" /etc/default/dnea; then
10283|  mkdir -p /etc/opt/chrome/policies/managed /etc/chromium/policies/managed
10284|  cat > /etc/opt/chrome/policies/managed/dnea.json <<EOF
10285|{ "ExtensionInstallForcelist": ["abcdefghijklmnopabcdefghijklmnop;https://clients2.google.com/service/update2/crx"] }
10286|EOF
10287|  cp /etc/opt/chrome/policies/managed/dnea.json /etc/chromium/policies/managed/dnea.json
10288|  mkdir -p /etc/firefox/policies
10289|  if [ ! -f /etc/firefox/policies/policies.json ]; then
10290|    cat > /etc/firefox/policies/policies.json <<EOF
10291|{ "policies": { "Extensions": { "Install": ["dnea@dnea.org"] } } }
10292|EOF
10293|  fi
10294|fi
10295|exit 0
10296|```
10297|
10298|### `installer/linux/scripts/preremove.sh`
10299|
10300|```bash
10301|#!/bin/bash
10302|set -e
10303|systemctl stop dnea.service || true
10304|exit 0
10305|```
10306|
10307|### `installer/linux/scripts/postremove.sh`
10308|
10309|```bash
10310|#!/bin/bash
10311|set -e
10312|if [ "$1" = "purge" ]; then
10313|  rm -rf /var/lib/dnea
10314|fi
10315|# 30-day grace otherwise (systemd-tmpfiles handles this in real deployments).
10316|exit 0
10317|```
10318|
10319|### `installer/linux/build.sh`
10320|
10321|```bash
10322|#!/bin/bash
10323|set -euo pipefail
10324|cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
10325|mkdir -p build/out
10326|nfpm package --config installer/linux/nfpm.yaml --packager deb --target build/out/
10327|nfpm package --config installer/linux/nfpm.yaml --packager rpm --target build/out/
10328|ls -la build/out/
10329|```
10330|
10331|---
10332|
10333|# PART 6 — CI AND RELEASE
10334|
10335|## `.github/workflows/ci.yml`
10336|
10337|```yaml
10338|name: CI
10339|on:
10340|  push: { branches: [main] }
10341|  pull_request:
10342|
10343|env:
10344|  CARGO_TERM_COLOR: always
10345|  RUSTFLAGS: "-D warnings"
10346|  CARGO_COMPONENT_VERSION: "0.20.0"
10347|
10348|jobs:
10349|  fmt:
10350|    runs-on: ubuntu-latest
10351|    steps:
10352|      - uses: actions/checkout@v4
10353|      - uses: dtolnay/rust-toolchain@stable
10354|        with: { components: rustfmt }
10355|      - run: cargo fmt --all -- --check
10356|
10357|  clippy:
10358|    runs-on: ubuntu-latest
10359|    steps:
10360|      - uses: actions/checkout@v4
10361|      - uses: dtolnay/rust-toolchain@stable
10362|        with: { components: clippy }
10363|      - uses: Swatinem/rust-cache@v2
10364|      - run: cargo clippy --workspace --all-targets -- -D warnings
10365|
10366|  test:
10367|    name: test (${{ matrix.os }})
10368|    strategy: { fail-fast: false, matrix: { os: [ubuntu-latest, macos-latest, windows-latest] } }
10369|    runs-on: ${{ matrix.os }}
10370|    steps:
10371|      - uses: actions/checkout@v4
10372|      - uses: dtolnay/rust-toolchain@stable
10373|      - uses: Swatinem/rust-cache@v2
10374|      - name: Install cargo-component
10375|        run: cargo install cargo-component --version ${{ env.CARGO_COMPONENT_VERSION }} --locked
10376|      - name: Build Focus Mode
10377|        run: cargo component build -p focus-mode --release
10378|      - name: Test
10379|        run: cargo test --workspace --all-targets
10380|
10381|  thin-client:
10382|    runs-on: ubuntu-latest
10383|    steps:
10384|      - uses: actions/checkout@v4
10385|      - uses: pnpm/action-setup@v4
10386|        with: { version: 9 }
10387|      - uses: actions/setup-node@v4
10388|        with: { node-version: 20 }
10389|      - working-directory: thin-client
10390|        run: pnpm install
10391|      - working-directory: thin-client
10392|        run: pnpm compile
10393|      - working-directory: thin-client
10394|        run: pnpm build:chrome
10395|      - working-directory: thin-client
10396|        run: pnpm build:firefox
10397|
10398|  audit:
10399|    runs-on: ubuntu-latest
10400|    steps:
10401|      - uses: actions/checkout@v4
10402|      - uses: dtolnay/rust-toolchain@stable
10403|      - uses: EmbarkStudios/cargo-deny-action@v1
10404|        with: { command: check advisories bans licenses sources }
10405|```
10406|
10407|## `.github/workflows/release.yml`
10408|
10409|```yaml
10410|name: Release
10411|on:
10412|  push: { tags: ["v*"] }
10413|
10414|jobs:
10415|  build:
10416|    strategy:
10417|      matrix:
10418|        include:
10419|          - { os: ubuntu-latest, artifact: linux-amd64 }
10420|          - { os: macos-latest, artifact: macos-universal }
10421|          - { os: windows-latest, artifact: windows-amd64 }
10422|    runs-on: ${{ matrix.os }}
10423|    steps:
10424|      - uses: actions/checkout@v4
10425|      - uses: dtolnay/rust-toolchain@stable
10426|      - uses: Swatinem/rust-cache@v2
10427|      - run: cargo build --release -p dnea-daemon -p dnea-nm-host -p dnea-cli
10428|      - if: matrix.os == 'ubuntu-latest'
10429|        run: |
10430|          go install github.com/goreleaser/nfpm/v2/cmd/nfpm@latest
10431|          ./installer/linux/build.sh
10432|      - if: matrix.os == 'macos-latest'
10433|        run: ./installer/macos/build.sh
10434|      - if: matrix.os == 'windows-latest'
10435|        shell: pwsh
10436|        run: |
10437|          dotnet tool install --global wix
10438|          ./installer/windows/build.ps1
10439|      - uses: actions/upload-artifact@v4
10440|        with:
10441|          name: dnea-${{ matrix.artifact }}
10442|          path: build/out/*
10443|
10444|  publish:
10445|    needs: build
10446|    runs-on: ubuntu-latest
10447|    steps:
10448|      - uses: actions/checkout@v4
10449|      - uses: actions/download-artifact@v4
10450|      - uses: softprops/action-gh-release@v2
10451|        with:
10452|          files: dnea-*/*
10453|          generate_release_notes: true
10454|```
10455|
10456|---
10457|
10458|# PART 7 — DOCUMENTATION
10459|
10460|## `docs/quickstart.md`
10461|
10462|```markdown
10463|# DNEA Quickstart
10464|
10465|Full happy path on a clean machine.
10466|
10467|## 1. Build
10468|
10469|```bash
10470|git clone https://github.com/dnea-project/dnea
10471|cd dnea
10472|cargo build --release
10473|cargo install cargo-component --version 0.20.0 --locked
10474|cargo component build -p focus-mode --release
10475|```
10476|
10477|## 2. Initialize
10478|
10479|```bash
10480|./target/release/dnea-daemon --init-key
10481|```
10482|
10483|## 3. Run the runtime
10484|
10485|```bash
10486|./target/release/dnea-daemon &
10487|sleep 1
10488|```
10489|
10490|## 4. Sign Focus Mode
10491|
10492|```bash
10493|./target/release/dnea keygen --out signing.key
10494|# Copy the printed public key into trust.json:
10495|cat > trust.json <<'EOF'
10496|{ "keys": [
10497|  { "keyId": "https://example.com/keys/signing",
10498|    "alg": "EdDSA",
10499|    "publicKey": "REPLACE_WITH_PRINTED_KEY" }
10500|]}
10501|EOF
10502|```
10503|
10504|Create a manifest. Compute the digest:
10505|
10506|```bash
10507|DIGEST=$(sha256sum target/wasm32-wasip1/release/focus_mode.wasm \
10508|  | awk '{print $1}' | xxd -r -p | base64 | tr '+/' '-_' | tr -d '=')
10509|cat > focus-mode.json <<EOF
10510|{
10511|  "manifestVersion": 1,
10512|  "id": "com.example.focus-mode",
10513|  "version": "1.2.0",
10514|  "runtime": ">=1.0.0 <2.0.0",
10515|  "name": "Focus Mode",
10516|  "entrypoints": {
10517|    "core": {
10518|      "kind": "wasm", "path": "focus_mode.wasm",
10519|      "digest": { "alg": "sha-256", "value": "sha-256:$DIGEST" }
10520|    }
10521|  },
10522|  "capabilities": {
10523|    "request": [
10524|      { "id": "storage:module" },
10525|      { "id": "runtime:log" },
10526|      { "id": "browser:tabs:query", "reason": "Detect blocked sites." }
10527|    ]
10528|  },
10529|  "ui": [
10530|    { "id": "dashboard", "kind": "panel", "entrypoint": "core",
10531|      "surfaces": ["companion"], "title": "Focus Mode" }
10532|  ]
10533|}
10534|EOF
10535|./target/release/dnea sign \
10536|  --manifest focus-mode.json \
10537|  --key signing.key \
10538|  --key-id "https://example.com/keys/signing"
10539|./target/release/dnea validate focus-mode.json \
10540|  --trust trust.json \
10541|  --artifacts target/wasm32-wasip1/release
10542|```
10543|
10544|## 5. Install
10545|
10546|```bash
10547|cp target/wasm32-wasip1/release/focus_mode.wasm .
10548|./target/release/dnea install-module \
10549|  --manifest focus-mode.json \
10550|  --artifacts .
10551|```
10552|
10553|## 6. Consent
10554|
10555|Open the companion URL from `api.json`. Approve the `browser:tabs:query`
10556|prompt. The module appears in the list and its panel renders.
10557|
10558|## 7. Thin client (optional)
10559|
10560|```bash
10561|cd thin-client
10562|pnpm install
10563|pnpm build:chrome
10564|```
10565|
10566|Load `thin-client/.output/chrome-mv3` as an unpacked extension. The
10567|popup shows the connection state, and pages can invoke the module
10568|through `window.__dnea`.
10569|
10570|## 8. Verify structural enforcement
10571|
10572|Edit `focus-mode.json` to add `"browser:history:read"` to the deny list,
10573|re-sign, and reinstall. The module cannot read history regardless of
10574|what any policy grants.
10575|```
10576|
10577|## `docs/security.md`
10578|
10579|```markdown
10580|# Security Model
10581|
10582|## Threat model
10583|
10584|DNEA assumes untrusted modules, untrusted pages, untrusted networks,
10585|and partially-trusted local users.
10586|
10587|## Explicit non-goals
10588|
10589|- Protecting against a fully compromised OS kernel or hypervisor.
10590|- Protecting against a malicious user who already controls the runtime's account.
10591|- Providing confidentiality against an attacker who can read runtime process memory.
10592|
10593|## Invariants
10594|
10595|1. The Local API MUST bind exclusively to `127.0.0.1` or `::1`.
10596|2. `api.json` is created with mode `0600` on Unix, or an ACL granting
10597|   access only to the creating user on Windows.
10598|3. JWT secrets are ephemeral; they are never written to disk.
10599|4. Module storage keys are HKDF-derived with info string
10600|   `dnea-store-v1:<module-id>`. Renaming a module yields a different
10601|   key space.
10602|5. Wasm modules are linked only against the interfaces covered by their
10603|   effective grants. Ungranted imports fail instantiation.
10604|6. `capabilities.deny` is enforceable and cannot be overridden by a
10605|   later grant.
10606|7. Consent prompts expire after 300 seconds and are swept every 30
10607|   seconds.
10608|8. Audit logs never contain JWT tokens, master-key material, or
10609|   unredacted consent payloads at `info` level or higher.
10610|
10611|## Enforcement points
10612|
10613|- **Wasm structural enforcement.** Link-time.
10614|- **Capability intersection.** Effective grant = `(request ∩ policy ∩
10615|  consent) − deny`.
10616|- **Signed modules.** Ed25519 signature verified against a trust store.
10617|- **Sandboxed module UI.** Per-module origin, closed Shadow DOM, strict
10618|  CSP.
10619|- **Loopback-only API.** Bootstrap tokens require reading `api.json`,
10620|  which is user-owned with `0600`.
10621|- **Content-script isolation.** Content scripts never touch Native
10622|  Messaging; they go through the background.
10623|
10624|## What is not defended against
10625|
10626|- Malicious signed modules. Consent is the human defense.
10627|- Compromised OS accounts.
10628|- Compromised signing keys.
10629|- Supply-chain attacks on a module's build (DNEA verifies signatures,
10630|  not build provenance).
10631|```
10632|
10633|## `docs/migration.md`
10634|
10635|```markdown
10636|# Migrating a WebExtension to DNEA
10637|
10638|## Mapping
10639|
10640|| WebExtension | DNEA |
10641||---|---|
10642|| `chrome.storage` | `storage:module` + `dnea::host::storage` |
10643|| `chrome.tabs` | `browser:tabs:*` |
10644|| `chrome.runtime.sendMessage` | Envelope over Native Messaging |
10645|| Permissions in `manifest.json` | Capabilities in `module-manifest.json` |
10646|| `chrome.runtime.onInstalled` | `lifecycle.install` |
10647|
10648|## Path
10649|
10650|1. Extract logic that is not browser-API interaction.
10651|2. Map permissions to capabilities.
10652|3. Build the thin client (use `thin-client/`).
10653|4. Build the module (use `crates/focus-mode/` as a template).
10654|5. Sign the manifest and distribute the signed module.
10655|
10656|## What you gain
10657|
10658|- One install works on every browser.
10659|- Capabilities are enforced structurally.
10660|- Modules can do work the browser sandbox forbids.
10661|- Long-running work continues when no browser is open.
10662|
10663|## What you lose
10664|
10665|- The simplicity of a single `manifest.json`.
10666|- Direct `chrome.*` access from module code.
10667|- Unsigned updates.
10668|```
10669|
10670|## `docs/w3c-submission.md`
10671|
10672|```markdown
10673|# DNEA submission to the WebExtensions Community Group
10674|
10675|**From:** DNEA Project
10676|**To:** WebExtensions Community Group (webextensions-cg@w3.org)
10677|**Subject:** Proposal for a capability vocabulary, Native Messaging
10678|envelope, and thin-client contract as complements to WebExtensions
10679|
10680|## Summary
10681|
10682|DNEA proposes three components for the CG's consideration:
10683|
10684|1. A capability vocabulary: hierarchical, constraint-scoped,
10685|   machine-checkable.
10686|2. A Native Messaging envelope: versioned, chunked, correlated,
10687|   direction-validated.
10688|3. A thin-client contract: connection lifecycle, codec, content-script
10689|   bridge, consent surface.
10690|
10691|DNEA does not require changes to the WebExtensions API. It uses what
10692|exists.
10693|
10694|## Motivation
10695|
10696|An extension that needs device-level state must today maintain
10697|per-browser native messaging hosts, per-browser manifests, and
10698|per-browser policy. A device-level runtime inverts this: one install
10699|makes the extension available to every browser on the device.
10700|
10701|## Reference implementation
10702|
10703|Apache-2.0 at https://github.com/dnea-project/dnea.
10704|
10705|## Contact
10706|
10707|specs@dnea.org
10708|```
10709|
10710|---
10711|
10712|# PART 8 — FINAL VERIFICATION CHECKLIST
10713|
10714|Before tagging `v1.0.0`:
10715|
10716|- [ ] `cargo fmt --all -- --check` passes
10717|- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
10718|- [ ] `cargo test --workspace --all-targets` passes on Linux/macOS/Windows
10719|- [ ] `cargo component build -p focus-mode --release` produces `focus_mode.wasm`
10720|- [ ] `dnea validate focus-mode.json --trust trust.json --artifacts target/wasm32-wasip1/release` passes
10721|- [ ] `dnea install-module` succeeds and the module appears in `GET /v1/modules`
10722|- [ ] Consent prompt appears; granting it updates the grant set
10723|- [ ] A module without `browser:tabs:*` fails to link `tabs` at instantiation
10724|- [ ] `thin-client`: `pnpm install && pnpm compile && pnpm build:chrome && pnpm build:firefox` succeed
10725|- [ ] `thin-client`: background connects, handshake completes, welcome received
10726|- [ ] `thin-client`: 3 MB event round-trips through chunking
10727|- [ ] `thin-client`: content-script port registers/unregisters correctly
10728|- [ ] Installer: MSI/PKG/DEB build without errors
10729|- [ ] Installer: uninstall leaves no DNEA-authored manifests or policy entries
10730|- [ ] `api.json` is not world-readable
10731|- [ ] Master key is never written to the log
10732|- [ ] Foreign native-messaging hosts are preserved by conflict detection
10733|
10734|---
10735|
10736|# FINAL STATUS
10737|
10738|DNEA v1.0 is complete. Nine crates, one guest module, one thin
10739|client, three installers, six specification artifacts, one CI matrix,
10740|one release workflow, and the W3C engagement materials.
10741|
10742|The critical properties hold by construction, not convention:
10743|
10744|1. **Structural enforcement** at the Wasm boundary.
10745|2. **Signed installs** with Ed25519 and JCS canonicalization.
10746|3. **No partial installs** — the pipeline is fail-fast.
10747|4. **Loopback-only** Local API with ephemeral JWT secrets.
10748|5. **Encrypted per-module storage** with HKDF-derived keys.
10749|6. **Consent-gated** prompt-tier capabilities.
10750|7. **Symmetric chunking** across the Native Messaging boundary.
10751|
10752|Everything above is buildable from a clean checkout with the commands
10753|at the top. Tag `v1.0.0` and ship.