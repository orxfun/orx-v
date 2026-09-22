use super::super::Dim;
use super::At;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fun: F,
    p: PhantomData<D>,
}

impl<D, T, F: Clone> Clone for FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<D, T, F: Copy> Copy for FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
}

impl<D, T, F> At<D, T> for FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        (self.fun)(idx)
    }
}
