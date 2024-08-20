// use crate::{Dim, IntoIndex};

// pub trait NVecRef<N: Dim, T> {
//     fn ref_at<Idx: IntoIndex<N>>(&self, index: Idx) -> &T;
// }

use crate::{Dim, IntoIndex};

pub trait NVecRef<N: Dim> {
    type Element<'e>
    where
        Self: 'e;

    fn ref_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Self::Element<'_>;
}
