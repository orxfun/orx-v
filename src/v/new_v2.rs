use crate::{
    constant_vec::ConstantVec, empty_vec::EmptyVec, DefaultLookup, Dim, FunVec, Lookup, SparseVec,
    UnboundedCard, D2,
};

/// `V2<T>` (`NVec<D2, T>`) builder.
pub struct NewV2;

impl NewV2 {
    /// Creates a constant vector of dimension `D2` which returns the same value for any input index.
    ///
    /// Since a constant vector assumes all positions of the vector is filled with `value`, the
    /// vector on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the constant vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// [`with_rectangular_bounds`]: `crate::ConstantVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::ConstantVec::with_variable_bounds`
    ///
    /// # Example
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(42);
    ///
    /// assert_eq!(v2.at([2, 0]), 42);
    /// assert_eq!(v2.at([3, 10]), 42);
    /// assert_eq!(v2.try_at([100, 100]), Some(42));
    /// ```
    ///
    /// Add rectangular bounds to the constant vector using `with_rectangular_bounds` transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(42).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(v2.at([0, 2]), 42);
    /// assert_eq!(v2.try_at([0, 2]), Some(42));
    /// assert_eq!(v2.all().sum::<usize>(), 6 * 42);
    /// assert_eq!(
    ///     v2.equality(&[vec![42, 42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal,
    /// );
    /// ```
    ///
    /// `V2` needs not be rectangular and can have variable number of elements for each
    /// row. A 2D sparse vector can be converted into a 2D sparse vec with variable bounds
    /// with `with_variable_bounds` method which takes any `num_cols: V1<usize>` as its
    /// argument where
    /// * `num_cols.card([])` represents the number of rows of the 2D vector, and
    /// * `num_cols.at([i])` returns the number of elements of the i-th row.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().constant(42).with_variable_bounds(&num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(v2.all().sum::<usize>(), 6 * 42);
    /// assert_eq!(
    ///     v2.equality(&[vec![42, 42], vec![42, 42, 42], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 74]), false);
    /// assert_eq!(v2.try_at([100, 74]), None);
    /// assert_eq!(v2.at([100, 74]), 42); // (!) un-compromised performance
    /// ```
    /// ***(!) un-compromised performance***
    ///
    /// *Main reason to add bounds to a sparse vector is to set its domain.
    /// However, calling `at` with an out-of-bounds index can still produce a valid
    /// element in order not to compromise performance.
    /// If we want to check whether or not an index is in bounds or not, we can
    /// use the `in_bounds` or `card` methods, or use `try_at` instead which would
    /// return `None` if the index is out of bounds.*
    pub fn constant<T: Copy>(self, value: T) -> ConstantVec<D2, T, UnboundedCard<D2>> {
        ConstantVec::new(value, UnboundedCard::default())
    }

    /// Creates an empty vector of dimension `D2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().empty::<i32>();
    ///
    /// assert_eq!(v2.card([]), 0);
    /// assert_eq!(v2.in_bounds([0, 0]), false);
    /// assert_eq!(v2.try_at([0, 0]), None);
    /// assert_eq!(v2.all().next(), None);
    /// ```
    pub fn empty<T>(self) -> EmptyVec<D2, T> {
        Default::default()
    }

    /// Creates a sparse vector of dimension `D2` with an initially empty lookup.
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
    /// [`with_rectangular_bounds`]: `crate::SparseVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::SparseVec::with_variable_bounds`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v2 = V.d2().sparse(42);
    ///
    /// assert!(v2.is_unbounded());
    /// assert_eq!(v2.card([]), usize::MAX);
    ///
    /// assert_eq!(v2.at([0, 0]), 42);
    /// assert_eq!(v2.at([175, 3]), 42);
    /// assert_eq!(v2.lookup_len(), 0);
    ///
    /// *v2.at_mut([1, 2]) = 10;
    /// v2.set([0, 1], 7);
    /// assert_eq!(v2.lookup_len(), 2);
    ///
    /// assert_eq!(v2.at([0, 0]), 42);
    /// assert_eq!(v2.at([1, 2]), 10);
    /// assert_eq!(v2.at([0, 1]), 7);
    /// assert_eq!(v2.at([3, 3]), 42);
    /// ```
    ///
    /// Add rectangular bounds to the sparse vector using `with_rectangular_bounds` transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v2 = V.d2().sparse(42).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(v2.at([1, 2]), 42);
    ///
    /// assert_eq!(v2.all().collect::<Vec<_>>(), vec![42, 42, 42, 42, 42, 42]);
    /// assert_eq!(v2.lookup_len(), 0);
    ///
    /// *v2.at_mut([0, 1]) = 10;
    /// v2.set([1, 2], 7);
    ///  assert_eq!(v2.all().collect::<Vec<_>>(), vec![42, 10, 42, 42, 42, 7]);
    /// assert_eq!(v2.lookup_len(), 2);
    /// ```
    ///
    ///  `V2` needs not be rectangular and can have variable number of elements for each
    /// row. A 2D sparse vector can be converted into a 2D sparse vec with variable bounds
    /// with `with_variable_bounds` method which takes any `num_cols: V1<usize>` as its
    /// argument where
    /// * `num_cols.card([])` represents the number of rows of the 2D vector, and
    /// * `num_cols.at([i])` returns the number of elements of the i-th row.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42] ]
    /// let num_cols = vec![2, 3, 1];
    /// let mut v2 = V.d2().sparse(42).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(v2.at([0, 1]), 42);
    /// assert_eq!(v2.at([1, 2]), 42);
    ///
    /// assert_eq!(v2.lookup_len(), 0);
    /// assert_eq!(
    ///     v2.equality(&[vec![42, 42], vec![42, 42, 42], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// *v2.at_mut([0, 1]) = 10;
    /// v2.set([1, 2], 7);
    ///
    /// assert_eq!(v2.lookup_len(), 2);
    /// assert_eq!(
    ///     v2.equality(&[vec![42, 10], vec![42, 42, 7], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 74]), false);
    /// assert_eq!(v2.try_at([100, 74]), None);
    /// assert_eq!(v2.at([100, 74]), 42); // (!) un-compromised performance
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
    ) -> SparseVec<D2, T, UnboundedCard<D2>, DefaultLookup<D2, T>> {
        SparseVec::new(Default::default(), default_value, UnboundedCard::default())
    }

    /// Creates a sparse vector of dimension `D2` with the provided `lookup`.
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
    /// [`with_rectangular_bounds`]: `crate::SparseVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::SparseVec::with_variable_bounds`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // HashMap or BTreeMap or any map implementing Lookup
    /// let map = DefaultLookup::<D2, i32>::from_iter([([0, 3], 10), ([1, 2], 30)].into_iter());
    ///
    /// let mut v2 = V.d2().sparse_from(map, 42).with_rectangular_bounds([20, 40]);
    ///
    /// assert_eq!(v2.at([0, 3]), 10);
    /// assert_eq!(v2.at([1, 2]), 30);
    /// assert_eq!(v2.at([0, 0]), 42);
    /// assert_eq!(v2.at([15, 33]), 42);
    /// assert_eq!(v2.lookup_len(), 2);
    ///
    /// *v2.at_mut([0, 3]) = 33;
    /// v2.set([2, 7], 7);
    ///
    /// assert_eq!(v2.at([0, 3]), 33);
    /// assert_eq!(v2.at([1, 2]), 30);
    /// assert_eq!(v2.at([2, 7]), 7);
    /// assert_eq!(v2.at([15, 33]), 42);
    ///
    /// assert_eq!(v2.lookup_len(), 3);
    ///
    /// let map: DefaultLookup<D2, i32> = v2.into_inner().0;
    /// let mut non_default_elems = map.into_iter().collect::<Vec<_>>();
    /// non_default_elems.sort();
    /// assert_eq!(
    ///     non_default_elems,
    ///     vec![([0, 3], 33), ([1, 2], 30), ([2, 7], 7)]
    /// );
    /// ```
    pub fn sparse_from<T: Copy, L: Lookup<<D2 as Dim>::Idx, T>>(
        self,
        lookup: L,
        default_value: T,
    ) -> SparseVec<D2, T, UnboundedCard<D2>, L> {
        SparseVec::new(lookup, default_value, UnboundedCard::default())
    }

    /// Creates a functional vector of dimension `D2`.
    ///
    /// Since the functional vector is capable of creating an element for any given index, the vector
    /// on construction has [`UnboundedCard`]; i.e., it has a value for any possible index.
    ///
    /// In order to convert the sparse vector into one with a provided bound, you may use the
    /// [`with_rectangular_bounds`] and [`with_variable_bounds`] methods.
    ///
    /// [`with_rectangular_bounds`]: `crate::FunVec::with_rectangular_bounds`
    /// [`with_variable_bounds`]: `crate::FunVec::with_variable_bounds`
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // [ [42, 42, ...], [42, 42, ...], ... ]
    /// let v2 = V.d2().fun(|_| 42);
    ///
    /// assert!(v2.is_unbounded());
    /// assert!(v2.child(4).is_unbounded());
    ///
    /// assert_eq!(v2.at([0, 0]), 42);
    /// assert_eq!(v2.at([175, 187]), 42);
    ///
    /// // [ [0, 1, ...], [100, 101, ...], [200, 201, ...], ... ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64);
    ///
    /// assert_eq!(v2.at([1, 5]), 105);
    /// assert_eq!(v2.at([5, 1]), 501);
    /// ```
    ///
    /// i-th child of a V2 is naturally a V1.
    ///
    /// For functional vectors, the i-th child is analogous to partially applying the
    /// left-most index of elements of the vector to i.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // [ [0, 1, ...], [100, 101, ...], [200, 201, ...], ... ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64);
    ///
    /// // [200, 201, ...]
    /// let v1 = v2.child(2);
    ///
    /// assert_eq!(v1.at([5]), 205);
    ///
    /// for j in 0..20 {
    ///     assert_eq!(v1.at([j]), v2.at([2, j]));
    /// }
    /// ```
    ///
    /// Add rectangular bounds to the functional vector using `with_rectangular_bounds`
    /// transformation.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // 2x3 => [ [0, 1, 2], [100, 101, 102] ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_rectangular_bounds([2, 3]);
    ///
    /// assert!(v2.is_bounded());
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(
    ///     v2.equality(&[[0, 1, 2], [100, 101, 102]]),
    ///     Equality::Equal,
    /// );
    ///
    /// let row1 = v2.child(1);
    /// assert_eq!(row1.card([]), 3);
    ///
    /// assert_eq!(
    ///     row1.equality(&[100, 101, 102]),
    ///     Equality::Equal,
    /// );
    /// ```
    ///
    /// `V2` needs not be rectangular and can have variable number of elements for each
    /// row. A 2D sparse vector can be converted into a 2D sparse vec with variable bounds
    /// with `with_variable_bounds` method which takes any `num_cols: V1<usize>` as its
    /// argument where
    /// * `num_cols.card([])` represents the number of rows of the 2D vector, and
    /// * `num_cols.at([i])` returns the number of elements of the i-th row.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200] ]
    /// let num_cols = [2, 3, 1];
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert!(v2.is_bounded());
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [0], [100, 101], [200], [300, 301] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 1,
    ///     false => 2,
    /// }).bounded(4);
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 1);
    /// assert_eq!(v2.card([1]), 2);
    /// assert_eq!(v2.card([2]), 1);
    /// assert_eq!(v2.card([3]), 2);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0], vec![100, 101], vec![200], vec![300, 301]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 74]), false);
    /// assert_eq!(v2.try_at([100, 74]), None);
    /// assert_eq!(v2.at([100, 74]), 10074); // (!) un-compromised performance
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
    pub fn fun<T, F>(self, at: F) -> FunVec<D2, T, F, UnboundedCard<D2>>
    where
        F: Fn(<D2 as Dim>::Idx) -> T,
    {
        FunVec::new(at, UnboundedCard::default())
    }
}
