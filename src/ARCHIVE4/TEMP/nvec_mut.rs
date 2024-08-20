use crate::{dimensions::*, IntoIndex, NVec};

pub trait NVecMut<N: Dim, T: Copy>: NVec<N, T> {
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T);
}
