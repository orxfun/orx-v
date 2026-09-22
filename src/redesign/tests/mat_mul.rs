use super::super::*;
use alloc::vec;

fn matmul<A, B, C>(a: &A, b: &B, c: &mut C)
where
    A: At<D2, u32>,
    B: At<D2, u32>,
    C: MutAt<D2, u32>,
{
    // TODO: FIX THIS
    let n = 4;

    for i in 0..n {
        for j in 0..n {
            *c.mut_at([i, j]) = 0;
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                *c.mut_at([i, j]) = *c.at([i, j]) + a.at([i, k]) * b.at([k, j]);
            }
        }
    }
}

#[test]
fn two_opt_var1() {
    let a = vec![
        vec![0, 3, 2, 5],
        vec![1, 6, 8, 2],
        vec![2, 9, 7, 4],
        vec![3, 5, 5, 6],
    ];
    let vec_b = vec![
        vec![0, 3, 2, 5],
        vec![1, 6, 8, 2],
        vec![2, 9, 7, 4],
        vec![3, 5, 5, 6],
    ];
    let b = vec_b.as_slice();

    let mut c = a.clone();

    // matmul(&a, &b.copied(), &mut c);
}
