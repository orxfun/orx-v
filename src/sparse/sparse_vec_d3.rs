use super::sparse_vec::SparseVec;
use crate::{children::ChildD3D2, Card, Dim, IntoIdx, Lookup, NVec, NVecCoreSealed, NVecMut, D3};

impl<T, L, C> NVecCoreSealed<D3, T> for SparseVec<D3, T, C, L>
where
    T: Copy,
    L: Lookup<<D3 as Dim>::Idx, T>,
    C: Card<D3>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.sparse_num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        self.sparse_card(idx)
    }

    fn core_child(&self, i: <D3 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D3 as Dim>::PrevDim, T> {
        ChildD3D2::<_, T> {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D3>, f: &mut F) -> O {
        f(&self.sparse_at(idx))
    }

    fn core_is_rectangular(&self) -> bool {
        self.sparse_cardinality().is_rectangular()
    }
}

// nvec

impl<T, L, C> NVec<D3, T> for SparseVec<D3, T, C, L>
where
    T: Copy,
    L: Lookup<<D3 as Dim>::Idx, T>,
    C: Card<D3>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D3>) -> T {
        self.sparse_at(idx)
    }

    #[inline(always)]
    fn in_bounds(&self, idx: impl Into<<D3 as Dim>::LeqIdx>) -> bool {
        self.sparse_in_bounds(idx)
    }

    fn child(&self, i: <D3 as Dim>::ChildIdx) -> impl NVec<<D3 as Dim>::PrevDim, T> {
        ChildD3D2 {
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

impl<T, L, C> NVecMut<D3, T> for SparseVec<D3, T, C, L>
where
    T: Copy,
    L: Lookup<<D3 as Dim>::Idx, T>,
    C: Card<D3>,
{
    #[inline(always)]
    fn at_mut<Idx: IntoIdx<D3>>(&mut self, idx: Idx) -> &mut T {
        self.sparse_at_mut(idx)
    }

    #[inline(always)]
    fn set<Idx: IntoIdx<D3>>(&mut self, idx: Idx, value: T) {
        self.sparse_set(idx, value);
    }

    fn child_mut(&mut self, i: <D3 as Dim>::ChildIdx) -> impl NVecMut<<D3 as Dim>::PrevDim, T> {
        ChildD3D2 {
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
