use super::super::Dim;

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;
}
