use super::{col::Col, row::Row};
use crate::{NVec, NVecMut, D1};

/// Layout for matrices with an underlying flat vector of `D1`.
pub trait V1MatrixLayout: Clone {
    /// Number of rows.
    fn num_rows(&self) -> usize;

    /// Number of columns.
    fn num_cols(&self) -> usize;

    /// Number of primary children:
    /// * number of rows if row-major,
    /// * number of columns if col-major.
    fn num_children(&self) -> usize;

    /// Number of children of the transpose of the matrix:
    /// * number of rows if col-major,
    /// * number of columns if row-major.
    fn num_children_secondary(&self) -> usize;

    /// Transformation of the row and column indices (`i`, `j`) into a one dimensional
    /// index for the underlying data.
    fn v1_idx(&self, i: usize, j: usize) -> usize;

    /// Child of the matrix:
    /// * row if row-major,
    /// * column if col-major.
    fn child<T, V>(&self, data: V, first_idx: usize) -> impl NVec<D1, T>
    where
        V: NVec<D1, T>;

    /// Mutable child of the matrix:
    /// * row if row-major,
    /// * column if col-major.
    fn child_mut<T, V>(&self, data: V, first_idx: usize) -> impl NVecMut<D1, T>
    where
        V: NVecMut<D1, T>;
}

// row major

/// Row major layout.
#[derive(Clone)]
pub struct V1MatrixRowMajor {
    num_rows: usize,
    num_cols: usize,
}

impl V1MatrixRowMajor {
    pub(super) fn new(num_rows: usize, num_cols: usize) -> Self {
        Self { num_rows, num_cols }
    }
}

impl V1MatrixLayout for V1MatrixRowMajor {
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

/// Column major layout.
#[derive(Clone)]
pub struct V1MatrixColMajor {
    num_rows: usize,
    num_cols: usize,
}

impl V1MatrixColMajor {
    pub(super) fn new(num_rows: usize, num_cols: usize) -> Self {
        Self { num_rows, num_cols }
    }
}

impl V1MatrixLayout for V1MatrixColMajor {
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
