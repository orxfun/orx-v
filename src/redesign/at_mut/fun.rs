use super::super::Dim;
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt<D, S, I, T, F, M>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, D::Idx) -> &'a T,
    M: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    pub(super) data: S,
    pub(super) f: F,
    pub(super) m: M,
    p: PhantomData<fn(&mut I) -> D>,
}

impl<D, S, I, T, F, M> FunMutAt<D, S, I, T, F, M>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, D::Idx) -> &'a T,
    M: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    pub fn into_data(self) -> S {
        self.data
    }

    pub(super) fn f(&self) -> &F {
        &self.f
    }

    pub(super) fn m_mut(&mut self) -> &mut M {
        &mut self.m
    }

    pub(super) fn data_mut(&mut self) -> &mut I {
        self.data.borrow_mut()
    }

    pub(super) fn core_at(&self, idx: D::Idx) -> &T {
        (self.f)(self.data.borrow(), idx)
    }

    pub(super) fn core_at_mut(&mut self, idx: D::Idx) -> &mut T {
        (self.m)(self.data.borrow_mut(), idx)
    }
}

impl<D, S: Clone, I, T, F: Clone, M: Clone> Clone for FunMutAt<D, S, I, T, F, M>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, D::Idx) -> &'a T,
    M: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            f: self.f.clone(),
            m: self.m.clone(),
            p: PhantomData,
        }
    }
}

impl<D, S: Copy, I, T, F: Copy, M: Copy> Copy for FunMutAt<D, S, I, T, F, M>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, D::Idx) -> &'a T,
    M: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
}
