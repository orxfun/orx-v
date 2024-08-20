use super::FunVec;
use crate::{dimensions::*, FromIndex};

pub trait IntoFunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    fn to_funvec(self) -> FunVec<N, T, I, F>;
}

impl<N, T, I, F> IntoFunVec<N, T, I, F> for F
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    fn to_funvec(self) -> FunVec<N, T, I, F> {
        FunVec::new(self)
    }
}
