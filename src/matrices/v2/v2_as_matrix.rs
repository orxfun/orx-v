use super::{v2_col_major::V2MatrixColMajor, v2_row_major::V2MatrixRowMajor};
use crate::{NVec, NVecMut, D2};

/// Creates matrix views of a rectangular `D2` vector.
pub trait V2AsMatrix<T> {
    /// Converts the rectangular `D2` vector into a row-major matrix.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a row-major matrix:
    /// * it is more efficient to iterate first over i, and then over j,
    /// * [`row(i)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    ///
    /// let mat = v2.v2_into_matrix();
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for row in mat.rows() {
    ///     assert_eq!(row.card([]), 3);
    /// }
    ///
    /// let row = mat.row(2);
    /// assert_eq!(row.equality(&[7, 8, 9]), Equality::Equal); // rows are contagious
    ///
    /// assert_eq!(mat.at([2, 1]), 8);
    ///
    /// assert_eq!(mat.all().count(), 12);
    /// ```
    ///
    /// Notice that any vector of `D2` can be viewed as a matrix.
    /// This allows for efficient representations.
    ///
    /// For instance, zeros or ones matrices do not require any allocation.
    /// Similarly, a unit diagonal matrix can be represented by a function.
    /// A general diagonal matrix can be represented with a `D1` vector and
    /// a function.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(0).with_rectangular_bounds([2, 3]);
    /// let zeros = v2.v2_into_matrix();
    ///
    /// let v2 = V.d2().constant(1).with_rectangular_bounds([2, 3]);
    /// let ones = v2.v2_into_matrix();
    ///
    /// let n = 5;
    ///
    /// let v2 = V
    ///     .d2()
    ///     .fun(|[i, j]| match i == j {
    ///         true => 1,
    ///         false => 0,
    ///     })
    ///     .with_rectangular_bounds([n, n]);
    /// let diagonal = v2.v2_into_matrix();
    ///
    /// for i in 0..diagonal.num_rows() {
    ///     for j in 0..diagonal.num_cols() {
    ///         if i == j {
    ///             assert_eq!(diagonal.at([i, j]), 1);
    ///         } else {
    ///             assert_eq!(diagonal.at([i, j]), 0);
    ///         }
    ///     }
    /// }
    ///
    /// let diagonal_entries = [7, 3, 5, 2, 1];
    /// let v2 = V
    ///     .d2()
    ///     .fun(|[i, j]| match i == j {
    ///         true => diagonal_entries[i],
    ///         false => 0,
    ///     })
    ///     .with_rectangular_bounds([n, n]);
    /// let diagonal = v2.v2_into_matrix();
    ///
    /// for i in 0..diagonal.num_rows() {
    ///     for j in 0..diagonal.num_cols() {
    ///         if i == j {
    ///             assert_eq!(diagonal.at([i, j]), diagonal_entries[i]);
    ///         } else {
    ///             assert_eq!(diagonal.at([i, j]), 0);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// More general approach to take benefit of sparsity is to use sparse vectors as the
    /// underlying storage of the matrix.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let n = 3;
    /// let m = 4;
    ///
    /// let mut v2 = V.d2().sparse(0).with_rectangular_bounds([n, m]);
    /// let mut matrix = v2.v2_as_matrix_mut();
    ///
    /// for row in matrix.rows() {
    ///     assert_eq!(row.equality(&[0, 0, 0, 0]), Equality::Equal);
    /// }
    ///
    /// matrix.set([0, 1], 3);
    /// *matrix.at_mut([2, 1]) = 7;
    ///
    /// assert_eq!(
    ///     matrix.equality(&[[0, 3, 0, 0], [0, 0, 0, 0], [0, 7, 0, 0]].v2_as_matrix()),
    ///     Equality::Equal
    /// );
    /// ```
    fn v2_into_matrix(self) -> V2MatrixRowMajor<T, Self>
    where
        Self: NVec<D2, T>,
    {
        V2MatrixRowMajor::new(self)
    }

    /// Creates a row-major matrix view over the rectangular `D2` vector.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a row-major matrix:
    /// * it is more efficient to iterate first over i, and then over j,
    /// * [`row(i)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    ///
    /// let mat = v2.v2_as_matrix();
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for row in mat.rows() {
    ///     assert_eq!(row.card([]), 3);
    /// }
    ///
    /// let row = mat.row(2);
    /// assert_eq!(row.equality(&[7, 8, 9]), Equality::Equal); // rows are contagious
    ///
    /// assert_eq!(mat.at([2, 1]), 8);
    ///
    /// assert_eq!(mat.all().count(), 12);
    /// ```
    ///
    /// Notice that any vector of `D2` can be viewed as a matrix.
    /// This allows for efficient representations.
    ///
    /// For instance, zeros or ones matrices do not require any allocation.
    /// Similarly, a unit diagonal matrix can be represented by a function.
    /// A general diagonal matrix can be represented with a `D1` vector and
    /// a function.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = V.d2().constant(0).with_rectangular_bounds([2, 3]);
    /// let zeros = v2.v2_as_matrix();
    ///
    /// let v2 = V.d2().constant(1).with_rectangular_bounds([2, 3]);
    /// let ones = v2.v2_as_matrix();
    ///
    /// let n = 5;
    ///
    /// let v2 = V
    ///     .d2()
    ///     .fun(|[i, j]| match i == j {
    ///         true => 1,
    ///         false => 0,
    ///     })
    ///     .with_rectangular_bounds([n, n]);
    /// let diagonal = v2.v2_as_matrix();
    ///
    /// for i in 0..diagonal.num_rows() {
    ///     for j in 0..diagonal.num_cols() {
    ///         if i == j {
    ///             assert_eq!(diagonal.at([i, j]), 1);
    ///         } else {
    ///             assert_eq!(diagonal.at([i, j]), 0);
    ///         }
    ///     }
    /// }
    ///
    /// let diagonal_entries = [7, 3, 5, 2, 1];
    /// let v2 = V
    ///     .d2()
    ///     .fun(|[i, j]| match i == j {
    ///         true => diagonal_entries[i],
    ///         false => 0,
    ///     })
    ///     .with_rectangular_bounds([n, n]);
    /// let diagonal = v2.v2_as_matrix();
    ///
    /// for i in 0..diagonal.num_rows() {
    ///     for j in 0..diagonal.num_cols() {
    ///         if i == j {
    ///             assert_eq!(diagonal.at([i, j]), diagonal_entries[i]);
    ///         } else {
    ///             assert_eq!(diagonal.at([i, j]), 0);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// More general approach to take benefit of sparsity is to use sparse vectors as the
    /// underlying storage of the matrix.
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let n = 3;
    /// let m = 4;
    ///
    /// let mut v2 = V.d2().sparse(0).with_rectangular_bounds([n, m]);
    /// let mut matrix = v2.v2_as_matrix_mut();
    ///
    /// for row in matrix.rows() {
    ///     assert_eq!(row.equality(&[0, 0, 0, 0]), Equality::Equal);
    /// }
    ///
    /// matrix.set([0, 1], 3);
    /// *matrix.at_mut([2, 1]) = 7;
    ///
    /// assert_eq!(
    ///     matrix.equality(&[[0, 3, 0, 0], [0, 0, 0, 0], [0, 7, 0, 0]].v2_as_matrix()),
    ///     Equality::Equal
    /// );
    /// ```
    fn v2_as_matrix(&self) -> V2MatrixRowMajor<T, &Self>
    where
        Self: NVec<D2, T>,
    {
        V2MatrixRowMajor::new(self)
    }

    /// Creates a mutable row-major matrix view over the rectangular `D2` vector.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a row-major matrix:
    /// * it is more efficient to iterate first over i, and then over j,
    /// * [`row(i)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// [`row(i)`]: crate::MatrixRowMajor::row
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v2 = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9],
    ///     vec![10, 11, 12],
    /// ];
    ///
    /// let mut mat = v2.v2_as_matrix_mut();
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// *mat.at_mut([0, 1]) = 22;
    ///
    /// mat.row_mut(1).mut_all(|x| *x *= 10);
    ///
    /// assert_eq!(mat.row(0).equality(&[1, 22, 3]), Equality::Equal);
    /// assert_eq!(mat.row(1).equality(&[40, 50, 60]), Equality::Equal);
    /// ```
    fn v2_as_matrix_mut(&mut self) -> V2MatrixRowMajor<T, &mut Self>
    where
        Self: NVecMut<D2, T>,
    {
        V2MatrixRowMajor::new(self)
    }

    /// Converts the rectangular `D2` vector into a column-major matrix.
    ///
    /// Note that since default `D2` vector layout is row-major, this method
    /// provides a transposed view of the original vector.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a column-major matrix:
    /// * it is more efficient to iterate first over j, and then over i,
    /// * [`col(j)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// [`col(j)`]: crate::MatrixColMajor::col
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]];
    /// let mat = v2.v2_into_matrix_col_major();
    ///
    /// assert_eq!(mat.num_rows(), 2);
    /// assert_eq!(mat.num_cols(), 4);
    ///
    /// assert_eq!(
    ///     mat.equality(&[[0, 2, 4, 6], [1, 3, 5, 7]].v2_as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let col = mat.col(2);
    /// assert_eq!(col.equality(&[4, 5]), Equality::Equal); // columns are contagious
    /// ```
    fn v2_into_matrix_col_major(self) -> V2MatrixColMajor<T, Self>
    where
        Self: NVec<D2, T>,
    {
        V2MatrixColMajor::new(self)
    }

    /// Creates a column-major matrix view over the rectangular `D2` vector.
    ///
    /// Note that since default `D2` vector layout is row-major, this method
    /// provides a transposed view of the original vector.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a column-major matrix:
    /// * it is more efficient to iterate first over j, and then over i,
    /// * [`col(j)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// [`col(j)`]: crate::MatrixColMajor::col
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v2 = vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7]];
    /// let mat = v2.v2_as_matrix_col_major();
    ///
    /// assert_eq!(mat.num_rows(), 2);
    /// assert_eq!(mat.num_cols(), 4);
    ///
    /// assert_eq!(
    ///     mat.equality(&[[0, 2, 4, 6], [1, 3, 5, 7]].v2_as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let col = mat.col(2);
    /// assert_eq!(col.equality(&[4, 5]), Equality::Equal); // columns are contagious
    /// ```
    fn v2_as_matrix_col_major(&self) -> V2MatrixColMajor<T, &Self>
    where
        Self: NVec<D2, T>,
    {
        V2MatrixColMajor::new(self)
    }

    /// Creates a mutable column-major matrix view over the rectangular `D2` vector.
    ///
    /// Note that since default `D2` vector layout is row-major, this method
    /// provides a transposed view of the original vector.
    ///
    /// Say i represents row-index and j represents col-index.
    /// In a column-major matrix:
    /// * it is more efficient to iterate first over j, and then over i,
    /// * [`col(j)`] often (1) returns a vector over a contagious memory location.
    ///
    /// *(1) When the data is represented by a complete allocation; however, recall that
    /// it is possible to use a function or a sparse vector backed up with a lookup as
    /// the underlying vector of the matrix.*
    ///
    /// [`col(j)`]: crate::MatrixColMajor::col
    ///
    /// # Panics
    ///
    /// Panics if the `D2` vector is not rectangular; i.e.,
    /// [`is_rectangular`] is false.
    ///
    /// [`is_rectangular`]: crate::NVec::is_rectangular
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v2 = vec![vec![0, 10], vec![1, 11], vec![2, 12], vec![3, 13]];
    /// let mut mat = v2.v2_as_matrix_col_major_mut();
    ///
    /// assert_eq!(mat.num_rows(), 2);
    /// assert_eq!(mat.num_cols(), 4);
    ///
    /// *mat.at_mut([1, 3]) = 33;
    ///
    /// {
    ///     let mut c1 = mat.col_mut(2);
    ///     c1.set(1, 22);
    ///     assert_eq!(c1.equality(&[2, 22]), Equality::Equal);
    /// }
    ///
    /// assert_eq!(
    ///     mat.equality(&[[0, 1, 2, 3], [10, 11, 22, 33]].v2_as_matrix()),
    ///     Equality::Equal
    /// );
    /// ```
    fn v2_as_matrix_col_major_mut(&mut self) -> V2MatrixColMajor<T, &mut Self>
    where
        Self: NVecMut<D2, T>,
    {
        V2MatrixColMajor::new(self)
    }
}

impl<T, V> V2AsMatrix<T> for V where V: NVec<D2, T> {}
