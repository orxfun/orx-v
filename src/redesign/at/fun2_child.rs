use super::super::{D2, Dim};
use super::{At, AtNever};
use derive_new::new;

#[derive(new)]
pub struct FunChildAt2<T, F>
where
    F: Fn([usize; 2]) -> T,
{
    c: usize,
    fun: F,
}

impl<T, F> At<<D2 as Dim>::PrevDim, T> for FunChildAt2<T, F>
where
    F: Fn([usize; 2]) -> T,
{
    fn at(&self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> T {
        let idx = <D2 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.fun)(idx)
    }

    fn try_at(&self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> Option<T> {
        Some(self.at(idx))
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;
}
