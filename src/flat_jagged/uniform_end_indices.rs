use crate::{Dim, IntoIdx, NVec, NVecCoreSealed, D1};

pub struct UniformEndIndices {
    num_rows: usize,
    num_cols: usize,
    flat_len: usize,
}

impl UniformEndIndices {
    pub(super) fn new(num_cols: usize, flat_len: usize) -> Self {
        debug_assert!(num_cols > 0);
        let n = flat_len / num_cols;
        let num_rows = n + match n * num_cols == flat_len {
            true => 0,
            false => 1,
        };
        Self {
            num_rows,
            num_cols,
            flat_len,
        }
    }

    #[inline(always)]
    fn end_index(&self, i: usize) -> usize {
        core::cmp::min((i + 1) * self.num_cols, self.flat_len)
    }
}

impl NVecCoreSealed<D1, usize> for UniformEndIndices {
    fn core_num_children(&self) -> usize {
        self.num_rows
    }

    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.num_rows
    }

    fn core_child(
        &self,
        _: <D1 as Dim>::ChildIdx,
    ) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, usize> {
        self
    }

    fn core_map<F: FnMut(&usize) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        f(&self.end_index(idx.into_idx()[0]))
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl NVec<D1, usize> for UniformEndIndices {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D1>) -> usize {
        self.end_index(idx.into_idx()[0])
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, usize> {
        self
    }

    fn all(&self) -> impl Iterator<Item = usize> {
        (0..self.num_rows).map(|i| match i + 1 < self.num_rows {
            true => self.num_cols,
            false => core::cmp::min((i + 1) * self.num_cols, self.flat_len),
        })
    }
}
