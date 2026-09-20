use super::{Copied, Dim};

pub trait NVec<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    // transformations

    fn copied<'a, U>(self) -> Copied<'a, D, U, Self>
    where
        U: Copy,
        Self: NVec<D, &'a U> + Sized,
    {
        Copied::new(self)
    }
}
