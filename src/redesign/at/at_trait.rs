use super::super::Dim;
use super::Copied;

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    // transform

    fn copied<'a>(self) -> Copied<'a, D, T, Self>
    where
        Self: Sized,
        T: Copy + 'a,
        Self: At<D, &'a T>,
    {
        Copied::new(self)
    }
}
