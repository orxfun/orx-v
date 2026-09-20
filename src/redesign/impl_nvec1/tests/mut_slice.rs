use super::utils::{second, second_mut, second_ref};
use alloc::vec;

#[test]
fn mut_slice_as_v1() {
    let mut v = vec![1, 2, 3];
    let mut s = v.as_mut_slice();

    assert_eq!(second(&s), &2);
    assert_eq!(second_ref(&s), &2);
    assert_eq!(second_mut(&mut s), &mut 2);
}
