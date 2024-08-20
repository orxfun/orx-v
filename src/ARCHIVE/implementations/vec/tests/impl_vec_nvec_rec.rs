use crate::{dimensions::dim::*, nvec_rec::NVecRec, NVec};
use alloc::{string::ToString, vec, vec::Vec};

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

// d3

fn d3_child_0<T, N: NVecRec<D3, T>>(nvec: &N) -> Option<&N::Child>
where
    N::Child: NVecRec<D2, T>,
{
    nvec.child(0)
}

fn d3_get_024<T, N: NVecRec<D3, T>>(nvec: &N) -> Option<T>
where
    N::Child: NVecRec<D2, T>,
{
    let child = d3_child_0(nvec);
    d2_get_24(child.unwrap())
}

#[test]
fn d3_nvec_vec_vec() {
    let vec = vec![
        vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]],
        vec![],
        vec![vec![11], vec![2, 5, 1], vec![0]],
    ];
    assert_eq!(d3_get_024(&vec), Some(42));

    let vec_string: Vec<Vec<Vec<_>>> = vec
        .iter()
        .map(|x| {
            x.iter()
                .map(|x| x.iter().map(|x| x.to_string()).collect())
                .collect()
        })
        .collect();
    let vec_refs: Vec<Vec<Vec<_>>> = vec_string
        .iter()
        .map(|x| x.iter().map(|x| x.iter().collect()).collect())
        .collect();
    assert_eq!(d3_get_024(&vec_refs), Some(&42.to_string()));
}
