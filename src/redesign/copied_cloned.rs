use super::{Dim, NVec};
use core::marker::PhantomData;

// copied

pub struct Copied<'a, D, T, V>(V, PhantomData<&'a (D, T)>)
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T>;

impl<'a, D, T, V> NVec<D, T> for Copied<'a, D, T, V>
where
    D: Dim,
    T: Copy + 'a,
    V: NVec<D, &'a T>,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        *self.0.at(idx)
    }
}

// cloned

pub struct Cloned<'a, D, T, V>(V, PhantomData<&'a (D, T)>)
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T>;

impl<'a, D, T, V> NVec<D, T> for Cloned<'a, D, T, V>
where
    D: Dim,
    T: Clone + 'a,
    V: NVec<D, &'a T>,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        self.0.at(idx).clone()
    }
}
