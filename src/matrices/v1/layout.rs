use super::{col::Col, row::Row};
use crate::{NVec, NVecMut, D1};

pub trait V1MatrixLayout {
    fn num_rows(&self) -> usize;

    fn num_cols(&self) -> usize;

    fn num_children(&self) -> usize;

    fn num_children_secondary(&self) -> usize;

    fn v1_idx(&self, i: usize, j: usize) -> usize;

    fn child<T, V>(&self, data: V, first_idx: usize) -> impl NVec<D1, T>
    where
        V: NVec<D1, T>;

    fn child_mut<T, V>(&self, data: V, first_idx: usize) -> impl NVecMut<D1, T>
    where
        V: NVecMut<D1, T>;
}

// row major

#[derive(Clone)]
pub struct RowMajor {
    num_rows: usize,
    num_cols: usize,
}

impl RowMajor {
    pub(super) fn new(num_rows: usize, num_cols: usize) -> Self {
        Self { num_rows, num_cols }
    }
}

impl V1MatrixLayout for RowMajor {
    #[inline(always)]
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline(always)]
    fn num_children(&self) -> usize {
        self.num_rows
    }

    #[inline(always)]
    fn num_children_secondary(&self) -> usize {
        self.num_cols
    }

    #[inline(always)]
    fn v1_idx(&self, i: usize, j: usize) -> usize {
        self.num_cols * i + j
    }

    fn child<T, V>(&self, data: V, first_idx: usize) -> impl NVec<D1, T>
    where
        V: NVec<D1, T>,
    {
        Row::new(data, self.clone(), first_idx)
    }

    fn child_mut<T, V>(&self, data: V, first_idx: usize) -> impl NVecMut<D1, T>
    where
        V: NVecMut<D1, T>,
    {
        Row::new(data, self.clone(), first_idx)
    }
}

// col major

#[derive(Clone)]
pub struct ColMajor {
    num_rows: usize,
    num_cols: usize,
}

impl ColMajor {
    pub(super) fn new(num_rows: usize, num_cols: usize) -> Self {
        Self { num_rows, num_cols }
    }
}

impl V1MatrixLayout for ColMajor {
    #[inline(always)]
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline(always)]
    fn num_children(&self) -> usize {
        self.num_cols
    }

    #[inline(always)]
    fn num_children_secondary(&self) -> usize {
        self.num_rows
    }

    #[inline(always)]
    fn v1_idx(&self, i: usize, j: usize) -> usize {
        self.num_rows * j + i
    }

    fn child<T, V>(&self, data: V, first_idx: usize) -> impl NVec<D1, T>
    where
        V: NVec<D1, T>,
    {
        Col::new(data, self.clone(), first_idx)
    }

    fn child_mut<T, V>(&self, data: V, first_idx: usize) -> impl NVecMut<D1, T>
    where
        V: NVecMut<D1, T>,
    {
        Col::new(data, self.clone(), first_idx)
    }
}
