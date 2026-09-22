use super::super::*;
use alloc::vec;

fn matmul<A, B, C>(a: &A, b: &B, c: &mut C)
where
    A: At<D2, u32>,
    B: At<D2, u32>,
    C: MutAt<D2, u32>,
{
    // TODO: FIX THIS after card
    let n = 4;

    for i in 0..n {
        for j in 0..n {
            // TODO: FIX THIS sequential operations
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

    matmul(&a, &b.copied(), &mut c);
    assert_eq!(c[1][2], 116);
}

#[test]
fn two_opt_var2() {
    // let make_flat_matrix = |data: &[u32]| {
    //     FunAt::new(|[i, j]: [usize; 2]| {
    //         let idx = i * 4 + j;
    //         data[idx]
    //     })
    // };

    // let data_a = vec![0, 3, 2, 5, 1, 6, 8, 2, 2, 9, 7, 4, 3, 5, 5, 6];
    // let a = make_flat_matrix(&data_a);

    // let data_b = vec![0, 3, 2, 5, 1, 6, 8, 2, 2, 9, 7, 4, 3, 5, 5, 6];
    // let b = make_flat_matrix(&data_b);

    // let mut c = a.clone();

    // matmul(&a, &b.copied(), &mut c);
    // assert_eq!(c[1][2], 116);
}
