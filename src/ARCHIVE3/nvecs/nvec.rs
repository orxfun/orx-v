use crate::{Dim, IntoIndex};

pub trait NVec<N: Dim, T> {
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T;
}
