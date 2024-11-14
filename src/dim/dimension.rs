use super::{index_card::CardIdx, IntoIdx, LeqIdx};
use crate::NVecCore;
use core::fmt::Debug;

/// Dimensionality of a structure,
/// such as D1 for one-dimensional or D2 for two-dimensional structures.
pub trait Dim: Sized + Copy + Debug + PartialEq + 'static {
    /// Index for this dimensionality.
    type Idx: Copy + Debug;

    /// Index to reach a child of a structure with this dimension,
    /// while the child belongs to the previous dimension.
    type ChildIdx: Copy + Debug + From<usize> + Into<usize>;

    /// One level smaller dimension.
    type PrevDim: Dim;

    /// Union of indices that can be used to query cardinality of the vector's
    /// any lower dimension children.
    ///
    /// Note that these indices belong to lesser dimensions.
    ///
    /// For instance, the cardinality index sum of `NVec<D2, T>` is
    /// `IdxLeqD1` which is an index of dimension `D0` or `D1`:
    /// * the only `D0` index `[]` returns the number of children of the vector;
    /// * the `D1` index `[i]` returns the number of elements in the i-th child
    ///   of the vector.
    type CardIdx: CardIdx<Self> + Debug;

    /// Union of indices that are less than or equal to this dimension.
    type LeqIdx: LeqIdx<Self>;

    /// Name of the dimension.
    fn dimension() -> usize;

    /// Left joins the `left_most_idx` to `lover_idx` of dimension say `N` to
    /// get the index of dimension `N+1`.
    fn left_join_from_lower_dim(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::Idx,
    ) -> Self::Idx;

    /// Left joins the `left_most_idx` to `lover_idx` of dimension say `N` to
    /// get the index of dimension `N+1`.
    fn left_join_from_lower_card_idx(
        left_most_idx: usize,
        lower_idx: <Self::PrevDim as Dim>::CardIdx,
    ) -> Self::CardIdx;

    /// Returns whether or not the `idx` is in bounds for the given `vec`.
    fn in_bounds<T>(idx: impl IntoIdx<Self>, vec: &impl NVecCore<Self, T>) -> bool;
}
