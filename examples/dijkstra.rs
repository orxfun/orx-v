use orx_priority_queue::*;
use orx_v::*;

// generic graph

type Cost = u32;

const INF: Cost = Cost::MAX;

#[derive(Clone, Copy)]
struct Edge {
    node: usize,
    cost: Cost,
}

struct Graph<V: V2<Edge>> {
    data: V,
}

impl<V: V2<Edge>> Graph<V> {
    fn new(data: V) -> Self {
        Self { data }
    }

    fn num_nodes(&self) -> usize {
        self.data.card([])
    }

    fn out_edges(&self, i: usize) -> impl V1<Edge> + '_ {
        self.data.child(i)
    }
}

fn from_costs_matrix(matrix: impl V2<Cost>) -> Graph<impl V2<Edge>> {
    let n = matrix.card([]);
    let fun = V
        .d2()
        .fun(move |[i, j]| Edge {
            node: j,
            cost: matrix.at([i, j]),
        })
        .with_rectangular_bounds([n, n]);
    Graph { data: fun }
}

// generic dijkstra

fn dijkstra(graph: &Graph<impl V2<Edge>>, start: usize, goal: usize) -> Option<Cost> {
    let mut visited = vec![false; graph.num_nodes()];
    let mut heap = BinaryHeapOfIndices::with_index_bound(graph.num_nodes());

    heap.push(start, 0);

    while let Some((i, cost)) = heap.pop() {
        if i == goal {
            return Some(cost);
        }

        for edge in graph
            .out_edges(i)
            .all()
            .filter(|x| x.cost < INF && !visited[x.node])
        {
            heap.try_decrease_key_or_push(&edge.node, cost + edge.cost);
        }

        visited[i] = true;
    }

    None
}

// let's call

fn with_complete_2d_adjacency_matrix() {
    // could've used any slice, or std Vec or ndarray, etc.
    let costs = [
        [0, 1, 4, INF, INF],
        [INF, 0, INF, 2, INF],
        [INF, 5, 0, 89, 2],
        [INF, INF, INF, 0, 82],
        [INF, 30, 1, INF, 0],
    ];

    let graph = from_costs_matrix(&costs);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(6));
}

fn with_complete_1d_adjacency_matrix() {
    // could've used any slice, or std Vec or ndarray, etc.
    let flat_costs = [
        0, 1, 4, INF, INF, INF, 0, INF, 2, INF, INF, 5, 0, 89, 2, INF, INF, INF, 0, 82, INF, 30, 1,
        INF, 0,
    ];
    let costs = flat_costs.as_jagged_with_uniform_lengths(5);

    let graph = from_costs_matrix(&costs);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(6));
}

#[cfg(feature = "std")]
fn with_sparse_2d_adjacency_matrix() {
    // could've used BTreeMap or any basic Lookup
    let sparse_costs = std::collections::HashMap::from_iter([
        ([0, 1], 1),
        ([0, 2], 4),
        ([1, 3], 2),
        ([2, 1], 5),
        ([2, 3], 89),
        ([2, 4], 2),
        ([3, 4], 82),
        ([4, 1], 30),
        ([4, 2], 1),
    ]);
    let costs = V
        .d2()
        .sparse_from(sparse_costs, INF)
        .with_rectangular_bounds([5, 5]);

    let graph = from_costs_matrix(&costs);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(6));
}

fn with_adjacency_list() {
    let e = |node, cost| Edge { cost, node };

    let list = vec![
        /*0->*/ vec![e(1, 1), e(2, 4)],
        /*1->*/ vec![e(3, 2)],
        /*2->*/ vec![e(1, 5), e(3, 89), e(4, 2)],
        /*3->*/ vec![e(4, 82)],
        /*4->*/ vec![e(1, 30), e(2, 1)],
    ];

    let graph = Graph::new(&list);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(6));
}

fn with_edge_set_and_uniform_distances() {
    let e = |node, cost| Edge { cost, node };

    let connections: std::collections::HashSet<[usize; 2]> = [
        [0, 1],
        [0, 2],
        [1, 3],
        [2, 1],
        [2, 3],
        [2, 4],
        [3, 4],
        [4, 1],
        [4, 2],
    ]
    .into_iter()
    .collect();

    let list = V
        .d2()
        .fun(|[i, j]| match connections.contains(&[i, j]) {
            true => e(j, 1),
            false => e(j, INF),
        })
        .with_rectangular_bounds([5, 5]);

    let graph = Graph::new(&list);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(2));
}

fn with_on_demand_cost_computation() {
    fn routing_service(from: usize, to: usize) -> Cost {
        match (from + to) % 2 == 0 {
            true => from as Cost + 2 * to as Cost + 3,
            false => INF,
        }
    }

    let list = V
        .d2()
        .fun(|[i, j]| Edge {
            node: j,
            cost: routing_service(i, j),
        })
        .with_rectangular_bounds([5, 5]);

    let graph = Graph::new(&list);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(11));
}

fn with_cached_on_demand_cost_computation() {
    fn routing_service(from: usize, to: usize) -> Cost {
        match (from + to) % 2 == 0 {
            true => from as Cost + 2 * to as Cost + 3,
            false => INF,
        }
    }

    let list = V
        .d2()
        .fun(|[i, j]| Edge {
            node: j,
            cost: routing_service(i, j),
        })
        .with_rectangular_bounds([5, 5])
        .into_cached();

    let graph = Graph::new(&list);

    let distance = dijkstra(&graph, 0, 4);
    assert_eq!(distance, Some(11));
}

fn main() {
    with_complete_2d_adjacency_matrix();

    with_complete_1d_adjacency_matrix();

    #[cfg(feature = "std")]
    with_sparse_2d_adjacency_matrix();

    with_adjacency_list();

    with_edge_set_and_uniform_distances();

    with_on_demand_cost_computation();

    with_cached_on_demand_cost_computation();
}
