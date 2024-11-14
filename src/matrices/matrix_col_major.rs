use super::{Matrix, MatrixMut};
use crate::{NVec, NVecMut, D1};

/// A column major matrix.
///
/// Say i represents row-index and j represents col-index.
/// In a column-major matrix:
/// * it is more efficient to iterate first over j, and then over i,
/// * [`col(j)`] often (1) returns a vector over a contagious memory location.
///
/// *(1) When the data is represented by a complete allocation; however, recall that
/// it is possible to use a function or a sparse vector backed up with a lookup as
/// the underlying vector of the matrix.*
pub trait MatrixColMajor<T>: Matrix<T> {
    /// Returns the `j`-th column of the matrix which is a `D1` vector.
    fn col(&self, j: usize) -> impl NVec<D1, T>;

    /// Returns an iterator over the columns of the matrix.
    fn cols(&self) -> impl Iterator<Item = impl NVec<D1, T>> {
        (0..self.num_cols()).map(|j| self.col(j))
    }
}

impl<T, M: MatrixColMajor<T>> MatrixColMajor<T> for &M {
    fn col(&self, j: usize) -> impl NVec<D1, T> {
        <M as MatrixColMajor<T>>::col(self, j)
    }
}

impl<T, M: MatrixColMajor<T>> MatrixColMajor<T> for &mut M {
    fn col(&self, j: usize) -> impl NVec<D1, T> {
        <M as MatrixColMajor<T>>::col(self, j)
    }
}

// mut

/// A mutable column major matrix.
pub trait MatrixColMajorMut<T>: MatrixColMajor<T> + MatrixMut<T> {
    /// Returns a mutable reference to the `j`-th column of the matrix which is a `D1` vector.
    fn col_mut(&mut self, j: usize) -> impl NVecMut<D1, T>;
}

impl<T, M: MatrixColMajorMut<T>> MatrixColMajorMut<T> for &mut M {
    fn col_mut(&mut self, j: usize) -> impl NVecMut<D1, T> {
        <M as MatrixColMajorMut<T>>::col_mut(self, j)
    }
}
