use num_traits::{Bounded, Zero};
use orx_nvec::*;
use std::collections::BTreeMap;
use std::ops::Add;

// NVec impl with INF

#[derive(Clone, Copy, derive_new::new)]
struct OutEdge<E> {
    head: usize,
    weight: E,
}

fn floyd_warshall_nvec<E, OutEdges, Graph, Out>(num_vertices: usize, graph: &Graph, d: &mut Out)
where
    E: Ord + Copy + Add<Output = E> + Zero + Bounded,
    OutEdges: IntoIterator<Item = OutEdge<E>>,
    Graph: NVec<D1, OutEdges>,
    Out: NVec<D2, E> + NVecMut<D2, E>,
{
    let inf: E = Bounded::max_value();

    for u in 0..num_vertices {
        d.set([u, u], Zero::zero());

        let out_edges = graph.at(u);
        for OutEdge { head, weight } in out_edges {
            d.set([u, head], weight);
        }
    }

    for k in 0..num_vertices {
        for i in 0..num_vertices {
            if d.at([i, k]) < inf {
                for j in 0..num_vertices {
                    if i != j && d.at([k, j]) < inf {
                        let d_ikj = d.at([i, k]) + d.at([k, j]);
                        if d.at([i, j]) > d_ikj {
                            d.set([i, j], d_ikj);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn validate_floyd_warshall_dense_output() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(2, 12), OutEdge::new(3, 60)],
        /*1*/ vec![OutEdge::new(0, 10)],
        /*2*/ vec![OutEdge::new(1, 20), OutEdge::new(3, 32)],
        /*3*/ vec![],
        /*4*/ vec![OutEdge::new(0, 7)],
    ];
    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let num_vertices = 5;

    let mut d: Vec<Vec<i32>> = vec![vec![i32::max_value(); num_vertices]; num_vertices];

    floyd_warshall_nvec(num_vertices, &graph, &mut d);
    validate_shortest_distances(num_vertices, &d);
}

#[test]
fn validate_floyd_warshall_sparse_output() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(2, 12), OutEdge::new(3, 60)],
        /*1*/ vec![OutEdge::new(0, 10)],
        /*2*/ vec![OutEdge::new(1, 20), OutEdge::new(3, 32)],
        /*3*/ vec![],
        /*4*/ vec![OutEdge::new(0, 7)],
    ];
    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let num_vertices = 5;

    let mut buffer: BTreeMap<[usize; 2], i32> = BTreeMap::new();
    let mut d = (&mut buffer).into_completed(i32::max_value());

    floyd_warshall_nvec(num_vertices, &graph, &mut d);
    validate_shortest_distances(num_vertices, &d);

    let sparse_output_len = buffer.len();
    assert_eq!(sparse_output_len, 18);
}

#[test]
fn validate_floyd_warshall_another_sparse_output() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(2, 12), OutEdge::new(3, 60)],
        /*1*/ vec![OutEdge::new(0, 10)],
        /*2*/ vec![OutEdge::new(1, 20), OutEdge::new(3, 32)],
        /*3*/ vec![],
        /*4*/ vec![OutEdge::new(0, 7)],
    ];
    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let num_vertices = 5;

    let mut buffer: Vec<BTreeMap<usize, i32>> = vec![BTreeMap::new(); num_vertices];
    let mut d = (&mut buffer).into_completed(i32::max_value());

    floyd_warshall_nvec(num_vertices, &graph, &mut d);
    validate_shortest_distances(num_vertices, &d);

    let sparse_output_len: usize = buffer.iter().map(|x| x.len()).sum();
    assert_eq!(sparse_output_len, 18);
}

fn validate_shortest_distances(num_vertices: usize, d: &impl NVec<D2, i32>) {
    let expected: BTreeMap<[usize; 2], i32> = [
        ([0, 0], 0),
        ([0, 1], 32),
        ([0, 2], 12),
        ([0, 3], 44),
        ([1, 0], 10),
        ([1, 1], 0),
        ([1, 2], 22),
        ([1, 3], 54),
        ([2, 0], 30),
        ([2, 1], 20),
        ([2, 2], 0),
        ([2, 3], 32),
        ([3, 3], 0),
        ([4, 0], 7),
        ([4, 1], 39),
        ([4, 2], 19),
        ([4, 3], 51),
        ([4, 4], 0),
    ]
    .into_iter()
    .collect();

    for i in 0..num_vertices {
        for j in 0..num_vertices {
            match expected.get(&[i, j]) {
                Some(&w) => assert_eq!(d.at([i, j]), w),
                _ => assert_eq!(d.at([i, j]), i32::max_value()),
            }
        }
    }
}

// NVec impl with Option

fn floyd_warshall_nvec_option<E, OutEdges, Graph, Out>(
    num_vertices: usize,
    graph: &Graph,
    d: &mut Out,
) where
    E: Ord + Copy + Add<Output = E> + Zero,
    OutEdges: IntoIterator<Item = OutEdge<E>>,
    Graph: NVec<D1, OutEdges>,
    Out: NVecMut<D2, Option<E>> + NVec<D2, Option<E>>,
{
    for u in 0..num_vertices {
        d.set([u, u], Some(Zero::zero()));

        let out_edges = graph.at(u);
        for OutEdge { head, weight } in out_edges {
            d.set([u, head], Some(weight));
        }
    }

    for k in 0..num_vertices {
        for i in 0..num_vertices {
            if let Some(d_ik) = d.at([i, k]) {
                for j in (0..num_vertices).filter(|j| i != *j) {
                    if let Some(d_kj) = d.at([k, j]) {
                        let d_ikj = d_ik + d_kj;

                        match d.at([i, j]) {
                            Some(d_ij) if d_ij > d_ikj => {
                                d.set([i, j], Some(d_ikj));
                            }
                            None => d.set([i, j], Some(d_ikj)),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn validate_floyd_warshall_optional_dense_output() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(2, 12), OutEdge::new(3, 60)],
        /*1*/ vec![OutEdge::new(0, 10)],
        /*2*/ vec![OutEdge::new(1, 20), OutEdge::new(3, 32)],
        /*3*/ vec![],
        /*4*/ vec![OutEdge::new(0, 7)],
    ];
    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let num_vertices = 5;

    let mut d: Vec<Vec<Option<i32>>> = vec![vec![None; num_vertices]; num_vertices];

    floyd_warshall_nvec_option(num_vertices, &graph, &mut d);
    validate_optional_shortest_distances(num_vertices, &d);
}

#[test]
fn validate_floyd_warshall_optional_sparse_output() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(2, 12), OutEdge::new(3, 60)],
        /*1*/ vec![OutEdge::new(0, 10)],
        /*2*/ vec![OutEdge::new(1, 20), OutEdge::new(3, 32)],
        /*3*/ vec![],
        /*4*/ vec![OutEdge::new(0, 7)],
    ];
    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let num_vertices = 5;

    let mut buffer: BTreeMap<[usize; 2], Option<i32>> = BTreeMap::new();
    let mut d = (&mut buffer).into_completed(None);

    floyd_warshall_nvec_option(num_vertices, &graph, &mut d);
    validate_optional_shortest_distances(num_vertices, &d);

    let sparse_buffer_len = buffer.len();
    assert_eq!(sparse_buffer_len, 18);
}

fn validate_optional_shortest_distances(num_vertices: usize, d: &impl NVec<D2, Option<i32>>) {
    let expected: BTreeMap<[usize; 2], i32> = [
        ([0, 0], 0),
        ([0, 1], 32),
        ([0, 2], 12),
        ([0, 3], 44),
        ([1, 0], 10),
        ([1, 1], 0),
        ([1, 2], 22),
        ([1, 3], 54),
        ([2, 0], 30),
        ([2, 1], 20),
        ([2, 2], 0),
        ([2, 3], 32),
        ([3, 3], 0),
        ([4, 0], 7),
        ([4, 1], 39),
        ([4, 2], 19),
        ([4, 3], 51),
        ([4, 4], 0),
    ]
    .into_iter()
    .collect();

    for i in 0..num_vertices {
        for j in 0..num_vertices {
            match expected.get(&[i, j]) {
                Some(&w) => assert_eq!(d.at([i, j]), Some(w)),
                _ => assert_eq!(d.at([i, j]), None),
            }
        }
    }
}

// Example taken from TheAlgorithms

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// https://github.com/TheAlgorithms/Rust/blob/master/src/graph/floyd_warshall.rs
///
/// Performs the Floyd-Warshall algorithm on the input graph.\
/// The graph is a weighted, directed graph with no negative cycles.
///
/// Returns a map storing the distance from each node to all the others.\
/// i.e. For each vertex `u`, `map[u][v] == Some(distance)` means
/// distance is the sum of the weights of the edges on the shortest path
/// from `u` to `v`.
///
/// For a key `v`, if `map[v].len() == 0`, then `v` cannot reach any other vertex, but is in the graph
/// (island node, or sink in the case of a directed graph)
pub fn floyd_warshall_from_algorithms<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + Zero>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();
    for (u, edges) in graph.iter() {
        if !map.contains_key(u) {
            map.insert(*u, BTreeMap::new());
        }
        map.entry(*u).or_default().insert(*u, Zero::zero());
        for (v, weight) in edges.iter() {
            if !map.contains_key(v) {
                map.insert(*v, BTreeMap::new());
            }
            map.entry(*v).or_default().insert(*v, Zero::zero());
            map.entry(*u).and_modify(|mp| {
                mp.insert(*v, *weight);
            });
        }
    }
    let keys = map.keys().copied().collect::<Vec<_>>();
    for &k in &keys {
        for &i in &keys {
            if !map[&i].contains_key(&k) {
                continue;
            }
            for &j in &keys {
                if i == j {
                    continue;
                }
                if !map[&k].contains_key(&j) {
                    continue;
                }
                let entry_i_j = map[&i].get(&j);
                let entry_i_k = map[&i][&k];
                let entry_k_j = map[&k][&j];
                match entry_i_j {
                    Some(&e) => {
                        if e > entry_i_k + entry_k_j {
                            map.entry(i).or_default().insert(j, entry_i_k + entry_k_j);
                        }
                    }
                    None => {
                        map.entry(i).or_default().insert(j, entry_i_k + entry_k_j);
                    }
                };
            }
        }
    }
    map
}

#[test]
fn validate_floyd_warshall_from_algorithms() {
    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
    }

    let mut graph = BTreeMap::new();
    add_edge(&mut graph, 0, 2, 12);
    add_edge(&mut graph, 0, 3, 60);
    add_edge(&mut graph, 1, 0, 10);
    add_edge(&mut graph, 2, 1, 20);
    add_edge(&mut graph, 2, 3, 32);
    add_edge(&mut graph, 4, 0, 7);

    let mut dists_a = BTreeMap::new();
    dists_a.insert(3, BTreeMap::new());

    dists_a.entry(0).or_insert(BTreeMap::new()).insert(0, 0);
    dists_a.entry(1).or_insert(BTreeMap::new()).insert(1, 0);
    dists_a.entry(2).or_insert(BTreeMap::new()).insert(2, 0);
    dists_a.entry(3).or_insert(BTreeMap::new()).insert(3, 0);
    dists_a.entry(4).or_insert(BTreeMap::new()).insert(4, 0);
    dists_a.entry(0).or_insert(BTreeMap::new()).insert(2, 12);
    dists_a.entry(2).or_insert(BTreeMap::new()).insert(0, 30);
    dists_a.entry(2).or_insert(BTreeMap::new()).insert(1, 20);
    dists_a.entry(2).or_insert(BTreeMap::new()).insert(3, 32);
    dists_a.entry(4).or_insert(BTreeMap::new()).insert(0, 7);
    dists_a.entry(1).or_insert(BTreeMap::new()).insert(0, 10);
    dists_a.entry(0).or_insert(BTreeMap::new()).insert(3, 44);
    dists_a.entry(0).or_insert(BTreeMap::new()).insert(1, 32);
    dists_a.entry(0).or_insert(BTreeMap::new()).insert(1, 32);
    dists_a.entry(1).or_insert(BTreeMap::new()).insert(2, 22);

    dists_a.entry(1).or_insert(BTreeMap::new()).insert(3, 54);
    dists_a.entry(4).or_insert(BTreeMap::new()).insert(2, 19);
    dists_a.entry(4).or_insert(BTreeMap::new()).insert(3, 51);
    dists_a.entry(4).or_insert(BTreeMap::new()).insert(1, 39);

    assert_eq!(floyd_warshall_from_algorithms(&graph), dists_a);
}
