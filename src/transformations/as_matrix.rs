use super::matrix_layouts::{ColumnMajor, Diagonal, LowerTriangular, RowMajor, UpperTriangular};
use crate::{failures::OUT_OF_BOUNDS, IntoIndex, NVec, NVecMut, D1, D2};
use core::marker::PhantomData;

// layout

pub trait MatrixLayout {
    fn to_d1_index(&self, ij: [usize; 2]) -> usize;

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize>;
}

// matrix view

pub struct VecD1AsMatrix<T, V, L>
where
    L: MatrixLayout,
{
    flat: V,
    layout: L,
    phantom: PhantomData<T>,
}

// nvecs

impl<T, V, L> NVec<D2, T> for VecD1AsMatrix<T, V, L>
where
    V: NVec<D1, T>,
    L: MatrixLayout,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let index = self.layout.to_d1_index(index.into_index());
        self.flat.at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let index = self.layout.try_d1_index(index.into_index())?;
        self.flat.try_at(index)
    }
}

impl<T, V, L> NVecMut<D2, T> for VecD1AsMatrix<T, V, L>
where
    V: NVecMut<D1, T>,
    L: MatrixLayout,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        let index = self
            .layout
            .try_d1_index(index.into_index())
            .expect(OUT_OF_BOUNDS);
        self.flat.set(index, value)
    }
}
// into

// TODO: put a bound on Self, currently it appears on all types!!

pub trait AsMatrix<T>
where
    Self: Sized,
{
    fn as_row_major_matrix(self, num_columns: usize) -> VecD1AsMatrix<T, Self, RowMajor>;

    fn as_col_major_matrix(self, num_rows: usize) -> VecD1AsMatrix<T, Self, ColumnMajor>;

    fn as_upper_triangular_matrix(
        self,
        num_rows_and_columns: usize,
    ) -> VecD1AsMatrix<T, Self, UpperTriangular>;

    fn as_lower_triangular_matrix(
        self,
        num_rows_and_columns: usize,
    ) -> VecD1AsMatrix<T, Self, LowerTriangular>;

    fn as_diagonal_matrix(self, num_rows_and_columns: usize) -> VecD1AsMatrix<T, Self, Diagonal>;
}

impl<T, V> AsMatrix<T> for V {
    fn as_row_major_matrix(self, num_columns: usize) -> VecD1AsMatrix<T, Self, RowMajor> {
        VecD1AsMatrix {
            flat: self,
            layout: RowMajor { num_columns },
            phantom: Default::default(),
        }
    }

    fn as_col_major_matrix(self, num_rows: usize) -> VecD1AsMatrix<T, Self, ColumnMajor> {
        VecD1AsMatrix {
            flat: self,
            layout: ColumnMajor { num_rows },
            phantom: Default::default(),
        }
    }

    fn as_upper_triangular_matrix(
        self,
        num_rows_and_columns: usize,
    ) -> VecD1AsMatrix<T, Self, UpperTriangular> {
        VecD1AsMatrix {
            flat: self,
            layout: UpperTriangular {
                num_rows_and_columns,
            },
            phantom: Default::default(),
        }
    }

    fn as_lower_triangular_matrix(
        self,
        num_rows_and_columns: usize,
    ) -> VecD1AsMatrix<T, Self, LowerTriangular> {
        VecD1AsMatrix {
            flat: self,
            layout: LowerTriangular {
                num_rows_and_columns,
            },
            phantom: Default::default(),
        }
    }

    fn as_diagonal_matrix(self, num_rows_and_columns: usize) -> VecD1AsMatrix<T, Self, Diagonal> {
        VecD1AsMatrix {
            flat: self,
            layout: Diagonal {
                num_rows_and_columns,
            },
            phantom: Default::default(),
        }
    }
}
