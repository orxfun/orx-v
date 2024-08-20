use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVec};
use alloc::collections::btree_map::BTreeMap;

impl<I, T: Copy> NVec<D1, T> for BTreeMap<I, T>
where
    I: FromIndex<D1>,
{
    #[inline(always)]
    fn try_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        let index = I::from_index(index.into_index());
        self.get(&index).copied()
    }
}

// full-indexed

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<I, T: Copy> NVec<$dim, T> for BTreeMap<I, T>
        where
            I: FromIndex<$dim>,
        {
            #[inline(always)]
            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let index = I::from_index(index.into_index());
                self.get(&index).copied()
            }
        }
    };
}

implement_idx!(D2);
implement_idx!(D3);
implement_idx!(D4);
implement_idx!(D5);
implement_idx!(D6);
implement_idx!(D7);
implement_idx!(D8);

// recursive

macro_rules! implement_rec {
    ($dim:tt) => {
        impl<A, T: Copy> NVec<$dim, T> for BTreeMap<usize, A>
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn try_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(&i).and_then(|c| c.try_at(index))
            }
        }
    };
}

implement_rec!(D2);
implement_rec!(D3);
implement_rec!(D4);
implement_rec!(D5);
implement_rec!(D6);
implement_rec!(D7);
implement_rec!(D8);
