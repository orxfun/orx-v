use super::FunVec;
use crate::{dimensions::dim::*, FromIndex, NVec};

pub struct Copied<'f, N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> &'f T,
{
    inner: &'f FunVec<N, &'f T, I, F>,
}

impl<'f, N, T, I, F> NVec<N, T> for Copied<'f, N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> &'f T,
    T: Copy,
{
    fn at<Idx: crate::IntoIndex<N>>(&self, index: Idx) -> T {
        // let x = self.inner.at(index);
        *self.inner.at(index)
    }
}

impl<'f, N, T, I, F> FunVec<N, &'f T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> &'f T,
    T: Copy,
{
    pub fn copied2(&'f self) -> Copied<'f, N, T, I, F> {
        Copied { inner: self }
    }
}
