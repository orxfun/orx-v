use super::super::D1;
use super::{At, AtNever, FunAt};

impl<T, F> At<D1, T> for FunAt<D1, T, F>
where
    F: Fn(usize) -> T,
{
    fn at(&self, idx: usize) -> T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        Some(self.core_at(idx))
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;
}
