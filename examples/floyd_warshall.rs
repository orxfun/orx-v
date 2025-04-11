use orx_v::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

// ALGORITHM IMPLEMENTATION

type Cost = u32;

const INF: Cost = Cost::MAX;

#[derive(Clone, Copy)]
struct Edge {
    head: usize,
    cost: Cost,
}

fn floyd_warshall(graph: impl V2<Edge>, distances: &mut impl MatrixMut<Cost>) {
    let n = graph.num_children();
    let d = distances;

    d.reset_all(INF);
    for (tail, node) in graph.children().enumerate() {
        for Edge { head, cost } in node.all() {
            *d.at_mut([tail, head]) = cost;
        }
    }

    for k in 0..n {
        for i in 0..n {
            if d.at([i, k]) < INF {
                for j in 0..n {
                    if d.at([k, j]) < INF {
                        let d_ikj = d.at([i, k]) + d.at([k, j]);

                        if d_ikj < d.at([i, j]) {
                            *d.at_mut([i, j]) = d_ikj;
                        }
                    }
                }
            }
        }
    }
}

fn random_graph(seed: u64, connectivity: f64, n: usize) -> impl V2<Edge> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let mut adj_list = vec![];

    for i in 0..n {
        let mut list = vec![];

        for j in 0..n {
            if rng.random_bool(connectivity) {
                let begin = i.saturating_sub(15) as u32;
                let end = i.saturating_add(15) as u32;
                list.push(Edge {
                    head: j,
                    cost: rng.random_range(begin..end),
                });
            }
        }

        adj_list.push(list);
    }

    adj_list
}

// let's call the algorithm

fn with_sparse_output(graph: impl V2<Edge>) {
    let n = graph.card([]);

    let mut distances = V.d1().sparse(INF).bounded(n * n);

    let mut matrix = distances.v1_as_matrix_mut(n, n);
    floyd_warshall(&graph, &mut matrix);
}

fn with_d1_output(graph: impl V2<Edge>) {
    let n = graph.card([]);

    let mut distances = vec![0; n * n];

    let mut matrix = distances.v1_as_matrix_mut(n, n);
    floyd_warshall(&graph, &mut matrix);
}
fn with_d1_output_col_major(graph: impl V2<Edge>) {
    let n = graph.card([]);

    let mut distances = vec![0; n * n];

    let mut matrix = distances.v1_as_matrix_col_major_mut(n, n);
    floyd_warshall(&graph, &mut matrix);
}

fn with_d2_output(graph: impl V2<Edge>) {
    let n = graph.card([]);

    let mut distances = vec![vec![0; n]; n];

    let mut matrix = distances.as_matrix_mut();
    floyd_warshall(&graph, &mut matrix);
}
fn with_d2_output_col_major(graph: impl V2<Edge>) {
    let n = graph.card([]);

    let mut distances = vec![vec![0; n]; n];

    let mut matrix = distances.as_matrix_col_major_mut();
    floyd_warshall(&graph, &mut matrix);
}

fn main() {
    let n = 100;
    let graph = random_graph(42, 0.005, n);

    with_sparse_output(&graph);

    with_d1_output(&graph);
    with_d1_output_col_major(&graph);

    with_d2_output(&graph);
    with_d2_output_col_major(&graph);
}
