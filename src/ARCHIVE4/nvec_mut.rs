use crate::{dimensions::*, IntoIndex};

pub trait NVecMut<N: Dim, T> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> &T;

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<&T>;

    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T);
}
