use super::{CardEquality, Equality};
use crate::{Dim, NVec, NVecCore};

/// Union of indices that can be used to query cardinality of any lower level dimension
/// of a vector with dimension `D`.
pub trait CardIdx<D: Dim>: PartialEq + Sized + From<[usize; 0]> {
    /// Returns whether or not the index is of dimension 0; i.e., has type `[usize; 0]`.
    fn is_d0(&self) -> bool;

    /// Returns the cardinality of the given `vec` at this index.
    fn card<T>(self, vec: &impl NVecCore<D, T>) -> usize;

    /// Returns the cardinality equality of the given vectors `a` and `b` at this index.
    fn card_equality<T>(a: &impl NVecCore<D, T>, b: &impl NVecCore<D, T>) -> CardEquality<D>;

    /// Returns the equality of the given vectors `a` and `b` at this index.
    fn equality<T: PartialEq>(a: &impl NVec<D, T>, b: &impl NVec<D, T>) -> Equality<D>;
}
