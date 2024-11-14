use super::DefaultLookup;
use crate::common_trait_helpers::debug::*;
use crate::{Card, Dim, IntoIdx, LeqIdx, Lookup, NVec, D1, D2, D3, D4};
use core::fmt::Debug;
use core::marker::PhantomData;

/// A sparse vector of dimension `D`.
///
/// Sparse vectors maintain a (idx, value) lookup under the hood and has a `default_value`, and
/// works as follows:
/// * `at(idx)` returns the corresponding value if the idx exists in the lookup, or the default
///   value otherwise.
/// * `at_mut(idx)` first adds `(idx, default_value)` to the lookup only if it is absent, and
///   returns a mutable reference to the value in the lookup.
///
/// The objective of sparse vectors are to significantly reduce the memory requirement of vectors
/// which has the same value for most of its positions. Consider for instance a 100x100 matrix
/// which is all zeros except for the element at the (42,42)-th position which is 42. This matrix
/// can be represented by a sparse vector with lookup containing only one element.
///
/// Since sparse vector assumes all indices absent in the lookup have the `default_value`, the
/// vector on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
///
/// In order to convert the sparse vector into one with a provided bound, you may use the `bounded`,
/// `with_rectangular_bounds` or `with_variable_bounds` transformations.
///
/// [`UnboundedCard`]: crate::UnboundedCard
pub struct SparseVec<D, T, C, L = DefaultLookup<D, T>>
where
    D: Dim,
    T: Copy,
    L: Lookup<D::Idx, T>,
    C: Card<D>,
{
    pub(super) lookup: L,
    pub(super) default_value: T,
    pub(super) card: C,
    pub(super) phantom: PhantomData<D>,
}

macro_rules! impl_debug {
    ($dim:ty, $dbg_fn:ident) => {
        impl<T, C, L> Debug for SparseVec<$dim, T, C, L>
        where
            T: Copy + Debug,
            L: Lookup<<$dim as Dim>::Idx, T>,
            C: Card<$dim>,
            Self: NVec<$dim, T>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f,
                    "{{ kind: SparseVec, dim: D{}, is_bounded: {}, default_value: {:?}, lookup_len: {}, values: ",
                    <$dim as Dim>::dimension(),
                    self.is_bounded(),
                    self.default_value,
                    self.lookup.len(),
                )?;
                $dbg_fn(f, self)?;
                write!(f, " }}")
            }
        }
    };
}

impl_debug!(D1, dbg_values_d1);
impl_debug!(D2, dbg_values_d2);
impl_debug!(D3, dbg_values_d3);
impl_debug!(D4, dbg_values_d4);

impl<D, T, L, C> SparseVec<D, T, C, L>
where
    D: Dim,
    T: Copy,
    L: Lookup<D::Idx, T>,
    C: Card<D>,
{
    /// Creates a new sparse vector with the given `card` where all elements that
    /// are absent in the `lookup` are equal to `value`.
    ///
    /// The lookup can later be extended by using `at_mut` or `set` methods.
    ///
    /// Alternatively, unbounded sparse vectors of different dimensions can be
    /// created by `V.d1().sparse(42)`, `V.d2().sparse(42)`, etc., which can
    /// later be transformed into bounded vectors by applying `bounded`,
    /// `with_rectangular_bounds` or `with_variable_bounds` transformations.
    ///
    /// Similarly, an unbounded sparse vector can be created by a non-empty lookup
    /// by `V.d1().sparse_from(lookup, 42)`, `V.d2().sparse_from(lookup, 42)`, etc.
    pub fn new(lookup: L, default_value: T, card: C) -> Self {
        Self {
            lookup,
            default_value,
            card,
            phantom: PhantomData,
        }
    }

    /// Destructs the sparse vector into its inner lookup and cardinality.
    pub fn into_inner(self) -> (L, C) {
        (self.lookup, self.card)
    }

    /// Returns the number of non-default elements which are actually stored in the
    /// lookup.
    pub fn lookup_len(&self) -> usize {
        self.lookup.len()
    }

    // helpers

    pub(crate) fn with_bounds<C2>(self, card: C2) -> SparseVec<D, T, C2, L>
    where
        C2: Card<D>,
    {
        SparseVec {
            lookup: self.lookup,
            default_value: self.default_value,
            card,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub(super) fn sparse_at(&self, idx: impl IntoIdx<D>) -> T {
        match self.lookup.get(&idx.into_idx()) {
            Some(x) => *x,
            None => self.default_value,
        }
    }

    #[inline(always)]
    pub(super) fn sparse_cardinality(&self) -> &C {
        &self.card
    }

    #[inline(always)]
    pub(super) fn sparse_num_children(&self) -> usize {
        self.card.cardinality_of([])
    }

    #[inline(always)]
    pub(super) fn sparse_card(&self, idx: impl Into<D::CardIdx>) -> usize {
        self.card.cardinality_of(idx)
    }

    #[inline(always)]
    pub(super) fn sparse_in_bounds(&self, idx: impl Into<D::LeqIdx>) -> bool
    where
        Self: NVec<D, T>,
    {
        let idx = idx.into();
        <D::LeqIdx as LeqIdx<D>>::in_leq_bounds(idx, self)
    }

    #[inline(always)]
    pub(super) fn sparse_at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T {
        self.lookup
            .entry_or_insert(idx.into_idx(), self.default_value)
    }

    #[inline(always)]
    pub(super) fn sparse_set<Idx: IntoIdx<D>>(&mut self, idx: Idx, value: T) {
        self.lookup.insert(idx.into_idx(), value);
    }

    pub(super) fn sparse_mut_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
        D: Fill<D>,
    {
        D::fill(self);
        for x in self.lookup.values_mut() {
            f(x);
        }
    }

    pub(super) fn sparse_reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
        D: Fill<D>,
    {
        match self.default_value == value {
            true => self.lookup.clear(),
            false => self.sparse_mut_all(|x| *x = value),
        }
    }
}

// fill

pub(super) trait Fill<D: Dim> {
    fn fill<T, L, C>(sparse_vec: &mut SparseVec<D, T, C, L>)
    where
        T: Copy,
        L: Lookup<D::Idx, T>,
        C: Card<D>;
}
impl Fill<D1> for D1 {
    fn fill<T, L, C>(sparse_vec: &mut SparseVec<D1, T, C, L>)
    where
        T: Copy,
        L: Lookup<<D1 as Dim>::Idx, T>,
        C: Card<D1>,
    {
        for i in 0..sparse_vec.card.cardinality_of([]) {
            _ = sparse_vec
                .lookup
                .entry_or_insert([i], sparse_vec.default_value);
        }
    }
}
impl Fill<D2> for D2 {
    fn fill<T, L, C>(sparse_vec: &mut SparseVec<D2, T, C, L>)
    where
        T: Copy,
        L: Lookup<<D2 as Dim>::Idx, T>,
        C: Card<D2>,
    {
        for i in 0..sparse_vec.card.cardinality_of([]) {
            for j in 0..sparse_vec.card.cardinality_of([i]) {
                _ = sparse_vec
                    .lookup
                    .entry_or_insert([i, j], sparse_vec.default_value);
            }
        }
    }
}
impl Fill<D3> for D3 {
    fn fill<T, L, C>(sparse_vec: &mut SparseVec<D3, T, C, L>)
    where
        T: Copy,
        L: Lookup<<D3 as Dim>::Idx, T>,
        C: Card<D3>,
    {
        for i in 0..sparse_vec.card.cardinality_of([]) {
            for j in 0..sparse_vec.card.cardinality_of([i]) {
                for k in 0..sparse_vec.card.cardinality_of([i, j]) {
                    _ = sparse_vec
                        .lookup
                        .entry_or_insert([i, j, k], sparse_vec.default_value)
                }
            }
        }
    }
}
impl Fill<D4> for D4 {
    fn fill<T, L, C>(sparse_vec: &mut SparseVec<D4, T, C, L>)
    where
        T: Copy,
        L: Lookup<<D4 as Dim>::Idx, T>,
        C: Card<D4>,
    {
        for i in 0..sparse_vec.card.cardinality_of([]) {
            for j in 0..sparse_vec.card.cardinality_of([i]) {
                for k in 0..sparse_vec.card.cardinality_of([i, j]) {
                    for l in 0..sparse_vec.card.cardinality_of([i, j, k]) {
                        _ = sparse_vec
                            .lookup
                            .entry_or_insert([i, j, k, l], sparse_vec.default_value)
                    }
                }
            }
        }
    }
}
