use crate::{dimensions::dim::*, FromIndex, FunVec, IntoIndex, NVec};

impl<D, T, I, F> NVec<D, T> for FunVec<D, T, I, F>
where
    D: Dim,
    I: FromIndex<D>,
    F: Fn(I) -> Option<T>,
{
    fn try_at<Idx: IntoIndex<D>>(&self, index: Idx) -> Option<T> {
        (self.fun)(I::from_index(index.into_index()))
    }
}
