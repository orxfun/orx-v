use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVecRef};
use alloc::collections::btree_map::BTreeMap;

impl<K, T> NVecRef<D1> for BTreeMap<K, T>
where
    K: FromIndex<D1>,
{
    type Element<'e> = Option<&'e T> where Self: 'e;

    fn ref_at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Self::Element<'_> {
        let index = K::from_index(index.into_index());
        self.get(&index)
    }
}

// full-indexed

impl<K, T> NVecRef<D2> for BTreeMap<K, T>
where
    K: FromIndex<D2>,
{
    type Element<'e> = Option<&'e T> where Self: 'e;

    fn ref_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Self::Element<'_> {
        let index = K::from_index(index.into_index());
        self.get(&index)
    }
}

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<K, T> NVecRef<$dim> for BTreeMap<K, T>
        where
            K: FromIndex<$dim>,
        {
            type Element<'e> = Option<&'e T> where Self: 'e;

            fn ref_at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Self::Element<'_> {
                let index = K::from_index(index.into_index());
                self.get(&index)
            }
        }
    };
}

implement_idx!(D3);
implement_idx!(D4);
implement_idx!(D5);
implement_idx!(D6);
implement_idx!(D7);
implement_idx!(D8);

// recursive

impl<V> NVecRef<D2> for BTreeMap<usize, V>
where
    V: NVecRef<<D2 as Dim>::PREVIOUS>,
{
    type Element<'e> = Option<V::Element<'e>> where Self: 'e;

    fn ref_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Self::Element<'_> {
        let (i, index) = index.split();
        self.get(&i).map(|x| x.ref_at(index))
    }
}
