use crate::{dimensions::dim::*, IntoIndex, NVec};

impl<T> NVec<D0, T> for T {
    #[inline(always)]
    fn try_at<Idx: IntoIndex<D0>>(&self, _: Idx) -> Option<T> {
        // TODO!
        panic!("NEVER HERE")
    }
}
