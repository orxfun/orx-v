use crate::{failures::OUT_OF_BOUNDS, IntoIndex, NVec, D1, D2};
use core::marker::PhantomData;

pub trait MatrixLayout {
    fn to_d1_index(&self, ij: [usize; 2]) -> usize;

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize>;
}

pub struct RowMajor {
    num_columns: usize,
}
impl MatrixLayout for RowMajor {
    #[inline]
    fn to_d1_index(&self, ij: [usize; 2]) -> usize {
        assert!(ij[1] < self.num_columns, "{}", OUT_OF_BOUNDS);
        self.num_columns * ij[0] + ij[1]
    }

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize> {
        match ij[1] < self.num_columns {
            true => Some(self.num_columns * ij[0] + ij[1]),
            false => None,
        }
    }
}

pub struct ColumnMajor {
    num_rows: usize,
}
impl MatrixLayout for ColumnMajor {
    #[inline]
    fn to_d1_index(&self, ij: [usize; 2]) -> usize {
        assert!(ij[0] < self.num_rows, "{}", OUT_OF_BOUNDS);
        self.num_rows * ij[1] + ij[0]
    }

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize> {
        match ij[0] < self.num_rows {
            true => Some(self.num_rows * ij[1] + ij[0]),
            false => None,
        }
    }
}

pub struct VecD1AsMatrix<T, V, L>
where
    V: NVec<D1, T>,
    L: MatrixLayout,
{
    flat: V,
    layout: L,
    phantom: PhantomData<T>,
}

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

// into

pub trait AsMatrix<T>
where
    Self: NVec<D1, T> + Sized,
{
    fn as_row_major_matrix(self, num_columns: usize) -> VecD1AsMatrix<T, Self, RowMajor>;

    fn as_col_major_matrix(self, num_rows: usize) -> VecD1AsMatrix<T, Self, ColumnMajor>;
}

impl<T, V> AsMatrix<T> for V
where
    V: NVec<D1, T>,
{
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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn row_major() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_row_major_matrix(2); // 6 x 2

        let mut expected = 0;

        for i in 0..6 {
            for j in 0..2 {
                assert_eq!(matrix.at([i, j]), &expected);
                assert_eq!(matrix.try_at([i, j]), Some(&expected));

                expected += 1;
            }
            assert_eq!(matrix.try_at([i, 2]), None);
        }

        assert_eq!(matrix.try_at([6, 0]), None);
    }

    #[test]
    #[should_panic]
    fn row_major_oob_row() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_row_major_matrix(2); // 6 x 2
        let _ = matrix.at([6, 0]);
    }

    #[test]
    #[should_panic]
    fn row_major_oob_col() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_row_major_matrix(2); // 6 x 2
        let _ = matrix.at([0, 2]);
    }

    #[test]
    fn col_major() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_col_major_matrix(2); // 2 x 6

        let mut expected = 0;

        for j in 0..6 {
            for i in 0..2 {
                assert_eq!(matrix.at([i, j]), &expected);
                assert_eq!(matrix.try_at([i, j]), Some(&expected));

                expected += 1;
            }
            assert_eq!(matrix.try_at([3, j]), None);
        }

        assert_eq!(matrix.try_at([0, 6]), None);
    }

    #[test]
    #[should_panic]
    fn col_major_oob_row() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_col_major_matrix(2); // 2 x 6
        let _ = matrix.at([2, 0]);
    }

    #[test]
    #[should_panic]
    fn col_major_oob_col() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_col_major_matrix(2); // 2 x 6
        let _ = matrix.at([0, 6]);
    }
}
