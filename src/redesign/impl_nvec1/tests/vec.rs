use super::super::super::NVec;
use super::utils::second;
use alloc::{string::ToString, vec};

#[test]
fn vec_as_v1_copy() {
    let v = vec![1, 2, 3];

    assert_eq!(second(&v), &2);
    assert_eq!(second(v.copied()), 2);
}

// #[test]
// fn vec_as_v1_ref() {
//     let v = vec!["x".to_string(), "y".to_string()];
//     assert_eq!(&"y".to_string(), second(&v));
// }
