use super::card::Card;
use crate::{Dim, NVec, D1};

/// A cardinality of dimension `D1` vectors, which is simply a length.
#[derive(Clone, Copy)]
pub struct CardD1(pub(super) usize);

impl From<usize> for CardD1 {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Card<D1> for CardD1 {
    fn is_rectangular(&self) -> bool {
        true
    }

    #[inline(always)]
    fn cardinality_of(&self, _: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.0
    }

    fn child_card(&self, _: <D1 as Dim>::ChildIdx) -> impl Card<<D1 as Dim>::PrevDim> {
        // TODO: NEVER!
        Self(self.0)
    }

    fn child_fun<T, F>(
        &self,
        _: <D1 as Dim>::ChildIdx,
        fun: F,
    ) -> impl Fn(<<D1 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D1 as Dim>::Idx) -> T,
    {
        fun
    }

    fn vec_all<'a, T, V>(&'a self, vec: &'a V) -> impl Iterator<Item = T>
    where
        V: NVec<D1, T> + 'a,
    {
        (0..self.cardinality_of([])).map(move |i| vec.at([i]))
    }

    fn vec_enumerate_all<'a, T, V>(
        &'a self,
        vec: &'a V,
    ) -> impl Iterator<Item = (<D1 as Dim>::Idx, T)>
    where
        V: NVec<D1, T> + 'a,
    {
        (0..self.cardinality_of([])).map(move |i| ([i], vec.at([i])))
    }
}
