use super::super::{D1, IdxNever};
use super::{At, AtNever};
use derive_new::new;

#[derive(new)]
pub struct FunAt3Child2<'a, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    c1: usize,
    c2: usize,
    fun: &'a F,
}

impl<T, F> At<D1, T> for FunAt3Child2<'_, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    fn at(&self, idx: usize) -> T {
        (self.fun)([self.c1, self.c2, idx])
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        Some(self.at(idx))
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}