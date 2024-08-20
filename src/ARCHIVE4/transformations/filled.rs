use crate::{CopyOrRef, Dim, IntoIndex, NVec};
use core::marker::PhantomData;

pub struct Filled<'v, N, T, V>
where
    N: Dim,
    V: NVec<N, T>,
    T: CopyOrRef<'v, T>,
{
    inner: V,
    fill_with: T,
    phantom: PhantomData<&'v N>,
}

impl<'v, N, T, V> NVec<N, T> for Filled<'v, N, T, V>
where
    N: Dim,
    V: NVec<N, T>,
    T: CopyOrRef<'v, T>,
{
    #[inline]
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        self.inner.try_at(index).unwrap_or(self.fill_with)
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        Some(self.at(index))
    }
}

// into

pub trait IntoFilled<'v, N, T>
where
    N: Dim + 'v,
    Self: NVec<N, T> + Sized,
    T: CopyOrRef<'v, T>,
{
    fn filled_with(self, fill_with: T) -> Filled<'v, N, T, Self>;
}

impl<'v, N, T, V> IntoFilled<'v, N, T> for V
where
    N: Dim + 'v,
    Self: NVec<N, T> + Sized,
    T: CopyOrRef<'v, T>,
{
    fn filled_with(self, fill_with: T) -> Filled<'v, N, T, V> {
        Filled {
            inner: self,
            fill_with,
            phantom: Default::default(),
        }
    }
}
