use super::LeqIdx;
use crate::{IdxLeqD1, NVecCore, D1};

impl LeqIdx<D1> for IdxLeqD1 {
    fn in_leq_bounds<T>(self, vec: &impl NVecCore<D1, T>) -> bool {
        match self {
            Self::IdxD0(_) => true,
            Self::IdxD1([i]) => i < vec.core_num_children(),
        }
    }
}
