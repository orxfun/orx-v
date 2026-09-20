use super::super::super::NVec;
use super::utils::{second_first, second_first_mut, second_first_ref};
use alloc::string::ToString;
use alloc::vec;

#[test]
fn vec_as_v2() {
    let mut v = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

    assert_eq!(second_first(&v), &1);

    assert_eq!(second_first_ref(&v), &1);
    assert_eq!(second_first_mut(&mut v), &mut 1);

    // assert_eq!(second(&&v), &2);
    // assert_eq!(second(&v.cloned()), 2);
    // assert_eq!(second(&v.copied()), 2);

    // assert_eq!(second_ref(&v), &2);
    // assert_eq!(second_mut(&mut v), &mut 2);
}

// #[test]
// fn vec_vec_as_v1_copy() {
//     let vec_vec = vec![vec![0], vec![1, 2, 3], vec![4, 5]];

//     assert_eq!(second_first(&vec_vec), &1);
//     assert_eq!(second_first(vec_vec.cloned()), 1);
//     assert_eq!(second_first(vec_vec.copied()), 1);
// }

// #[test]
// fn vec_vec_as_v1_clone() {
//     let vec_vec = vec![
//         vec!['x'.to_string()],
//         vec!['y'.to_string(), 'z'.to_string()],
//     ];

//     assert_eq!(second_first(&vec_vec), &"y".to_string());
//     assert_eq!(second_first(vec_vec.cloned()), "y".to_string());
// }

// #[test]
// fn vec_slice_as_v1_copy() {
//     let a = vec![0];
//     let b = vec![1, 2, 3];
//     let c = vec![4, 5];
//     let vec_slice = vec![a.as_slice(), &b, &c];

//     assert_eq!(second_first(&vec_slice), &1);
//     assert_eq!(second_first(vec_slice.cloned()), 1);
//     assert_eq!(second_first(vec_slice.copied()), 1);
// }
