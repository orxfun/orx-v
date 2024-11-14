use super::{
    card::{child_fun_unchecked, Card},
    panic_d1,
};
use crate::{cardinality::d1::CardD1, Dim, IdxLeqD1, NVec, D2, V1};

/// A variable cardinality of dimension `D2` vectors such that every child
/// in every lower dimension can have different numbers of children.
#[derive(Clone, Copy)]
pub struct VariableCardD2<V: V1<usize>>(pub(super) V);

impl<V: V1<usize>> From<V> for VariableCardD2<V> {
    fn from(value: V) -> Self {
        Self(value)
    }
}

impl<V: V1<usize>> Card<D2> for VariableCardD2<V> {
    fn is_rectangular(&self) -> bool {
        match self.0.card([]) {
            0 => true,
            n => {
                let m = self.0.at([0]);
                for i in 1..n {
                    if self.0.at([i]) != m {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn cardinality_of(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        match idx.into() {
            IdxLeqD1::IdxD0([]) => self.0.card([]),
            IdxLeqD1::IdxD1([i]) => match i < self.0.card([]) {
                true => self.0.at([i]),
                false => panic_d1(i, self.0.card([])),
            },
        }
    }

    fn child_card(&self, i: usize) -> impl Card<<D2 as Dim>::PrevDim> {
        match i < self.0.card([]) {
            true => CardD1(self.0.at([i])),
            false => panic_d1(i, self.0.card([])),
        }
    }

    fn child_fun<T, F>(&self, i: usize, fun: F) -> impl Fn(<<D2 as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D2 as Dim>::Idx) -> T,
    {
        match i < self.0.card([]) {
            true => child_fun_unchecked::<D2, _, _>(i, fun),
            false => panic_d1(i, self.0.card([])),
        }
    }

    fn vec_all<'a, T, N>(&'a self, vec: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D2, T> + 'a,
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
