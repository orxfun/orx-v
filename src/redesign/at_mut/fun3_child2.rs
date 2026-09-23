use super::super::D1;
use super::{AtMut, AtMutNever};
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt3Child2<'a, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 3]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 3]) -> &'b mut T,
{
    c1: usize,
    c2: usize,
    data: &'a mut I,
    f: &'a F,
    m: &'a mut M,
    p: PhantomData<fn(&mut I) -> T>,
}

impl<I, T, F, M> AtMut<D1, T> for FunMutAt3Child2<'_, I, T, F, M>
where
    F: for<'b> Fn(&'b I, [usize; 3]) -> &'b T,
    M: for<'b> FnMut(&'b mut I, [usize; 3]) -> &'b mut T,
{
    fn at(&self, idx: usize) -> &T {
        (self.f)(self.data, [self.c1, self.c2, idx])
    }

    fn try_at(&self, idx: usize) -> Option<&T> {
        Some(self.at(idx))
    }

    fn at_mut(&mut self, idx: usize) -> &mut T {
        (self.m)(self.data, [self.c1, self.c2, idx])
    }

    fn try_at_mut(&mut self, idx: usize) -> Option<&mut T> {
        Some(self.at_mut(idx))
    }

    type ChildMut<'c>
        = AtMutNever
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: super::super::IdxNever) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: super::super::IdxNever) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}