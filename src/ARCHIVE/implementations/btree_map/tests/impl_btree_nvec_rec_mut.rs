use crate::{dimensions::dim::*, NVec, NVecMut, NVecRecMut, ValueSetError};
use alloc::{collections::BTreeMap, vec, vec::Vec};

fn pairs_to_map<T>(pairs: Vec<(usize, T)>) -> BTreeMap<usize, T> {
    pairs.into_iter().map(|x| (x.0, x.1)).collect()
}

// d2

fn d1_set_4<T, N: NVecMut<D1, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    nvec.set(4, value)
}

fn d2_child_2<T, N: NVecRecMut<D2, T>>(nvec: &mut N) -> Option<&mut N::Child> {
    nvec.child_mut(2)
}

fn d2_set_24<T, N: NVecRecMut<D2, T>>(nvec: &mut N, value: T) -> Result<(), ValueSetError> {
    let child = d2_child_2(nvec);
    d1_set_4(child.unwrap(), value)
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
