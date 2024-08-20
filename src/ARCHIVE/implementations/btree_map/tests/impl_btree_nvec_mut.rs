use crate::{dimensions::dim::*, NVec, NVecMut, ValueSetError};
use alloc::{collections::BTreeMap, vec, vec::Vec};

// d1

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

fn d1_set_1<T, N: NVecMut<D1, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set(1, value)
}

#[test]
fn d1_nvec_btree() {
    let arr_map = || vec![(0, 1), (1, 42), (3, 13)];

    let mut map = pairs_to_map(arr_map());
    assert_eq!(d1_set_1(&mut map, 7), Ok(()));
    assert_eq!(map.try_at(1), Some(7));
}

// d2

fn d2_set_24<T, N: NVecMut<D2, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set([2, 4], value)
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

    let mut map: BTreeMap<(usize, usize), _> = arr_map()
        .into_iter()
        .flat_map(|(i, x)| x.into_iter().map(move |(j, val)| ((i, j), val)))
        .collect();
    assert_eq!(d2_set_24(&mut map, 7), Ok(()));
    assert_eq!(map.try_at([2, 4]), Some(7));
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

    let mut map: BTreeMap<usize, BTreeMap<usize, _>> = arr_map()
        .into_iter()
        .map(|x| (x.0, pairs_to_map(x.1)))
        .collect();
    assert_eq!(d2_set_24(&mut map, 7), Ok(()));
    assert_eq!(map.try_at([2, 4]), Some(7));
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

    let mut map: BTreeMap<usize, Vec<_>> = arr_map().into_iter().map(|x| (x.0, x.1)).collect();
    assert_eq!(d2_set_24(&mut map, 7), Ok(()));
    assert_eq!(map.try_at([2, 4]), Some(7));
}
