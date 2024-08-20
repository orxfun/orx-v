use crate::{dimensions::dim::*, IntoIndex, NVec};
use alloc::vec::Vec;

impl<T: Copy> NVec<D1, T> for Vec<T> {
    #[inline(always)]
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<T: Copy, A> NVec<$dim, T> for Vec<A>
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(i).and_then(|c| c.try_at(index))
            }
        }
    };
}

implement!(D2);
implement!(D3);
implement!(D4);
implement!(D5);
implement!(D6);
implement!(D7);
implement!(D8);

// impl<'a, T> NVec<D1, &'a Vec<T>> for Vec<Vec<T>> {
//     fn try_at<I: IntoIndex<D1>>(&self, index: I) -> Option<&'a Vec<T>> {
//         let index = index.into_index();
//         self.get(index.into_index())
//     }
// }
