use crate::dim::*;
use core::fmt::Debug;

/// A type that can be transformed into the standard index of dimension `D`.
pub trait IntoIdx<D: Dim>: Sized + Copy + Debug {
    /// Converts self into the standard index of dimension `D`.
    fn into_idx(self) -> D::Idx;
}

impl<D: Dim> IntoIdx<D> for <D as Dim>::Idx {
    #[inline(always)]
    fn into_idx(self) -> <D as Dim>::Idx {
        self
    }
}

// d1

impl IntoIdx<D1> for usize {
    #[inline(always)]
    fn into_idx(self) -> <D1 as Dim>::Idx {
        [self]
    }
}

// tuples

impl IntoIdx<D2> for (usize, usize) {
    #[inline(always)]
    fn into_idx(self) -> <D2 as Dim>::Idx {
        [self.0, self.1]
    }
}

impl IntoIdx<D3> for (usize, usize, usize) {
    #[inline(always)]
    fn into_idx(self) -> <D3 as Dim>::Idx {
        [self.0, self.1, self.2]
    }
}

impl IntoIdx<D4> for (usize, usize, usize, usize) {
    #[inline(always)]
    fn into_idx(self) -> <D4 as Dim>::Idx {
        [self.0, self.1, self.2, self.3]
    }
}
