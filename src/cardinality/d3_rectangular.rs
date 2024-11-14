use super::{
    card::{child_fun_unchecked, Card},
    panic_d1, panic_d2,
};
use crate::{cardinality::d2_rectangular::RectangularCardD2, Dim, IdxLeqD2, NVec, D3};

/// A rectangular cardinality of dimension `D3` vectors, which is equal to three
/// lengths across each dimensions.
#[derive(Clone, Copy)]
pub struct RectangularCardD3 {
    pub(super) card_idx0: usize,
    pub(super) card_idx1: usize,
    pub(super) card_idx2: usize,
}

impl From<[usize; 3]> for RectangularCardD3 {
    fn from(value: [usize; 3]) -> Self {
        Self {
            card_idx0: value[0],
            card_idx1: value[1],
            card_idx2: value[2],
        }
    }
}

impl Card<D3> for RectangularCardD3 {
    fn is_rectangular(&self) -> bool {
        true
    }

    fn cardinality_of(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD2::IdxD0([]) => self.card_idx0,
            IdxLeqD2::IdxD1([i]) => match i < self.card_idx0 {
                true => self.card_idx1,
                false => panic_d1(i, self.card_idx0),
            },
            IdxLeqD2::IdxD2([i, j]) => match i < self.card_idx0 {
                true => match j < self.card_idx1 {
                    true => self.card_idx2,
                    false => panic_d2(i, j, self.card_idx1),
                },
                false => panic_d1(i, self.card_idx0),
            },
        }
    }

    fn child_card(&self, i: usize) -> impl Card<<D3 as Dim>::PrevDim> {
        match i < self.card_idx0 {
            true => RectangularCardD2 {
                card_idx0: self.card_idx1,
                card_idx1: self.card_idx2,
            },
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn child_fun<T, F>(&self, i: usize, fun: F) -> impl Fn(<<D3 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D3 as Dim>::Idx) -> T,
    {
        match i < self.card_idx0 {
            true => child_fun_unchecked::<D3, _, _>(i, fun),
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn vec_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D3, T> + 'a,
    {
        (0..self.cardinality_of([])).flat_map(move |i| {
            (0..self.cardinality_of([i]))
                .flat_map(move |j| (0..self.cardinality_of([i, j])).map(move |k| vec.at([i, j, k])))
        })
    }

    fn vec_enumerate_all<'a, T, N>(
        &'a self,
        vec: &'a N,
    ) -> impl Iterator<Item = (<D3 as Dim>::Idx, T)>
    where
        N: NVec<D3, T> + 'a,
    {
        (0..self.cardinality_of([])).flat_map(move |i| {
            (0..self.cardinality_of([i])).flat_map(move |j| {
                (0..self.cardinality_of([i, j])).map(move |k| ([i, j, k], vec.at([i, j, k])))
            })
        })
    }
}
