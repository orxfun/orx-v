use super::{Cloned, Copied, Dim};

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

    fn cloned<'a, U>(self) -> Cloned<'a, D, U, Self>
    where
        U: Clone,
        Self: NVec<D, &'a U> + Sized,
    {
        Cloned::new(self)
    }
}
