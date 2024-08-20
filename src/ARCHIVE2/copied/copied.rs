use crate::{dimensions::dim::*, NVec};
use core::marker::PhantomData;

pub struct Copied<'v, N, V, C, F>
where
    N: Dim,
    V: NVec<N>,
    F: Fn(V::Element<'v>) -> C,
{
    pub(super) inner: &'v V,
    pub(super) copy: F,
    phantom: PhantomData<(N, C)>,
}

impl<'v, N, V, C, F> Copied<'v, N, V, C, F>
where
    N: Dim,
    V: NVec<N>,
    F: Fn(V::Element<'v>) -> C,
{
    pub(crate) fn new(inner: &'v V, copy: F) -> Self {
        Self {
            inner,
            copy,
            phantom: Default::default(),
        }
    }
}
