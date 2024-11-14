use crate::{Dim, IntoIdx, NVecCoreSealed, D1};
use core::ops::Range;

impl NVecCoreSealed<D1, usize> for Range<usize> {
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    fn core_card(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.end - self.start
    }

    #[inline(always)]
    fn core_child(
        &self,
        _: <D1 as Dim>::ChildIdx,
    ) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, usize> {
        self
    }

    #[inline(always)]
    fn core_map<F: FnMut(&usize) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        f(&(self.start + idx.into_idx()[0]))
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}
