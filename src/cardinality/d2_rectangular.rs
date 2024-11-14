use super::{
    card::{child_fun_unchecked, Card},
    d1::CardD1,
    panic_d1,
};
use crate::{Dim, IdxLeqD1, NVec, D2};

/// A rectangular cardinality of dimension `D2` vectors, which is equal to two
/// lengths across each dimensions.
#[derive(Clone, Copy)]
pub struct RectangularCardD2 {
    pub(super) card_idx0: usize,
    pub(super) card_idx1: usize,
}

impl From<[usize; 2]> for RectangularCardD2 {
    fn from(value: [usize; 2]) -> Self {
        Self {
            card_idx0: value[0],
            card_idx1: value[1],
        }
    }
}

impl Card<D2> for RectangularCardD2 {
    fn is_rectangular(&self) -> bool {
        true
    }

    fn cardinality_of(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0([]) => self.card_idx0,
            IdxLeqD1::IdxD1([i]) => match i < self.card_idx0 {
                true => self.card_idx1,
                false => panic_d1(i, self.card_idx0),
            },
        }
    }

    fn child_card(&self, i: <D2 as Dim>::ChildIdx) -> impl Card<<D2 as Dim>::PrevDim> {
        match i < self.card_idx0 {
            true => CardD1(self.card_idx1),
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn child_fun<T, F>(
        &self,
        i: <D2 as Dim>::ChildIdx,
        fun: F,
    ) -> impl Fn(<<D2 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D2 as Dim>::Idx) -> T,
    {
        match i < self.card_idx0 {
            true => child_fun_unchecked::<D2, _, _>(i, fun),
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn vec_all<'a, T, V>(&'a self, vec: &'a V) -> impl Iterator<Item = T>
    where
        V: NVec<D2, T> + 'a,
    {
        (0..self.cardinality_of([]))
            .flat_map(move |i| (0..self.cardinality_of([i])).map(move |j| vec.at([i, j])))
    }

    fn vec_enumerate_all<'a, T, N>(
        &'a self,
        vec: &'a N,
    ) -> impl Iterator<Item = (<D2 as Dim>::Idx, T)>
    where
        N: NVec<D2, T> + 'a,
    {
        (0..self.cardinality_of([]))
            .flat_map(move |i| (0..self.cardinality_of([i])).map(move |j| ([i, j], vec.at([i, j]))))
    }
}
