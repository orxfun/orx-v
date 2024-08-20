use crate::{Dim, IntoIndex, NVec, NVecMut};
use core::marker::PhantomData;

pub struct Completed<V, N, T>
where
    N: Dim,
{
    inner: V,
    complete_with: T,
    phantom: PhantomData<N>,
}

impl<V, N, T> Completed<V, N, T>
where
    N: Dim,
{
    pub(crate) fn new(inner: V, complete_with: T) -> Self {
        Self {
            inner,
            complete_with,
            phantom: Default::default(),
        }
    }
}

// nvecs

impl<V, N, T> NVec<N, T> for Completed<V, N, T>
where
    V: NVec<N, T>,
    N: Dim,
    T: Copy,
{
    #[inline]
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        self.inner.try_at(index).unwrap_or(self.complete_with)
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        Some(self.at(index))
    }
}

impl<V, N, T> NVecMut<N, T> for Completed<V, N, T>
where
    V: NVecMut<N, T>,
    N: Dim,
    T: Copy,
{
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) {
        self.inner.set(index, value)
    }
}

// into

// TODO: put a bound on Self, currently it appears on all types!!

pub trait IntoCompleted<N, T>
where
    N: Dim,
    Self: Sized,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, N, T>;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn nvec_and_mut() {
        let mut vec = vec![1];
        let mut completed = (&mut vec).into_completed(7);

        assert_eq!(completed.at(0), 1);
        assert_eq!(completed.at(3), 7);

        completed.set(0, 42);

        assert_eq!(completed.at(0), 42);
        assert_eq!(completed.at(3), 7);
    }
}
