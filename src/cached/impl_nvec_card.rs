use super::{cache::Cache, cached_vec::CachedVec};
use crate::{
    CardD1, Dim, FunVec, NVec, NVecCoreSealed, VariableCardD2, VariableCardD3, D1, D2, D3, D4,
};

// D1

impl<T, V, C> NVecCoreSealed<D1, T> for CachedVec<D1, T, V, C>
where
    V: NVec<D1, T>,
    C: Cache<<D1 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.vec.num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D1 as Dim>::CardIdx>) -> usize {
        self.vec.card(idx)
    }

    fn core_child(&self, _: <D1 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D1 as Dim>::PrevDim, T> {
        self
    }

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl crate::IntoIdx<D1>, f: &mut F) -> O {
        f(&*unsafe { self.entry_or_insert_with(idx) })
    }

    fn core_is_rectangular(&self) -> bool {
        self.vec.core_is_rectangular()
    }
}

// D2

impl<T, V, C> NVecCoreSealed<D2, T> for CachedVec<D2, T, V, C>
where
    V: NVec<D2, T>,
    C: Cache<<D2 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.vec.num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D2 as Dim>::CardIdx>) -> usize {
        self.vec.card(idx)
    }

    fn core_child(&self, i: <D2 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D2 as Dim>::PrevDim, T> {
        let fun = move |idx: <<D2 as Dim>::PrevDim as Dim>::Idx| {
            let idx = D2::left_join_from_lower_dim(i, idx);
            self.at(idx)
        };
        FunVec::new(fun, CardD1::from(self.vec.card([i])))
    }

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl crate::IntoIdx<D2>, f: &mut F) -> O {
        f(&*unsafe { self.entry_or_insert_with(idx) })
    }

    fn core_is_rectangular(&self) -> bool {
        self.vec.core_is_rectangular()
    }
}

// D3

impl<T, V, C> NVecCoreSealed<D3, T> for CachedVec<D3, T, V, C>
where
    V: NVec<D3, T>,
    C: Cache<<D3 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.vec.num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D3 as Dim>::CardIdx>) -> usize {
        self.vec.card(idx)
    }

    fn core_child(&self, i: <D3 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D3 as Dim>::PrevDim, T> {
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

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl crate::IntoIdx<D3>, f: &mut F) -> O {
        f(&*unsafe { self.entry_or_insert_with(idx) })
    }

    fn core_is_rectangular(&self) -> bool {
        self.vec.core_is_rectangular()
    }
}

// D4

impl<T, V, C> NVecCoreSealed<D4, T> for CachedVec<D4, T, V, C>
where
    V: NVec<D4, T>,
    C: Cache<<D4 as Dim>::Idx, T>,
    T: Copy,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.vec.num_children()
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D4 as Dim>::CardIdx>) -> usize {
        self.vec.card(idx)
    }

    fn core_child(&self, i: <D4 as Dim>::ChildIdx) -> impl NVecCoreSealed<<D4 as Dim>::PrevDim, T> {
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

    #[inline(always)]
    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl crate::IntoIdx<D4>, f: &mut F) -> O {
        f(&*unsafe { self.entry_or_insert_with(idx) })
    }

    fn core_is_rectangular(&self) -> bool {
        self.vec.core_is_rectangular()
    }
}
