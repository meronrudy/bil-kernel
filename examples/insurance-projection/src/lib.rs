use bil_core::{KernelEvent, Projection};

pub struct InsuranceProjection;

#[derive(Debug, PartialEq, Eq)]
pub struct InsuranceView {
    pub local_label: &'static str,
    pub evidence_present: bool,
    pub prior_link_present: bool,
}

impl Projection<KernelEvent> for InsuranceProjection {
    type Output = InsuranceView;

    fn project(event: &KernelEvent) -> Self::Output {
        InsuranceView {
            local_label: "insurance-local-view",
            evidence_present: true,
            prior_link_present: event.previous.is_some(),
        }
    }
}

