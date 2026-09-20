use super::super::super::NVec;
use super::utils::second_first;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn vec_vec_as_v1_copy() {
    let vec_vec = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

    assert_eq!(second_first(&vec_vec), &1);
    assert_eq!(second_first(vec_vec.cloned()), 1);
    assert_eq!(second_first(vec_vec.copied()), 1);
}

#[test]
fn vec_vec_as_v1_clone() {
    let vec_vec = vec![
        vec!['x'.to_string()],
        vec!['y'.to_string(), 'z'.to_string()],
    ];

    assert_eq!(second_first(&vec_vec), &"y".to_string());
    assert_eq!(second_first(vec_vec.cloned()), "y".to_string());
}

#[test]
fn vec_slice_as_v1_copy() {
    let a = vec![0];
    let b = vec![1, 2, 3];
    let c = vec![4, 5];
    let vec_slice = vec![a.as_slice(), &b, &c];

    assert_eq!(second_first(&vec_slice), &1);
    assert_eq!(second_first(vec_slice.cloned()), 1);
    assert_eq!(second_first(vec_slice.copied()), 1);
}
