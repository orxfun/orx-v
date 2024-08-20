use crate::{failures::OUT_OF_BOUNDS, transformations::as_matrix::MatrixLayout};

pub struct UpperTriangular {
    pub(crate) num_rows_and_columns: usize,
}
impl MatrixLayout for UpperTriangular {
    fn to_d1_index(&self, ij: [usize; 2]) -> usize {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        assert!(j < n && i <= j, "{}", OUT_OF_BOUNDS);
        ((2 * n - 1) * i - i * i + 2 * j) / 2
    }

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize> {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        match j < n && i <= j {
            true => Some(((2 * n - 1) * i - i * i + 2 * j) / 2),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::BTreeMap;

    #[test]
    fn upper_triangular() {
        let mut buffer: BTreeMap<usize, u32> = BTreeMap::new();

        let mut upper = (&mut buffer)
            .into_completed(1_000)
            .as_upper_triangular_matrix(4);

        for i in 0..4 {
            for j in i..4 {
                assert_eq!(upper.at([i, j]), 1_000);
            }
        }

        upper.set([2, 3], 23);

        for i in 0..4 {
            for j in i..4 {
                let expected = match (i, j) {
                    (2, 3) => 23,
                    _ => 1000,
                };
                assert_eq!(upper.at([i, j]), expected);
            }
        }

        assert_eq!(
            &buffer.iter().map(|(i, x)| (*i, *x)).collect::<Vec<_>>(),
            &[(8, 23)]
        );
    }
}
