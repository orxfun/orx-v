use crate::{dimensions::dim::*, IntoIndex, NVec};
use alloc::vec::Vec;

impl<T: Copy> NVec<D1, T> for Vec<T> {
    #[inline]
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> T {
        self[index.into_index()[0]]
    }
}

impl<V, T> NVec<D2, T> for Vec<V>
where
    V: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> T {
        let (i, index) = index.split();
        self[i].at(index)
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<V, T> NVec<$dim, T> for Vec<V>
        where
            V: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> T {
                let (i, index) = index.split();
                self[i].at(index)
            }
        }
    };
}

implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);
