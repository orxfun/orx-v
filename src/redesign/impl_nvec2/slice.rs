use super::super::{D1, D2, Dim, NVec};

impl<'a, T, V: NVec<D1, &'a T>> NVec<D2, &'a T> for &'a [V] {
    #[inline(always)]
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &'a T {
        self[i].at(j)
    }
}
