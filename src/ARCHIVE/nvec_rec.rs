use crate::{Dim, IntoIndex, NVec, D1};

pub trait NVecRec<N: Dim, T>: NVec<N, T> {
    type Child: NVec<<N as Dim>::PREVIOUS, T>;

    fn num_children(&self) -> usize;

    fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child>;

    fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
    where
        Self::Child: 'c;
}
