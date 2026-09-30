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