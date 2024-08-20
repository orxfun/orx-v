use crate::{dimensions::*, FromIndex};
use core::marker::PhantomData;

#[derive(Copy)]
pub struct FunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    pub(crate) fun: F,
    phantom: PhantomData<(N, T, I)>,
}

impl<N, T, I, F> FunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    pub fn new(fun: F) -> Self {
        Self {
            fun,
            phantom: PhantomData,
        }
    }
}

impl<N, T, I, F> Clone for FunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T + Clone,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            phantom: Default::default(),
        }
    }
}
