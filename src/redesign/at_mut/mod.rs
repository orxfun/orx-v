#[cfg(test)]
mod tests;

mod at_mut_trait;
mod fun;
mod fun1;
mod fun2;
mod fun2_child;
mod slice;
mod vec;

pub use at_mut_trait::{AtMut, AtMutNever};
pub use fun::FunMutAt;
pub use fun2_child::FunMutAt2Child;
