use super::{Dim, NVec};
use core::marker::PhantomData;

pub struct Copied<'a, D, T, V>(V, PhantomData<&'a (D, T)>)
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T>;

impl<'a, D, T, V> Copied<'a, D, T, V>
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T>,
{
    pub(crate) fn new(v: V) -> Self {
        Self(v, PhantomData)
    }
}

impl<'a, D, T, V> Clone for Copied<'a, D, T, V>
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T> + Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<'a, D, T, V> Copy for Copied<'a, D, T, V>
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T> + Copy,
{
}

impl<'b, D, T, V> NVec<D, T> for Copied<'b, D, T, V>
where
    D: Dim,
    T: Copy + 'b,
    V: NVec<D, &'b T>,
{
    #[inline(always)]
    fn at<'r>(&'r self, idx: <D as Dim>::Idx) -> T
    where
        T: 'r,
    {
        self.0.at(idx).clone()
    }
}
