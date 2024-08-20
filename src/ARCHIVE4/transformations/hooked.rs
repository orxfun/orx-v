use crate::{Dim, IntoIndex, NVec};
use core::marker::PhantomData;

pub struct Hooked<N, T, V, H>
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, Option<&T>),
{
    inner: V,
    hook: H,
    phantom: PhantomData<(N, T)>,
}

impl<N, T, V, H> NVec<N, T> for Hooked<N, T, V, H>
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, Option<&T>),
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let idx = index.into_index();
        let value = self.inner.at(index);
        (self.hook)(idx, Some(&value));
        value
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        let idx = index.into_index();
        let maybe = self.inner.try_at(index);
        (self.hook)(idx, maybe.as_ref());
        maybe
    }
}

// into

pub trait IntoHooked<N, T, H>
where
    N: Dim,
    Self: NVec<N, T> + Sized,
    H: Fn(N::Idx, Option<&T>),
{
    fn hooked(self, hook: H) -> Hooked<N, T, Self, H>;
}

impl<N, T, V, H> IntoHooked<N, T, H> for V
where
    N: Dim,
    V: NVec<N, T>,
    H: Fn(N::Idx, Option<&T>),
{
    fn hooked(self, hook: H) -> Hooked<N, T, V, H> {
        Hooked {
            inner: self,
            hook,
            phantom: Default::default(),
        }
    }
}
