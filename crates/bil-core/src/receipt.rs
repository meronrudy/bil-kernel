use core::marker::PhantomData;

use crate::{
    error::BilError,
    event::KernelEvent,
    primitives::{ReceiptHash, Signature},
};

pub struct Draft;
pub struct StructurallyVerified;
pub struct Signed;
pub struct Anchored;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Receipt<State> {
    event: KernelEvent,
    hash: ReceiptHash,
    signature: Option<Signature>,
    state: PhantomData<State>,
}

impl Receipt<Draft> {
    pub const fn new(event: KernelEvent, hash: ReceiptHash) -> Self {
        Self {
            event,
            hash,
            signature: None,
            state: PhantomData,
        }
    }

    pub fn verify_structure(
        self,
        verifier: &dyn StructuralVerifier,
    ) -> Result<Receipt<StructurallyVerified>, BilError> {
        verifier.verify_structure(&self.event, &self.hash)?;

        Ok(Receipt {
            event: self.event,
            hash: self.hash,
            signature: None,
            state: PhantomData,
        })
    }
}

impl Receipt<StructurallyVerified> {
    pub fn sign(self, signer: &dyn ReceiptSigner) -> Result<Receipt<Signed>, BilError> {
        let signature = signer.sign_receipt(&self.event, &self.hash)?;

        Ok(Receipt {
            event: self.event,
            hash: self.hash,
            signature: Some(signature),
            state: PhantomData,
        })
    }
}

impl Receipt<Signed> {
    pub fn anchor(self, anchor: &mut dyn ReceiptAnchor) -> Result<Receipt<Anchored>, BilError> {
        anchor.anchor_receipt(&self.event, self.hash, self.signature)?;

        Ok(Receipt {
            event: self.event,
            hash: self.hash,
            signature: self.signature,
            state: PhantomData,
        })
    }
}

impl Receipt<Anchored> {
    pub const fn event(&self) -> &KernelEvent {
        &self.event
    }

    pub const fn hash(&self) -> ReceiptHash {
        self.hash
    }

    pub const fn signature(&self) -> Option<Signature> {
        self.signature
    }
}

pub trait StructuralVerifier {
    fn verify_structure(&self, event: &KernelEvent, hash: &ReceiptHash) -> Result<(), BilError>;
}

pub trait ReceiptSigner {
    fn sign_receipt(
        &self,
        event: &KernelEvent,
        hash: &ReceiptHash,
    ) -> Result<Signature, BilError>;
}

pub trait ReceiptAnchor {
    fn anchor_receipt(
        &mut self,
        event: &KernelEvent,
        hash: ReceiptHash,
        signature: Option<Signature>,
    ) -> Result<(), BilError>;
}
