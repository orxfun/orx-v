use crate::{dimensions::dim::*, NVec};
use core::marker::PhantomData;

pub struct Flattened<'v, N, V, T, F>
where
    N: Dim,
    V: NVec<N>,
    F: Fn(V::Element<'v>) -> T,
{
    pub(super) inner: &'v V,
    pub(super) unwrap: F,
    phantom: PhantomData<(N, T)>,
}

impl<'v, N, V, T, F> Flattened<'v, N, V, T, F>
where
    N: Dim,
    V: NVec<N>,
    F: Fn(V::Element<'v>) -> T,
{
    pub(crate) fn new(inner: &'v V, unwrap: F) -> Self {
        Self {
            inner,
            unwrap,
            phantom: Default::default(),
        }
    }
}
