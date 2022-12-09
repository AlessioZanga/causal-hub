#[generic_tests::define]
mod tests {
    use std::cmp::Ordering;

    use causal_hub::{
        graphs::{
            BaseGraph, DefaultGraph, DirectedDenseMatrixGraph, ErrorGraph as E, PartialOrdGraph,
            UndirectedDenseMatrixGraph,
        },
        types::DenseAdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn eq<T>()
    where
        T: BaseGraph<Vertex = String>
            + DefaultGraph
            + PartialOrdGraph
            + TryFrom<(Vec<&'static str>, DenseAdjacencyMatrix), Error = E>,
    {
        // Test database as (input, output) pairs.
        let data: [(
            (Vec<&str>, DenseAdjacencyMatrix),
            (Vec<&str>, DenseAdjacencyMatrix, bool),
        ); 6] = [
            // Empty vertex set and adjacency matrix.
            ((vec![], Default::default()), (vec![], Default::default(), true)),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec![], Default::default()), (vec!["A"], array![[false]], false)),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), (vec![], Default::default(), false)),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), (vec!["A"], array![[false]], true)),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                (vec!["A", "B"], array![[false, false], [false, false]], true),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (vec!["A", "B"], array![[false, true], [true, false]], true),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_vertices, test_adjacency_matrix, flag)) in data {
            // ... construct the graph ...
            let g = T::try_from((vertices, adjacency_matrix)).unwrap();
            let h = T::try_from((test_vertices, test_adjacency_matrix)).unwrap();
            // ... assert result.
            assert_eq!(g.eq(&h), flag);
        }
    }

    #[test]
    fn partial_cmp<T>()
    where
        T: BaseGraph<Vertex = String>
            + DefaultGraph
            + PartialOrdGraph
            + TryFrom<(Vec<&'static str>, DenseAdjacencyMatrix), Error = E>,
    {
        // Test database as (input, output) pairs.
        let data: [(
            (Vec<&str>, DenseAdjacencyMatrix),
            (Vec<&str>, DenseAdjacencyMatrix, Option<Ordering>),
        ); 7] = [
            // Empty vertex set and adjacency matrix.
            (
                (vec![], Default::default()),
                (vec![], Default::default(), Some(Ordering::Equal)),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec![], Default::default()),
                (vec!["A"], array![[false]], Some(Ordering::Less)),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                (vec![], Default::default(), Some(Ordering::Greater)),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                (vec!["A"], array![[false]], Some(Ordering::Equal)),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                (
                    vec!["A", "B"],
                    array![[false, false], [false, false]],
                    Some(Ordering::Equal),
                ),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (
                    vec!["A", "B"],
                    array![[false, true], [true, false]],
                    Some(Ordering::Equal),
                ),
            ),
            // Non-empty vertex set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                (vec!["A", "C"], array![[false, true], [true, false]], None),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_vertices, test_adjacency_matrix, ordering)) in data {
            // ... construct the graph ...
            let g = T::try_from((vertices, adjacency_matrix)).unwrap();
            let h = T::try_from((test_vertices, test_adjacency_matrix)).unwrap();
            // ... assert result.
            assert_eq!(g.partial_cmp(&h), ordering);
        }
    }

    #[instantiate_tests(<UndirectedDenseMatrixGraph>)]
    mod undirected_dense_matrix_graph {}

    #[instantiate_tests(<DirectedDenseMatrixGraph>)]
    mod directed_dense_matrix_graph {}
}
