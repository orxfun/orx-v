use crate::{dimensions::dim::*, FromIndex, NVecSized};
use alloc::collections::btree_map::BTreeMap;

// full-indexed

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<I, T: Copy> NVecSized<$dim, T> for BTreeMap<I, T>
        where
            I: FromIndex<$dim>,
        {
            #[inline(always)]
            fn cardinality(&self) -> usize {
                self.len()
            }

            fn elements(&self) -> impl Iterator<Item = T> {
                self.values().copied()
            }
        }
    };
}

implement_idx!(D1);
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
        impl<A, T: Copy> NVecSized<$dim, T> for BTreeMap<usize, A>
        where
            A: NVecSized<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn cardinality(&self) -> usize {
                self.values().map(|x| x.cardinality()).sum()
            }

            fn elements(&self) -> impl Iterator<Item = T> {
                self.values().flat_map(|x| x.elements())
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
