use crate::{failures::OUT_OF_BOUNDS, transformations::as_matrix::MatrixLayout};

pub struct LowerTriangular {
    pub(crate) num_rows_and_columns: usize,
}
impl MatrixLayout for LowerTriangular {
    fn to_d1_index(&self, ij: [usize; 2]) -> usize {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        assert!(i < n && j <= i, "{}", OUT_OF_BOUNDS);
        (i * i + i) / 2 + j
    }

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize> {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        match i < n && j <= i {
            true => Some((i * i + i) / 2 + j),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn lower_triangular() {
        let n = 4;
        let buffer_len = n * (n + 1) / 2;
        let mut buffer = vec![0; buffer_len];

        let mut lower = (&mut buffer).as_lower_triangular_matrix(n);

        for i in 0..n {
            for j in 0..=i {
                lower.set([i, j], i * 1000 + j);
            }
        }

        for i in 0..n {
            for j in 0..=i {
                assert_eq!(lower.at([i, j]), i * 1000 + j);
            }
        }

        assert_eq!(
            &buffer,
            &[0, 1000, 1001, 2000, 2001, 2002, 3000, 3001, 3002, 3003]
        );
    }
}
