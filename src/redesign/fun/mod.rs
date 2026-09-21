#[cfg(test)]
mod tests;

mod fun_vec1;
mod fun_vec2;
mod fun_vec3;
mod fun_vec4;
mod funvec;

pub use fun_vec1::{FunVec1, FunVec1ChildOfD2, FunVec1ChildOfD3, FunVec1ChildOfD4};
pub use fun_vec2::{FunVec2, FunVec2ChildOfD3, FunVec2ChildOfD4};
pub use fun_vec3::{FunVec3, FunVec3ChildOfD4};
pub use fun_vec4::FunVec4;
pub use funvec::FunVec;
