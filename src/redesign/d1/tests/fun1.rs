use super::super::super::*;
use alloc::vec;

fn nvec_sec<T>(v: impl NVec<D1, T>) -> T {
    v.at(1)
}

#[test]
fn fun1_val() {
    let v = Fun1::val(|i| i + 1);
    assert_eq!(v.at(1), 2);
    assert_eq!(nvec_sec(v), 2);
}

#[test]
fn fun1_val_byref() {
    let data = vec![3, 1];
    let v = Fun1::val(|i| i + data[1]);
    assert_eq!(v.at(1), 2);
    assert_eq!(nvec_sec(v), 2);
}

#[test]
fn fun1_val_capture() {
    let data = vec![3, 1];
    let v = Fun1::val(move |i| i + data[1]);
    assert_eq!(v.at(1), 2);
    assert_eq!(nvec_sec(v), 2);
}

#[test]
fn fun1_ref_byref() {
    let data = vec![3, 2];
    let v = Fun1::rf(|i| &data[i]);
    assert_eq!(v.at(1), &2);
    assert_eq!(nvec_sec(v), &2);
}

#[test]
fn fun1_ref_capture() {
    let data = vec![3, 2];
    let v = Fun1::rf(move |i| &data[i]);
    assert_eq!(v.at(1), &2);
    assert_eq!(nvec_sec(v), &2);
}
