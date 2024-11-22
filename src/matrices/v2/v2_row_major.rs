use super::super::{
    matrix::Matrix,
    matrix_row_major::{MatrixRowMajor, MatrixRowMajorMut},
};
use crate::{matrices::MatrixMut, IntoIdx, NVec, NVecMut, D1, D2};
use core::marker::PhantomData;

/// A row major matrix.
pub struct V2MatrixRowMajor<T, V>
where
    V: NVec<D2, T>,
{
    data: V,
    phantom: PhantomData<T>,
}

impl<T, V> V2MatrixRowMajor<T, V>
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

impl<T, V> Matrix<T> for V2MatrixRowMajor<T, V>
where
    V: NVec<D2, T>,
{
    #[inline(always)]
    fn num_rows(&self) -> usize {
        self.data.card([])
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {
        match self.num_rows() {
            0 => 0,
            _ => self.data.card([0]),
        }
    }

    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        self.data.at(idx)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.data.all()
    }
}

impl<T, V> MatrixMut<T> for V2MatrixRowMajor<T, V>
where
    V: NVecMut<D2, T>,
{
    #[inline(always)]
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        self.data.at_mut(idx)
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

impl<T, V> MatrixRowMajor<T> for V2MatrixRowMajor<T, V>
where
    V: NVec<D2, T>,
{
    fn row(&self, i: usize) -> impl NVec<D1, T> {
        self.data.child(i)
    }
}

impl<T, V> MatrixRowMajorMut<T> for V2MatrixRowMajor<T, V>
where
    V: NVecMut<D2, T>,
{
    fn row_mut(&mut self, i: usize) -> impl NVecMut<D1, T> {
        self.data.child_mut(i)
    }
}
