use super::super::Dim;
use super::MutAt;
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
    data: S,
    f: F,
    m: M,
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

impl<D, S, I, T, F, M> MutAt<D, T> for FunMutAt<D, S, I, T, F, M>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, D::Idx) -> &'a T,
    M: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    fn at(&self, idx: <D as Dim>::Idx) -> &T {
        (self.f)(self.data.borrow(), idx)
    }

    fn mut_at(&mut self, idx: <D as Dim>::Idx) -> &mut T {
        (self.m)(self.data.borrow_mut(), idx)
    }
}
