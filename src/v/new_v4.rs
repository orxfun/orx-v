use crate::{
    constant_vec::ConstantVec, empty_vec::EmptyVec, DefaultLookup, Dim, FunVec, Lookup, SparseVec,
    UnboundedCard, D4,
};

/// `V4<T>` (`NVec<D4, T>`) builder.
pub struct NewV4;

impl NewV4 {
    /// Creates a constant vector of dimension `D4` which returns the same value for any input index.
    ///
    /// Since a constant vector assumes all positions of the vector is filled with `value`, the
    /// vector on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the constant vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// See [`V.d2().constant`] for examples.
    ///
    /// [`with_rectangular_bounds`]: `crate::ConstantVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::ConstantVec::with_variable_bounds`
    /// [`V.d2().constant`]: `crate::v::NewV2::constant`
    pub fn constant<T: Copy>(self, value: T) -> ConstantVec<D4, T, UnboundedCard<D4>> {
        ConstantVec::new(value, UnboundedCard::default())
    }

    /// Creates an empty vector of dimension `D4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v4 = V.d4().empty::<i32>();
    ///
    /// assert_eq!(v4.card([]), 0);
    /// assert_eq!(v4.in_bounds([0, 0, 0, 0]), false);
    /// assert_eq!(v4.try_at([0, 0, 0, 0]), None);
    /// assert_eq!(v4.all().next(), None);
    /// ```
    pub fn empty<T>(self) -> EmptyVec<D4, T> {
        Default::default()
    }

    /// Creates a sparse vector of dimension `D4` with an initially empty lookup.
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
    /// In order to convert the sparse vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// See [`V.d2().sparse`] for examples.
    ///
    /// [`V.d2().sparse`]: `crate::v::NewV2::sparse`
    /// [`with_rectangular_bounds`]: `crate::SparseVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::SparseVec::with_variable_bounds`
    pub fn sparse<T: Copy>(
        self,
        default_value: T,
    ) -> SparseVec<D4, T, UnboundedCard<D4>, DefaultLookup<D4, T>> {
        SparseVec::new(Default::default(), default_value, UnboundedCard::default())
    }

    /// Creates a sparse vector of dimension `D4` with the provided `lookup`.
    ///
    /// Sparse vectors maintain a (idx, value) lookup under the hood and has a `default_value`, and
    /// works as follows:
    /// * `at(idx)` returns the corresponding value if the idx exists in the lookup, or the default
    ///   value otherwise.
    /// * `at_mut(idx)` first adds `(idx, default_value)` to the lookup only if it is absent, and
    ///   returns a mutable reference to the value in the lookup.
    ///
    /// There might be alternative choices of the lookup type. It is required that the collection
    /// implements the [`Lookup`] trait. The std collection `HashMap` and no-std collection
    /// `BTreeMap` already implement this trait and can be readily be usd in sparse vectors.
    ///
    /// The objective of sparse vectors are to significantly reduce the memory requirement of vectors
    /// which has the same value for most of its positions. Consider for instance a 100x100 matrix
    /// which is all zeros except for the element at the (42,42)-th position which is 42. This matrix
    /// can be represented by a sparse vector with lookup containing only one element.
    ///
    /// Since sparse vector assumes all indices absent in the lookup have the `default_value`, the
    /// vector on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the sparse vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// See [`V.d2().sparse_from`] for examples.
    ///
    /// [`V.d2().sparse_from`]: `crate::v::NewV2::sparse_from`
    /// [`with_rectangular_bounds`]: `crate::SparseVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::SparseVec::with_variable_bounds`
    pub fn sparse_from<T: Copy, L: Lookup<<D4 as Dim>::Idx, T>>(
        self,
        lookup: L,
        default_value: T,
    ) -> SparseVec<D4, T, UnboundedCard<D4>, L> {
        SparseVec::new(lookup, default_value, UnboundedCard::default())
    }

    /// Creates a functional vector of dimension `D4`.
    ///
    /// Since the functional vector is capable of creating an element for any given index, the vector
    /// on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the sparse vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// See [`V.d2().fun`] for examples.
    ///
    /// [`V.d2().fun`]: `crate::v::NewV2::fun`
    /// [`with_rectangular_bounds`]: `crate::FunVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::FunVec::with_variable_bounds`
    pub fn fun<T, F>(self, at: F) -> FunVec<D4, T, F, UnboundedCard<D4>>
    where
        F: Fn(<D4 as Dim>::Idx) -> T,
    {
        FunVec::new(at, UnboundedCard::default())
    }
}
