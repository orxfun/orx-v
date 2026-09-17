use super::super::super::{D1, NVec};
use super::utils::{sum, sum_ref};
use alloc::vec;

#[test]
fn vec_as_v1() {
    let sum = sum(vec![1, 3, 4, 5, 6, 7]);
    assert_eq!(sum, 8);
}
