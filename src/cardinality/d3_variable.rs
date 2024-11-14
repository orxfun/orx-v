use super::{
    card::{child_fun_unchecked, Card},
    panic_d1, panic_d2,
};
use crate::{
    cardinality::d2_variable::VariableCardD2, CardD1, Dim, FunVec, IdxLeqD2, NVec, D3, V2,
};

/// A variable cardinality of dimension `D3` vectors such that every child
/// in every lower dimension can have different numbers of children.
#[derive(Clone, Copy)]
pub struct VariableCardD3<V: V2<usize>>(pub(super) V);

impl<V: V2<usize>> From<V> for VariableCardD3<V> {
    fn from(value: V) -> Self {
        Self(value)
    }
}

impl<V: V2<usize>> Card<D3> for VariableCardD3<V> {
    fn is_rectangular(&self) -> bool {
        let n = self.0.card([]);
        let m = match n {
            0 => 0,
            _ => self.0.card([0]),
        };
        let p = match m {
            0 => 0,
            _ => self.0.at([0, 0]),
        };

        for i in 0..n {
            if self.0.card([i]) != m {
                return false;
            }

            for j in 0..m {
                if self.0.at([i, j]) != p {
                    return false;
                }
            }
        }

        true
    }

    fn cardinality_of(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD2::IdxD0([]) => self.0.card([]),
            IdxLeqD2::IdxD1([i]) => match i < self.0.card([]) {
                true => self.0.card([i]),
                false => panic_d1(i, self.0.card([])),
            },
            IdxLeqD2::IdxD2([i, j]) => match i < self.0.card([]) {
                true => match j < self.0.card([i]) {
                    true => self.0.at([i, j]),
                    false => panic_d2(i, j, self.0.card([i])),
                },
                false => panic_d1(i, self.0.card([])),
            },
        }
    }

    fn child_card(&self, i: usize) -> impl Card<<D3 as Dim>::PrevDim> {
        match i < self.0.card([]) {
            true => {
                let card_idx0 = self.0.card([i]);
                let card_idx1 = FunVec::new(move |[j]| self.0.at([i, j]), CardD1::from(card_idx0));
                VariableCardD2(card_idx1)
            }
            false => panic_d1(i, self.0.card([])),
        }
    }

    fn child_fun<T, F>(&self, i: usize, fun: F) -> impl Fn(<<D3 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D3 as Dim>::Idx) -> T,
    {
        match i < self.0.card([]) {
            true => child_fun_unchecked::<D3, _, _>(i, fun),
            false => panic_d1(i, self.0.card([])),
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
