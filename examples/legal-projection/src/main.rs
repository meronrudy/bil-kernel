use bil_core::{KernelEvent, Projection};

pub struct LegalProjection;

#[derive(Debug, Eq, PartialEq)]
pub struct LegalView {
    pub is_admissible: bool,
    pub has_authority: bool,
}

impl Projection<KernelEvent> for LegalProjection {
    type Output = LegalView;

    fn project(_event: &KernelEvent) -> Self::Output {
        LegalView {
            is_admissible: true,
            has_authority: true,
        }
    }
}

fn main() {
    println!("Legal projection example");
}
