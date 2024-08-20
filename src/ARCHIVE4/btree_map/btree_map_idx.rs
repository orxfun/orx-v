use crate::{dimensions::*, failures::OUT_OF_BOUNDS, NVec};
use alloc::collections::btree_map::BTreeMap;

impl<'v, N, K, T> NVec<N, &'v T> for &'v BTreeMap<K, T>
where
    N: Dim,
    K: FromIndex<N>,
{
    #[inline]
    fn at<Idx: IntoIndex<N>>(&self, index: Idx) -> &'v T {
        let index = K::from_index(index.into_index());
        self.get(&index).expect(OUT_OF_BOUNDS)
    }

    #[inline]
    fn try_at<Idx: IntoIndex<N>>(&self, index: Idx) -> Option<&'v T> {
        let index = K::from_index(index.into_index());
        self.get(&index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoCopied, IntoFilled};

    #[test]
    fn d1() {
        let arr = [(0, 1), (7, 2), (9, 3)];
        let map: BTreeMap<usize, i32> = arr.into_iter().collect();
        let nvec = &map;

        assert_eq!(nvec.at(0), &1);
        assert_eq!(nvec.try_at(7), Some(&2));
        assert_eq!(nvec.try_at(1), None);

        let nvec = map.filled_with(&42);
        assert_eq!(nvec.at(0), &1);
        assert_eq!(nvec.try_at(7), Some(&2));
        assert_eq!(nvec.try_at(1), Some(&42));
        assert_eq!(nvec.at(1), &42);

        let nvec = map.copied();
        assert_eq!(nvec.at(0), 1);
        assert_eq!(nvec.try_at(7), Some(2));
        assert_eq!(nvec.try_at(1), None);

        let nvec = map.copied().filled_with(42);
        assert_eq!(nvec.at(0), 1);
        assert_eq!(nvec.try_at(7), Some(2));
        assert_eq!(nvec.at(1), 42);
    }

    #[test]
    fn d2() {
        let arr = [((0, 0), 1), ((7, 3), 2), ((9, 6), 3)];
        let map: BTreeMap<(usize, usize), i32> = arr.into_iter().collect();
        let nvec = &map;

        assert_eq!(nvec.at([0, 0]), &1);
        assert_eq!(nvec.try_at((7, 3)), Some(&2));
        assert_eq!(nvec.try_at([1, 0]), None);

        let nvec = nvec.copied();
        assert_eq!(nvec.at([0, 0]), 1);
        assert_eq!(nvec.try_at((7, 3)), Some(2));
        assert_eq!(nvec.try_at([1, 0]), None);

        let nvec = map.filled_with(&42);
        assert_eq!(nvec.try_at((7, 3)), Some(&2));
        assert_eq!(nvec.try_at([1, 0]), Some(&42));

        let nvec = map.filled_with(&42).copied();
        assert_eq!(nvec.try_at((7, 3)), Some(2));
        assert_eq!(nvec.at([1, 0]), 42);
    }
}
