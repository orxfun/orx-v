use crate::{dimensions::*, failures::OUT_OF_BOUNDS, NVec};
use alloc::collections::btree_map::BTreeMap;

impl<'v, E, T> NVec<D2, &'v T> for &'v BTreeMap<usize, E>
where
    &'v E: NVec<D1, &'v T>,
{
    fn at<Idx: IntoIndex<D2>>(&self, index: Idx) -> &'v T {
        let (i, index) = index.split();
        self.get(&i).expect(OUT_OF_BOUNDS).at(index)
    }

    fn try_at<Idx: IntoIndex<D2>>(&self, index: Idx) -> Option<&'v T> {
        let (i, index) = index.split();
        self.get(&i).and_then(|x| x.try_at(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoCopied, IntoFilled};
    use alloc::vec::Vec;

    #[test]
    fn d2_map_map() {
        let arr = [(0, [(0, 1)]), (7, [(3, 2)]), (9, [(6, 3)])];
        let map: BTreeMap<usize, BTreeMap<usize, i32>> = arr
            .into_iter()
            .map(|x| (x.0, x.1.into_iter().collect()))
            .collect();
        let nvec = &map;

        assert_eq!(nvec.at([0, 0]), &1);
        assert_eq!(nvec.try_at((7, 3)), Some(&2));
        assert_eq!(nvec.try_at([1, 0]), None);

        assert_eq!(nvec.at(0).at(0), &1);
        assert_eq!(nvec.try_at(7).and_then(|x| x.try_at(3)), Some(&2));
        assert_eq!(nvec.try_at(1).map(|x| x.at(0)), None);

        let nvec = map.filled_with(&42);
        assert_eq!(nvec.at([0, 0]), &1);
        assert_eq!(nvec.try_at((7, 3)), Some(&2));
        assert_eq!(nvec.try_at([1, 0]), Some(&42));
        assert_eq!(nvec.at([1, 0]), &42);

        let nvec = map.copied();
        assert_eq!(nvec.at([0, 0]), 1);
        assert_eq!(nvec.try_at((7, 3)), Some(2));
        assert_eq!(nvec.try_at([1, 0]), None);
    }

    #[test]
    fn d2_map_vec() {
        let arr = [(0, [0, 1]), (7, [3, 2]), (9, [6, 3])];
        let map: BTreeMap<usize, Vec<i32>> = arr
            .into_iter()
            .map(|x| (x.0, x.1.into_iter().collect()))
            .collect();

        let nvec = &map;

        assert_eq!(nvec.at([0, 1]), &1);
        assert_eq!(nvec.try_at([7, 0]), Some(&3));
        assert_eq!(nvec.try_at([1, 0]), None);
        assert_eq!(nvec.try_at([0, 2]), None);

        let nvec = nvec.copied();
        assert_eq!(nvec.at([0, 1]), 1);
        assert_eq!(nvec.try_at([7, 0]), Some(3));
        assert_eq!(nvec.try_at([1, 0]), None);
        assert_eq!(nvec.try_at([0, 2]), None);

        let nvec = map.filled_with(&42).copied();
        assert_eq!(nvec.at([0, 1]), 1);
        assert_eq!(nvec.try_at([7, 0]), Some(3));
        assert_eq!(nvec.try_at([1, 0]), Some(42));
        assert_eq!(nvec.at([0, 2]), 42);
    }
}
