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
    println!("BIL PROJECTION DEMO\n");

    let event = KernelEvent::new(
        EventId([1u8; 32]),
        IdentityId([2u8; 32]),
        AuthorityRef([3u8; 32]),
        TimeAnchor([4u8; 32]),
        EventTypeId([5u8; 32]),
        EvidenceHash([6u8; 32]),
        Some(Commitment([8u8; 32])),
        None,
        None,
    );

    let engine = MockHashEngine;
    let hash = hash_event(&event, &engine).expect("Failed to hash event");

    println!("Kernel receipt hash: 0x{}", hex::encode(hash.0));

    let mut anchor = MockAnchor;
    let signer = LocalSigner(MockSignatureEngine);

    let anchored = Receipt::<Draft>::new(event, hash)
        .verify_structure(&MockVerifier)
        .expect("Verification failed")
        .sign(&signer)
        .expect("Signing failed")
        .anchor(&mut anchor)
        .expect("Anchoring failed");

    let bank_view = BankProjection::project(anchored.event());
    println!("Bank projection: {:?}", bank_view);

    let insurance_view = InsuranceProjection::project(anchored.event());
    println!("Insurance projection: {:?}", insurance_view);

    let legal_view = LegalProjection::project(anchored.event());
    println!("Legal projection: {:?}", legal_view);

    assert_eq!(anchored.hash(), hash);
    println!("\n[PASS] same receipt hash under all projections");
    println!("RESULT: SHARED FACTS WITHOUT SHARED MEANING");
}
