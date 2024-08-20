use super::FunVec;
use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVec};

impl<D, T, I, F> NVec<D> for FunVec<D, T, I, F>
where
    D: Dim,
    I: FromIndex<D>,
    F: Fn(I) -> T,
{
    type Element<'e> = T where Self: 'e;

    #[inline]
    fn at<'e, Idx: IntoIndex<D>>(&'e self, index: Idx) -> Self::Element<'e>
    where
        Self: 'e,
    {
        (self.fun)(I::from_index(index.into_index()))
    }
}
