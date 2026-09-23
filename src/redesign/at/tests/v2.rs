use super::super::super::{D1, D2};
use super::super::{At, AtCopied, FunAt};
use alloc::string::{String, ToString};
use alloc::vec;

fn target_fun<'a>(v1: &impl At<D2, usize>, v2: &impl At<D2, &'a String>) -> usize {
    v1.at([1, 0]) + v2.at([0, 1]).len()
}

fn vec2_sum(m: &impl At<D2, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(child) = m.try_child(i) {
            sum += vec1_sum(&child);
        }
    }
    sum
}

fn vec1_sum(m: &impl At<D1, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(value) = m.try_at(i) {
            sum += value
        }
    }
    sum
}

#[test]
fn vec_vec_as_at2() {
    let v1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v2 = vec![vec!["x".to_string(), "y".to_string()]];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 5);

    let res = target_fun(&(&v1).copied(), &&v2);
    assert_eq!(res, 5);

    // child

    let sum = vec2_sum(&v1);
    assert_eq!(sum, 12);

    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(1).is_some());
    assert!(v1.try_child(2).is_none());
}

#[test]
fn slice_vec_as_at2() {
    let vec1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v1 = vec1.as_slice();

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = vec2.as_slice();

    let res = target_fun(&v1.copied(), &v2);
    assert_eq!(res, 5);

    // child

    let sum = vec2_sum(&v1.copied());
    assert_eq!(sum, 12);

    assert!(At::<D2, &usize>::try_child(&v1, 0).is_some());
    assert!(At::<D2, &usize>::try_child(&v1, 1).is_some());
    assert!(At::<D2, &usize>::try_child(&v1, 2).is_none());

    assert!(AtCopied::<D2>::copied(v1).try_child(0).is_some());
    assert!(AtCopied::<D2>::copied(v1).try_child(1).is_some());
    assert!(AtCopied::<D2>::copied(v1).try_child(2).is_none());
}

#[test]
fn fun_as_at2() {
    let v1 = FunAt::new(|[i, j]: [usize; 2]| 3 + i + j);

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = FunAt::new(|[i, j]: [usize; 2]| &vec2[i][j]);

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 5);

    // child

    assert_eq!(vec2_sum(&v1), 1200);

    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(100).is_some());
}
