use crate::{dimensions::dim::*, nvec_rec::NVecRec, IntoIndex, NVec};
use alloc::vec::Vec;

impl<T: Copy> NVecRec<D1, T> for Vec<T> {
    type Child = T;

    #[inline(always)]
    fn num_children(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
        self.get(index.into_index()[0])
    }

    fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
    where
        Self::Child: 'c,
    {
        self.iter()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<T: Copy, A> NVecRec<$dim, T> for Vec<A>
        where
            A: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            type Child = A;

            #[inline(always)]
            fn num_children(&self) -> usize {
                self.len()
            }

            #[inline(always)]
            fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
                self.get(index.into_index()[0])
            }

            fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
            where
                Self::Child: 'c,
            {
                self.iter()
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
