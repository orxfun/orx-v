use super::super::{D3, Dim};
use super::{At, FunAt3Child2};
use derive_new::new;

#[derive(new)]
pub struct FunAt3Child<'a, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    c: usize,
    fun: &'a F,
}

impl<T, F> At<<D3 as Dim>::PrevDim, T> for FunAt3Child<'_, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    fn at(&self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> T {
        let idx = <D3 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.fun)(idx)
    }

    fn try_at(&self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> Option<T> {
        Some(self.at(idx))
    }

    type Child<'c>
        = FunAt3Child2<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        FunAt3Child2::new(self.c, c, self.fun)
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        Some(self.child(c))
    }
}
