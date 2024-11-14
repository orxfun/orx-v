use crate::{Dim, NVecCore, NVecCoreSealed};
use core::marker::PhantomData;

pub struct VRef<D, V>
where
    D: Dim,
    V: NVecCore<D, T>,
{
    pub(super) inner: V,
    phantom: PhantomData<D>,
}

impl<D, V> From<V> for VRef<D, V>
where
    D: Dim,
    V: NVecCore<D, T>,
{
    fn from(inner: V) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<D, V> VRef<D, V>
where
    D: Dim,
    V: NVecCore<D, T>,
{
    pub fn into_inner(self) -> V {
        self.inner
    }
}

// core

impl<D, V, T> NVecCoreSealed<D, T> for VRef<D, V, T>
where
    D: Dim,
    V: NVecCore<D, T>,
{
    fn core_num_children(&self) -> usize {
        self.inner.core_num_children()
    }

    fn cardinality(&self, idx: impl Into<<D as Dim>::CardIdx>) -> usize {
        self.inner.cardinality(idx)
    }

    fn card_child(&self, i: <D as Dim>::ChildIdx) -> impl NVecCoreSealed<<D as Dim>::PrevDim> {
        self.inner.card_child(i)
    }
}
