use super::card::{child_fun_unchecked, Card};
use crate::{CardIdx, Dim, NVec};
use core::{fmt::Debug, marker::PhantomData};

/// Empty cardinality.
///
/// Practically this means that the cardinality of the child dimension, and hence, the
/// cardinality of all lower dimension children, of the vector is equal to 0.
///
/// Whenever the vector has zero cardinality, it is not capable of producing any valid
/// value; i.e., there exists no `in_bounds` index.
///
/// This cardinality is useful in providing a shorthand to create empty or default vectors
/// of any type.
#[derive(Clone, Copy)]
pub struct EmptyCard<D: Dim> {
    phantom: PhantomData<D>,
}

impl<D: Dim> Default for EmptyCard<D> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<D: Dim> EmptyCard<D> {
    #[allow(clippy::panic)]
    pub(crate) fn panic_oob<Idx: Debug>(idx: Idx) -> ! {
        panic!(
            "Out of bounds. \
            Cardinality at index {:?} is requested from a vec with EmptyCard.",
            idx
        )
    }
}

impl<D: Dim> Card<D> for EmptyCard<D> {
    fn is_rectangular(&self) -> bool {
        true
    }

    fn cardinality_of(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        let idx = idx.into();
        match CardIdx::<D>::is_d0(&idx) {
            true => 0,
            false => Self::panic_oob(idx),
        }
    }

    #[allow(unreachable_code)]
    fn child_card(&self, i: <D as Dim>::ChildIdx) -> impl Card<<D as Dim>::PrevDim> {
        Self::panic_oob(i);
        EmptyCard::<D::PrevDim>::default()
    }

    #[allow(unreachable_code)]
    fn child_fun<T, F>(
        &self,
        i: <D as Dim>::ChildIdx,
        _fun: F,
    ) -> impl Fn(<<D as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D as Dim>::Idx) -> T,
    {
        Self::panic_oob(i);
        child_fun_unchecked::<D, _, _>(i, _fun)
    }

    fn vec_all<'a, T, N>(&'a self, _: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D, T> + 'a,
    {
        core::iter::empty()
    }

    fn vec_enumerate_all<'a, T, N>(&'a self, _: &'a N) -> impl Iterator<Item = (<D as Dim>::Idx, T)>
    where
        N: NVec<D, T> + 'a,
    {
        core::iter::empty()
    }
}
