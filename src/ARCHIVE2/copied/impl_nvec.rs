use super::copied::Copied;
use crate::{dimensions::dim::*, IntoIndex, NVec};

impl<'v, N, V, C, F> NVec<N> for Copied<'v, N, V, C, F>
where
    N: Dim,
    V: NVec<N>,
    F: Fn(V::Element<'v>) -> C,
{
    type Element<'e> = C where Self: 'e;

    #[inline]
    fn at<'e, Idx: IntoIndex<N>>(&'e self, index: Idx) -> Self::Element<'e>
    where
        Self: 'e,
    {
        (self.copy)(self.inner.at(index))
    }
}
