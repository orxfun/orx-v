use super::{cache::Cache, cached_vec::CachedVec};
use crate::{CardD1, Dim, FunVec, IntoIdx, NVec, VariableCardD2, VariableCardD3, D1, D2, D3, D4};

// D1

impl<T, V, C> NVec<D1, T> for CachedVec<D1, T, V, C>
where
    V: NVec<D1, T>,
    C: Cache<<D1 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D1>) -> T {
        *unsafe { self.entry_or_insert_with(idx) }
    }

    fn child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVec<<D1 as Dim>::PrevDim, T> {
        self
    }

    fn all(&self) -> impl Iterator<Item = T> {
        (0..self.num_children()).map(|i| self.at(i))
    }
}

// D2

impl<T, V, C> NVec<D2, T> for CachedVec<D2, T, V, C>
where
    V: NVec<D2, T>,
    C: Cache<<D2 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D2>) -> T {
        *unsafe { self.entry_or_insert_with(idx) }
    }

    fn child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVec<<D2 as Dim>::PrevDim, T> {
        let fun = move |idx: <<D2 as Dim>::PrevDim as Dim>::Idx| {
            let idx = D2::left_join_from_lower_dim(i, idx);
            self.at(idx)
        };
        FunVec::new(fun, CardD1::from(self.vec.card([i])))
    }

    fn all(&self) -> impl Iterator<Item = T> {
        (0..self.num_children())
            .flat_map(move |i| (0..self.card([i])).map(move |j| self.at([i, j])))
    }
}

// D3

impl<T, V, C> NVec<D3, T> for CachedVec<D3, T, V, C>
where
    V: NVec<D3, T>,
    C: Cache<<D3 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D3>) -> T {
        *unsafe { self.entry_or_insert_with(idx) }
    }

    fn child(&self, i: <D3 as Dim>::ChildIdx) -> impl NVec<<D3 as Dim>::PrevDim, T> {
        let fun = move |idx: <<D3 as Dim>::PrevDim as Dim>::Idx| {
            let idx = D3::left_join_from_lower_dim(i, idx);
            self.at(idx)
        };

        let card = FunVec::new(
            move |[j]| self.vec.card([i, j]),
            CardD1::from(self.card([i])),
        );

        FunVec::new(fun, VariableCardD2::from(card))
    }

    fn all(&self) -> impl Iterator<Item = T> {
        (0..self.num_children()).flat_map(move |i| {
            (0..self.card([i]))
                .flat_map(move |j| (0..self.card([i, j])).map(move |k| self.at([i, j, k])))
        })
    }
}

// D4

impl<T, V, C> NVec<D4, T> for CachedVec<D4, T, V, C>
where
    V: NVec<D4, T>,
    C: Cache<<D4 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D4>) -> T {
        *unsafe { self.entry_or_insert_with(idx) }
    }

    fn child(&self, i: <D4 as Dim>::ChildIdx) -> impl NVec<<D4 as Dim>::PrevDim, T> {
        let fun = move |idx: <<D4 as Dim>::PrevDim as Dim>::Idx| {
            let idx = D4::left_join_from_lower_dim(i, idx);
            self.at(idx)
        };

        let card1 = FunVec::new(
            move |[j]| self.vec.card([i, j]),
            CardD1::from(self.card([i])),
        );

        let card = FunVec::new(
            move |[j, k]| self.vec.card([i, j, k]),
            VariableCardD2::from(card1),
        );

        FunVec::new(fun, VariableCardD3::from(card))
    }

    fn all(&self) -> impl Iterator<Item = T> {
        (0..self.num_children()).flat_map(move |i| {
            (0..self.card([i])).flat_map(move |j| {
                (0..self.card([i, j])).flat_map(move |k| {
                    (0..self.card([i, j, k])).map(move |l| self.at([i, j, k, l]))
                })
            })
        })
    }
}
