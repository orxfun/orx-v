#[cfg(test)]
mod tests;

mod fun;
mod mut_at_trait;
mod slice;
mod vec;

pub use fun::FunMutAt;
pub use mut_at_trait::MutAt;
