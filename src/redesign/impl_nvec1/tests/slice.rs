use super::super::super::NVec;
use super::utils::second;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn slice_as_v1_copy() {
    let vec = vec![1, 3, 4, 5, 6];
    let slice = vec.as_slice();

    assert_eq!(second(slice), &3);
    assert_eq!(second(slice.cloned()), 3);
    assert_eq!(second(slice.copied()), 3);
}

#[test]
fn slice_as_v1_clone() {
    let vec = vec!["x".to_string(), "y".to_string()];
    let slice = vec.as_slice();

    assert_eq!(second(slice), &"y".to_string());
    assert_eq!(second(slice.cloned()), "y".to_string());
}
