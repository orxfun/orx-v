use super::super::super::NVec;
use super::utils::{second, second_ref};
use alloc::vec;

#[test]
fn slice_as_v1() {
    let v = vec![1, 2, 3];
    let s = v.as_slice();

    assert_eq!(second(&s), &2);
    assert_eq!(second(s.cloned()), 2);
    assert_eq!(second(s.copied()), 2);

    assert_eq!(second_ref(&s), &2);
}
