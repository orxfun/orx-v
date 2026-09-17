use super::super::super::NVec;
use super::utils::{sum, sum_ref};
use alloc::vec;

#[test]
fn slice_as_v1() {
    let result = sum_ref(vec![1, 3, 4, 5, 6].as_slice());
    assert_eq!(result, 8);

    let result = sum(vec![1, 3, 4, 5, 6].as_slice().copied());
    assert_eq!(result, 8);
}
