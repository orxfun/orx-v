#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
extern crate std;

mod cached;
mod cardinality;
mod children;
mod common_trait_helpers;
mod constant_vec;
mod dim;
mod empty_vec;
mod flat_jagged;
mod fun;
mod impl_nvec;
mod impl_nvec_core;
mod impl_nvec_mut;
mod nvec;
mod nvec_aliases;
mod nvec_core;
mod nvec_core_sealed;
mod nvec_mut;
mod sparse;
mod v;

/// Matrix representations.
pub mod matrices;

// pub(crate) use

pub(crate) use nvec_core_sealed::NVecCoreSealed;

// pub use

pub use cached::{Cache, CachedVec, DefaultCache, IntoCached};
pub use cardinality::{
    Card, CardD1, EmptyCard, RectangularCardD2, RectangularCardD3, RectangularCardD4,
    UnboundedCard, VariableCardD2, VariableCardD3, VariableCardD4,
};
pub use constant_vec::ConstantVec;
pub use dim::*;
pub use empty_vec::EmptyVec;
pub use flat_jagged::{FlatJagged, IntoJagged};
pub use fun::FunVec;
pub use matrices::{
    Matrix, MatrixColMajor, MatrixColMajorMut, MatrixMut, MatrixRowMajor, MatrixRowMajorMut,
    V1AsMatrix, V2AsMatrix,
};
pub use nvec::NVec;
pub use nvec_aliases::*;
pub use nvec_core::NVecCore;
pub use nvec_mut::NVecMut;
pub use sparse::{DefaultLookup, Lookup, SparseVec};
pub use v::{NewV1, NewV2, NewV3, NewV4, V};
