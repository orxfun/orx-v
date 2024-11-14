use super::FunVec;
use crate::{Card, Dim, IntoCached, NVec};
use core::hash::Hash;

impl<D, T, F, C> IntoCached<D, T> for FunVec<D, T, F, C>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
    C: Card<D>,
    T: Copy,
    D::Idx: Ord + Hash,
    Self: NVec<D, T>,
{
}
