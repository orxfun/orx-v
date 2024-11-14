use super::{dimension::Dim, IdxLeqD2, IdxLeqD3, IdxLeqD4, IntoIdx, D3};
use crate::{NVecCore, NVecCoreSealed};
use core::fmt::Debug;

/// Four dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D4;

impl Dim for D4 {
    type Idx = [usize; 4];

    type ChildIdx = usize;

    type PrevDim = D3;

    type CardIdx = IdxLeqD3;

    type LeqIdx = IdxLeqD4;

    fn dimension() -> usize {
        4
    }

    #[inline(always)]
    fn left_join_from_lower_dim(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        [left_most_idx, lower_idx[0], lower_idx[1], lower_idx[2]]
    }

    #[inline(always)]
    fn left_join_from_lower_card_idx(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::CardIdx,
    ) -> Self::CardIdx {
        match lower_idx {
            IdxLeqD2::IdxD0([]) => Self::CardIdx::IdxD1([left_most_idx]),
            IdxLeqD2::IdxD1([j]) => Self::CardIdx::IdxD2([left_most_idx, j]),
            IdxLeqD2::IdxD2([j, k]) => Self::CardIdx::IdxD3([left_most_idx, j, k]),
        }
    }

    #[inline(always)]
    fn in_bounds<T>(idx: impl IntoIdx<Self>, vec: &impl NVecCore<Self, T>) -> bool {
        let [i, j, k, l] = idx.into_idx();
        if i < vec.core_num_children() {
            let child = vec.core_child(i);
            if j < child.core_num_children() {
                let child = child.core_child(j);
                if k < child.core_num_children() {
                    let child = child.core_child(k);
                    if l < child.core_num_children() {
                        return true;
                    }
                }
            }
        }
        false
    }
}
