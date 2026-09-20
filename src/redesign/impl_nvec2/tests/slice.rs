use super::super::super::NVec;
use super::utils::second_first;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn slice_vec_as_v1_copy() {
    let vec_vec = vec![vec![0], vec![1, 2, 3], vec![4, 5]];
    let slice_vec = vec_vec.as_slice();

    assert_eq!(second_first(&slice_vec), &1);
    assert_eq!(second_first(slice_vec.cloned()), 1);
    assert_eq!(second_first(slice_vec.copied()), 1);
}

#[test]
fn slice_vec_as_v1_clone() {
    let vec_vec = vec![
        vec!['x'.to_string()],
        vec!['y'.to_string(), 'z'.to_string()],
    ];
    let slice_vec = vec_vec.as_slice();

    assert_eq!(second_first(&slice_vec), &"y".to_string());
    assert_eq!(second_first(slice_vec.cloned()), "y".to_string());
}

#[test]
fn slice_slice_as_v1_copy() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let vec_slice = vec![a.as_slice(), &b, &c];
    let slice_slice = vec_slice.as_slice();

    assert_eq!(second_first(&slice_slice), &1);
    assert_eq!(second_first(slice_slice.cloned()), 1);
    assert_eq!(second_first(slice_slice.copied()), 1);
}
