#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BilError {
    StructuralVerificationFailed,
    SignatureFailed,
    AnchorFailed,
}
