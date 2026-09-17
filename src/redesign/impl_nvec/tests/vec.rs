use super::super::super::{D1, NVec};
use alloc::vec;

fn sum_1_3(v: impl NVec<D1, i32>) -> i32
where
{
    v.at(1) + v.at(3)
}

#[test]
fn vec_as_v1() {
    let sum = sum_1_3(vec![1, 3, 4, 5, 6, 7]);
    assert_eq!(sum, 8);
}
