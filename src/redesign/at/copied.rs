use super::super::Dim;
use super::At;
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    v: V,
    p: PhantomData<fn() -> (D, &'a T)>,
}

impl<'a, D, T, V: Clone> Clone for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, V: Copy> Copy for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
}

impl<'a, D, T, V> At<D, T> for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        *self.v.at(idx)
    }
}
