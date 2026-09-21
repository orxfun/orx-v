use super::{Cloned, Copied, Dim};

pub trait NVec<D: Dim, T> {
    fn at<'r>(&'r self, idx: D::Idx) -> T
    where
        T: 'r;

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

pub trait Vzzz<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    type Child<'a>: Vzzz<D::PrevDim, T>
    where
        Self: 'a;

    fn child(&self, idx: D::ChildIdx) -> Self::Child<'_> {
        unreachable!()
    }
}

pub trait V<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;
}

pub trait Vm<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> &T;

    fn mut_at(&mut self, idx: D::Idx) -> &mut T;
}
