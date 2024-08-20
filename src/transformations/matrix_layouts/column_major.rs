use crate::{failures::OUT_OF_BOUNDS, transformations::as_matrix::MatrixLayout};

pub struct ColumnMajor {
    pub(crate) num_rows: usize,
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_col_major_matrix() {
        let n = 2;
        let m = 3;
        let mut buffer = vec![0; n * m];
        let mut matrix = (&mut buffer).as_col_major_matrix(n);

        matrix.set([0, 0], 0);
        matrix.set([0, 1], 1);
        matrix.set([0, 2], 2);
        matrix.set([1, 0], 10);
        matrix.set([1, 1], 11);
        matrix.set([1, 2], 12);

        assert_eq!(&buffer, &[0, 10, 1, 11, 2, 12]);
    }

    #[test]
    #[should_panic]
    fn col_major_oob_col() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_col_major_matrix(2); // 2 x 6
        let _ = matrix.at([0, 6]);
    }

    #[test]
    #[should_panic]
    fn col_major_oob_row() {
        let vec: Vec<usize> = (0..12).collect();
        let matrix = vec.as_col_major_matrix(2); // 2 x 6
        let _ = matrix.at([2, 0]);
    }

    #[test]
    fn col_major() {
        let mut vec: Vec<usize> = (0..12).collect();
        let mut matrix = (&mut vec).as_col_major_matrix(2); // 2 x 6

        let mut expected = 0;

        for j in 0..6 {
            for i in 0..2 {
                assert_eq!(matrix.at([i, j]), expected);
                assert_eq!(matrix.try_at([i, j]), Some(expected));

                expected += 1;
            }
            assert_eq!(matrix.try_at([3, j]), None);
        }

        assert_eq!(matrix.try_at([0, 6]), None);

        matrix.set([1, 0], 42);
        assert_eq!(matrix.at([1, 0]), 42);
        assert_eq!(matrix.try_at([1, 0]), Some(42));
        assert_eq!(vec[1], 42);
    }
}
