use super::{Dim, NVec};
use core::marker::PhantomData;

pub struct Cloned<'a, D, T, V>(V, PhantomData<&'a (D, T)>)
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T>;

impl<'a, D, T, V> Cloned<'a, D, T, V>
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T>,
{
    pub(crate) fn new(v: V) -> Self {
        Self(v, PhantomData)
    }
}

impl<'a, D, T, V> Clone for Cloned<'a, D, T, V>
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T> + Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<'a, D, T, V> Copy for Cloned<'a, D, T, V>
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T> + Copy,
{
}

impl<'a, D, T, V> NVec<D, T> for Cloned<'a, D, T, V>
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T>,
{
    #[inline(always)]
    fn at<'b>(&'b self, idx: <D as Dim>::Idx) -> T
    where
        T: 'b,
    {
        self.0.at(idx).clone()
    }
}
