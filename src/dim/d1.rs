use super::{dimension::Dim, index_sums::IdxLeqD0, IdxLeqD1, IntoIdx};
use crate::NVecCore;
use core::fmt::Debug;

/// An index that can never be created or used to call an element or cardinality of a vector;
/// used as a boundary condition on the recursive dimension types.
#[derive(Clone, Copy, Debug)]
pub enum IdxNever {}
impl From<usize> for IdxNever {
    #[allow(clippy::panic)]
    fn from(_: usize) -> Self {
        panic!("never");
    }
}
impl From<IdxNever> for usize {
    #[allow(clippy::panic)]
    fn from(_: IdxNever) -> Self {
        panic!("never");
    }
}

/// One dimension.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D1;

impl Dim for D1 {
    type Idx = [usize; 1];

    type ChildIdx = IdxNever;

    type PrevDim = Self;

    type CardIdx = IdxLeqD0;

    type LeqIdx = IdxLeqD1;

    fn dimension() -> usize {
        1
    }

    #[inline(always)]
    fn left_join_from_lower_dim(left_most_idx: usize, _: <Self::PrevDim as Dim>::Idx) -> Self::Idx {
        [left_most_idx]
    }

    #[allow(clippy::panic)]
    fn left_join_from_lower_card_idx(
        _: usize,
        _: <Self::PrevDim as Dim>::CardIdx,
    ) -> Self::CardIdx {
        panic!("never")
    }

    #[inline(always)]
    fn in_bounds<T>(idx: impl IntoIdx<Self>, vec: &impl NVecCore<Self, T>) -> bool {
        idx.into_idx()[0] < vec.core_num_children()
    }
}
