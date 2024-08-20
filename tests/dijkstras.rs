use std::collections::{BTreeMap, BTreeSet};

use orx_nvec::*;
use orx_priority_queue::*;

// ALGORITHM IMPLEMENTATION (https://en.wikipedia.org/wiki/2-opt)

#[derive(Clone, Copy, derive_new::new)]
struct OutEdge {
    head: usize,
    weight: u32,
}

fn dijkstras<'a, Graph, OutEdges>(
    graph: &Graph,
    num_nodes: usize,
    source: usize,
    sink: usize,
) -> Option<u32>
where
    OutEdges: IntoIterator<Item = OutEdge>,
    Graph: NVec<D1, OutEdges> + 'a,
{
    let mut queue = BinaryHeapOfIndices::with_index_bound(num_nodes);
    let mut visited = vec![false; num_nodes];

    visited[source] = true;
    queue.push(source, 0);

    // iterate
    while let Some((node, cost)) = queue.pop() {
        if node == sink {
            return Some(cost);
        }

        let out_edges = graph.at(node);
        for OutEdge { head, weight } in out_edges {
            if !visited[head] {
                queue.try_decrease_key_or_push(&head, cost + weight);
            }
        }
        visited[node] = true;
    }

    None
}

// GRAPH VARIANTS

#[test]
fn vec_adjacency_matrix() {
    let matrix = vec![
        /*0*/ vec![1_000, 2, 5, 1_000],
        /*1*/ vec![1_000, 1_000, 1, 4],
        /*2*/ vec![1_000, 1_000, 1_000, 2],
        /*3*/ vec![1_000, 1_000, 1_000, 1_000],
    ];

    let edges_from = (|i: usize| {
        matrix[i]
            .iter()
            .enumerate()
            .map(|(head, weight)| OutEdge::new(head, *weight))
    })
    .to_funvec();

    let d03 = dijkstras(&edges_from, 4, 0, 3);
    assert_eq!(d03, Some(5));

    let distance_1_0 = dijkstras(&edges_from, 4, 1, 0);
    assert_eq!(distance_1_0, Some(1_000));
}

#[test]
fn vec_adjacency_matrix_of_edges() {
    let complete = vec![
        /*0*/
        vec![
            OutEdge::new(0, 1_000),
            OutEdge::new(1, 2),
            OutEdge::new(2, 5),
            OutEdge::new(3, 1_000),
        ],
        /*1*/
        vec![
            OutEdge::new(0, 1_000),
            OutEdge::new(1, 1_000),
            OutEdge::new(2, 1),
            OutEdge::new(3, 4),
        ],
        /*2*/
        vec![
            OutEdge::new(0, 1_000),
            OutEdge::new(1, 1_000),
            OutEdge::new(2, 1_000),
            OutEdge::new(3, 2),
        ],
        /*3*/
        vec![
            OutEdge::new(0, 1_000),
            OutEdge::new(1, 1_000),
            OutEdge::new(2, 1_000),
            OutEdge::new(3, 1_000),
        ],
    ];

    let graph = (|i: usize| complete[i].iter().copied()).to_funvec();

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(5));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, Some(1_000));
}

#[test]
fn vec_sparse_graph() {
    let sparse = vec![
        /*0*/ vec![OutEdge::new(1, 2), OutEdge::new(2, 5)],
        /*1*/ vec![OutEdge::new(2, 1), OutEdge::new(3, 4)],
        /*2*/ vec![OutEdge::new(3, 2)],
        /*3*/ vec![],
    ];

    let graph = FunVecBuilder::d1().new(|i: usize| sparse[i].iter().copied());

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(5));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, None);
}

#[test]
fn map_sparse_graph() {
    let sparse: BTreeMap<usize, Vec<OutEdge>> = [
        /*0*/ (0, vec![OutEdge::new(1, 2), OutEdge::new(2, 5)]),
        /*1*/ (1, vec![OutEdge::new(2, 1), OutEdge::new(3, 4)]),
        /*2*/ (2, vec![OutEdge::new(3, 2)]),
    ]
    .into_iter()
    .collect();

    let graph = FunVecBuilder::d1().new(|i: usize| {
        sparse
            .get(&i)
            .map(|x| x.iter().copied())
            .unwrap_or_default()
    });

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(5));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, None);
}

#[test]
fn arcs_list() {
    let list: Vec<(usize, usize, u32)> =
        vec![(0, 1, 2), (0, 2, 5), (1, 2, 1), (1, 3, 4), (2, 3, 2)];

    let graph = FunVecBuilder::d1().new(|i: usize| {
        list.iter()
            .filter(move |x| x.0 == i)
            .map(|x| OutEdge::new(x.1, x.2))
    });

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(5));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, None);
}

#[test]
fn fully_connected_uniform() {
    let graph = FunVecBuilder::d1().new(|_: usize| (0..4).map(|j| OutEdge::new(j, 1)));

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(1));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, Some(1));
}

#[test]
fn sparse_uniform() {
    let edges: BTreeSet<(usize, usize)> = [(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]
        .into_iter()
        .collect();
    let edges_ref = &edges;

    let graph = FunVecBuilder::d1().new(|i: usize| {
        (0..4)
            .filter(move |j| edges_ref.contains(&(i, *j)))
            .map(|j| OutEdge::new(j, 1))
    });

    let distance_0_3 = dijkstras(&graph, 4, 0, 3);
    assert_eq!(distance_0_3, Some(2));

    let distance_1_0 = dijkstras(&graph, 4, 1, 0);
    assert_eq!(distance_1_0, None);
}
