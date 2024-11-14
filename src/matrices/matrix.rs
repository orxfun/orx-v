use crate::{Dim, Equality, IdxLeqD1, IdxLeqD2, IntoIdx, D2};

/// A matrix or a matrix view over a `D2` vector with rectangular cardinality,
/// or over a flattened representation by a `D1` vector.
///
/// An owned matrix can be created by:
/// * calling [`v2_into_matrix`] method on a `D2`
///   vector implementing `NVec<D2, _>`, or equivalently, `V2<_>`; or by:
/// * calling [`v1_into_matrix`] method on a `D1`
///   vector implementing `NVec<D1, _>`, or equivalently, `V1<_>`.
///
/// Alternatively, matrix views can be created by:
/// * calling [`v2_as_matrix`] or [`v2_as_matrix_col_major`] methods on a `D2`
///   vector; or by:
/// * calling [`v1_as_matrix`] or [`v1_as_matrix_col_major`] methods on a `D1`
///   vector.
///
/// [`v2_into_matrix`]: crate::V2AsMatrix::v2_into_matrix
/// [`v2_as_matrix`]: crate::V2AsMatrix::v2_as_matrix
/// [`v2_as_matrix_col_major`]: crate::V2AsMatrix::v2_as_matrix_col_major
/// [`v1_into_matrix`]: crate::V1AsMatrix::v1_into_matrix
/// [`v1_as_matrix`]: crate::V1AsMatrix::v1_as_matrix
/// [`v1_as_matrix_col_major`]: crate::V1AsMatrix::v1_as_matrix_col_major
///
/// All above mentioned methods have their `_mut` versions to create a
/// mutable matrix view.
pub trait Matrix<T> {
    /// Number of rows.
    fn num_rows(&self) -> usize;

    /// Number of columns.
    fn num_cols(&self) -> usize;

    /// Returns the element at the given `idx` of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the `idx` is not `in_bounds`.
    fn at(&self, idx: impl IntoIdx<D2>) -> T;

    /// Returns an iterator of all elements of the matrix.
    /// The direction of iteration depends on whether the matrix is row-major
    /// or column-major.
    ///
    /// Row-major matrices are created by:
    /// * calling [`v2_as_matrix`] on a `D2` vector, or
    /// * calling [`v1_as_matrix`] on a `D1` vector.
    ///
    /// Column-major matrices are created by:
    /// * calling [`v2_as_matrix_col_major`] on a `D2` vector, or
    /// * calling [`v1_as_matrix_col_major`] on a `D1` vector.
    ///
    /// [`v2_as_matrix`]: crate::V2AsMatrix::v2_as_matrix
    /// [`v2_as_matrix_col_major`]: crate::V2AsMatrix::v2_as_matrix_col_major
    /// [`v1_as_matrix`]: crate::V1AsMatrix::v1_as_matrix
    /// [`v1_as_matrix_col_major`]: crate::V1AsMatrix::v1_as_matrix_col_major
    ///
    /// All above mentioned methods have their `_mut` versions to create a
    /// mutable matrix view.
    fn all(&self) -> impl Iterator<Item = T>;

    // provided

    /// Returns true if the given `idx` is in bounds of the matrix.
    #[inline(always)]
    fn in_bounds(&self, idx: impl Into<<D2 as Dim>::LeqIdx>) -> bool {
        match idx.into() {
            IdxLeqD2::IdxD0([]) => true,
            IdxLeqD2::IdxD1([i]) => i < self.num_rows(),
            IdxLeqD2::IdxD2([i, j]) => i < self.num_rows() && j < self.num_cols(),
        }
    }

    /// Returns the element at the given `idx` if it is `in_bounds`;
    /// returns None otherwise.
    fn try_at(&self, idx: impl IntoIdx<D2>) -> Option<T> {
        let [i, j] = idx.into_idx();
        match i < self.num_rows() && j < self.num_cols() {
            true => Some(self.at(idx)),
            false => None,
        }
    }

    /// Returns the equality result of comparing this matrix to the `other`.
    fn equality(&self, other: &impl Matrix<T>) -> Equality<D2>
    where
        T: PartialEq,
    {
        if self.num_rows() != other.num_rows() {
            Equality::UnequalCard(IdxLeqD1::IdxD0([]), self.num_rows(), other.num_rows())
        } else if self.num_cols() != other.num_cols() {
            Equality::UnequalCard(IdxLeqD1::IdxD1([0]), self.num_rows(), other.num_rows())
        } else {
            for i in 0..self.num_rows() {
                for j in 0..self.num_cols() {
                    if self.at([i, j]) != other.at([i, j]) {
                        return Equality::UnequalValue([i, j]);
                    }
                }
            }
            Equality::Equal
        }
    }
}

// &V auto impl

impl<T, M: Matrix<T>> Matrix<T> for &M {
    fn num_rows(&self) -> usize {
        <M as Matrix<T>>::num_rows(self)
    }

    fn num_cols(&self) -> usize {
        <M as Matrix<T>>::num_cols(self)
    }

    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        <M as Matrix<T>>::at(self, idx)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        <M as Matrix<T>>::all(self)
    }
}

// &mut V auto impl

impl<T, M: Matrix<T>> Matrix<T> for &mut M {
    fn num_rows(&self) -> usize {
        <M as Matrix<T>>::num_rows(self)
    }

    fn num_cols(&self) -> usize {
        <M as Matrix<T>>::num_cols(self)
    }

    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        <M as Matrix<T>>::at(self, idx)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        <M as Matrix<T>>::all(self)
    }
}
