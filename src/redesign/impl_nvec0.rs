use super::{D0, Dim, NVec};

impl<'a, T> NVec<D0, &'a T> for &'a T {
    #[inline(always)]
    fn at(&self, []: <D0 as Dim>::Idx) -> &'a T {
        self
    }
}
