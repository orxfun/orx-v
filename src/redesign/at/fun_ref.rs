use super::super::Dim;
use super::At;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunRefAt<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    fun: F,
    p: PhantomData<D>,
}

impl<'a, D, T, F: Clone> Clone for FunRefAt<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, F: Copy> Copy for FunRefAt<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
}

impl<'a, D, T, F> At<D, &'a T> for FunRefAt<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> &'a T {
        (self.fun)(idx)
    }
}
