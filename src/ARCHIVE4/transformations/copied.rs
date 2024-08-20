use crate::{Dim, IntoIndex, NVec};
use core::marker::PhantomData;

pub struct Copied<'v, N, T, V>
where
    N: Dim,
    T: Copy + 'v,
    V: NVec<N, &'v T>,
{
    inner: V,
    phantom: PhantomData<&'v (N, T)>,
}

impl<'v, N, T, V> NVec<N, T> for Copied<'v, N, T, V>
where
    N: Dim,
    T: Copy + 'v,
    V: NVec<N, &'v T>,
{
    #[inline]
    fn at<Idx: crate::IntoIndex<N>>(&self, index: Idx) -> T {
        *self.inner.at(index)
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        self.inner.try_at(index).copied()
    }
}

// into

pub trait IntoCopied<'v, N, T>
where
    N: Dim + 'v,
    T: Copy + 'v,
    Self: Sized + NVec<N, &'v T>,
{
    fn copied(self) -> Copied<'v, N, T, Self>;
}

impl<'v, N, T, V> IntoCopied<'v, N, T> for V
where
    N: Dim + 'v,
    T: Copy + 'v,
    V: NVec<N, &'v T>,
{
    fn copied(self) -> Copied<'v, N, T, V> {
        Copied {
            inner: self,
            phantom: Default::default(),
        }
    }
}
