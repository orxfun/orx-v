use super::LeqIdx;
use crate::{IdxLeqD2, NVecCore, NVecCoreSealed, D2};

impl LeqIdx<D2> for IdxLeqD2 {
    fn in_leq_bounds<T>(self, vec: &impl NVecCore<D2, T>) -> bool {
        match self {
            Self::IdxD0(_) => true,
            Self::IdxD1([i]) => i < vec.core_num_children(),
            Self::IdxD2([i, j]) => {
                i < vec.core_num_children() && j < vec.core_child(i).core_num_children()
            }
        }
    }
}
