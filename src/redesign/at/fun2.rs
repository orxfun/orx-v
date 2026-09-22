use super::super::D2;
use super::{At, FunAt, FunAt2Child};

impl<T, F> At<D2, T> for FunAt<D2, T, F>
where
    F: Fn([usize; 2]) -> T,
{
    fn at(&self, idx: [usize; 2]) -> T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: [usize; 2]) -> Option<T> {
        Some(self.core_at(idx))
    }

    type Child<'c>
        = FunAt2Child<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        FunAt2Child::new(c, &self.fun())
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        Some(FunAt2Child::new(c, &self.fun()))
    }
}
