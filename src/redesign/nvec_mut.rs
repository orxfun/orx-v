use super::{Dim, NVec, NVecRef};

pub trait NVecMut<D: Dim, T>: NVecRef<D, T> {
    fn at_mut(&mut self, idx: D::Idx) -> &mut T;
}
