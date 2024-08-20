use crate::{Dim, IntoIndex, NVec, NVecRef};
use core::marker::PhantomData;

pub struct Flattened<'v, N, V, T>
where
    N: Dim,
{
    inner: &'v V,
    phantom: PhantomData<(N, T)>,
}

impl<'v, N, V, T> NVec<N, Option<T>> for Flattened<'v, N, V, T>
where
    N: Dim,
    V: NVec<N, Option<Option<T>>>,
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        self.inner.at(index).flatten()
    }
}

impl<'v, N, V, T> NVecRef<N> for Flattened<'v, N, V, T>
where
    N: Dim,
    V: NVecRef<N, Element<'v> = Option<Option<&'v T>>>,
    T: 'v,
{
    type Element<'e> = Option<&'v T> where Self: 'e;

    fn ref_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Self::Element<'_> {
        self.inner.ref_at(index).flatten()
    }
}

// INTO

pub trait IntoFlattened<N, T>
where
    N: Dim,
    Self: Sized + NVec<N, Option<Option<T>>>,
{
    fn flattened(&self) -> Flattened<N, Self, T>;
}

impl<N, V, T> IntoFlattened<N, T> for V
where
    N: Dim,
    V: NVec<N, Option<Option<T>>>,
{
    fn flattened(&self) -> Flattened<N, Self, T> {
        Flattened {
            inner: self,
            phantom: Default::default(),
        }
    }
}
