#![no_std]
#![forbid(unsafe_code)]

pub trait HashEngine {
    fn hash32(&self, bytes: &[u8]) -> [u8; 32];
}

pub trait SignatureEngine {
    fn sign(&self, bytes: &[u8]) -> [u8; 64];
    fn verify(&self, bytes: &[u8], signature: &[u8; 64]) -> bool;
}
