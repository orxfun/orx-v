use super::sparse_vec::SparseVec;
use crate::{
    dim::*, CardD1, Lookup, RectangularCardD2, RectangularCardD3, RectangularCardD4, UnboundedCard,
    VariableCardD2, VariableCardD3, VariableCardD4, V1, V2, V3,
};

// D1

impl<T, L> SparseVec<D1, T, UnboundedCard<D1>, L>
where
    T: Copy,
    L: Lookup<<D1 as Dim>::Idx, T>,
{
    /// Converts an unbounded sparse vector into one with a provided bound.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `bounded` method, domain of the vector is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = V.d1().sparse(42);
    /// assert!(v1.is_unbounded());
    ///
    /// let mut v1 = V.d1().sparse(42).bounded(4);
    /// assert!(v1.is_bounded());
    /// assert_eq!(v1.card([]), 4);
    ///
    /// *v1.at_mut(0) = 10;
    /// v1.set(2, 4);
    ///
    /// assert_eq!(
    ///     v1.equality(&[10, 42, 4, 42]),
    ///     Equality::Equal
    /// );
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
    pub fn bounded(self, num_elements: usize) -> SparseVec<D1, T, CardD1, L> {
        self.with_bounds(num_elements.into())
    }
}

// D2

impl<T, L> SparseVec<D2, T, UnboundedCard<D2>, L>
where
    T: Copy,
    L: Lookup<<D2 as Dim>::Idx, T>,
{
    /// Converts an unbounded sparse vector into one with rectangular bounds as in
    /// matrices:
    /// * the vector has `dimensions[0]` children, and
    /// * each children has `dimensions[1]` elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().sparse(42);
    /// assert!(v2.is_unbounded());
    ///
    /// let mut v2 = V.d2().sparse(42).with_rectangular_bounds([2, 3]);
    /// assert!(v2.is_bounded());
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 4);
    ///
    /// assert_eq!(
    ///     v2.equality(&[[10, 42, 42], [42, 42, 4]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 27]), false);
    /// assert_eq!(v2.try_at([100, 27]), None);
    /// assert_eq!(v2.at([100, 27]), 42); // (!) un-compromised performance
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
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 2],
    ) -> SparseVec<D2, T, RectangularCardD2, L> {
        self.with_bounds(dimensions.into())
    }

    /// Converts an unbounded sparse vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.at([i])` elements.
    ///
    /// # Examples
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
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42, 42], [42, 42, 42] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let mut v2 = V.d2().sparse(42).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 27]), false);
    /// assert_eq!(v2.try_at([100, 27]), None);
    /// assert_eq!(v2.at([100, 27]), 42); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> SparseVec<D2, T, VariableCardD2<C>, L>
    where
        C: V1<usize>,
    {
        self.with_bounds(cardinality.into())
    }
}

// D3

impl<T, L> SparseVec<D3, T, UnboundedCard<D3>, L>
where
    T: Copy,
    L: Lookup<<D3 as Dim>::Idx, T>,
{
    /// Converts an unbounded sparse vector into one with rectangular bounds as in
    /// a `dimensions[0]` x `dimensions[1]` x `dimensions[2]` matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v3 = V.d3().sparse(42);
    /// assert!(v3.is_unbounded());
    ///
    /// let mut v3 = V.d3().sparse(42).with_rectangular_bounds([2, 1, 3]);
    /// assert!(v3.is_bounded());
    /// assert_eq!(v3.card([]), 2);
    /// assert_eq!(v3.card([0]), 1);
    /// assert_eq!(v3.card([1]), 1);
    /// assert_eq!(v3.card([0, 0]), 3);
    /// assert_eq!(v3.card([1, 0]), 3);
    ///
    /// *v3.at_mut([0, 0, 0]) = 10;
    /// v3.set([1, 0, 2], 4);
    ///
    /// assert_eq!(
    ///     v3.equality(&[[[10, 42, 42]], [[42, 42, 4]]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v3.in_bounds([100, 27, 75]), false);
    /// assert_eq!(v3.try_at([100, 27, 75]), None);
    /// assert_eq!(v3.at([100, 27, 75]), 42); // (!) un-compromised performance
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
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 3],
    ) -> SparseVec<D3, T, RectangularCardD3, L> {
        self.with_bounds(dimensions.into())
    }

    /// Converts an unbounded sparse vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.card([i])` children, and
    /// * j-th child of the i-th child has `cardinality.at([i, j])` elements.
    ///
    /// # Examples
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
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42, 42], [42, 42, 42] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let mut v2 = V.d2().sparse(42).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 27]), false);
    /// assert_eq!(v2.try_at([100, 27]), None);
    /// assert_eq!(v2.at([100, 27]), 42); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> SparseVec<D3, T, VariableCardD3<C>, L>
    where
        C: V2<usize>,
    {
        self.with_bounds(cardinality.into())
    }
}

// D4

impl<T, L> SparseVec<D4, T, UnboundedCard<D4>, L>
where
    T: Copy,
    L: Lookup<<D4 as Dim>::Idx, T>,
{
    /// Converts an unbounded sparse vector into one with rectangular bounds as in
    /// a `dimensions[0]` x `dimensions[1]` x `dimensions[2]` x `dimensions[3]` matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v3 = V.d3().sparse(42);
    /// assert!(v3.is_unbounded());
    ///
    /// let mut v3 = V.d3().sparse(42).with_rectangular_bounds([2, 1, 3]);
    /// assert!(v3.is_bounded());
    /// assert_eq!(v3.card([]), 2);
    /// assert_eq!(v3.card([0]), 1);
    /// assert_eq!(v3.card([1]), 1);
    /// assert_eq!(v3.card([0, 0]), 3);
    /// assert_eq!(v3.card([1, 0]), 3);
    ///
    /// *v3.at_mut([0, 0, 0]) = 10;
    /// v3.set([1, 0, 2], 4);
    ///
    /// assert_eq!(
    ///     v3.equality(&[[[10, 42, 42]], [[42, 42, 4]]]),
    ///     Equality::Equal
    /// );
    ///
    /// assert_eq!(v3.in_bounds([100, 27, 75]), false);
    /// assert_eq!(v3.try_at([100, 27, 75]), None);
    /// assert_eq!(v3.at([100, 27, 75]), 42); // (!) un-compromised performance
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
    pub fn with_rectangular_bounds(
        self,
        dimensions: [usize; 4],
    ) -> SparseVec<D4, T, RectangularCardD4, L> {
        self.with_bounds(dimensions.into())
    }

    /// Converts an unbounded sparse vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.card([i])` children, and
    /// * j-th child of the `i`-th child has `cardinality.card([i, j])` children, and
    /// * k-th child of the `[i,j]`-th child has `cardinality.at([i, j, k])` elements.
    ///
    /// # Examples
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
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [42, 42], [42, 42, 42], [42, 42], [42, 42, 42] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let mut v2 = V.d2().sparse(42).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// *v2.at_mut([0, 0]) = 10;
    /// v2.set([1, 2], 20);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![10, 42], vec![42, 42, 20], vec![42, 42], vec![42, 42, 42]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 27]), false);
    /// assert_eq!(v2.try_at([100, 27]), None);
    /// assert_eq!(v2.at([100, 27]), 42); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> SparseVec<D4, T, VariableCardD4<C>, L>
    where
        C: V3<usize>,
    {
        self.with_bounds(cardinality.into())
    }
}
