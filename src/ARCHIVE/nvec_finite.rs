use crate::{Dim, NVec};

pub trait NVecSized<N: Dim, T>: NVec<N, T> {
    fn cardinality(&self) -> usize;

    fn elements(&self) -> impl Iterator<Item = T>;
}
