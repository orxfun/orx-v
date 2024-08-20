use crate::{dimensions::dim::*, IntoIndex, NVecMut, NVecRecMut};
use alloc::collections::btree_map::BTreeMap;

macro_rules! implement {
    ($dim:tt) => {
        impl<A, T: Copy> NVecRecMut<$dim, T> for BTreeMap<usize, A>
        where
            A: NVecMut<<$dim as Dim>::PREVIOUS, T>,
        {
            #[inline(always)]
            fn child_mut<Idx1: IntoIndex<D1>>(&mut self, index: Idx1) -> Option<&mut Self::Child> {
                self.get_mut(&index.into_index()[0])
            }

            fn children_mut<'c>(&'c mut self) -> impl Iterator<Item = &mut Self::Child>
            where
                Self::Child: 'c,
            {
                self.values_mut()
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
