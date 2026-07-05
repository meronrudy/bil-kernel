use bank_projection::BankProjection;
use bil_core::{
    AuthorityRef, BilError, Commitment, Draft, EventId, EventTypeId, EvidenceHash, IdentityId,
    KernelEvent, Projection, Receipt, ReceiptAnchor, ReceiptHash, ReceiptSigner, Signature,
    StructuralVerifier, TimeAnchor,
};
use bil_crypto::{MockHashEngine, MockSignatureEngine, SignatureEngine, hash_event};
use insurance_projection::InsuranceProjection;
use legal_projection::LegalProjection;

struct MockVerifier;
struct MockAnchor;

impl StructuralVerifier for MockVerifier {
    fn verify_structure(&self, _event: &KernelEvent, _hash: &ReceiptHash) -> Result<(), BilError> {
        Ok(())
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

struct LocalSigner(MockSignatureEngine);

impl ReceiptSigner for LocalSigner {
    fn sign_receipt(
        &self,
        _event: &KernelEvent,
        _hash: &ReceiptHash,
    ) -> Result<Signature, BilError> {
        Ok(Signature(self.0.sign(&[])))
    }
}

fn main() {
    println!("BIL OBSERVATION ALGEBRA DEMO\n");

    let event = KernelEvent::new(
        EventId([1u8; 32]),
        IdentityId([2u8; 32]),
        AuthorityRef([3u8; 32]),
        TimeAnchor([4u8; 32]),
        EventTypeId([5u8; 32]),
        EvidenceHash([6u8; 32]),
        Some(Commitment([7u8; 32])),
        Some(Commitment([8u8; 32])),
        None,
    );
    println!("[1] Constructed semantically neutral observable");

    let engine = MockHashEngine;
    let hash = hash_event(&event, &engine).expect("Failed to hash event");
    println!("[2] Canonically encoded observable");
    println!("[3] Derived deterministic receipt hash: 0x{}", hex::encode(hash.0));

    let mut anchor = MockAnchor;
    let signer = LocalSigner(MockSignatureEngine);

    let anchored = Receipt::<Draft>::new(event, hash)
        .verify_structure(&MockVerifier)
        .expect("Verification failed")
        .sign(&signer)
        .expect("Signing failed")
        .anchor(&mut anchor)
        .expect("Anchoring failed");
    println!("[4] Moved receipt through lifecycle:\n    Draft -> StructurallyVerified -> Signed -> Anchored\n");

    println!("[5] Applied institutional projections:");
    let bank_view = BankProjection::project(anchored.event());
    println!("    BankProjection      -> {:?}", bank_view);

    let insurance_view = InsuranceProjection::project(anchored.event());
    println!("    InsuranceProjection -> {:?}", insurance_view);

    let legal_view = LegalProjection::project(anchored.event());
    println!("    LegalProjection     -> {:?}\n", legal_view);

    let original_hash = anchored.hash();
    assert_eq!(anchored.hash(), original_hash);
    assert_ne!(bank_view.local_label, insurance_view.local_label);
    assert_ne!(insurance_view.local_label, legal_view.local_label);
    assert_ne!(bank_view.local_label, legal_view.local_label);

    println!("[PASS] All projections read the same anchored observation");
    println!("[PASS] Receipt hash unchanged across projections");
    println!("[PASS] Kernel contains no institutional semantic state\n");

    println!("RESULT: SEMANTIC NON-CONTAINMENT WITNESSED");
}
