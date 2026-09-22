use super::super::*;
use alloc::{vec, vec::Vec};
use num::integer::Roots;

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
                *c.mut_at([i, j]) += a.at([i, k]) * b.at([k, j]);
            }
        }
    }
}

#[test]
fn matmul_var1() {
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
fn matmul_var2() {
    let a = FunAt::new(|_: [usize; 2]| 1);
    let b = FunAt::new(|_: [usize; 2]| 7);
    let mut c: Vec<Vec<_>> = (0..4).map(|_| (0..4).map(|_| 0u32).collect()).collect();

    matmul(&a, &b, &mut c);
    assert_eq!(c[1][2], 28);
}

#[test]
fn matmul_var3() {
    fn make_flat_matrix<'a>(data: &'a [u32]) -> impl At<D2, u32> + 'a {
        let m = data.len().sqrt();
        FunAt::new(move |[i, j]: [usize; 2]| {
            let idx = i * m + j;
            data[idx]
        })
    }

    let data_a = vec![0, 3, 2, 5, 1, 6, 8, 2, 2, 9, 7, 4, 3, 5, 5, 6];
    let a = make_flat_matrix(&data_a);

    let data_b = vec![0, 3, 2, 5, 1, 6, 8, 2, 2, 9, 7, 4, 3, 5, 5, 6];
    let b = make_flat_matrix(&data_b);

    let mut data_c = (4, vec![0; data_a.len()]);

    fn f<'a>((m, data): &'a (usize, Vec<u32>), [i, j]: [usize; 2]) -> &'a u32 {
        let idx = i * *m + j;
        &data[idx]
    }
    fn m<'a>((m, data): &'a mut (usize, Vec<u32>), [i, j]: [usize; 2]) -> &'a mut u32 {
        let idx = i * *m + j;
        &mut data[idx]
    }
    let mut c = FunMutAt::new(&mut data_c, f, m);

    matmul(&a, &b, &mut c);
    assert_eq!(data_c.1[1 * 4 + 2], 116);
}
