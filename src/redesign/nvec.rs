use super::Dim;

pub trait NVec<D: Dim, T> {
    type ItemRef<'a>
    where
        Self: 'a;

    fn at(&self, idx: impl Into<D::Idx>) -> Self::ItemRef<'_>;
}
