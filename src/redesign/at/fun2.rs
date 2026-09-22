use super::super::D2;
use super::{At, FunAt, FunChildAt2};

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
        = FunChildAt2<T, F>
    where
        Self: 'c;
}
