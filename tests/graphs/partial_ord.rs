#[generic_tests::define]
mod tests {
    use std::cmp::Ordering;

    use causal_hub::{
        graphs::{BaseGraph, DefaultGraph, DirectedDenseMatrixGraph, PartialOrdGraph, UndirectedDenseMatrixGraph},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn eq<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        let data: [((Vec<&str>, AdjacencyMatrix), (Vec<&str>, AdjacencyMatrix, bool)); 6] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), (vec![], Default::default(), true)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec![], Default::default()), (vec!["A"], array![[false]], false)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), (vec![], Default::default(), false)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), (vec!["A"], array![[false]], true)),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                (vec!["A", "B"], array![[false, false], [false, false]], true),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (vec!["A", "B"], array![[false, true], [true, false]], true),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_vertices, test_adjacency_matrix, flag)) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            let h = T::with_adjacency_matrix(test_vertices, test_adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.eq(&h), flag);
        }
    }

    #[test]
    fn partial_cmp<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        let data: [(
            (Vec<&str>, AdjacencyMatrix),
            (Vec<&str>, AdjacencyMatrix, Option<Ordering>),
        ); 7] = [
            // Empty vertices set and adjacency matrix.
            (
                (vec![], Default::default()),
                (vec![], Default::default(), Some(Ordering::Equal)),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec![], Default::default()),
                (vec!["A"], array![[false]], Some(Ordering::Less)),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                (vec![], Default::default(), Some(Ordering::Greater)),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                (vec!["A"], array![[false]], Some(Ordering::Equal)),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                (
                    vec!["A", "B"],
                    array![[false, false], [false, false]],
                    Some(Ordering::Equal),
                ),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (
                    vec!["A", "B"],
                    array![[false, true], [true, false]],
                    Some(Ordering::Equal),
                ),
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (vec!["A", "C"], array![[false, true], [true, false]], None),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_vertices, test_adjacency_matrix, ordering)) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            let h = T::with_adjacency_matrix(test_vertices, test_adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.partial_cmp(&h), ordering);
        }
    }

    #[instantiate_tests(<UndirectedDenseMatrixGraph>)]
    mod undirected_dense_matrix_graph {}

    #[instantiate_tests(<DirectedDenseMatrixGraph>)]
    mod directed_dense_matrix_graph {}
}
