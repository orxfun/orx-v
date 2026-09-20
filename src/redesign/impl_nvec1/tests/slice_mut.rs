use super::utils::{second, second_mut, second_mut_cloned};
use alloc::vec;

#[test]
fn mut_slice_as_v1_copy() {
    let mut vec = vec![1, 2, 3];
    let mut slice = vec.as_mut_slice();

    assert_eq!(second(&slice), &2);
    assert_eq!(second_mut_cloned(&mut slice), 2);

    *second_mut(&mut slice) = 22;

    assert_eq!(second(&slice), &22);
    assert_eq!(second_mut_cloned(&mut slice), 22);

    assert_eq!(slice, [1, 22, 3]);
}
