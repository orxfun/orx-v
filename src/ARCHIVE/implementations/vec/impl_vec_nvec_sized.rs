use crate::{dimensions::dim::*, NVecSized};
use alloc::vec::Vec;

impl<T: Copy> NVecSized<D1, T> for Vec<T> {
    #[inline(always)]
    fn cardinality(&self) -> usize {
        self.len()
    }

    fn elements(&self) -> impl Iterator<Item = T> {
        self.iter().copied()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<T: Copy, A> NVecSized<$dim, T> for Vec<A>
        where
            A: NVecSized<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn cardinality(&self) -> usize {
                self.iter().map(|x| x.cardinality()).sum()
            }

            fn elements(&self) -> impl Iterator<Item = T> {
                self.iter().flat_map(|x| x.elements())
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
