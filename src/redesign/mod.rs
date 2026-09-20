mod impl_nvec0;
mod impl_nvec1;
// mod impl_nvec2;

mod copied_cloned;
mod dimensions;
mod fun;
mod nvec;
mod nvec_mut;

pub use copied_cloned::{Cloned, Copied};
pub use dimensions::{D0, D1, D2, DNever, Dim};
pub use nvec::NVec;
pub use nvec_mut::NVecMut;
