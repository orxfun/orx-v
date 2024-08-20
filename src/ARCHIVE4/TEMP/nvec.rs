use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim, T: Copy> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        Some(self.at(index))
    }
}
