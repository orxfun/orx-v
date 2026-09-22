use super::super::Dim;
use super::MutAt;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunMutAt<'a, D, T, F>
where
    D: Dim,
    F: FnMut(D::Idx) -> &'a mut T,
    T: 'a,
{
    fun: F,
    p: PhantomData<D>,
}

impl<'a, D, T, F: Clone> Clone for FunMutAt<'a, D, T, F>
where
    D: Dim,
    F: FnMut(D::Idx) -> &'a mut T,
    T: 'a,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, F: Copy> Copy for FunMutAt<'a, D, T, F>
where
    D: Dim,
    F: FnMut(D::Idx) -> &'a mut T,
    T: 'a,
{
}

impl<'a, D, T, F> MutAt<D, T> for FunMutAt<'a, D, T, F>
where
    D: Dim,
    F: FnMut(D::Idx) -> &'a mut T,
    T: 'a,
{
    fn mut_at(&mut self, idx: <D as Dim>::Idx) -> &mut T {
        (self.fun)(idx)
    }
}
