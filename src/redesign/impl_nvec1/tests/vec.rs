use super::super::super::NVec;
use super::utils::{second, second_mut, second_ref};
use alloc::vec;

#[test]
fn vec_as_v1() {
    let mut v = vec![1, 2, 3];

    assert_eq!(second(&&v), &2);
    assert_eq!(second(&v.cloned()), 2);
    assert_eq!(second(&v.copied()), 2);

    assert_eq!(second_ref(&v), &2);
    assert_eq!(second_mut(&mut v), &mut 2);
}
