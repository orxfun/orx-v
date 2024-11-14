use super::LeqIdx;
use crate::{IdxLeqD4, NVecCore, NVecCoreSealed, D4};

impl LeqIdx<D4> for IdxLeqD4 {
    fn in_leq_bounds<T>(self, vec: &impl NVecCore<D4, T>) -> bool {
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
            Self::IdxD4([i, j, k, l]) => {
                i < vec.core_num_children()
                    && j < vec.core_child(i).core_num_children()
                    && k < vec.core_child(i).core_child(j).core_num_children()
                    && l < vec
                        .core_child(i)
                        .core_child(j)
                        .core_child(k)
                        .core_num_children()
            }
        }
    }
}
