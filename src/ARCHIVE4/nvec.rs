use crate::{dimensions::*, IntoIndex};

pub trait NVec<N: Dim, T> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T>;
}
