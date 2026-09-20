use super::super::super::NVec;
use super::utils::{second, second_mut, second_mut_cloned};
use alloc::{string::ToString, vec};

#[test]
fn vec_as_v1_copy() {
    let v = vec![1, 2, 3];

    assert_eq!(second(&v), &2);
    assert_eq!(second(v.cloned()), 2);
    assert_eq!(second(v.copied()), 2);
}

#[test]
fn vec_as_v1_clone() {
    let v = vec!["x".to_string(), "y".to_string()];

    assert_eq!(second(&v), &"y".to_string());
    assert_eq!(second(v.cloned()), "y".to_string());
}

#[test]
fn vec_as_v1_mut() {
    let mut v = vec![1, 2, 3];

    assert_eq!(second(&v), &2);
    assert_eq!(second_mut_cloned(&mut v), 2);

    *second_mut(&mut v) = 22;

    assert_eq!(second(&v), &22);
    assert_eq!(second_mut_cloned(&mut v), 22);

    assert_eq!(v, vec![1, 22, 3]);
}
