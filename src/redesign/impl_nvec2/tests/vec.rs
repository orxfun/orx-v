use super::super::super::NVec;
use super::utils::second_first;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn vec_as_v1_copy() {
    let vec_vec = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

    assert_eq!(second_first(&vec_vec), &1);
    assert_eq!(second_first(vec_vec.cloned()), 1);
    assert_eq!(second_first(vec_vec.copied()), 1);
}

#[test]
fn vec_as_v1_clone() {
    let vec_vec = vec![
        vec!['x'.to_string()],
        vec!['y'.to_string(), 'z'.to_string()],
    ];

    assert_eq!(second_first(&vec_vec), &"y".to_string());
    assert_eq!(second_first(vec_vec.cloned()), "y".to_string());
}
