use crate::{dimensions::dim::*, FromIndex, IntoIndex, NVec};
use alloc::collections::btree_map::BTreeMap;

impl<K, T: Copy> NVec<D1, Option<T>> for BTreeMap<K, T>
where
    K: FromIndex<D1>,
{
    fn at<Idx: IntoIndex<D1>>(&self, index: Idx) -> Option<T> {
        let index = K::from_index(index.into_index());
        self.get(&index).copied()
    }
}

// full-indexed

impl<K, T: Copy> NVec<D2, Option<T>> for BTreeMap<K, T>
where
    K: FromIndex<D2>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let index = K::from_index(index.into_index());
        self.get(&index).copied()
    }
}

macro_rules! implement_idx {
    ($dim:tt) => {
        impl<K, T: Copy> NVec<$dim, Option<T>> for BTreeMap<K, T>
        where
            K: FromIndex<$dim>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let index = K::from_index(index.into_index());
                self.get(&index).copied()
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

impl<V, T> NVec<D2, Option<T>> for BTreeMap<usize, V>
where
    V: NVec<<D2 as Dim>::PREVIOUS, T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<T> {
        let (i, index) = index.split();
        self.get(&i).map(|c| c.at(index))
    }
}

macro_rules! implement_recursive {
    ($dim:tt) => {
        impl<V, T> NVec<$dim, Option<T>> for BTreeMap<usize, V>
        where
            V: NVec<<$dim as Dim>::PREVIOUS, T>,
        {
            fn at<Idx: IntoIndex<$dim>>(&self, index: Idx) -> Option<T> {
                let (i, index) = index.split();
                self.get(&i).map(|c| c.at(index))
            }
        }
    };
}

implement_recursive!(D3);
implement_recursive!(D4);
implement_recursive!(D5);
implement_recursive!(D6);
implement_recursive!(D7);
implement_recursive!(D8);
