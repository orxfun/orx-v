use super::FunVec;
use crate::{
    dim::*, CardD1, RectangularCardD2, RectangularCardD3, RectangularCardD4, UnboundedCard,
    VariableCardD2, VariableCardD3, VariableCardD4, V1, V2, V3,
};

// D1

impl<T, F> FunVec<D1, T, F, UnboundedCard<D1>>
where
    F: Fn(<D1 as Dim>::Idx) -> T,
{
    /// Converts an unbounded functional vector into one with a provided bound.
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
    /// // [0, 10, 20, 30]
    /// let v1 = V.d1().fun(|[i]| 10 * i as i32).bounded(4);
    ///
    /// assert_eq!(v1.card([]), 4);
    /// assert_eq!(v1.all().collect::<Vec<_>>(), vec![0, 10, 20, 30]);
    /// assert_eq!(v1.equality(&[0, 10, 20, 30]), Equality::Equal);
    /// assert_eq!(v1.all().sum::<i32>(), 60);
    ///
    /// assert_eq!(v1.in_bounds([7]), false);
    /// assert_eq!(v1.try_at([7]), None);
    /// assert_eq!(v1.at([7]), 70); // (!) un-compromised performance
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
    pub fn bounded(self, num_elements: usize) -> FunVec<D1, T, F, CardD1> {
        FunVec::new(self.fun, num_elements.into())
    }
}

// D2

impl<T, F> FunVec<D2, T, F, UnboundedCard<D2>>
where
    F: Fn(<D2 as Dim>::Idx) -> T,
{
    /// Converts an unbounded functional vector into one with rectangular bounds as in
    /// matrices:
    /// * the vector has `dimensions[0]` children, and
    /// * each children has `dimensions[1]` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // 2x3 => [ [0, 1, 2], [100, 101, 102] ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(v2.all().collect::<Vec<_>>(), vec![0, 1, 2, 100, 101, 102]);
    ///
    /// let row1 = v2.child(1);
    /// assert_eq!(row1.card([]), 3);
    /// assert_eq!(row1.at([0]), 100);
    /// assert_eq!(row1.all().collect::<Vec<_>>(), vec![100, 101, 102]);
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    ) -> FunVec<D2, T, F, RectangularCardD2> {
        FunVec::new(self.fun, dimensions.into())
    }

    /// Converts an unbounded functional vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.at([i])` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200, 201], [300, 301, 302] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200, 201], vec![300, 301, 302]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> FunVec<D2, T, F, VariableCardD2<C>>
    where
        C: V1<usize>,
    {
        FunVec::new(self.fun, cardinality.into())
    }
}

// D3

impl<T, F> FunVec<D3, T, F, UnboundedCard<D3>>
where
    F: Fn(<D3 as Dim>::Idx) -> T,
{
    /// Converts an unbounded functional vector into one with rectangular bounds as in
    /// multi-dimensional matrices. The matrix has
    /// `dimensions[0]` x `dimensions[1]` x `dimensions[2]` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // 2x3 => [ [0, 1, 2], [100, 101, 102] ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(v2.all().collect::<Vec<_>>(), vec![0, 1, 2, 100, 101, 102]);
    ///
    /// let row1 = v2.child(1);
    /// assert_eq!(row1.card([]), 3);
    /// assert_eq!(row1.at([0]), 100);
    /// assert_eq!(row1.all().collect::<Vec<_>>(), vec![100, 101, 102]);
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    ) -> FunVec<D3, T, F, RectangularCardD3> {
        FunVec::new(self.fun, dimensions.into())
    }

    /// Converts an unbounded functional vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.card([i])` children, and
    /// * j-th child of the i-th child has `cardinality.at([i, j])` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200, 201], [300, 301, 302] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200, 201], vec![300, 301, 302]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> FunVec<D3, T, F, VariableCardD3<C>>
    where
        C: V2<usize>,
    {
        FunVec::new(self.fun, cardinality.into())
    }
}

// D4

impl<T, F> FunVec<D4, T, F, UnboundedCard<D4>>
where
    F: Fn(<D4 as Dim>::Idx) -> T,
{
    /// Converts an unbounded functional vector into one with rectangular bounds as in
    /// multi-dimensional matrices. The matrix has
    /// `dimensions[0]` x `dimensions[1]` x `dimensions[2]` x `dimensions[3]` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // 2x3 => [ [0, 1, 2], [100, 101, 102] ]
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_rectangular_bounds([2, 3]);
    ///
    /// assert_eq!(v2.card([]), 2);
    /// assert_eq!(v2.card([0]), 3);
    /// assert_eq!(v2.card([1]), 3);
    ///
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(v2.all().collect::<Vec<_>>(), vec![0, 1, 2, 100, 101, 102]);
    ///
    /// let row1 = v2.child(1);
    /// assert_eq!(row1.card([]), 3);
    /// assert_eq!(row1.at([0]), 100);
    /// assert_eq!(row1.all().collect::<Vec<_>>(), vec![100, 101, 102]);
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    ) -> FunVec<D4, T, F, RectangularCardD4> {
        FunVec::new(self.fun, dimensions.into())
    }

    /// Converts an unbounded functional vector into one with variable bounds as in
    /// jagged arrays:
    /// * the vector has `cardinality.card([])` children, and
    /// * i-th child has `cardinality.card([i])` children, and
    /// * j-th child of the `i`-th child has `cardinality.card([i, j])` children, and
    /// * k-th child of the `[i,j]`-th child has `cardinality.at([i, j, k])` elements.
    ///
    /// Note that in practice, unbounded cardinality corresponds to a length of usize::MAX.
    /// One needs to be careful in using the `all` method which would iterate over the
    /// entire 0..usize::MAX range unless it is stopped on a condition such as in `find`
    /// method or limited by methods such as `take`.
    ///
    /// With `with_rectangular_bounds` or `with_variable_bounds` methods, domain of the vector
    /// is defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200] ]
    /// let num_cols = vec![2, 3, 1];
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 3);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 1);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200]]),
    ///     Equality::Equal,
    /// );
    ///
    /// // jagged => [ [0, 1], [100, 101, 102], [200, 201], [300, 301, 302] ]
    /// let num_cols = V.d1().fun(|[i]| match i % 2 == 0 {
    ///     true => 2,
    ///     false => 3,
    /// }).bounded(4);
    /// let v2 = V.d2().fun(|[i, j]| (100 * i + j) as i64).with_variable_bounds(num_cols);
    ///
    /// assert_eq!(v2.card([]), 4);
    /// assert_eq!(v2.card([0]), 2);
    /// assert_eq!(v2.card([1]), 3);
    /// assert_eq!(v2.card([2]), 2);
    /// assert_eq!(v2.card([3]), 3);
    ///
    /// assert_eq!(v2.at([0, 1]), 1);
    /// assert_eq!(v2.at([1, 2]), 102);
    ///
    /// assert_eq!(
    ///     v2.equality(&[vec![0, 1], vec![100, 101, 102], vec![200, 201], vec![300, 301, 302]]),
    ///     Equality::Equal,
    /// );
    ///
    /// assert_eq!(v2.in_bounds([100, 72]), false);
    /// assert_eq!(v2.try_at([100, 72]), None);
    /// assert_eq!(v2.at([100, 72]), 100 * 100 + 72); // (!) un-compromised performance
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
    pub fn with_variable_bounds<C>(self, cardinality: C) -> FunVec<D4, T, F, VariableCardD4<C>>
    where
        C: V3<usize>,
    {
        FunVec::new(self.fun, cardinality.into())
    }
}
