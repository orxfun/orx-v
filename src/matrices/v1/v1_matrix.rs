use super::layout::{V1MatrixColMajor, V1MatrixLayout, V1MatrixRowMajor};
use crate::{
    matrices::{
        Matrix, MatrixColMajor, MatrixColMajorMut, MatrixMut, MatrixRowMajor, MatrixRowMajorMut,
    },
    IntoIdx, NVec, NVecMut, D1, D2,
};
use core::marker::PhantomData;

/// A matrix represented by a flat one-dimensional vector `V1`.
#[derive(Clone)]
pub struct V1Matrix<T, V, L>
where
    V: NVec<D1, T>,
    L: V1MatrixLayout,
{
    layout: L,
    data: V,
    phantom: PhantomData<T>,
}

impl<T, V, L> V1Matrix<T, V, L>
where
    V: NVec<D1, T>,
    L: V1MatrixLayout,
{
    pub(super) fn new(layout: L, data: V) -> Self {
        let (num_rows, num_cols) = (layout.num_rows(), layout.num_cols());
        assert_eq!(num_rows * num_cols, data.card([]));

        Self {
            layout,
            data,
            phantom: PhantomData,
        }
    }
}

// matrix

impl<T, V, L> Matrix<T> for V1Matrix<T, V, L>
where
    V: NVec<D1, T>,
    L: V1MatrixLayout,
{
    #[inline(always)]
    fn num_rows(&self) -> usize {
        self.layout.num_rows()
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {
        self.layout.num_cols()
    }

    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        let [i, j] = idx.into_idx();
        let idx = self.layout.v1_idx(i, j);
        self.data.at(idx)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.data.all()
    }
}

impl<T, V, L> MatrixMut<T> for V1Matrix<T, V, L>
where
    V: NVecMut<D1, T>,
    L: V1MatrixLayout,
{
    fn at_mut<Idx: IntoIdx<D2>>(&mut self, idx: Idx) -> &mut T {
        let [i, j] = idx.into_idx();
        let idx = self.layout.v1_idx(i, j);
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

impl<T, V> MatrixRowMajor<T> for V1Matrix<T, V, V1MatrixRowMajor>
where
    V: NVec<D1, T>,
{
    fn row(&self, i: usize) -> impl NVec<D1, T> {
        self.layout.child(&self.data, i)
    }
}

impl<T, V> MatrixRowMajorMut<T> for V1Matrix<T, V, V1MatrixRowMajor>
where
    V: NVecMut<D1, T>,
{
    fn row_mut(&mut self, i: usize) -> impl NVecMut<D1, T> {
        self.layout.child_mut(&mut self.data, i)
    }
}

impl<T, V> MatrixColMajor<T> for V1Matrix<T, V, V1MatrixColMajor>
where
    V: NVec<D1, T>,
{
    fn col(&self, i: usize) -> impl NVec<D1, T> {
        self.layout.child(&self.data, i)
    }
}

impl<T, V> MatrixColMajorMut<T> for V1Matrix<T, V, V1MatrixColMajor>
where
    V: NVecMut<D1, T>,
{
    fn col_mut(&mut self, i: usize) -> impl NVecMut<D1, T> {
        self.layout.child_mut(&mut self.data, i)
    }
}
