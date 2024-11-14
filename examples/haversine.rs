use orx_v::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::f64::consts::PI;

// implementation taken and adapted from TheAlgorithms repository:
// https://github.com/TheAlgorithms/Rust/blob/master/src/navigation/haversine.rs

const EARTH_RADIUS: f64 = 6371000.00;

fn haversine(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let delta_dist_lat = (lat2 - lat1) * PI / 180.0;
    let delta_dist_lng = (lng2 - lng1) * PI / 180.0;

    let cos1 = lat1 * PI / 180.0;
    let cos2 = lat2 * PI / 180.0;

    let delta_lat = (delta_dist_lat / 2.0).sin().powf(2.0);
    let delta_lng = (delta_dist_lng / 2.0).sin().powf(2.0);

    let a = delta_lat + delta_lng * cos1.cos() * cos2.cos();
    let result = 2.0 * a.asin().sqrt();

    result * EARTH_RADIUS
}

struct Location {
    lat: f64,
    lng: f64,
}

fn path_distance(path: impl V1<usize>, distance_matrix: impl Matrix<f64>) -> f64 {
    match path.try_at(0) {
        None => 0.0,
        Some(mut a) => {
            let mut distance = 0.0;
            for b in path.all().skip(1) {
                distance += distance_matrix.at([a, b]);
                a = b;
            }
            distance
        }
    }
}

fn calc_avg_distance(paths: &[Vec<usize>], distance_matrix: impl Matrix<f64>) -> u64 {
    (paths
        .iter()
        .map(|p| path_distance(p, &distance_matrix))
        .sum::<f64>()
        / paths.len() as f64) as u64
}

fn random_locations(rng: &mut ChaCha8Rng, n: usize) -> Vec<Location> {
    (0..n)
        .map(|_| Location {
            lat: -90.0 + 180.0 * rng.gen::<f64>(),
            lng: -180.0 + 360.0 * rng.gen::<f64>(),
        })
        .collect()
}

fn random_path(rng: &mut ChaCha8Rng, n: usize) -> Vec<usize> {
    let path_len = rng.gen_range(0..n);
    (0..path_len).map(|_| rng.gen_range(0..n)).collect()
}

fn random_paths(rng: &mut ChaCha8Rng, n: usize, num_paths: usize) -> Vec<Vec<usize>> {
    (0..num_paths).map(|_| random_path(rng, n)).collect()
}

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(468);

    let n = 100;
    let num_paths = 1000;
    let locations = random_locations(&mut rng, n);
    let paths = random_paths(&mut rng, n, num_paths);

    // matrix from Vec<Vec<f64>>
    let mut full_storage_d2 = vec![vec![0.0; n]; n];
    let mut full_matrix_d2 = full_storage_d2.v2_as_matrix_mut();
    for (i, l1) in locations.iter().enumerate() {
        let mut row = full_matrix_d2.row_mut(i);
        for (j, l2) in locations.iter().enumerate() {
            *row.at_mut(j) = haversine(l1.lat, l1.lng, l2.lat, l2.lng);
        }
    }
    let avg_dist = calc_avg_distance(&paths, full_matrix_d2);
    println!("Average path distance = {}", avg_dist);

    // matrix from Vec<f64>
    let mut flat_mat = vec![0.0; n * n];
    let mut full_matrix_d1 = flat_mat.v1_as_matrix_mut(n, n);
    for (i, l1) in locations.iter().enumerate() {
        let mut row = full_matrix_d1.row_mut(i);
        for (j, l2) in locations.iter().enumerate() {
            *row.at_mut(j) = haversine(l1.lat, l1.lng, l2.lat, l2.lng);
        }
    }
    let avg = calc_avg_distance(&paths, full_matrix_d1);
    assert_eq!(avg, avg_dist);

    // matrix from fun -> no allocation
    let fun_vec_d2 = V
        .d2()
        .fun(|[i, j]| {
            let (l1, l2) = (&locations[i], &locations[j]);
            haversine(l1.lat, l1.lng, l2.lat, l2.lng)
        })
        .with_rectangular_bounds([n, n]);
    let fun_mat_d2 = fun_vec_d2.v2_as_matrix();
    let avg = calc_avg_distance(&paths, fun_mat_d2);
    assert_eq!(avg, avg_dist);

    // matrix from fun with cache -> stores elements on demand
    let cached_fun_vec_d2 = V
        .d2()
        .fun(|[i, j]| {
            let (l1, l2) = (&locations[i], &locations[j]);
            haversine(l1.lat, l1.lng, l2.lat, l2.lng)
        })
        .with_rectangular_bounds([n, n])
        .into_cached();
    let cached_fun_mat_d2 = cached_fun_vec_d2.v2_as_matrix();
    let avg = calc_avg_distance(&paths, cached_fun_mat_d2);
    assert_eq!(avg, avg_dist);
}
