use super::card::{child_fun_unchecked, Card};
use crate::{cardinality::card::panic_on_all_when_udd, Dim, NVec};
use core::marker::PhantomData;

/// Unbounded cardinality.
///
/// Practically this means that the cardinality of any child dimension of the vector
/// is equal to `usize::MAX`.
///
/// Whenever the vector is unbounded, it is capable of producing the corresponding element
/// for any possible index.
///
/// The following vectors are naturally unbounded on construction:
/// * functional vectors which can be created by [`fun`],
/// * sparse vectors that can be created by [`sparse`] or [`sparse_from`],
/// * constant vectors that can be created by [`constant`].
///
/// The methods `all` and `enumerate_all` panic for vectors with unbounded cardinality,
/// as it would lead to an infinite loop (practically).
///
/// In order to iterate over flattened scalar elements of an unbounded vector, one can use
/// the `all_in` iterator instead, where the indices to iterate over are provided as an
/// input.
///
/// Alternatively, the unbounded domain of the vector can be transformed into a bounded one
/// by using:
/// * `bounded` method if the vector is of dimension `D1`,
/// * `with_rectangular_bounds` or `with_variable_bounds` methods otherwise.
///
/// [`fun`]: crate::v::NewV1::fun
/// [`sparse`]: crate::v::NewV1::sparse
/// [`sparse_from`]: crate::v::NewV1::sparse_from
/// [`constant`]: crate::v::NewV1::constant
#[derive(Clone, Copy)]
pub struct UnboundedCard<D: Dim> {
    phantom: PhantomData<D>,
}

impl<D: Dim> Default for UnboundedCard<D> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<D: Dim> Card<D> for UnboundedCard<D> {
    fn is_rectangular(&self) -> bool {
        false
    }

    #[inline(always)]
    fn cardinality_of(&self, _: impl Into<<D as Dim>::CardIdx>) -> usize {
        usize::MAX
    }

    fn child_card(&self, _: <D as Dim>::ChildIdx) -> impl Card<<D as Dim>::PrevDim> {
        UnboundedCard::<D::PrevDim>::default()
    }

    fn child_fun<T, F>(
        &self,
        i: <D as Dim>::ChildIdx,
        fun: F,
    ) -> impl Fn(<<D as Dim>::PrevDim as Dim>::Idx) -> T
    where
        F: Fn(<D as Dim>::Idx) -> T,
    {
        child_fun_unchecked::<D, _, _>(i, fun)
    }

    fn vec_all<'a, T, N>(&'a self, _: &'a N) -> impl Iterator<Item = T>
    where
        N: NVec<D, T> + 'a,
    {
        panic_on_all_when_udd(true);
        core::iter::empty()
    }

    fn vec_enumerate_all<'a, T, N>(&'a self, _: &'a N) -> impl Iterator<Item = (<D as Dim>::Idx, T)>
    where
        N: NVec<D, T> + 'a,
    {
        panic_on_all_when_udd(true);
        core::iter::empty()
    }
}
