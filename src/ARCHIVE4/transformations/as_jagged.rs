use crate::{failures::OUT_OF_BOUNDS, IntoIndex, NVec, D1, D2};
use core::{marker::PhantomData, usize};

pub struct VecD1AsJagged<T, V, O>
where
    V: NVec<D1, T>,
    O: NVec<D1, usize>,
{
    flat: V,
    row_end_indices: O,
    phantom: PhantomData<T>,
}

impl<T, V, O> NVec<D2, T> for VecD1AsJagged<T, V, O>
where
    V: NVec<D1, T>,
    O: NVec<D1, usize>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let [i, j] = index.into_index();
        let begin = match i {
            0 => 0,
            _ => self.row_end_indices.at(i - 1),
        };
        let end = self.row_end_indices.at(i);
        let index = begin + j;
        assert!(index < end, "{}", OUT_OF_BOUNDS);
        self.flat.at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let [i, j] = index.into_index();
        let begin = match i {
            0 => 0,
            _ => self.row_end_indices.try_at(i - 1)?,
        };
        let end = self.row_end_indices.try_at(i)?;
        let index = begin + j;
        match index < end {
            true => self.flat.try_at(begin + j),
            false => None,
        }
    }
}

// into

pub trait AsJagged<T, O>
where
    Self: NVec<D1, T> + Sized,
    O: NVec<D1, usize>,
{
    fn as_jagged(self, row_end_indices: O) -> VecD1AsJagged<T, Self, O>;
}

impl<T, V, O> AsJagged<T, O> for V
where
    V: NVec<D1, T>,
    O: NVec<D1, usize>,
{
    fn as_jagged(self, row_end_indices: O) -> VecD1AsJagged<T, Self, O> {
        VecD1AsJagged {
            flat: self,
            row_end_indices,
            phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn jagged() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(row_end_indices.copied()).copied();

        let mut expected = 0;

        for i in 0..row_end_indices.len() {
            let begin = match i {
                0 => 0,
                _ => row_end_indices[i - 1],
            };

            let len = row_end_indices[i] - begin;

            for j in 0..len {
                assert_eq!(jagged.at([i, j]), expected);
                assert_eq!(jagged.try_at([i, j]), Some(expected));
                expected += 1;
            }

            assert_eq!(jagged.try_at([i, len]), None);
        }
        assert_eq!(jagged.try_at([row_end_indices.len(), 0]), None);
    }

    #[test]
    #[should_panic]
    fn jagged_out_of_bounds_column() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(row_end_indices.copied()).copied();

        let _ = jagged.at([0, 4]);
    }

    #[test]
    #[should_panic]
    fn jagged_out_of_bounds_row() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(row_end_indices.copied()).copied();

        let _ = jagged.at([4, 0]);
    }
}
