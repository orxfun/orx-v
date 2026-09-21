use super::super::{Dim, NVec};
use core::marker::PhantomData;
use derive_new::new;

#[derive(new)]
pub struct FunVal<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    f: F,
    p: PhantomData<D>,
}

impl<D, T, F> NVec<D, T> for FunVal<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        (self.f)(idx)
    }
}
