use super::{Matrix, MatrixMut};
use crate::{NVec, NVecMut, D1};

/// A row major matrix.
///
/// Say i represents row-index and j represents col-index.
/// In a row-major matrix:
/// * it is more efficient to iterate first over i, and then over j,
/// * [`row(i)`] often (1) returns a vector over a contagious memory location.
///
/// *(1) When the data is represented by a complete allocation; however, recall that
/// it is possible to use a function or a sparse vector backed up with a lookup as
/// the underlying vector of the matrix.*
pub trait MatrixRowMajor<T>: Matrix<T> {
    /// Returns the `i`-th row of the matrix which is a `D1` vector.
    fn row(&self, i: usize) -> impl NVec<D1, T>;

    /// Returns an iterator over the rows of the matrix.
    fn rows(&self) -> impl Iterator<Item = impl NVec<D1, T>> {
        (0..self.num_rows()).map(|i| self.row(i))
    }
}

impl<T, M: MatrixRowMajor<T>> MatrixRowMajor<T> for &M {
    fn row(&self, j: usize) -> impl NVec<D1, T> {
        <M as MatrixRowMajor<T>>::row(self, j)
    }
}

impl<T, M: MatrixRowMajor<T>> MatrixRowMajor<T> for &mut M {
    fn row(&self, j: usize) -> impl NVec<D1, T> {
        <M as MatrixRowMajor<T>>::row(self, j)
    }
}

// mut

/// A mutable row major matrix.
pub trait MatrixRowMajorMut<T>: MatrixRowMajor<T> + MatrixMut<T> {
    /// Returns a mutable reference to the `i`-th row of the matrix which is a `D1` vector.
    fn row_mut(&mut self, i: usize) -> impl NVecMut<D1, T>;
}

impl<T, M: MatrixRowMajorMut<T>> MatrixRowMajorMut<T> for &mut M {
    fn row_mut(&mut self, j: usize) -> impl NVecMut<D1, T> {
        <M as MatrixRowMajorMut<T>>::row_mut(self, j)
    }
}
