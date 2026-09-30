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