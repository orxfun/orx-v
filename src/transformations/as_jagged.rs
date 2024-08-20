use crate::{failures::OUT_OF_BOUNDS, IntoIndex, NVec, NVecMut, D1, D2};
use core::{marker::PhantomData, usize};

pub struct VecD1AsJagged<T, V, O>
where
    O: NVec<D1, usize>,
{
    flat: V,
    row_end_indices: O,
    phantom: PhantomData<T>,
}

impl<T, V, O> VecD1AsJagged<T, V, O>
where
    O: NVec<D1, usize>,
{
    fn to_d1_index<Idx: IntoIndex<D2>>(&self, index: Idx) -> usize {
        let [i, j] = index.into_index();
        let begin = match i {
            0 => 0,
            _ => self.row_end_indices.at(i - 1),
        };
        let end = self.row_end_indices.at(i);
        let index = begin + j;
        assert!(index < end, "{}", OUT_OF_BOUNDS);
        index
    }

    fn try_to_d1_index<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<usize> {
        let [i, j] = index.into_index();
        let begin = match i {
            0 => 0,
            _ => self.row_end_indices.try_at(i - 1)?,
        };
        let end = self.row_end_indices.try_at(i)?;
        let index = begin + j;
        match index < end {
            true => Some(index),
            false => None,
        }
    }

    pub fn into_inner(self) -> (V, O) {
        (self.flat, self.row_end_indices)
    }
}

// nvecs

impl<T, V, O> NVec<D2, T> for VecD1AsJagged<T, V, O>
where
    V: NVec<D1, T>,
    O: NVec<D1, usize>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        self.flat.at(self.to_d1_index(index))
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        self.flat.try_at(self.try_to_d1_index(index)?)
    }
}

impl<T, V, O> NVecMut<D2, T> for VecD1AsJagged<T, V, O>
where
    V: NVecMut<D1, T>,
    O: NVec<D1, usize>,
{
    fn set<Idx: IntoIndex<D2>>(&mut self, index: Idx, value: T) {
        self.flat.set(self.to_d1_index(index), value)
    }
}

// into

pub trait AsJagged<T>
where
    Self: Sized,
{
    fn as_jagged<O>(self, row_end_indices: O) -> VecD1AsJagged<T, Self, O>
    where
        O: NVec<D1, usize>;

    fn as_jagged_from_num_columns(
        self,
        num_columns: impl IntoIterator<Item = usize>,
    ) -> VecD1AsJagged<T, Self, Vec<usize>> {
        let row_end_indices: Vec<_> = num_columns
            .into_iter()
            .scan(0, |x, y| {
                *x += y;
                Some(*x)
            })
            .collect();
        self.as_jagged(row_end_indices)
    }
}

impl<T, V> AsJagged<T> for V {
    fn as_jagged<O>(self, row_end_indices: O) -> VecD1AsJagged<T, Self, O>
    where
        O: NVec<D1, usize>,
    {
        VecD1AsJagged {
            flat: self,
            row_end_indices,
            phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::*;

    fn validate_jagged<J: NVec<D2, usize>>(jagged: &J) {
        let row_end_indices = vec![4, 5, 8, 10];

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
    fn jagged() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(&row_end_indices);

        validate_jagged(&jagged);
    }

    #[test]
    fn jagged_from_num_columns() {
        let vec: Vec<usize> = (0..10).collect();
        let num_columns = vec![4, 1, 3, 2];
        let jagged = vec.as_jagged_from_num_columns(num_columns.iter().copied());

        validate_jagged(&jagged);
    }

    #[test]
    #[should_panic]
    fn jagged_out_of_bounds_column() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(&row_end_indices);

        let _ = jagged.at([0, 4]);
    }

    #[test]
    #[should_panic]
    fn jagged_out_of_bounds_row() {
        let vec: Vec<usize> = (0..10).collect();
        let row_end_indices = vec![4, 5, 8, 10];
        let jagged = vec.as_jagged(&row_end_indices);

        let _ = jagged.at([4, 0]);
    }

    #[test]
    fn create_jagged_from_vec() {
        let num_columns = vec![1, 2, 0, 4];
        let num_elements: usize = num_columns.iter().sum();
        let mut buffer = vec![0; num_elements];

        let mut jagged = (&mut buffer).as_jagged_from_num_columns(num_columns.iter().copied());

        for i in 0..num_columns.len() {
            for j in 0..num_columns[i] {
                jagged.set([i, j], i * 100 + j);
            }
        }

        for i in 0..num_columns.len() {
            for j in 0..num_columns[i] {
                assert_eq!(jagged.at([i, j]), i * 100 + j);
            }
        }
    }

    #[test]
    fn create_jagged_from_vec_of_maps() {
        let num_columns = vec![1, 2, 0, 4];
        let mut buffer: BTreeMap<usize, usize> = BTreeMap::new();

        let mut jagged = (&mut buffer)
            .into_completed(1000)
            .as_jagged_from_num_columns(num_columns.iter().copied());

        jagged.set([0, 0], 0);
        jagged.set([1, 0], 10);
        jagged.set([3, 0], 30);

        let (_, row_end_indices) = jagged.into_inner();

        assert_eq!(
            &buffer.iter().map(|(i, x)| (*i, *x)).collect::<Vec<_>>(),
            &[(0, 0), (1, 10), (3, 30)]
        );

        let jagged = (&buffer).into_completed(1000).as_jagged(row_end_indices);

        for i in 0..num_columns.len() {
            for j in 0..num_columns[i] {
                let expected = match j {
                    0 => i * 10,
                    _ => 1000,
                };
                assert_eq!(jagged.at([i, j]), expected);
            }
        }
    }
}
