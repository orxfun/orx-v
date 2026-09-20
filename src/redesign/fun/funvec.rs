use super::super::{Dim, NVec};
use core::marker::PhantomData;

pub struct FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fun: F,
    p: PhantomData<(D, T)>,
}

impl<D, T, F> NVec<D, T> for FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    #[inline(always)]
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        (self.fun)(idx)
    }
}
