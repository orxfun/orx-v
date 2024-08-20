use crate::{dimensions::dim::*, NVecSized};
use alloc::{string::ToString, vec, vec::Vec};

// d1

fn d1_to_vec<T, N: NVecSized<D1, T>>(nvec: &N) -> Vec<T> {
    let card = nvec.cardinality();
    let vec: Vec<_> = nvec.elements().collect();
    assert_eq!(vec.len(), card);
    vec
}

#[test]
fn d1_nvec_vec() {
    let vec = || vec![1, 42, 13];
    assert_eq!(d1_to_vec(&vec()), vec());

    let vec_string: Vec<_> = vec().iter().map(|x| x.to_string()).collect();
    let vec_refs = || vec_string.iter().collect::<Vec<_>>();
    assert_eq!(d1_to_vec(&vec_refs()), vec_refs());
}

// d2

fn d2_to_vec<T, N: NVecSized<D2, T>>(nvec: &N) -> Vec<T> {
    let card = nvec.cardinality();
    let vec: Vec<_> = nvec.elements().collect();
    assert_eq!(vec.len(), card);
    vec
}

#[test]
fn d2_nvec_vec_vec() {
    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    let expected: Vec<_> = vec.iter().flat_map(|x| x).copied().collect();
    assert_eq!(d2_to_vec(&vec), expected);

    let vec_string: Vec<Vec<_>> = vec
        .iter()
        .map(|x| x.iter().map(|x| x.to_string()).collect())
        .collect();
    let vec_refs: Vec<Vec<_>> = vec_string.iter().map(|x| x.iter().collect()).collect();
    let expected: Vec<_> = vec_refs.iter().flat_map(|x| x).copied().collect();
    assert_eq!(d2_to_vec(&vec_refs), expected);
}
