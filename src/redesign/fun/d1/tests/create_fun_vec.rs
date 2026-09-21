use super::super::*;
use alloc::{vec, vec::Vec};

#[test]
fn funvec1_from_vec() {
    fn f<'a>(d: &'a Vec<i32>, i: usize) -> &'a i32 {
        &d[i]
    }

    // let data = vec![1, 2, 3];
    // let v = FunVec1::with_data(data, f);

    // let data = vec![1, 2, 3];
    // let v2 = FunVec1::with_data(&data, |d: &Vec<i32>, i: usize| d[i] + i as i32);
}
