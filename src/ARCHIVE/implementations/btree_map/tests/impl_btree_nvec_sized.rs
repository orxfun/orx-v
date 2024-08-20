use crate::{dimensions::dim::*, NVecSized};
use alloc::{collections::BTreeMap, vec, vec::Vec};
use core::fmt::Debug;

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

// d1

fn d1_test<T: Debug + Eq, N: NVecSized<D1, T>>(nvec: &N, expected: &[T]) {
    let vec: Vec<_> = nvec.elements().collect();
    assert_eq!(nvec.cardinality(), expected.len());
    assert_eq!(vec, expected);
}

#[test]
fn d1_nvec_btree() {
    fn arr_map() -> Vec<(usize, i32)> {
        vec![(0, 1), (1, 42), (3, 13)]
    }

    let map = pairs_to_map(arr_map());
    d1_test(
        &map,
        &arr_map().into_iter().map(|x| x.1).collect::<Vec<_>>(),
    );
}

// d2

fn d2_test<T: Debug + Eq, N: NVecSized<D2, T>>(nvec: &N, expected: &[T]) {
    let vec: Vec<_> = nvec.elements().collect();
    assert_eq!(nvec.cardinality(), expected.len());
    assert_eq!(vec, expected);
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

    let expected: Vec<_> = arr_map()
        .into_iter()
        .flat_map(|x| x.1.into_iter().map(|y| y.1))
        .collect();

    let map: BTreeMap<(usize, usize), _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ((i, j), val)))
        .collect();
    d2_test(&map, &expected);

    let map: BTreeMap<[usize; 2], _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ([i, j], val)))
        .collect();
    d2_test(&map, &expected);
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
    let expected: Vec<_> = arr_map()
        .into_iter()
        .flat_map(|x| x.1.into_iter().map(|y| y.1))
        .collect();

    let map: BTreeMap<usize, BTreeMap<usize, _>> = arr_map()
        .into_iter()
        .map(|x| (x.0, pairs_to_map(x.1)))
        .collect();
    d2_test(&map, &expected);
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
    let expected: Vec<_> = arr_map().into_iter().flat_map(|x| x.1).collect();

    let map: BTreeMap<usize, Vec<_>> = arr_map().into_iter().map(|x| (x.0, x.1)).collect();
    d2_test(&map, &expected);
}
