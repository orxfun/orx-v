use crate::{Dim, IntoIdx, NVec, D1};
use core::ops::Range;

impl NVec<D1, usize> for Range<usize> {
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D1>) -> usize {
        self.start + idx.into_idx()[0]
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, usize> {
        self
    }

    fn all(&self) -> impl Iterator<Item = usize> {
        self.start..self.end
    }
}
