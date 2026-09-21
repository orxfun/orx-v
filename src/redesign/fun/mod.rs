#[cfg(test)]
mod tests;

mod funvec;
mod funvec1;
mod funvec2;
mod funvec3;
mod funvec4;

pub use funvec::FunVec;
pub use funvec1::{FunVec1, FunVec1ChildOfD2, FunVec1ChildOfD3, FunVec1ChildOfD4};
pub use funvec2::{FunVec2, FunVec2ChildOfD3, FunVec2ChildOfD4};
pub use funvec3::{FunVec3, FunVec3ChildOfD4};
pub use funvec4::FunVec4;
