use super::super::D2;
use super::{AtMut, FunMutAt, FunMutAt2Child};
use core::borrow::BorrowMut;

impl<S, I, T, F, M> AtMut<D2, T> for FunMutAt<D2, S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, [usize; 2]) -> &'a T,
    M: for<'a> FnMut(&'a mut I, [usize; 2]) -> &'a mut T,
{
    fn at(&self, idx: [usize; 2]) -> &T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: [usize; 2]) -> Option<&T> {
        Some(self.core_at(idx))
    }

    fn at_mut(&mut self, idx: [usize; 2]) -> &mut T {
        self.core_at_mut(idx)
    }

    fn try_at_mut(&mut self, idx: [usize; 2]) -> Option<&mut T> {
        Some(self.core_at_mut(idx))
    }

    type ChildMut<'c>
        = FunMutAt2Child<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: usize) -> Self::ChildMut<'c> {
        FunMutAt2Child::new(c, self.data.borrow_mut(), &self.f, &mut self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: usize) -> Option<Self::ChildMut<'c>> {
        Some(self.child_mut(c))
    }
}
