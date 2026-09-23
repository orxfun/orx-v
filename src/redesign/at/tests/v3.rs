use super::super::super::{At, AtCopied, D1, D2, D3, FunAt};
use alloc::vec;

fn target_fun<'a>(v1: &impl At<D3, usize>, v2: &impl At<D3, &'a usize>) -> usize {
    v1.at([1, 0, 2]) + v2.at([0, 1, 2])
}

fn vec3_sum(m: &impl At<D3, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(child) = m.try_child(i) {
            sum += vec2_sum(&child);
        }
    }
    sum
}

fn vec2_sum(m: &impl At<D2, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(child) = m.try_child(i) {
            sum += vec1_sum(&child);
        }
    }
    sum
}

fn vec1_sum(m: &impl At<D1, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(value) = m.try_at(i) {
            sum += value
        }
    }
    sum
}

#[test]
fn vec_vec_vec_as_at3() {
    let values = vec![
        vec![vec![1, 2], vec![3, 4, 5]],
        vec![vec![6, 7, 8]],
        vec![vec![9]],
    ];
    assert_eq!(values[1][0][2], 8);
    assert_eq!(values[0][1][2], 5);

    assert_eq!(target_fun(&(&values).copied(), &&values), 13);

    assert_eq!(vec3_sum(&(&values).copied()), 45);
}

#[test]
fn slices_and_copied_as_at3() {
    let owned = vec![
        vec![vec![1, 2], vec![3, 4, 5]],
        vec![vec![6, 7, 8]],
        vec![vec![9]],
    ];
    let values = owned.as_slice();

    assert_eq!(target_fun(&values.copied(), &values), 13);

    assert_eq!(vec3_sum(&values.copied()), 45);
}

#[test]
fn fun_as_at3() {
    let vec3 = vec![
        vec![vec![1, 2], vec![3, 4, 5]],
        vec![vec![6, 7, 8]],
        vec![vec![9]],
    ];
    let v1 = FunAt::new(|[i, j, k]: [usize; 3]| i * 100 + j * 10 + k);
    let v2 = FunAt::new(|[i, j, k]: [usize; 3]| &vec3[i][j][k]);

    assert_eq!(target_fun(&v1, &v2), 107);

    assert_eq!(vec3_sum(&v1), 499500);
}
