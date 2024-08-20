use crate::{Dim, IntoIndex, NVec};
use core::marker::PhantomData;

pub struct Cloned<'v, N, T, V>
where
    N: Dim,
    T: Clone + 'v,
    V: NVec<N, &'v T>,
{
    inner: V,
    phantom: PhantomData<&'v (N, T)>,
}

impl<'v, N, T, V> NVec<N, T> for Cloned<'v, N, T, V>
where
    N: Dim,
    T: Clone + 'v,
    V: NVec<N, &'v T>,
{
    #[inline]
    fn at<Idx: crate::IntoIndex<N>>(&self, index: Idx) -> T {
        self.inner.at(index).clone()
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        self.inner.try_at(index).cloned()
    }
}

// into

pub trait IntoCloned<'v, N, T>
where
    N: Dim + 'v,
    T: Clone + 'v,
    Self: Sized + NVec<N, &'v T>,
{
    fn cloned(self) -> Cloned<'v, N, T, Self>;
}

impl<'v, N, T, V> IntoCloned<'v, N, T> for V
where
    N: Dim + 'v,
    T: Clone + 'v,
    V: NVec<N, &'v T>,
{
    fn cloned(self) -> Cloned<'v, N, T, V> {
        Cloned {
            inner: self,
            phantom: Default::default(),
        }
    }
}
