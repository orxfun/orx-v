use crate::{Dim, IntoIndex, NVecMut, NVecRec, D1};

pub trait NVecRecMut<N: Dim, T>:
    NVecMut<N, T> + NVecRec<N, T, Child: NVecMut<<N as Dim>::PREVIOUS, T>>
{
    fn child_mut<Idx1: IntoIndex<D1>>(&mut self, index: Idx1) -> Option<&mut Self::Child>;

    fn children_mut<'c>(&'c mut self) -> impl Iterator<Item = &mut Self::Child>
    where
        Self::Child: 'c;
}
