use bil_core::{KernelEvent, Projection};

pub struct LegalProjection;

#[derive(Debug, PartialEq, Eq)]
pub struct LegalView {
    pub local_label: &'static str,
    pub authority_reference_present: bool,
    pub evidence_commitment_present: bool,
}

impl Projection<KernelEvent> for LegalProjection {
    type Output = LegalView;

    fn project(_event: &KernelEvent) -> Self::Output {
        LegalView {
            local_label: "legal-local-view",
            authority_reference_present: true,
            evidence_commitment_present: true,
        }
    }
}

