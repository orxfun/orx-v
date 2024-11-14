use super::sparse_vec::SparseVec;
use crate::{children::ChildD4D3, Card, Dim, IntoIdx, Lookup, NVec, NVecCoreSealed, NVecMut, D4};

impl<T, L, C> NVecCoreSealed<D4, T> for SparseVec<D4, T, C, L>
where
    T: Copy,
    L: Lookup<<D4 as Dim>::Idx, T>,
    C: Card<D4>,
{
    fn core_num_children(&self) -> usize {
        self.sparse_num_children()
    }

    fn core_card(&self, idx: impl Into<<D4 as Dim>::CardIdx>) -> usize {
        self.sparse_card(idx)
    }

    fn core_child(&self, i: <D4 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D4 as Dim>::PrevDim, T> {
        ChildD4D3::<_, T> {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D4>, f: &mut F) -> O {
        f(&self.sparse_at(idx))
    }

    fn core_is_rectangular(&self) -> bool {
        self.sparse_cardinality().is_rectangular()
    }
}

// nvec

impl<T, L, C> NVec<D4, T> for SparseVec<D4, T, C, L>
where
    T: Copy,
    L: Lookup<<D4 as Dim>::Idx, T>,
    C: Card<D4>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D4>) -> T {
        self.sparse_at(idx)
    }

    #[inline(always)]
    fn in_bounds(&self, idx: impl Into<<D4 as Dim>::LeqIdx>) -> bool {
        self.sparse_in_bounds(idx)
    }

    fn child(&self, i: <D4 as Dim>::ChildIdx) -> impl NVec<<D4 as Dim>::PrevDim, T> {
        ChildD4D3 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.sparse_cardinality().vec_all(self)
    }
}

// nvec-mut

impl<T, L, C> NVecMut<D4, T> for SparseVec<D4, T, C, L>
where
    T: Copy,
    L: Lookup<<D4 as Dim>::Idx, T>,
    C: Card<D4>,
{
    #[inline(always)]
    fn at_mut<Idx: IntoIdx<D4>>(&mut self, idx: Idx) -> &mut T {
        self.sparse_at_mut(idx)
    }

    #[inline(always)]
    fn set<Idx: IntoIdx<D4>>(&mut self, idx: Idx, value: T) {
        self.sparse_set(idx, value);
    }

    fn child_mut(&mut self, i: <D4 as Dim>::ChildIdx) -> impl NVecMut<<D4 as Dim>::PrevDim, T> {
        ChildD4D3 {
            i,
            parent: self,
            phantom: Default::default(),
        }
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
