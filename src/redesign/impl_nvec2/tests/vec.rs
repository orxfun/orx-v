use super::utils::{sum, sum_ref};
use alloc::vec;

#[test]
fn vec_as_v1() {
    let vec_vec = vec![vec![0], vec![1, 2, 3], vec![4, 5]];
    let result = sum(vec_vec);
    assert_eq!(result, 8);

    // let result = sum_ref(&vec![1, 3, 4, 5, 6, 7]);
    // assert_eq!(result, 8);
}
