use crate::{dimensions::dim::*, NVec};
use alloc::{collections::BTreeMap, string::ToString, vec, vec::Vec};

// d1

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

fn pairs_to_map_of_refs<T>(pairs: &[(usize, T)]) -> BTreeMap<usize, &T> {
    pairs.iter().map(|x| (x.0, &x.1)).collect()
}

fn d1_get_1<T, N: NVec<D1, T>>(nvec: &N) -> Option<T> {
    nvec.try_at(1)
}

#[test]
fn d1_nvec_btree() {
    let arr_map = || vec![(0, 1), (1, 42), (3, 13)];

    let map = pairs_to_map(arr_map());
    assert_eq!(d1_get_1(&map), Some(42));

    let arr_map: Vec<_> = arr_map()
        .into_iter()
        .map(|x| (x.0, x.1.to_string()))
        .collect();
    let map_refs = pairs_to_map_of_refs(&arr_map);
    assert_eq!(d1_get_1(&map_refs), Some(&42.to_string()));
}

// d2

fn d2_get_24<T, N: NVec<D2, T>>(nvec: &N) -> Option<T> {
    nvec.try_at([2, 4])
}

#[test]
fn d2_nvec_btree() {
    let arr_map = || {
        vec![
            (0, vec![(0, 1)]),
            (1, vec![]),
            (2, vec![(0, 2), (1, 5), (2, 1), (3, 3), (4, 42), (5, 7)]),
            (3, vec![(0, 4)]),
        ]
    };

    let map: BTreeMap<(usize, usize), _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ((i, j), val)))
        .collect();
    assert_eq!(d2_get_24(&map), Some(42));

    let map: BTreeMap<[usize; 2], _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ([i, j], val)))
        .collect();
    assert_eq!(d2_get_24(&map), Some(42));
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
