use super::super::super::{FunVec, NVec};
use super::utils::{reduce_first_child, second_first, second_first_mut, second_first_ref};
use alloc::vec;

#[test]
fn vec_vec_as_v2() {
    let mut v = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

    assert_eq!(second_first(&&v), &1);
    assert_eq!(second_first(&v.cloned()), 1);
    assert_eq!(second_first(&v.copied()), 1);

    assert_eq!(second_first_ref(&v), &1);
    assert_eq!(second_first_mut(&mut v), &mut 1);

    let sum_c1 = reduce_first_child(&v, 0, |a, b| a + b);
    assert_eq!(sum_c1, 6);
}

#[test]
fn vec_slice_as_v2() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let v = vec![a.as_slice(), b.as_slice(), c.as_slice()];

    assert_eq!(second_first(&&v), &1);
    assert_eq!(second_first(&v.cloned()), 1);
    assert_eq!(second_first(&v.copied()), 1);

    assert_eq!(second_first_ref(&v), &1);

    let sum_c1 = reduce_first_child(&v, 0, |a, b| a + b);
    assert_eq!(sum_c1, 6);
}

#[test]
fn vec_vecref_as_v2() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let v = vec![&a, &b, &c];

    let f = FunVec::new(|[i, j]: [usize; 2]| &v[i][j]);

    assert_eq!(second_first(&f), &1);
    assert_eq!(second_first(&f.cloned()), 1);
    assert_eq!(second_first(&f.copied()), 1);

    assert_eq!(second_first_ref(&v), &1);
}
