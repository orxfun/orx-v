use crate::{dimensions::dim::*, FunVecBuilder, NVec};
use alloc::vec;

// d1

fn d1_get_1<T, N: NVec<D1, T>>(nvec: &N) -> Option<T> {
    nvec.try_at(1)
}

#[test]
fn d1_nvec_fun_vec() {
    let fun = FunVecBuilder::d1().complete(|_: usize| 42);
    assert_eq!(d1_get_1(&fun), Some(42));

    let fun = FunVecBuilder::d1().complete(|i: usize| match i {
        1 => 42,
        _ => 0,
    });
    assert_eq!(d1_get_1(&fun), Some(42));

    let fun = FunVecBuilder::d1().sparse(|_: usize| Some(42));
    assert_eq!(d1_get_1(&fun), Some(42));

    let fun = FunVecBuilder::d1().sparse(|i: usize| match i {
        1 => Some(42),
        _ => None,
    });
    assert_eq!(d1_get_1(&fun), Some(42));

    // capture

    let vec = vec![0, 42, 1, 2, 3];
    let fun = FunVecBuilder::d1().sparse(|i: usize| vec.get(i).copied());
    assert_eq!(d1_get_1(&fun), Some(42));

    let vec = vec![0, 42, 1, 2, 3];
    let fun = FunVecBuilder::d1().sparse(move |i: usize| vec.get(i).copied());
    assert_eq!(d1_get_1(&fun), Some(42));
}

// d2

fn d2_get_24<T, N: NVec<D2, T>>(nvec: &N) -> Option<T> {
    nvec.try_at([2, 4])
}

#[test]
fn d2_nvec_vec_vec() {
    let fun = FunVecBuilder::d2().complete(|_: (usize, usize)| 42);
    assert_eq!(d2_get_24(&fun), Some(42));

    let fun = FunVecBuilder::d2().complete(|(i, j)| match (i, j) {
        (2, 4) => 42,
        _ => 7,
    });
    assert_eq!(d2_get_24(&fun), Some(42));

    // capture

    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    let fun = FunVecBuilder::d2().sparse(|(i, j)| vec.get(i).and_then(|x| x.get(j)).copied());
    assert_eq!(d2_get_24(&fun), Some(42));

    let vec = vec![vec![1], vec![], vec![2, 5, 1, 3, 42, 7], vec![4]];
    let fun = FunVecBuilder::d2().sparse(move |(i, j)| vec.get(i).and_then(|x| x.get(j)).copied());
    assert_eq!(d2_get_24(&fun), Some(42));
}
