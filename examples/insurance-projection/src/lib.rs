use bil_core::{KernelEvent, Projection};

pub struct InsuranceProjection;

#[derive(Debug, Eq, PartialEq)]
pub struct InsuranceView {
    pub evidence_present: bool,
    pub has_prior_link: bool,
}

impl Projection<KernelEvent> for InsuranceProjection {
    type Output = InsuranceView;

    fn project(event: &KernelEvent) -> Self::Output {
        InsuranceView {
            evidence_present: true,
            has_prior_link: event.previous.is_some(),
        }
    }
}

