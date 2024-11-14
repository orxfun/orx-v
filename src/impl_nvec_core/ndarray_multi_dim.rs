use crate::children::{ChildD2D1, ChildD3D2, ChildD4D3};
use crate::{dim::*, NVecCoreSealed};
use ndarray::{Array, Ix2, Ix3, Ix4};

// d2 - full-indexed

impl<T> NVecCoreSealed<D2, T> for Array<T, Ix2> {
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.dim().0
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0(_) => self.dim().0,
            IdxLeqD1::IdxD1(_) => self.dim().1,
        }
    }

    fn core_child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
        ChildD2D1 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D2>, f: &mut F) -> O {
        f(&self[idx.into_idx()])
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

// d3 - full-indexed

impl<T> NVecCoreSealed<D3, T> for Array<T, Ix3> {
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.dim().0
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD2::IdxD0(_) => self.dim().0,
            IdxLeqD2::IdxD1(_) => self.dim().1,
            IdxLeqD2::IdxD2(_) => self.dim().2,
        }
    }

    fn core_child(&self, i: usize) -> impl NVecCoreSealed<<D3 as Dim>::PrevDim, T> {
        ChildD3D2 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D3>, f: &mut F) -> O {
        f(&self[idx.into_idx()])
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

// d4 - full-indexed

impl<T> NVecCoreSealed<D4, T> for Array<T, Ix4> {
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.dim().0
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D4 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD3::IdxD0(_) => self.dim().0,
            IdxLeqD3::IdxD1(_) => self.dim().1,
            IdxLeqD3::IdxD2(_) => self.dim().2,
            IdxLeqD3::IdxD3(_) => self.dim().3,
        }
    }

    fn core_child(&self, i: usize) -> impl NVecCoreSealed<<D4 as Dim>::PrevDim, T> {
        ChildD4D3 {
            i,
            parent: self,
            phantom: Default::default(),
        }
    }

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D4>, f: &mut F) -> O {
        f(&self[idx.into_idx()])
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}
