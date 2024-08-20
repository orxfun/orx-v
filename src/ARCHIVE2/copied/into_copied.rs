use super::copied::Copied;
use crate::{Dim, NVec};

pub trait IntoCopied<'v, N, C>
where
    N: Dim,
    Self: NVec<N> + Sized + 'v,
    Self::Element<'v>: 'v,
{
    fn copied(&'v self) -> Copied<'v, N, Self, C, impl Fn(Self::Element<'v>) -> C>;
}
