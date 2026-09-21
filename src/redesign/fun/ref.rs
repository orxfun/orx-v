use super::super::{Dim, NVec, NVecRef};
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunRef<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    f: F,
    p: PhantomData<D>,
}

impl<'a, D, T, F> NVec<D, &'a T> for FunRef<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> &'a T {
        (self.f)(idx)
    }
}

impl<'a, D, T, F> NVecRef<D, T> for FunRef<'a, D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> &'a T,
    T: 'a,
{
    fn ref_at(&self, idx: <D as Dim>::Idx) -> &T {
        (self.f)(idx)
    }
}
