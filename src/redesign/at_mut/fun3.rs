use super::super::D3;
use super::{AtMut, FunMutAt, FunMutAt3Child};
use core::borrow::BorrowMut;

impl<S, I, T, F, M> AtMut<D3, T> for FunMutAt<D3, S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, [usize; 3]) -> &'a T,
    M: for<'a> FnMut(&'a mut I, [usize; 3]) -> &'a mut T,
{
    fn at(&self, idx: [usize; 3]) -> &T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: [usize; 3]) -> Option<&T> {
        Some(self.core_at(idx))
    }

    fn at_mut(&mut self, idx: [usize; 3]) -> &mut T {
        self.core_at_mut(idx)
    }

    fn try_at_mut(&mut self, idx: [usize; 3]) -> Option<&mut T> {
        Some(self.core_at_mut(idx))
    }

    type ChildMut<'c>
        = FunMutAt3Child<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: usize) -> Self::ChildMut<'c> {
        FunMutAt3Child::new(c, self.data.borrow_mut(), &self.f, &mut self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: usize) -> Option<Self::ChildMut<'c>> {
        Some(self.child_mut(c))
    }
}