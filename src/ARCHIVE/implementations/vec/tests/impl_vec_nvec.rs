use crate::{dimensions::dim::*, NVec};
use alloc::{string::ToString, vec, vec::Vec};

// d1

fn d1_get_1<T, N: NVec<D1, T>>(nvec: &N) -> Option<T> {
    nvec.try_at(1)
}

#[test]
fn d1_nvec_vec() {
    let vec = vec![1, 42, 13];
    assert_eq!(d1_get_1(&vec), Some(42));

    let vec_string: Vec<_> = vec.iter().map(|x| x.to_string()).collect();
    let vec_refs: Vec<_> = vec_string.iter().collect();
    assert_eq!(d1_get_1(&vec_refs), Some(&42.to_string()));
}

// d2

fn d2_get_24<T, N: NVec<D2, T>>(nvec: &N) -> Option<T> {
    nvec.try_at([2, 4])
}

#[test]
fn d2_nvec_vec_vec() {
    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    assert_eq!(d2_get_24(&vec), Some(42));

    let vec_string: Vec<Vec<_>> = vec
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect())
        .collect();
    let vec_refs: Vec<Vec<_>> = vec_string.iter().map(|x| x.iter().collect()).collect();
    assert_eq!(d2_get_24(&vec_refs), Some(&42.to_string()));
}
