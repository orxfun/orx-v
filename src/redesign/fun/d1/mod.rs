#[cfg(test)]
mod tests;

mod fun_trait;
mod fun_vec;

pub use fun_trait::{Fun1, FunWithData1, FunWithoutData1};
pub use fun_vec::FunVec1;
