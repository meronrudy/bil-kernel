#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod event;
pub mod primitives;
pub mod receipt;

pub use error::*;
pub use event::*;
pub use primitives::*;
pub use receipt::*;

pub trait Projection<E> {
    type Output;

    fn project(event: &E) -> Self::Output;
}
