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

specs@github.com/LoopyLuci/Universal-Browser-Extension