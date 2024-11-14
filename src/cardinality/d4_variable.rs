use super::{
    card::{child_fun_unchecked, Card},
    panic_d1, panic_d2, panic_d3, CardD1, VariableCardD2,
};
use crate::{cardinality::d3_variable::VariableCardD3, Dim, FunVec, IdxLeqD3, NVec, D4, V3};

/// A variable cardinality of dimension `D4` vectors such that every child
/// in every lower dimension can have different numbers of children.
#[derive(Clone, Copy)]
pub struct VariableCardD4<V: V3<usize>>(V);

impl<V: V3<usize>> From<V> for VariableCardD4<V> {
    fn from(value: V) -> Self {
        Self(value)
    }
}

impl<V: V3<usize>> Card<D4> for VariableCardD4<V> {
    fn is_rectangular(&self) -> bool {
        let n = self.0.card([]);
        let m = match n {
            0 => 0,
            _ => self.0.card([0]),
        };
        let p = match m {
            0 => 0,
            _ => self.0.card([0, 0]),
        };
        let q = match p {
            0 => 0,
            _ => self.0.at([0, 0, 0]),
        };

        for i in 0..n {
            if self.0.card([i]) != m {
                return false;
            }

            for j in 0..m {
                if self.0.card([i, j]) != p {
                    return false;
                }

                for k in 0..p {
                    if self.0.at([i, j, k]) != q {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn cardinality_of(&self, idx: impl Into<<D4 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD3::IdxD0([]) => self.0.card([]),
            IdxLeqD3::IdxD1([i]) => match i < self.0.card([]) {
                true => self.0.card([i]),
                false => panic_d1(i, self.0.card([])),
            },
            IdxLeqD3::IdxD2([i, j]) => match i < self.0.card([]) {
                true => match j < self.0.card([i]) {
                    true => self.0.card([i, j]),
                    false => panic_d2(i, j, self.0.card([i])),
                },
                false => panic_d1(i, self.0.card([])),
            },
            IdxLeqD3::IdxD3([i, j, k]) => match i < self.0.card([]) {
                true => match j < self.0.card([i]) {
                    true => match k < self.0.card([i, j]) {
                        true => self.0.at([i, j, k]),
                        false => panic_d3(i, j, k, self.0.card([i, j])),
                    },
                    false => panic_d2(i, j, self.0.card([i])),
                },
                false => panic_d1(i, self.0.card([])),
            },
        }
    }

    fn child_card(&self, i: usize) -> impl Card<<D4 as Dim>::PrevDim> {
        match i < self.0.card([]) {
            true => {
                let card_of_card = FunVec::new(
                    move |[j]: [usize; 1]| self.0.card([i, j]),
                    CardD1::from(self.0.card([])),
                );
                let card = FunVec::new(
                    move |[j, k]: [usize; 2]| self.0.at([i, j, k]),
                    VariableCardD2::from(card_of_card),
                );
                // let card_of_card = V.d1().fun(move |[j]: [usize; 1]| self.0.card([i, j]));
                // let card = V
                //     .d2()
                //     .fun(move |[j, k]| self.0.at([i, j, k]))
                //     .with_variable_bounds(card_of_card);

                VariableCardD3(card)
            }
            false => panic_d1(i, self.0.card([])),
        }
    }

    fn child_fun<T, F>(&self, i: usize, fun: F) -> impl Fn(<<D4 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D4 as Dim>::Idx) -> T,
    {
        match i < self.0.card([]) {
            true => child_fun_unchecked::<D4, _, _>(i, fun),
            false => panic_d1(i, self.0.card([])),
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
