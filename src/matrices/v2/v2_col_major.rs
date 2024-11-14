use super::super::{matrix::Matrix, MatrixColMajor, MatrixColMajorMut};
use crate::{matrices::MatrixMut, IntoIdx, NVec, NVecMut, D1, D2};
use core::marker::PhantomData;

pub struct V2MatrixColMajor<T, V>
where
    V: NVec<D2, T>,
{
    data: V,
    phantom: PhantomData<T>,
}

impl<T, V> V2MatrixColMajor<T, V>
where
    V: NVec<D2, T>,
{
    pub(super) fn new(data: V) -> Self {
        assert!(
            data.is_rectangular(),
            "D2 vector to be converted to Matrix does not have rectangular cardinality."
        );
        Self {
            data,
            phantom: PhantomData,
        }
    }
}

// matrix

impl<T, V> Matrix<T> for V2MatrixColMajor<T, V>
where
    V: NVec<D2, T>,
{
    #[inline(always)]
    fn num_rows(&self) -> usize {
        match self.num_cols() {
            0 => 0,
            _ => self.data.card([0]),
        }
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {
        self.data.card([])
    }

    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        let [i, j] = idx.into_idx();
        self.data.at([j, i])
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.data.all()
    }
}

impl<T, V> MatrixMut<T> for V2MatrixColMajor<T, V>
where
    V: NVecMut<D2, T>,
{
    #[inline(always)]
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        let [i, j] = idx.into_idx();
        self.data.at_mut([j, i])
    }

    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        self.data.mut_all(f);
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.data.reset_all(value);
    }
}

impl<T, V> MatrixColMajor<T> for V2MatrixColMajor<T, V>
where
    V: NVec<D2, T>,
{
    fn col(&self, i: usize) -> impl NVec<D1, T> {
        self.data.child(i)
    }
}

impl<T, V> MatrixColMajorMut<T> for V2MatrixColMajor<T, V>
where
    V: NVecMut<D2, T>,
{
    fn col_mut(&mut self, i: usize) -> impl NVecMut<D1, T> {
        self.data.child_mut(i)
    }
}
