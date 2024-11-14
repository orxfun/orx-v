use super::{dimension::Dim, IdxLeqD1, IdxLeqD2, IdxLeqD3, IntoIdx, D2};
use crate::{NVecCore, NVecCoreSealed};
use core::fmt::Debug;

/// Three dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3;

impl Dim for D3 {
    type Idx = [usize; 3];

    type ChildIdx = usize;

    type PrevDim = D2;

    type CardIdx = IdxLeqD2;

    type LeqIdx = IdxLeqD3;

    fn dimension() -> usize {
        3
    }

    #[inline(always)]
    fn left_join_from_lower_dim(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx {
        [left_most_idx, lower_idx[0], lower_idx[1]]
    }

    #[inline(always)]
    fn left_join_from_lower_card_idx(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::CardIdx,
    ) -> Self::CardIdx {
        match lower_idx {
            IdxLeqD1::IdxD0([]) => Self::CardIdx::IdxD1([left_most_idx]),
            IdxLeqD1::IdxD1([j]) => Self::CardIdx::IdxD2([left_most_idx, j]),
        }
    }

    #[inline(always)]
    fn in_bounds<T>(idx: impl IntoIdx<Self>, vec: &impl NVecCore<Self, T>) -> bool {
        let [i, j, k] = idx.into_idx();
        if i < vec.core_num_children() {
            let child = vec.core_child(i);
            if j < child.core_num_children() {
                let child = child.core_child(j);
                if k < child.core_num_children() {
                    return true;
                }
            }
        }
        false
    }
}
