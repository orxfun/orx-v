use crate::{dim::*, Card, FunVec, NVec};

impl<D, T, F, C> NVec<D, T> for FunVec<D, T, F, C>
where
    D: Dim,
    F: Fn(<D as Dim>::Idx) -> T,
    C: Card<D>,
{
    #[inline(always)]
    fn at(&self, idx: impl IntoIdx<D>) -> T {
        (self.fun)(idx.into_idx())
    }

    fn child(&self, i: <D as Dim>::ChildIdx) -> impl NVec<<D as Dim>::PrevDim, T> {
        let i: usize = i.into();
        let fun = self.card.child_fun(i.into(), &self.fun);
        let card = self.card.child_card(i.into());
        FunVec::new(fun, card)
    }

    fn all(&self) -> impl Iterator<Item = T> {
        self.card.vec_all(self)
    }
}
