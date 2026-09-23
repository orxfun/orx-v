use super::super::super::{D1, D2};
use super::super::{AtMut, FunMutAt};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

fn target_fun<'a>(v1: &mut (impl AtMut<D2, usize> + ?Sized), mut v2: impl AtMut<D2, String>) {
    v2.at_mut([0, 1]).push_str("z");
    let x = v2.at([0, 1]).len();
    *v1.at_mut([1, 0]) += x;
}

fn vec_inc(v: &mut (impl AtMut<D1, usize> + ?Sized)) {
    for i in 0..2 {
        *v.at_mut(i) += 10;
    }
}

fn vec2_inc(m: &mut (impl AtMut<D2, usize> + ?Sized), num_rows: usize) {
    for c in 0..num_rows {
        let mut row = m.child_mut(c);
        vec_inc(&mut row);
    }
}

#[test]
fn vec_vec_as_at_mut2() {
    let mut v1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v2 = vec![vec!["x".to_string(), "y".to_string()]];
    target_fun(&mut v1, v2);
    assert_eq!(v1[1][0], 6);

    // child_mut

    vec2_inc(&mut v1, 2);
    assert_eq!(v1[0][0], 11);
    assert_eq!(v1[0][1], 12);
    assert_eq!(v1[0][2], 3);
    assert_eq!(v1[1][0], 16);
    assert_eq!(v1[1][1], 15);

    assert!(AtMut::<D2, usize>::try_child_mut(&mut v1, 0).is_some());
    assert!(AtMut::<D2, usize>::try_child_mut(&mut v1, 1).is_some());
    assert!(AtMut::<D2, usize>::try_child_mut(&mut v1, 2).is_none());
}

#[test]
fn slice_vec_as_at_mut2() {
    let mut vec1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v1 = vec1.as_mut_slice();

    let mut vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = vec2.as_mut_slice();

    target_fun(v1, v2);
    assert_eq!(vec1[1][0], 6);
    assert_eq!(&vec2[0][1], "yz");

    // child_mut

    vec2_inc(&mut vec1.as_mut_slice(), 2);
    assert_eq!(vec1[0][0], 11);
    assert_eq!(vec1[0][1], 12);
    assert_eq!(vec1[0][2], 3);
    assert_eq!(vec1[1][0], 16);
    assert_eq!(vec1[1][1], 15);

    let mut slice = vec1.as_mut_slice();
    assert!(AtMut::<D2, usize>::try_child_mut(&mut slice, 0).is_some());
    assert!(AtMut::<D2, usize>::try_child_mut(&mut slice, 1).is_some());
    assert!(AtMut::<D2, usize>::try_child_mut(&mut slice, 2).is_none());
}

#[test]
fn fun_as_at_mut2() {
    let mut data1 = (3, vec![0; 9]);

    fn f1<'a>((m, data): &'a (usize, Vec<usize>), [i, j]: [usize; 2]) -> &'a usize {
        let idx = i * *m + j;
        &data[idx]
    }
    fn m1<'a>((m, data): &'a mut (usize, Vec<usize>), [i, j]: [usize; 2]) -> &'a mut usize {
        let idx = i * *m + j;
        &mut data[idx]
    }

    let mut v1 = FunMutAt::new(&mut data1, f1, m1);

    let mut data2 = vec![vec!["x".to_string(), "y".to_string()]];
    fn f2<'a>(data: &'a Vec<Vec<String>>, [i, j]: [usize; 2]) -> &'a String {
        &data[i][j]
    }
    fn m2<'a>(data: &'a mut Vec<Vec<String>>, [i, j]: [usize; 2]) -> &'a mut String {
        &mut data[i][j]
    }
    let v2 = FunMutAt::new(&mut data2, f2, m2);

    target_fun(&mut v1, v2);
    assert_eq!(*v1.at([1, 0]), 2);
    assert_eq!(data2[0][1], "yz");

    // child_mut

    assert!(v1.try_child_mut(0).is_some());
    assert!(v1.try_child_mut(100).is_some());

    vec2_inc(&mut v1, 3);
    assert_eq!(*v1.at([0, 0]), 10);
    assert_eq!(*v1.at([0, 1]), 10);
    assert_eq!(*v1.at([1, 0]), 12);
    assert_eq!(*v1.at([1, 1]), 10);
    assert_eq!(data1.1[0 * 3 + 0], 10);
    assert_eq!(data1.1[0 * 3 + 1], 10);
    assert_eq!(data1.1[1 * 3 + 0], 12);
    assert_eq!(data1.1[1 * 3 + 1], 10);
}
