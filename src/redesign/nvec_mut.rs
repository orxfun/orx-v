use super::{Cloned, Copied, Dim};

pub trait NVecMut<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn at_mut(&mut self, idx: D::Idx) -> &mut T;
}
