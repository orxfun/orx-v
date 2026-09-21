use super::super::super::*;
use super::super::*;
use alloc::vec;

fn second_first<T>(v: impl V<D2, T>) -> T {
    v.at([1, 0])
}

fn reduce_first_child<'a, T: 'a>(v: impl V<D2, &'a T>, mut init: T, f: impl Fn(T, &T) -> T) -> T {
    let c = v.child(1);
    for j in 0..3 {
        init = f(init, c.at(j));
    }
    init
}

fn reduce_first_child_copied<T>(v: impl V<D2, T>, mut init: T, f: impl Fn(T, T) -> T) -> T {
    let c = v.child(1);
    for j in 0..3 {
        init = f(init, c.at(j));
    }
    init
}

#[test]
fn vec_vec_as_v2() {
    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

    let f = |[i, j]: [usize; 2]| &data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), &1);
    assert_eq!(reduce_first_child(&v, 0, |a, b| a + b), 6);

    // copied
    let f = |[i, j]: [usize; 2]| data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), 1);
    assert_eq!(reduce_first_child_copied(&v, 0, |a, b| a + b), 6);
}

#[test]
fn vec_slice_as_v2() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let data = vec![a.as_slice(), b.as_slice(), c.as_slice()];

    let f = |[i, j]: [usize; 2]| &data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), &1);
    assert_eq!(reduce_first_child(&v, 0, |a, b| a + b), 6);

    // copied
    let f = |[i, j]: [usize; 2]| data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), 1);
    assert_eq!(reduce_first_child_copied(&v, 0, |a, b| a + b), 6);
}

#[test]
fn vec_vecref_as_v2() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let data = vec![&a, &b, &c];

    let f = |[i, j]: [usize; 2]| &data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), &1);
    assert_eq!(reduce_first_child(&v, 0, |a, b| a + b), 6);

    // copied
    let f = |[i, j]: [usize; 2]| data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), 1);
    assert_eq!(reduce_first_child_copied(&v, 0, |a, b| a + b), 6);
}

#[test]
fn slice_vecref_as_v2() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let vec = vec![&a, &b, &c];
    let data = vec.as_slice();

    let f = |[i, j]: [usize; 2]| &data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), &1);
    assert_eq!(reduce_first_child(&v, 0, |a, b| a + b), 6);

    // copied
    let f = |[i, j]: [usize; 2]| data[i][j];
    let v = FunVec2::new(&f);

    assert_eq!(second_first(&v), 1);
    assert_eq!(reduce_first_child_copied(&v, 0, |a, b| a + b), 6);
}
