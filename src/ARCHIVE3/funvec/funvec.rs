use crate::{dimensions::dim::*, FromIndex};
use core::marker::PhantomData;

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
