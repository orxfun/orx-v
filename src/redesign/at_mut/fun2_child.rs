use super::super::{D2, Dim, IdxNever};
use super::{AtMut, AtMutNever};
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt2Child<'a, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 2]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 2]) -> &'b mut T,
{
    c: usize,
    data: &'a mut I,
    f: &'a F,
    m: &'a mut M,
    p: PhantomData<fn(&mut I) -> T>,
}

impl<I, T, F, M> AtMut<<D2 as Dim>::PrevDim, T> for FunMutAt2Child<'_, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 2]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 2]) -> &'b mut T,
{
    fn at(&self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> &T {
        let idx = <D2 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.f)(self.data, idx)
    }

    fn try_at(&self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> Option<&T> {
        Some(self.at(idx))
    }

    fn at_mut(&mut self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> &mut T {
        let idx = <D2 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.m)(self.data, idx)
    }

    fn try_at_mut(&mut self, idx: <<D2 as Dim>::PrevDim as Dim>::Idx) -> Option<&mut T> {
        Some(self.at_mut(idx))
    }

    type ChildMut<'c>
        = AtMutNever
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: IdxNever) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: IdxNever) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}
