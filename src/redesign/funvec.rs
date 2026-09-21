use super::{Dim, NVec};
use core::marker::PhantomData;

pub struct FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fun: F,
    p: PhantomData<(D, T)>,
}

impl<D, T, F> FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    pub fn new(fun: F) -> Self {
        Self {
            fun,
            p: PhantomData,
        }
    }
}

impl<D, T, F> NVec<D, T> for FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    #[inline(always)]
    fn at<'a>(&'a self, idx: <D as Dim>::Idx) -> T
    where
        T: 'a,
    {
        (self.fun)(idx)
    }

    // child
}

// clone & copy

impl<D, T, F> Clone for FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T + Clone,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<D, T, F> Copy for FunVec<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T + Copy,
{
}
