mod impl_nvec0;
mod impl_nvec1;
mod impl_nvec2;

mod cloned;
mod copied;
mod dimensions;
mod fun;
mod nvec;
mod nvec_mut;
mod nvec_ref;

pub use cloned::Cloned;
pub use copied::Copied;
pub use dimensions::{D1, D2, D3, D4, DNever, Dim, IdxNever};
pub use fun::FunVec;
pub use impl_nvec0::NVecNever;
pub use nvec::NVec;
pub use nvec_mut::NVecMut;
pub use nvec_ref::NVecRef;
