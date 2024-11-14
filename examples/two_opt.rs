use orx_v::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::u32;

fn apply_two_opt(tour: &mut impl V1Mut<usize>, i: usize, j: usize) {
    let mut i = i + 1;
    let mut j = j;
    while i < j {
        let t = tour.at(i);
        *tour.at_mut(i) = tour.at(j);
        *tour.at_mut(j) = t;
        i += 1;
        j -= 1;
    }
}

fn two_opt(distances: impl V2<u32>, tour: &mut impl V1Mut<usize>) -> u32 {
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
                    apply_two_opt(tour, i, j);
                }
            }
        }
    }

    improvement
}

fn complete_distance_matrix_d2(n: usize) -> Vec<Vec<u32>> {
    let mut rng = ChaCha8Rng::seed_from_u64(75);
    let mut mat = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let d = rng.gen_range(1..20);
            mat[i][j] = d;
            mat[j][i] = d;
        }
    }
    mat
}

#[cfg(feature = "ndarray")]
fn complete_ndarray_d2(n: usize) -> ndarray::Array2<u32> {
    let mut rng = ChaCha8Rng::seed_from_u64(512);
    let mut mat = ndarray::Array2::zeros((n, n));
    for i in 0..n {
        for j in (i + 1)..n {
            let d = rng.gen_range(1..20);
            mat[[i, j]] = d;
            mat[[j, i]] = d;
        }
    }
    mat
}

fn complete_distance_matrix_d1(n: usize) -> Vec<u32> {
    let mut rng = ChaCha8Rng::seed_from_u64(8);
    let mut mat = vec![0; n * n];
    for i in 0..n {
        for j in (i + 1)..n {
            let d = rng.gen_range(1..20);
            mat[i * n + j] = d;
            mat[j * n + i] = d;
        }
    }
    mat
}

#[cfg(feature = "ndarray")]
fn complete_ndarray_d1(n: usize) -> ndarray::Array1<u32> {
    let mut rng = ChaCha8Rng::seed_from_u64(8);
    let mut mat = ndarray::Array1::zeros(n * n);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = rng.gen_range(1..20);
            mat[i * n + j] = d;
            mat[j * n + i] = d;
        }
    }
    mat
}

#[cfg(feature = "std")]
fn finite_distances_map(n: usize) -> std::collections::HashMap<[usize; 2], u32> {
    let mut rng = ChaCha8Rng::seed_from_u64(89);
    let num_finite = 5 * n;
    (0..num_finite)
        .flat_map(|_| {
            let i = rng.gen_range(0..n);
            let j = rng.gen_range(0..n);
            let distance = rng.gen_range(1..20);
            [([i, j], distance), ([j, i], distance)]
        })
        .collect()
}

struct Location(i32, i32);

fn euclidean_distance(a: &Location, b: &Location) -> u32 {
    (((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64).sqrt() as u32
}

fn routing_engine(a: &Location, b: &Location) -> u32 {
    // assume an api call
    (((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)) as f64).sqrt() as u32
}

fn get_locations(n: usize) -> Vec<Location> {
    let mut rng = ChaCha8Rng::seed_from_u64(33);
    (0..n)
        .map(|_| Location(rng.gen_range(-20..20), rng.gen_range(-20..20)))
        .collect()
}

fn initial_tour(n: usize) -> Vec<usize> {
    (0..n).collect()
}

fn main() {
    let n = 100;

    // complete matrix stored as a V2
    {
        // Vec<Vec<u32>>
        let distances: Vec<Vec<u32>> = complete_distance_matrix_d2(n);
        let mut tour: Vec<_> = initial_tour(n);
        let _improvement = two_opt(&distances, &mut tour);

        #[cfg(feature = "ndarray")]
        {
            // ndarray::Array2
            let distances: ndarray::Array2<u32> = complete_ndarray_d2(n);
            let mut tour: Vec<_> = initial_tour(n);
            let _improvement = two_opt(&distances, &mut tour);
        }
    }

    // complete matrix stored as a flattened V1
    {
        // Vec<u32> as flattened matrix
        let distances: Vec<u32> = complete_distance_matrix_d1(n);
        let mut tour: Vec<_> = initial_tour(n);
        let _improvement = two_opt(distances.as_jagged_with_uniform_lengths(n), &mut tour);

        #[cfg(feature = "ndarray")]
        {
            // ndarray::Array1 as flattened matrix
            let distances: ndarray::Array1<u32> = complete_ndarray_d1(n);
            let mut tour: Vec<_> = initial_tour(n);
            let _improvement = two_opt(distances.as_jagged_with_uniform_lengths(n), &mut tour);
        }
    }

    // sparse matrix
    #[cfg(feature = "std")]
    {
        use std::collections::HashMap;

        let finite_distances: HashMap<[usize; 2], u32> = finite_distances_map(n);
        let distances = V.d2().sparse_from(finite_distances, 10000);
        let mut tour: Vec<_> = initial_tour(n);
        let _improvement = two_opt(&distances, &mut tour);
    }

    // functional matrix
    let locations: Vec<Location> = get_locations(n);
    let distances = V
        .d2()
        .fun(|[i, j]| euclidean_distance(&locations[i], &locations[j]));
    let mut tour: Vec<_> = initial_tour(n);
    let _improvement = two_opt(&distances, &mut tour);

    // functional matrix: ignore from-to depot (node 0) links
    let locations: Vec<Location> = get_locations(n);
    let distances = V.d2().fun(|[i, j]| match (i, j) {
        (0, _) => 0,
        (_, 0) => 0,
        _ => euclidean_distance(&locations[i], &locations[j]),
    });
    let mut tour: Vec<_> = initial_tour(n);
    let _improvement = two_opt(&distances, &mut tour);

    // cached matrix
    let locations: Vec<Location> = get_locations(n);
    let distances = V
        .d2()
        .fun(|[i, j]| routing_engine(&locations[i], &locations[j]))
        .into_cached();
    let mut tour: Vec<_> = initial_tour(n);
    let _improvement = two_opt(&distances, &mut tour);

    // uniform distances
    let distances = V.d2().constant(10);
    let mut tour: Vec<_> = initial_tour(n);
    let _improvement = two_opt(&distances, &mut tour);
}
