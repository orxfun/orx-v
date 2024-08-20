use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim> {
    type Element<'e>
    where
        Self: 'e;

    fn at<'e, Idx: IntoIndex<N>>(&'e self, index: Idx) -> Self::Element<'e>
    where
        Self: 'e;
}
