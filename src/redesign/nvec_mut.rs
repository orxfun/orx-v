use super::{Dim, NVec};

pub trait NVecMut<D: Dim, T>: NVec<D, T> {
    type ItemMut<'a>
    where
        Self: 'a;

    fn at_mut(&self, idx: impl Into<D::Idx>) -> Self::ItemMut<'_>;
}
