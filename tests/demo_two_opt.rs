use orx_nvec::*;
use std::collections::{BTreeMap, HashMap};

// ALGORITHM IMPLEMENTATION (https://en.wikipedia.org/wiki/2-opt)

/// Applies the two-opt move on the given path
fn apply_two_opt(path: &mut [usize], i: usize, j: usize) {
    path[(i + 1)..(j + 1)].reverse()
}

/// Applies two-opt moves one after the other; until there is no possibility to improve the path
fn two_opt<W>(weights: &W, path: &mut [usize]) -> u32
where
    W: NVec<D2, u32>,
{
    let (mut improved, mut total_gain) = (true, 0);
    let n = path.len();

    while improved {
        improved = false;

        for ci in 0..(n - 1) {
            for cj in (ci + 1)..n {
                if let Some(gain) = two_opt_gain(weights, path, ci, cj) {
                    apply_two_opt(path, ci, cj);
                    (improved, total_gain) = (true, total_gain + gain);
                }
            }
        }
    }

    total_gain
}

fn two_opt_gain<W>(w: &W, path: &[usize], ci: usize, cj: usize) -> Option<u32>
where
    W: NVec<D2, u32>,
{
    let (i, i2) = (path[ci], path[(ci + 1) % path.len()]);
    let (j, j2) = (path[cj], path[(cj + 1) % path.len()]);

    let removed = w.at([i, i2]) + w.at([j, j2]);
    let added = w.at([i, j]) + w.at([i2, j2]);

    match removed > added {
        true => Some(removed - added),
        false => None,
    }
}

// WEIGHT VARIANTS

#[test]
fn full_matrix() {
    let weights: Vec<Vec<u32>> = vec![
        vec![0, 9, 1, 3],
        vec![2, 0, 7, 3],
        vec![1, 4, 0, 11],
        vec![3, 6, 2, 0],
    ];

    let mut path = vec![0, 1, 2, 3];
    let improvement = two_opt(&weights, &mut path);
    assert!(improvement > 0);
}

#[test]
fn flat_full_matrix() {
    let weights: Vec<u32> = vec![0, 9, 1, 3, 2, 0, 7, 3, 1, 4, 0, 11, 3, 6, 2, 0];

    let mut path = vec![0, 1, 2, 3];

    let improvement = two_opt(&weights.as_row_major_matrix(4), &mut path);
    assert!(improvement > 0);
}

#[test]
fn map_of_indices_to_weight() {
    let weights: BTreeMap<[usize; 2], u32> = [
        ([0, 1], 13),
        ([0, 2], 3),
        ([1, 2], 11),
        ([1, 3], 1),
        ([2, 3], 7),
        ([2, 1], 4),
        ([3, 0], 8),
        ([3, 1], 23),
    ]
    .into_iter()
    .collect();

    let mut path = vec![0, 1, 2, 3];
    let improvement = two_opt(&weights.into_completed(1_000), &mut path);
    assert!(improvement > 0);
}

#[test]
fn euclidean() {
    #[derive(derive_new::new)]
    struct Pnt {
        x: i32,
        y: i32,
    }

    fn euclidean_distance(a: &Pnt, b: &Pnt) -> u32 {
        (((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f64).sqrt() as u32
    }

    let points = vec![
        Pnt::new(3, 4),
        Pnt::new(-5, 18),
        Pnt::new(4, 4),
        Pnt::new(-10, 15),
    ];

    let fun = (|(i, j)| euclidean_distance(&points[i], &points[j])).to_funvec();

    let mut path = vec![0, 1, 2, 3];

    let improvement = two_opt(&fun, &mut path);
    assert!(improvement > 0);
}

#[test]
fn euclidean_if_not_on_other_side_of_river() {
    #[derive(PartialEq)]
    struct RiverSide(bool);

    #[derive(derive_new::new)]
    struct Pnt {
        x: i32,
        y: i32,
        side: RiverSide,
    }

    fn euclidean_distance(a: &Pnt, b: &Pnt) -> u32 {
        (((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f64).sqrt() as u32
    }

    let points = vec![
        Pnt::new(3, 4, RiverSide(true)),
        Pnt::new(-5, 18, RiverSide(false)),
        Pnt::new(4, 4, RiverSide(true)),
        Pnt::new(-10, 15, RiverSide(false)),
    ];

    let river_crossing_time = 100;

    let fun = |(i, j): (usize, usize)| {
        let river_penalty = match &points[i].side == &points[j].side {
            true => 0,
            false => river_crossing_time,
        };
        euclidean_distance(&points[i], &points[j]) + river_penalty
    };

    let mut path = vec![0, 1, 2, 3];

    let improvement = two_opt(&fun.to_funvec(), &mut path);

    assert!(improvement >= 2 * river_crossing_time);
}

#[test]
fn cached_euclidean() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn euclidean_distance(a: &Point, b: &Point) -> u32 {
        (((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f64).sqrt() as u32
    }

    let points: Vec<Point> = [(3, 4), (-5, 18), (4, 4), (-10, 15)]
        .map(|p| Point { x: p.0, y: p.1 })
        .into_iter()
        .collect();

    let fun = |(i, j): (usize, usize)| euclidean_distance(&points[i], &points[j]);
    let cached = fun.to_funvec().cached();

    let mut path = vec![0, 1, 2, 3];
    let improvement = two_opt(&cached, &mut path);
    assert!(improvement > 0);

    let cache: HashMap<[usize; 2], u32> = cached.into_inner_cache();
    assert_eq!(cache.len(), 10);
}

#[test]
fn uniform_weights() {
    let mut path = vec![0, 1, 2, 3];
    let improvement = two_opt(&FunVecBuilder::d2().constant(1), &mut path);
    assert_eq!(improvement, 0);
}

#[test]
fn taboo_connections() {
    let taboo_connections = vec![(0, 2)];

    let weights = vec![
        vec![0, 9, 1, 3],
        vec![2, 0, 7, 3],
        vec![1, 4, 0, 11],
        vec![3, 6, 2, 0],
    ];

    let fun = (|(i, j)| match taboo_connections.contains(&(i, j)) {
        true => 1_000,
        false => weights[i][j],
    })
    .to_funvec();

    let mut path = vec![0, 1, 2, 3];

    let improvement = two_opt(&fun, &mut path);

    assert!(improvement > 0);
}
