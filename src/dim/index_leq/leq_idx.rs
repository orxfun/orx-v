use crate::{Dim, NVecCore};

/// Union of indices that are less than or equal to dimension `D`.
pub trait LeqIdx<D: Dim>: From<D::Idx> {
    /// Checks whether or not this index is in bounds for the given `vec`.
    fn in_leq_bounds<T>(self, vec: &impl NVecCore<D, T>) -> bool;
}
