use super::super::{D3, Dim};
use super::{AtMut, FunMutAt3Child2};
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt3Child<'a, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 3]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 3]) -> &'b mut T,
{
    c: usize,
    data: &'a mut I,
    f: &'a F,
    m: &'a mut M,
    p: PhantomData<fn(&mut I) -> T>,
}

impl<I, T, F, M> AtMut<<D3 as Dim>::PrevDim, T> for FunMutAt3Child<'_, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 3]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 3]) -> &'b mut T,
{
    fn at(&self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> &T {
        let idx = <D3 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.f)(self.data, idx)
    }

    fn try_at(&self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> Option<&T> {
        Some(self.at(idx))
    }

    fn at_mut(&mut self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> &mut T {
        let idx = <D3 as Dim>::combine_child_and_remining_indices(self.c, idx);
        (self.m)(self.data, idx)
    }

    fn try_at_mut(&mut self, idx: <<D3 as Dim>::PrevDim as Dim>::Idx) -> Option<&mut T> {
        Some(self.at_mut(idx))
    }

    type ChildMut<'c>
        = FunMutAt3Child2<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: usize) -> Self::ChildMut<'c> {
        FunMutAt3Child2::new(self.c, c, self.data, self.f, self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: usize) -> Option<Self::ChildMut<'c>> {
        Some(self.child_mut(c))
    }
}
