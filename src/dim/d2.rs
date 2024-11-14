use super::{dimension::Dim, IdxLeqD0, IdxLeqD1, IdxLeqD2, IntoIdx, D1};
use crate::{NVecCore, NVecCoreSealed};
use core::fmt::Debug;

/// Two dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2;

impl Dim for D2 {
    type Idx = [usize; 2];

    type ChildIdx = usize;

    type PrevDim = D1;

    type CardIdx = IdxLeqD1;

    type LeqIdx = IdxLeqD2;

    fn dimension() -> usize {
        2
    }

    #[inline(always)]
    fn left_join_from_lower_dim(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        [left_most_idx, lower_idx[0]]
    }

    #[inline(always)]
    fn left_join_from_lower_card_idx(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::CardIdx,
    ) -> Self::CardIdx {
        match lower_idx {
            IdxLeqD0::IdxD0([]) => Self::CardIdx::IdxD1([left_most_idx]),
        }
    }

    #[inline(always)]
    fn in_bounds<T>(idx: impl IntoIdx<Self>, vec: &impl NVecCore<Self, T>) -> bool {
        let [i, j] = idx.into_idx();
        i < vec.core_num_children() && j < vec.core_child(i).core_num_children()
    }
}
