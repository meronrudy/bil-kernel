#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod event;
pub mod primitives;
pub mod receipt;

pub use error::*;
pub use event::*;
pub use primitives::*;
pub use receipt::*;

pub trait Projection<E> {
    type Output;

    fn project(event: &E) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockVerifier;
    struct MockSigner;
    struct MockAnchor;

    impl StructuralVerifier for MockVerifier {
        fn verify_structure(
            &self,
            _event: &KernelEvent,
            _hash: &ReceiptHash,
        ) -> Result<(), BilError> {
            Ok(())
        }
    }

    impl ReceiptSigner for MockSigner {
        fn sign_receipt(
            &self,
            _event: &KernelEvent,
            _hash: &ReceiptHash,
        ) -> Result<Signature, BilError> {
            Ok(Signature([7u8; 64]))
        }
    }

    impl ReceiptAnchor for MockAnchor {
        fn anchor_receipt(
            &mut self,
            _event: &KernelEvent,
            _hash: ReceiptHash,
            _signature: Option<Signature>,
        ) -> Result<(), BilError> {
            Ok(())
        }
    }

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
    fn draft_to_anchored_lifecycle_works() {
        let event = fixture_event();
        let hash = ReceiptHash([9u8; 32]);

        let mut anchor = MockAnchor;

        let anchored = Receipt::<Draft>::new(event, hash)
            .verify_structure(&MockVerifier)
            .unwrap()
            .sign(&MockSigner)
            .unwrap()
            .anchor(&mut anchor)
            .unwrap();

        assert_eq!(anchored.hash(), hash);
        assert_eq!(anchored.event(), &event);
    }
}
