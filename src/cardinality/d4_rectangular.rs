use super::{
    card::{child_fun_unchecked, Card},
    panic_d1, panic_d2, panic_d3,
};
use crate::{cardinality::d3_rectangular::RectangularCardD3, Dim, IdxLeqD3, NVec, D4};

/// A rectangular cardinality of dimension `D4` vectors, which is equal to four
/// lengths across each dimensions.
#[derive(Clone, Copy)]
pub struct RectangularCardD4 {
    card_idx0: usize,
    card_idx1: usize,
    card_idx2: usize,
    card_idx3: usize,
}

impl From<[usize; 4]> for RectangularCardD4 {
    fn from(value: [usize; 4]) -> Self {
        Self {
            card_idx0: value[0],
            card_idx1: value[1],
            card_idx2: value[2],
            card_idx3: value[3],
        }
    }
}

impl Card<D4> for RectangularCardD4 {
    fn is_rectangular(&self) -> bool {
        true
    }

    fn cardinality_of(&self, idx: impl Into<<D4 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD3::IdxD0([]) => self.card_idx0,
            IdxLeqD3::IdxD1([i]) => match i < self.card_idx0 {
                true => self.card_idx1,
                false => panic_d1(i, self.card_idx0),
            },
            IdxLeqD3::IdxD2([i, j]) => match i < self.card_idx0 {
                true => match j < self.card_idx1 {
                    true => self.card_idx2,
                    false => panic_d2(i, j, self.card_idx1),
                },
                false => panic_d1(i, self.card_idx0),
            },
            IdxLeqD3::IdxD3([i, j, k]) => match i < self.card_idx0 {
                true => match j < self.card_idx1 {
                    true => match k < self.card_idx2 {
                        true => self.card_idx3,
                        false => panic_d3(i, j, k, self.card_idx2),
                    },
                    false => panic_d2(i, j, self.card_idx1),
                },
                false => panic_d1(i, self.card_idx0),
            },
        }
    }

    fn child_card(&self, i: usize) -> impl Card<<D4 as Dim>::PrevDim> {
        match i < self.card_idx0 {
            true => RectangularCardD3 {
                card_idx0: self.card_idx1,
                card_idx1: self.card_idx2,
                card_idx2: self.card_idx3,
            },
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn child_fun<T, F>(&self, i: usize, fun: F) -> impl Fn(<<D4 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D4 as Dim>::Idx) -> T,
    {
        match i < self.card_idx0 {
            true => child_fun_unchecked::<D4, _, _>(i, fun),
            false => panic_d1(i, self.card_idx0),
        }
    }

    fn vec_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D4, T> + 'a,
    {
        (0..self.cardinality_of([])).flat_map(move |i| {
            (0..self.cardinality_of([i])).flat_map(move |j| {
                (0..self.cardinality_of([i, j])).flat_map(move |k| {
                    (0..self.cardinality_of([i, j, k])).map(move |l| vec.at([i, j, k, l]))
                })
            })
        })
    }

    fn vec_enumerate_all<'a, T, N>(
        &'a self,
        vec: &'a N,
    ) -> impl Iterator<Item = (<D4 as Dim>::Idx, T)>
    where
        N: NVec<D4, T> + 'a,
    {
        (0..self.cardinality_of([])).flat_map(move |i| {
            (0..self.cardinality_of([i])).flat_map(move |j| {
                (0..self.cardinality_of([i, j])).flat_map(move |k| {
                    (0..self.cardinality_of([i, j, k]))
                        .map(move |l| ([i, j, k, l], vec.at([i, j, k, l])))
                })
            })
        })
    }
}
