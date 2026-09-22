use super::super::*;

fn apply_two_opt<T>(tour: &mut T, i: usize, j: usize)
where
    T: At<D1, usize> + MutAt<D1, usize>,
{
    let mut i = i + 1;
    let mut j = j;
    while i < j {
        let t = tour.at(i);
        *tour.mut_at(i) = tour.at(j);
        *tour.mut_at(j) = t;
        i += 1;
        j -= 1;
    }
}

fn two_opt<D, T>(distances: &D, mut tour: T) -> u32
where
    D: At<D2, u32>,
    T: At<D1, usize> + MutAt<D1, usize>,
{
    let mut improvement = 0;
    let d = distances;
    let n = tour.card([]);

    let mut improved = true;
    while improved {
        improved = false;

        for i in 0..(n - 1) {
            let i1 = tour.at(i);
            let i2 = tour.at(i + 1);

            for j in (i + 2)..n {
                let j1 = tour.at(j);
                let j2 = tour.at((j + 1) % n);

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
