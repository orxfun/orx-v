use super::{
    layout::{V1LayoutColMajor, V1LayoutRowMajor},
    v1_matrix::V1Matrix,
};
use crate::{NVec, NVecMut, D1};

/// Creates matrix views of a flat `D1` vector.
pub trait V1AsMatrix<T> {
    /// Converts the flat `D1` vector into a row-major matrix.
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
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mat = v1.v1_into_matrix(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for row in mat.rows() {
    ///     assert_eq!(row.card([]), 3);
    /// }
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let row = mat.row(2);
    /// assert_eq!(row.equality(&[7, 8, 9]), Equality::Equal); // rows are contagious
    ///
    /// assert_eq!(mat.at([2, 1]), 8);
    ///
    /// assert_eq!(mat.all().count(), 12);
    /// ```
    fn v1_into_matrix(self, num_rows: usize, num_cols: usize) -> V1Matrix<T, Self, V1LayoutRowMajor>
    where
        Self: NVec<D1, T>,
    {
        V1Matrix::new(V1LayoutRowMajor::new(num_rows, num_cols), self)
    }

    /// Creates a row-major matrix view over the flat `D1` vector.
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
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mat = v1.v1_as_matrix(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for row in mat.rows() {
    ///     assert_eq!(row.card([]), 3);
    /// }
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let row = mat.row(2);
    /// assert_eq!(row.equality(&[7, 8, 9]), Equality::Equal); // rows are contagious
    ///
    /// assert_eq!(mat.at([2, 1]), 8);
    ///
    /// assert_eq!(mat.all().count(), 12);
    /// ```
    fn v1_as_matrix(&self, num_rows: usize, num_cols: usize) -> V1Matrix<T, &Self, V1LayoutRowMajor>
    where
        Self: NVec<D1, T>,
    {
        V1Matrix::new(V1LayoutRowMajor::new(num_rows, num_cols), self)
    }

    /// Creates a mutable row-major matrix view over the flat `D1` vector.
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
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mut mat = v1.v1_as_matrix_mut(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// *mat.at_mut([0, 1]) = 22;
    ///
    /// mat.row_mut(1).mut_all(|x| *x *= 10);
    ///
    /// assert_eq!(mat.row(0).equality(&[1, 22, 3]), Equality::Equal);
    /// assert_eq!(mat.row(1).equality(&[40, 50, 60]), Equality::Equal);
    /// ```
    fn v1_as_matrix_mut(
        &mut self,
        num_rows: usize,
        num_cols: usize,
    ) -> V1Matrix<T, &mut Self, V1LayoutRowMajor>
    where
        Self: NVecMut<D1, T>,
    {
        V1Matrix::new(V1LayoutRowMajor::new(num_rows, num_cols), self)
    }

    /// Converts the flat `D1` vector into a column-major matrix.
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
    /// # Panics
    ///
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mat = v1.v1_into_matrix_col_major(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for col in mat.cols() {
    ///     assert_eq!(col.card([]), 4);
    /// }
    ///
    /// assert_eq!(mat.at([2, 1]), 7);
    ///
    /// assert_eq!(mat.all().count(), 12);
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let col = mat.col(1);
    /// assert_eq!(col.equality(&[5, 6, 7, 8]), Equality::Equal); // columns are contagious
    /// ```
    fn v1_into_matrix_col_major(
        self,
        num_rows: usize,
        num_cols: usize,
    ) -> V1Matrix<T, Self, V1LayoutColMajor>
    where
        Self: NVec<D1, T>,
    {
        V1Matrix::new(V1LayoutColMajor::new(num_rows, num_cols), self)
    }

    /// Creates a column-major matrix view over the flat `D1` vector.
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
    /// # Panics
    ///
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mat = v1.v1_into_matrix_col_major(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// for col in mat.cols() {
    ///     assert_eq!(col.card([]), 4);
    /// }
    ///
    /// assert_eq!(mat.at([2, 1]), 7);
    ///
    /// assert_eq!(mat.all().count(), 12);
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 5, 9], [2, 6, 10], [3, 7, 11], [4, 8, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let col = mat.col(1);
    /// assert_eq!(col.equality(&[5, 6, 7, 8]), Equality::Equal); // columns are contagious
    /// ```
    fn v1_as_matrix_col_major(
        &self,
        num_rows: usize,
        num_cols: usize,
    ) -> V1Matrix<T, &Self, V1LayoutColMajor>
    where
        Self: NVec<D1, T>,
    {
        V1Matrix::new(V1LayoutColMajor::new(num_rows, num_cols), self)
    }

    /// Creates a mutable column-major matrix view over the flat `D1` vector.
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
    /// # Panics
    ///
    /// Panics if cardinality of the `D1` vector is not equal to
    /// `num_rows * num_cols`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_v::*;
    ///
    /// let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    ///
    /// let mut mat = v1.v1_as_matrix_col_major_mut(4, 3);
    ///
    /// assert_eq!(mat.num_rows(), 4);
    /// assert_eq!(mat.num_cols(), 3);
    ///
    /// *mat.at_mut([0, 2]) = 42;
    ///
    /// mat.col_mut(1).mut_all(|x| *x += 10); // columns are contagious
    ///
    /// assert_eq!(
    ///     mat.equality(&[[1, 15, 42], [2, 16, 10], [3, 17, 11], [4, 18, 12]].as_matrix()),
    ///     Equality::Equal
    /// );
    ///
    /// let col = mat.col(1);
    /// assert_eq!(col.equality(&[15, 16, 17, 18]), Equality::Equal); // columns are contagious
    /// ```
    fn v1_as_matrix_col_major_mut(
        &mut self,
        num_rows: usize,
        num_cols: usize,
    ) -> V1Matrix<T, &mut Self, V1LayoutColMajor>
    where
        Self: NVecMut<D1, T>,
    {
        V1Matrix::new(V1LayoutColMajor::new(num_rows, num_cols), self)
    }
}

impl<T, V> V1AsMatrix<T> for V where V: NVec<D1, T> {}
