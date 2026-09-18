use super::utils::second;
use alloc::vec;

#[test]
fn vec_as_v1() {
    let v = vec![1, 2, 3];
    assert_eq!(2, second(v));

    let v = vec![1, 2, 3];
    // assert_eq!(2, second(&v));
}
