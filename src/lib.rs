#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod btree_map;
mod dimensions;
mod failures;
mod funvec;
mod nvec;
mod nvec_mut;
mod overloads;
mod slice;
mod transformations;
mod vec;

pub use dimensions::*;
pub use funvec::{FunVec, FunVecBuilder, IntoFunVec};
pub use nvec::NVec;
pub use nvec_mut::NVecMut;
pub use overloads::CopyOrRef;
pub use transformations::{
    AsJagged, AsMatrix, Cached, Completed, Hooked, IntoCached, IntoCompleted, IntoHooked,
    VecD1AsJagged, VecD1AsMatrix,
};
