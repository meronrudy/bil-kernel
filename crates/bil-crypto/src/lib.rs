#![no_std]
#![forbid(unsafe_code)]

use bil_codec::CanonicalEncode;
use bil_core::{KernelEvent, primitives::ReceiptHash};

pub trait HashEngine {
    fn hash32(&self, bytes: &[u8]) -> [u8; 32];
}

pub trait SignatureEngine {
    fn sign(&self, bytes: &[u8]) -> [u8; 64];
    fn verify(&self, bytes: &[u8], signature: &[u8; 64]) -> bool;
}

pub struct MockHashEngine;

impl HashEngine for MockHashEngine {
    fn hash32(&self, bytes: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        for (i, &b) in bytes.iter().enumerate() {
            hash[i % 32] ^= b;
        }
        hash
    }
}

pub struct MockSignatureEngine;

impl SignatureEngine for MockSignatureEngine {
    fn sign(&self, _bytes: &[u8]) -> [u8; 64] {
        [7u8; 64]
    }

    fn verify(&self, _bytes: &[u8], signature: &[u8; 64]) -> bool {
        signature == &[7u8; 64]
    }
}

pub fn hash_event(event: &KernelEvent, engine: &dyn HashEngine) -> Result<ReceiptHash, bil_codec::CodecError> {
    let mut buf = [0u8; 512];
    let len = event.encode_canonical(&mut buf)?;
    Ok(ReceiptHash(engine.hash32(&buf[..len])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bil_core::{
        AuthorityRef, Commitment, EventId, EventTypeId, EvidenceHash, Hash32, IdentityId,
        TimeAnchor,
    };

    fn fixture_event() -> KernelEvent {
        KernelEvent::new(
            EventId([1u8; 32]),
            IdentityId([2u8; 32]),
            AuthorityRef([3u8; 32]),
            TimeAnchor([4u8; 32]),
            EventTypeId([5u8; 32]),
            EvidenceHash([6u8; 32]),
            Some(Commitment([8u8; 32])),
            None,
            None,
        )
    }

    #[test]
    fn same_bytes_same_hash() {
        let engine = MockHashEngine;
        let bytes = b"hello world";
        assert_eq!(engine.hash32(bytes), engine.hash32(bytes));
    }

    #[test]
    fn different_bytes_different_hash_fixture() {
        let engine = MockHashEngine;
        assert_ne!(engine.hash32(b"hello"), engine.hash32(b"world"));
    }

    #[test]
    fn mock_signature_verifies() {
        let engine = MockSignatureEngine;
        let bytes = b"hello";
        let sig = engine.sign(bytes);
        assert!(engine.verify(bytes, &sig));
    }

    #[test]
    fn bad_signature_fails() {
        let engine = MockSignatureEngine;
        let bytes = b"hello";
        let bad_sig = [8u8; 64];
        assert!(!engine.verify(bytes, &bad_sig));
    }

    #[test]
    fn same_event_same_receipt_hash() {
        let engine = MockHashEngine;
        let event1 = fixture_event();
        let event2 = fixture_event();

        let hash1 = hash_event(&event1, &engine).unwrap();
        let hash2 = hash_event(&event2, &engine).unwrap();

        assert_eq!(hash1, hash2);
    }
}
