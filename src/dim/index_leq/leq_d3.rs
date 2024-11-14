use super::LeqIdx;
use crate::{IdxLeqD3, NVecCore, NVecCoreSealed, D3};

impl LeqIdx<D3> for IdxLeqD3 {
    fn in_leq_bounds<T>(self, vec: &impl NVecCore<D3, T>) -> bool {
        match self {
            Self::IdxD0(_) => true,
            Self::IdxD1([i]) => i < vec.core_num_children(),
            Self::IdxD2([i, j]) => {
                i < vec.core_num_children() && j < vec.core_child(i).core_num_children()
            }
            Self::IdxD3([i, j, k]) => {
                i < vec.core_num_children()
                    && j < vec.core_child(i).core_num_children()
                    && k < vec.core_child(i).core_child(j).core_num_children()
            }
        }
    }
}
