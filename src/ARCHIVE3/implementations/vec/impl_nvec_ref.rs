use crate::{dimensions::dim::*, IntoIndex, NVecRef};
use alloc::vec::Vec;

impl<T> NVecRef<D1> for Vec<T> {
    type Element<'e> = &'e T where Self: 'e;

    fn ref_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Self::Element<'_> {
        &self[index.into_index()[0]]
    }
}

macro_rules! implement {
    ($dim:tt) => {
        impl<V> NVecRef<$dim> for Vec<V>
        where
            V: NVecRef<<$dim as Dim>::PREVIOUS>,
        {
            type Element<'e> = V::Element<'e> where Self: 'e;

            fn ref_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Self::Element<'_> {
                let (i, index) = index.split();
                self[i].ref_at(index)
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
