use bil_core::{KernelEvent, Projection};

pub struct BankProjection;

#[derive(Debug, Eq, PartialEq)]
pub struct BankView {
    pub event_recorded: bool,
    pub authority_observed: bool,
}

impl Projection<KernelEvent> for BankProjection {
    type Output = BankView;

    fn project(_event: &KernelEvent) -> Self::Output {
        BankView {
            event_recorded: true,
            authority_observed: true,
        }
    }
}

