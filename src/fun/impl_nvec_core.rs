use crate::{dim::*, Card, FunVec, NVecCoreSealed};

impl<D, T, F, C> NVecCoreSealed<D, T> for FunVec<D, T, F, C>
where
    D: Dim,
    F: Fn(<D as Dim>::Idx) -> T,
    C: Card<D>,
{
    #[inline(always)]
    fn core_num_children(&self) -> usize {
        self.card.cardinality_of([])
    }

    #[inline(always)]
    fn core_card(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        idx.into().card(self)
    }

    fn core_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim, T> {
        let i: usize = i.into();
        let fun = self.card.child_fun(i.into(), &self.fun);
        let card = self.card.child_card(i.into());
        FunVec::new(fun, card)
    }

    fn core_map<M: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D>, f: &mut M) -> O {
        f(&(self.fun)(idx.into_idx()))
    }

    fn core_is_rectangular(&self) -> bool {
        self.card.is_rectangular()
    }
}
