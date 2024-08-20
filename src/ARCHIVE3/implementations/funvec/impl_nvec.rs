use crate::{dimensions::dim::*, FromIndex, FunVec, IntoIndex, NVec};

impl<N, T, I, F> NVec<N, T> for FunVec<N, T, I, F>
where
    N: Dim,
    I: FromIndex<N>,
    F: Fn(I) -> T,
{
    #[inline]
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        (self.fun)(I::from_index(index.into_index()))
    }
}
