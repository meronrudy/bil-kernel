#![no_std]
#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    BufferTooSmall,
}

pub trait CanonicalEncode {
    fn encode_canonical(&self, out: &mut [u8]) -> Result<usize, CodecError>;
}
