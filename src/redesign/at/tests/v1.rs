use super::super::super::D1;
use super::super::{At, FunAt};
use alloc::string::{String, ToString};
use alloc::vec;

fn target_fun<'a>(v1: &impl At<D1, usize>, v2: &impl At<D1, &'a String>) -> usize {
    v1.at(2) + v2.at(1).len()
}

#[test]
fn vec_as_v1() {
    let v1 = vec![1, 2, 3];
    let v2 = vec!["x".to_string(), "y".to_string()];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 4);
}

#[test]
fn slice_as_v1() {
    let vec1 = vec![1, 2, 3];
    let v1 = vec1.as_slice();

    let vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = vec2.as_slice();

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 4);
}

#[test]
fn fun_as_v1() {
    let v1 = FunAt::new(|i| i + 1);

    let vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = FunAt::new(|i| &vec2[i]);

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 4);
}
