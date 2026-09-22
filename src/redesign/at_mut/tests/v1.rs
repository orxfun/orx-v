use super::super::super::D1;
use super::super::AtMut;
use alloc::string::{String, ToString};
use alloc::vec;

fn target_fun<'a>(v1: &mut impl AtMut<D1, usize>, mut v2: impl AtMut<D1, String>) {
    v2.at_mut(1).push_str("z");
    let x = v2.at_mut(1).len();
    *v1.at_mut(2) += x;
}

#[test]
fn vec_as_mut_at1() {
    let mut v1 = vec![1, 2, 3];
    let v2 = vec!["x".to_string(), "y".to_string()];
    target_fun(&mut v1, v2);
    assert_eq!(v1[2], 5);
}

#[test]
fn slice_as_mut_at1() {
    let mut vec1 = vec![1, 2, 3];
    let mut v1 = vec1.as_mut_slice();

    let mut vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = vec2.as_mut_slice();

    target_fun(&mut v1, v2);
    assert_eq!(vec1[2], 5);
    assert_eq!(&vec2[1], "yz");
}
