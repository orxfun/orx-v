use crate::{failures::OUT_OF_BOUNDS, transformations::as_matrix::MatrixLayout};

pub struct Diagonal {
    pub(crate) num_rows_and_columns: usize,
}
impl MatrixLayout for Diagonal {
    fn to_d1_index(&self, ij: [usize; 2]) -> usize {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        assert!(i < n && i == j, "{}", OUT_OF_BOUNDS);
        i
    }

    fn try_d1_index(&self, ij: [usize; 2]) -> Option<usize> {
        let [i, j] = ij;
        let n = self.num_rows_and_columns;
        match i < n && i == j {
            true => Some(i),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::BTreeMap;

    #[test]
    fn diagonal() {
        let mut buffer: BTreeMap<usize, u32> = BTreeMap::new();

        let mut diagonal = (&mut buffer).as_diagonal_matrix(4);

        for i in 0..4 {
            diagonal.set([i, i], 10 + i as u32);
        }

        for i in 0..4 {
            assert_eq!(diagonal.at([i, i]), 10 + i as u32);
        }

        assert_eq!(
            &buffer.iter().map(|(i, x)| (*i, *x)).collect::<Vec<_>>(),
            &[(0, 10), (1, 11), (2, 12), (3, 13)]
        );
    }
}
