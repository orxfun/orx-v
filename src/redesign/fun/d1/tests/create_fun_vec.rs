use super::super::*;
use alloc::{vec, vec::Vec};

#[test]
fn funvec1_from_vec() {
    let data = vec![1, 2, 3];

    let v = FunVec1::with_data(&data, |d: &Vec<_>, i| &d[i]);
}
