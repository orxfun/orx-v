use super::super::Dim;
use super::MutAt;
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt<D, S, I, T, F>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    data: S,
    fun: F,
    p: PhantomData<fn(&mut I) -> D>,
}

impl<D, S, I, T, F> FunMutAt<D, S, I, T, F>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    pub fn into_data(self) -> S {
        self.data
    }
}

impl<D, S: Clone, I, T, F: Clone> Clone for FunMutAt<D, S, I, T, F>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<D, S: Copy, I, T, F: Copy> Copy for FunMutAt<D, S, I, T, F>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
}

impl<D, S, I, T, F> MutAt<D, T> for FunMutAt<D, S, I, T, F>
where
    D: Dim,
    S: BorrowMut<I>,
    F: for<'a> FnMut(&'a mut I, D::Idx) -> &'a mut T,
{
    fn mut_at(&mut self, idx: <D as Dim>::Idx) -> &mut T {
        (self.fun)(self.data.borrow_mut(), idx)
    }
}
