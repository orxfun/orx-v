use super::super::super::D2;
use super::super::{At, AtCopied, FunAt};
use alloc::string::{String, ToString};
use alloc::vec;

fn target_fun<'a>(v1: &impl At<D2, usize>, v2: &impl At<D2, &'a String>) -> usize {
    v1.at([1, 0]) + v2.at([0, 1]).len()
}

#[test]
fn vec_vec_as_at2() {
    let v1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v2 = vec![vec!["x".to_string(), "y".to_string()]];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 5);

    let res = target_fun(&(&v1).copied(), &&v2);
    assert_eq!(res, 5);
}

#[test]
fn slice_vec_as_at2() {
    let vec1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v1 = vec1.as_slice();

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = vec2.as_slice();

    let res = target_fun(&v1.copied(), &v2);
    assert_eq!(res, 5);
}

#[test]
fn fun_as_at2() {
    let v1 = FunAt::new(|[i, j]: [usize; 2]| 3 + i + j);

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = FunAt::new(|[i, j]: [usize; 2]| &vec2[i][j]);

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 5);
}
