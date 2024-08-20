use crate::{dimensions::dim::*, IntoIndex, NVec};

impl<'c, T: Copy> NVec<D1, T> for &'c [T] {
    #[inline(always)]
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

impl<'c, T: Copy> NVec<D1, T> for &'c mut [T] {
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        self.get(index.into_index()[0]).copied()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<'c, T: Copy, A> NVec<$dim, T> for &'c [A]
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(i).and_then(|c| c.try_at(index))
            }
        }

        impl<'c, T: Copy, A> NVec<$dim, T> for &'c mut [A]
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

// impl<'a, T> NVec<D1, &'a Vec<T>> for Vec<&'a Vec<T>> {
//     fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<&'a Vec<T>> {
//         self.get(3).copied()
//     }
// }

// std::iter::Copied<std::slice::Iter<T>>
