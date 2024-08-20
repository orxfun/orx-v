use crate::{dimensions::dim::*, NVec, NVecRec};
use alloc::{collections::BTreeMap, vec, vec::Vec};

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

// d2

fn d1_get_4<T, N: NVec<D1, T>>(nvec: &N) -> Option<T> {
    nvec.try_at(4)
}

fn d2_child_2<T, N: NVecRec<D2, T>>(nvec: &N) -> Option<&N::Child> {
    nvec.child(2)
}

fn d2_get_24<T, N: NVecRec<D2, T>>(nvec: &N) -> Option<T> {
    let child = d2_child_2(nvec);
    d1_get_4(child.unwrap())
}

#[test]
fn d2_nvec_btree_btree() {
    let arr_map = || {
        vec![
            (0, vec![(0, 1)]),
            (1, vec![]),
            (2, vec![(0, 2), (1, 5), (2, 1), (3, 3), (4, 42), (5, 7)]),
            (3, vec![(0, 4)]),
        ]
    };

    let map: BTreeMap<usize, BTreeMap<usize, _>> = arr_map()
        .into_iter()
        .map(|x| (x.0, pairs_to_map(x.1)))
        .collect();
    assert_eq!(d2_get_24(&map), Some(42));
}

#[test]
fn d2_nvec_btree_vec() {
    let arr_map = || {
        vec![
            (0, vec![(1)]),
            (1, vec![]),
            (2, vec![(2), (5), (1), (3), (42), (7)]),
            (3, vec![(4)]),
        ]
    };

    let map: BTreeMap<usize, Vec<_>> = arr_map().into_iter().map(|x| (x.0, x.1)).collect();
    assert_eq!(d2_get_24(&map), Some(42));
}
