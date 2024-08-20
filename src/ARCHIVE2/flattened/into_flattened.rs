use super::flattened::Flattened;
use crate::{Dim, NVec};

pub trait IntoFlattened<'v, N, C>
where
    N: Dim,
    Self: NVec<N> + Sized + 'v,
    Self::Element<'v>: 'v,
{
    fn flattened(&'v self) -> Flattened<'v, N, Self, C, impl Fn(Self::Element<'v>) -> C>;
}
