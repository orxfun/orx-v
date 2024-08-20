use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVec, NVecRec};
use alloc::collections::btree_map::BTreeMap;

impl<I, T: Copy> NVecRec<D1, T> for BTreeMap<I, T>
where
    I: FromIndex<D1>,
{
    type Child = T;

    #[inline(always)]
    fn num_children(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn child<Idx1: IntoIndex<D1>>(&self, index: Idx1) -> Option<&Self::Child> {
        let index = I::from_index(index.into_index());
        self.get(&index)
    }

    fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
    where
        Self::Child: 'c,
    {
        self.values()
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<A, T: Copy> NVecRec<$dim, T> for BTreeMap<usize, A>
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
                self.get(&index.into_index()[0])
            }

            fn children<'c>(&'c self) -> impl Iterator<Item = &Self::Child>
            where
                Self::Child: 'c,
            {
                self.values()
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
