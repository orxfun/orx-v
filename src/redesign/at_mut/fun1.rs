use super::super::{D1, IdxNever};
use super::{AtMut, AtMutNever, FunMutAt};
use core::borrow::BorrowMut;

impl<S, I, T, F, M> AtMut<D1, T> for FunMutAt<D1, S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize) -> &'a mut T,
{
    fn at(&self, idx: usize) -> &T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: usize) -> Option<&T> {
        Some(self.core_at(idx))
    }

    fn at_mut(&mut self, idx: usize) -> &mut T {
        self.core_at_mut(idx)
    }

    fn try_at_mut(&mut self, idx: usize) -> Option<&mut T> {
        Some(self.core_at_mut(idx))
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
