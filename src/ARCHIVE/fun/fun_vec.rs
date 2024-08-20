use crate::{dimensions::dim::*, FromIndex};
use core::marker::PhantomData;

pub struct FunVec<D, T, I, F>
where
    D: Dim,
    I: FromIndex<D>,
    F: Fn(I) -> Option<T>,
{
    pub(crate) fun: F,
    phantom: PhantomData<(D, T, I)>,
}

impl<D, T, I, F> FunVec<D, T, I, F>
where
    D: Dim,
    I: FromIndex<D>,
    F: Fn(I) -> Option<T>,
{
    pub fn new(fun: F) -> Self {
        Self {
            fun,
            phantom: PhantomData,
        }
    }
}
