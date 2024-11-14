use crate::{
    constant_vec::ConstantVec, empty_vec::EmptyVec, DefaultLookup, Dim, FunVec, Lookup, SparseVec,
    UnboundedCard, D1,
};

/// `V1<T>` (`NVec<D1, T>`) builder.
pub struct NewV1;

impl NewV1 {
    /// Creates a constant vector of dimension `D1` which returns the same value for any input index.
    ///
    /// Since a constant vector assumes all positions of the vector is filled with `value`, the
    /// vector on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the sparse vector into one with a provided bound, you may use the [`bounded`]
    /// method.
    ///
    /// [`bounded`]: `crate::ConstantVec::bounded`
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().constant(42);
    ///
    /// assert_eq!(v1.at([2]), 42);
    /// assert_eq!(v1.at([10]), 42);
    /// assert_eq!(v1.try_at([100]), Some(42));
    /// ```
    ///
    /// Add bounds to the constant vector using `bounded` transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().constant(42).bounded(4);
    ///
    /// assert_eq!(v1.card([]), 4);
    /// assert_eq!(v1.at([2]), 42);
    /// assert_eq!(v1.try_at([2]), Some(42));
    /// assert_eq!(v1.all().sum::<usize>(), 4 * 42);
    /// assert_eq!(
    ///     v1.equality(&[42, 42, 42, 42]),
    ///     Equality::Equal
    /// );
    ///
    ///
    /// assert_eq!(v1.in_bounds([100]), false);
    /// assert_eq!(v1.try_at([100]), None);
    /// assert_eq!(v1.at([100]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a sparse vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn constant<T: Copy>(self, value: T) -> ConstantVec<D1, T, UnboundedCard<D1>> {
        ConstantVec::new(value, UnboundedCard::default())
    }

    /// Creates an empty vector of dimension `D1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().empty::<i32>();
    ///
    /// assert_eq!(v1.card([]), 0);
    /// assert_eq!(v1.in_bounds([0]), false);
    /// assert_eq!(v1.try_at([0]), None);
    /// assert_eq!(v1.all().next(), None);
    /// ```
    pub fn empty<T>(self) -> EmptyVec<D1, T> {
        Default::default()
    }

    /// Creates a sparse vector of dimension `D1` with an initially empty lookup.
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
    /// In order to convert the sparse vector into one with a provided bound, you may use the [`bounded`]
    /// method.
    ///
    /// [`bounded`]: `crate::SparseVec::bounded`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = V.d1().sparse(42);
    ///
    /// assert!(v1.is_unbounded());
    /// assert_eq!(v1.card([]), usize::MAX);
    /// assert_eq!(v1.at([0]), 42);
    /// assert_eq!(v1.at([175]), 42);
    /// assert_eq!(v1.all_in(0..5).collect::<Vec<_>>(), vec![42, 42, 42, 42, 42]);
    /// assert_eq!(v1.lookup_len(), 0);
    ///
    /// *v1.at_mut(3) = 10;
    /// v1.set(1, 7);
    /// assert_eq!(v1.all_in(0..5).collect::<Vec<_>>(), vec![42, 7, 42, 10, 42]);
    /// assert_eq!(v1.lookup_len(), 2);
    /// ```
    ///
    /// Add bounds to the sparse vector using `bounded` transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = V.d1().sparse(42).bounded(4);
    ///
    /// assert_eq!(v1.card([]), 4);
    /// assert_eq!(v1.at([0]), 42);
    /// assert_eq!(v1.at([2]), 42);
    /// assert_eq!(v1.try_at([2]), Some(42));
    /// assert_eq!(v1.all().collect::<Vec<_>>(), vec![42, 42, 42, 42]);
    ///
    /// *v1.at_mut(3) = 10;
    /// v1.set(1, 7);
    /// assert_eq!(v1.all().collect::<Vec<_>>(), vec![42, 7, 42, 10]);
    ///
    /// assert_eq!(v1.in_bounds([100]), false);
    /// assert_eq!(v1.try_at([100]), None);
    /// assert_eq!(v1.at([100]), 42); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a sparse vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn sparse<T: Copy>(
        self,
        default_value: T,
    ) -> SparseVec<D1, T, UnboundedCard<D1>, DefaultLookup<D1, T>> {
        SparseVec::new(Default::default(), default_value, UnboundedCard::default())
    }

    /// Creates a sparse vector of dimension `D1` with the provided `lookup`.
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
    /// In order to convert the sparse vector into one with a provided bound, you may use the [`bounded`]
    /// method.
    ///
    /// [`bounded`]: `crate::SparseVec::bounded`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // HashMap (default with std) or BTreeMap (default when no-std) or any map implementing Lookup
    /// let map = DefaultLookup::<D1, i32>::from_iter([([0], 10), ([2], 30)].into_iter());
    /// let mut v1 = V.d1().sparse_from(map, 42).bounded(5);
    ///
    /// assert_eq!(v1.card([]), 5);
    /// assert_eq!(v1.lookup_len(), 2); // level of sparsity
    /// assert_eq!(v1.all().collect::<Vec<_>>(), vec![10, 42, 30, 42, 42]);
    ///
    /// *v1.at_mut(2) += 3;
    /// v1.set(1, 7);
    ///
    /// assert_eq!(v1.lookup_len(), 3); // level of sparsity
    /// assert_eq!(v1.all().take(5).collect::<Vec<_>>(), vec![10, 7, 33, 42, 42]);
    ///
    /// let map: DefaultLookup<D1, i32> = v1.into_inner().0;
    /// let mut non_default_elems = map.into_iter().collect::<Vec<_>>();
    /// non_default_elems.sort();
    /// assert_eq!(non_default_elems, vec![([0], 10), ([1], 7), ([2], 33)]);
    /// ```
    pub fn sparse_from<T: Copy, L: Lookup<<D1 as Dim>::Idx, T>>(
        self,
        lookup: L,
        default_value: T,
    ) -> SparseVec<D1, T, UnboundedCard<D1>, L> {
        SparseVec::new(lookup, default_value, UnboundedCard::default())
    }

    /// Creates a functional vector of dimension `D1`.
    ///
    /// Since the functional vector is capable of creating an element for any given index, the vector
    /// on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the sparse vector into one with a provided bound, you may use the [`bounded`]
    /// method.
    ///
    /// [`bounded`]: `crate::FunVec::bounded`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    /// use std::collections::HashMap;
    ///
    /// // [42, 43, 44, ...]
    /// let v1 = V.d1().fun(|[i]| 42 + i);
    ///
    /// assert_eq!(v1.at([0]), 42);
    /// assert_eq!(v1.at([175]), 42 + 175);
    ///
    /// // [-3, 43, 44, -12, 46, 47, ...]
    /// let special_values: HashMap<usize, i64> = [(0, -3), (3, -12)].into_iter().collect();
    /// let v1 = V.d1().fun(|[i]| special_values.get(&i).copied().unwrap_or_else(|| 42 + i as i64));
    ///
    /// assert_eq!(v1.at([0]), -3);
    /// assert_eq!(v1.at([3]), -12);
    /// assert_eq!(v1.at([175]), 42 + 175);
    /// ```
    ///
    /// Add bounds to the functional vector using `bounded` transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // [42, 43, 44, 45 ]
    /// let v1 = V.d1().fun(|[i]| 42 + i).bounded(4);
    ///
    /// assert_eq!(v1.card([]), 4);
    /// assert_eq!(v1.at([0]), 42);
    /// assert_eq!(v1.at([2]), 42 + 2);
    ///
    /// assert_eq!(v1.in_bounds([175]), false);
    /// assert_eq!(v1.try_at([175]), None);
    /// assert_eq!(v1.at([175]), 42 + 175); // (!) un-compromised performance
    /// ```
    ///
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a sparse vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn fun<T, F>(self, at: F) -> FunVec<D1, T, F, UnboundedCard<D1>>
    where
        F: Fn(<D1 as Dim>::Idx) -> T,
    {
        FunVec::new(at, UnboundedCard::default())
    }
}
