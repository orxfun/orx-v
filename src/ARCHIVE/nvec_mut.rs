use crate::{Dim, IntoIndex, NVec, ValueSetError};

pub trait NVecMut<N: Dim, T>: NVec<N, T> {
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) -> Result<(), ValueSetError>;
}
