use crate::{dimensions::*, failures::OUT_OF_BOUNDS, Completed, IntoCompleted, NVec, NVecMut};
use alloc::collections::btree_map::BTreeMap;

impl<N, K, T> NVec<N, T> for BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
    T: Copy,
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let index = K::from_index(index.into_index());
        *self.get(&index).expect(OUT_OF_BOUNDS)
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        let index = K::from_index(index.into_index());
        self.get(&index).copied()
    }
}

impl<N, K, T> NVecMut<N, T> for BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
    T: Copy,
{
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) {
        let index = K::from_index(index.into_index());
        match self.get_mut(&index) {
            Some(x) => *x = value,
            None => _ = self.insert(index, value),
        }
    }
}

impl<N, K, T> IntoCompleted<N, T> for BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, N, T> {
        Completed::new(self, complete_with)
    }
}

// &

impl<N, K, T> NVec<N, T> for &BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
    T: Copy,
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let index = K::from_index(index.into_index());
        *self.get(&index).expect(OUT_OF_BOUNDS)
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        let index = K::from_index(index.into_index());
        self.get(&index).copied()
    }
}

impl<N, K, T> IntoCompleted<N, T> for &BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, N, T> {
        Completed::new(self, complete_with)
    }
}

// &mut

impl<N, K, T> NVec<N, T> for &mut BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
    T: Copy,
{
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> T {
        let index = K::from_index(index.into_index());
        *self.get(&index).expect(OUT_OF_BOUNDS)
    }

    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<T> {
        let index = K::from_index(index.into_index());
        self.get(&index).copied()
    }
}

impl<N, K, T> NVecMut<N, T> for &mut BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
    T: Copy,
{
    fn set<Idx: IntoIndex<N>>(&mut self, index: Idx, value: T) {
        let index = K::from_index(index.into_index());
        match self.get_mut(&index) {
            Some(x) => *x = value,
            None => _ = self.insert(index, value),
        }
    }
}

impl<N, K, T> IntoCompleted<N, T> for &mut BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
{
    fn into_completed(self, complete_with: T) -> Completed<Self, N, T> {
        Completed::new(self, complete_with)
    }
}
