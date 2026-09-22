use super::super::*;
use alloc::vec;

fn apply_two_opt<T>(tour: &mut T, i: usize, j: usize)
where
    T: MutAt<D1, usize>,
{
    let mut i = i + 1;
    let mut j = j;
    while i < j {
        let t = *tour.at(i);
        *tour.mut_at(i) = *tour.at(j);
        *tour.mut_at(j) = t;
        i += 1;
        j -= 1;
    }
}

fn two_opt<D, T>(distances: &D, mut tour: T) -> u32
where
    D: At<D2, u32>,
    T: MutAt<D1, usize>,
{
    let mut improvement = 0;
    let d = distances;
    // let n = tour.card([]);
    // TODO: FIX THIS AFTER CARD
    let n = 4;

    let mut improved = true;
    while improved {
        improved = false;

        for i in 0..(n - 1) {
            let i1 = *tour.at(i);
            let i2 = *tour.at(i + 1);

            for j in (i + 2)..n {
                let j1 = *tour.at(j);
                let j2 = *tour.at((j + 1) % n);

                let removed_len = d.at([i1, i2]) + d.at([j1, j2]);
                let added_len = d.at([i1, j1]) + d.at([i2, j2]);

                if removed_len > added_len {
                    improved = true;
                    improvement += removed_len - added_len;
                    apply_two_opt(&mut tour, i, j);
                }
            }
        }
    }

    improvement
}

#[test]
fn two_opt_var1() {
    let distances = vec![
        vec![0, 92, 3, 7],
        vec![8, 0, 3, 12],
        vec![1, 6, 0, 9],
        vec![7, 3, 3, 0],
    ];
    let tour = vec![0, 1, 2, 3];
    let result = two_opt(&distances, tour);
    assert_eq!(result, 170);
}

#[test]
fn two_opt_var2() {
    let distances = [
        vec![0, 92, 3, 7],
        vec![8, 0, 3, 12],
        vec![1, 6, 0, 9],
        vec![7, 3, 3, 0],
    ];
    let mut tour = vec![0, 1, 2, 3];

    let result = two_opt(&distances.as_slice().copied(), tour.as_mut_slice());
    assert_eq!(result, 170);
}

#[test]
fn two_opt_var3() {
    let distances = FunAt::new(|[i, j]: [usize; 2]| (5 * i + 3 * j / 2) as u32);
    let mut tour = vec![0, 1, 2, 3];

    let result = two_opt(&distances, &mut tour);
    assert_eq!(result, 13);
}

#[test]
fn two_opt_var4() {
    let distances = FunAt::new(|[i, j]: [usize; 2]| match i == j {
        true => 0,
        false => 1,
    });
    let mut tour = vec![0, 1, 2, 3];

    let result = two_opt(&distances, &mut tour);
    assert_eq!(result, 0);
}
