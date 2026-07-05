use bil_core::{KernelEvent, Projection};

pub struct BankProjection;

#[derive(Debug, PartialEq, Eq)]
pub struct BankView {
    pub local_label: &'static str,
    pub authority_observed: bool,
    pub value_commitment_present: bool,
}

impl Projection<KernelEvent> for BankProjection {
    type Output = BankView;

    fn project(event: &KernelEvent) -> Self::Output {
        BankView {
            local_label: "bank-local-view",
            authority_observed: true,
            value_commitment_present: event.value.is_some(),
        }
    }
}

