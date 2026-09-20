use super::{Dim, NVec};

pub trait NVecMut<D: Dim, T>
where
    for<'a> &'a Self: NVec<D, &'a T>,
{
    fn at_mut(&mut self, idx: D::Idx) -> &mut T;
}
