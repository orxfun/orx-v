use super::sparse_vec::SparseVec;
use crate::{Card, Dim, IntoIdx, Lookup, NVec, NVecCoreSealed, NVecMut, D1};

impl<T, L, C> NVecCoreSealed<D1, T> for SparseVec<D1, T, C, L>
where
    T: Copy + Default,
    L: Lookup<<D1 as Dim>::Idx, T>,
    C: Card<D1>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.sparse_num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.sparse_card(idx)
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D1>, f: &mut F) -> O {
        f(&self.sparse_at(idx))
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

// nvec

impl<T, L, C> NVec<D1, T> for SparseVec<D1, T, C, L>
where
    T: Copy + Default,
    L: Lookup<<D1 as Dim>::Idx, T>,
    C: Card<D1>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        self.sparse_at(idx)
    }

    #[inline(always)]
    fn in_bounds(&self, idx: impl Into<<D1 as Dim>::LeqIdx>) -> bool {
        self.sparse_in_bounds(idx)
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    #[inline(always)]
    fn all(&self) -> impl Iterator<Item = T> {
        self.sparse_cardinality().vec_all(self)
    }
}

// nvec-mut

impl<T, L, C> NVecMut<D1, T> for SparseVec<D1, T, C, L>
where
    T: Copy + Default,
    L: Lookup<<D1 as Dim>::Idx, T>,
    C: Card<D1>,
{
    #[inline(always)]
    fn at_mut<Idx: IntoIdx<D1>>(&mut self, idx: Idx) -> &mut T {
        self.sparse_at_mut(idx)
    }

    #[inline(always)]
    fn set<Idx: IntoIdx<D1>>(&mut self, idx: Idx, value: T) {
        self.sparse_set(idx, value);
    }

    fn child_mut(&mut self, _: <D1 as Dim>::ChildIdx) -> impl NVecMut<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn mut_all<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        self.sparse_mut_all(f);
    }

    fn reset_all(&mut self, value: T)
    where
        T: PartialEq + Copy,
    {
        self.sparse_reset_all(value);
    }
}
