use crate::{CardIdx, Dim, EmptyCard, IntoIdx, NVec, NVecCoreSealed, NVecMut};
use core::fmt::Debug;
use core::marker::PhantomData;

/// An empty vector of dimension `D` with no elements.
#[derive(Clone, Copy)]
pub struct EmptyVec<D: Dim, T> {
    phantom: PhantomData<(D, T)>,
}

impl<D: Dim, T> Default for EmptyVec<D, T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<D: Dim, T> Debug for EmptyVec<D, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ kind: EmptyVec, dim: D{}, values: [] }}",
            D::dimension()
        )
    }
}

impl<D: Dim, T> NVecCoreSealed<D, T> for EmptyVec<D, T> {
    fn core_num_children(&self) -> usize {
        0
    }

    fn core_card(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        let idx = idx.into();
        match CardIdx::<D>::is_d0(&idx) {
            true => 0,
            false => EmptyCard::<D>::panic_oob(idx),
        }
    }

    #[allow(unreachable_code)]
    fn core_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim, T> {
        EmptyCard::<D>::panic_oob(i);
        EmptyVec::<_, T>::default()
    }

    fn core_map<F: FnMut(&T) -> O, O>(&self, idx: impl IntoIdx<D>, _: &mut F) -> O {
        EmptyCard::<D>::panic_oob(idx)
    }

    fn core_is_rectangular(&self) -> bool {
        true
    }
}

impl<D: Dim, T> NVec<D, T> for EmptyVec<D, T> {
    fn at(&self, idx: impl IntoIdx<D>) -> T {
        EmptyCard::<D>::panic_oob(idx)
    }

    #[allow(unreachable_code)]
    fn child(&self, i: <D as Dim>::ChildIdx) -> impl NVec<<D as Dim>::PrevDim, T> {
        EmptyCard::<D>::panic_oob(i);
        EmptyVec::default()
    }

    fn all(&self) -> impl Iterator<Item = T> {
        core::iter::empty()
    }
}

impl<D: Dim, T> NVecMut<D, T> for EmptyVec<D, T> {
    fn at_mut<Idx: IntoIdx<D>>(&mut self, idx: Idx) -> &mut T {
        EmptyCard::<D>::panic_oob(idx)
    }

    fn set<Idx: IntoIdx<D>>(&mut self, idx: Idx, _: T) {
        EmptyCard::<D>::panic_oob(idx)
    }

    #[allow(unreachable_code)]
    fn child_mut(&mut self, i: <D as Dim>::ChildIdx) -> impl NVecMut<<D as Dim>::PrevDim, T> {
        EmptyCard::<D>::panic_oob(i);
        EmptyVec::default()
    }

    fn mut_all<F>(&mut self, _: F)
    where
        F: FnMut(&mut T),
    {
    }

    fn reset_all(&mut self, _: T)
    where
        T: PartialEq + Copy,
    {
    }
}
