#![no_std]
#![forbid(unsafe_code)]

use bil_core::KernelEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    BufferTooSmall,
}

pub trait CanonicalEncode {
    fn encode_canonical(&self, out: &mut [u8]) -> Result<usize, CodecError>;
}

impl CanonicalEncode for KernelEvent {
    fn encode_canonical(&self, out: &mut [u8]) -> Result<usize, CodecError> {
        let required_len = 32 * 6 + 1 + 32 + 1 + 32 + 1 + 32; // 291 bytes
        if out.len() < required_len {
            return Err(CodecError::BufferTooSmall);
        }

        let mut offset = 0;

        out[offset..offset + 32].copy_from_slice(&self.id.0);
        offset += 32;

        out[offset..offset + 32].copy_from_slice(&self.subject.0);
        offset += 32;

        out[offset..offset + 32].copy_from_slice(&self.authority.0);
        offset += 32;

        out[offset..offset + 32].copy_from_slice(&self.time.0);
        offset += 32;

        out[offset..offset + 32].copy_from_slice(&self.event_type.0);
        offset += 32;

        out[offset..offset + 32].copy_from_slice(&self.evidence.0);
        offset += 32;

        if let Some(val) = &self.value {
            out[offset] = 1;
            offset += 1;
            out[offset..offset + 32].copy_from_slice(&val.0);
            offset += 32;
        } else {
            out[offset] = 0;
            offset += 1;
            out[offset..offset + 32].fill(0);
            offset += 32;
        }

        if let Some(st) = &self.state {
            out[offset] = 1;
            offset += 1;
            out[offset..offset + 32].copy_from_slice(&st.0);
            offset += 32;
        } else {
            out[offset] = 0;
            offset += 1;
            out[offset..offset + 32].fill(0);
            offset += 32;
        }

        if let Some(prev) = &self.previous {
            out[offset] = 1;
            offset += 1;
            out[offset..offset + 32].copy_from_slice(&prev.0);
            offset += 32;
        } else {
            out[offset] = 0;
            offset += 1;
            out[offset..offset + 32].fill(0);
            offset += 32;
        }

        Ok(offset)
    }
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
    fn same_event_same_canonical_bytes() {
        let event1 = fixture_event();
        let event2 = fixture_event();

        let mut buf1 = [0u8; 300];
        let mut buf2 = [0u8; 300];

        let len1 = event1.encode_canonical(&mut buf1).unwrap();
        let len2 = event2.encode_canonical(&mut buf2).unwrap();

        assert_eq!(len1, len2);
        assert_eq!(&buf1[..len1], &buf2[..len2]);
    }

    #[test]
    fn optional_commitments_encode_deterministically() {
        let mut event = fixture_event();
        event.value = None;

        let mut buf = [0u8; 300];
        let len = event.encode_canonical(&mut buf).unwrap();

        // Check value flag is 0
        assert_eq!(buf[32 * 6], 0);
        // Check value bytes are 0
        assert_eq!(&buf[32 * 6 + 1..32 * 6 + 1 + 32], &[0u8; 32]);
        assert_eq!(len, 291);
    }

    #[test]
    fn different_previous_hash_changes_encoding() {
        let mut event1 = fixture_event();
        let mut event2 = fixture_event();
        event2.previous = Some(Hash32([9u8; 32]));

        let mut buf1 = [0u8; 300];
        let mut buf2 = [0u8; 300];

        let len1 = event1.encode_canonical(&mut buf1).unwrap();
        let len2 = event2.encode_canonical(&mut buf2).unwrap();

        assert_eq!(len1, len2);
        assert_ne!(&buf1[..len1], &buf2[..len2]);
    }

    #[test]
    fn canonical_encoding_length_is_stable() {
        let event = fixture_event();
        let mut buf = [0u8; 300];
        let len = event.encode_canonical(&mut buf).unwrap();
        assert_eq!(len, 291);
    }
}
