#[cfg(test)]
mod tests {
    use causal_hub::{
        graphs::{UndirectedDenseAdjacencyMatrixGraph, UndirectedGraph},
        types::DenseAdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn neighbors() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, DenseAdjacencyMatrix), (&str, Vec<&str>)); 5] = [
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), ("A", vec![])),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]]), ("A", vec!["A"])),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]]), ("A", vec!["B"])),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, true], [true, false]]), ("B", vec!["A"])),
            // Large vertex set.
            (
                (
                    vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"],
                    array![
                        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                    ].mapv(|x| x != 0)
                ),
                ("A", vec!["A", "C"]),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_x, test_neighbors)) in data {
            // ... construct the graph ...
            let g = UndirectedDenseAdjacencyMatrixGraph::try_from((vertices, adjacency_matrix)).unwrap();
            // ... assert result.
            assert!(g.neighbors(&test_x.into()).eq(test_neighbors.into_iter()));
        }
    }

    #[test]
    fn is_neighbor() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, DenseAdjacencyMatrix), Vec<(&str, &str)>); 6] = [
            // Empty vertex set and adjacency matrix.
            ((vec![], Default::default()), vec![]),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), vec![]),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]]), vec![("A", "A")]),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]]), vec![("A", "B")]),
            // Non-empty vertex set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, true], [true, false]]), vec![("A", "B")]),
            // Large vertex set.
            (
                (
                    vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"],
                    array![
                        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                    ].mapv(|x| x != 0)
                ),
                vec![("A", "A"), ("A", "C"), ("B", "D"), ("D", "G"), ("I", "J")],
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), test_edges) in data {
            // ... construct the graph ...
            let g = UndirectedDenseAdjacencyMatrixGraph::try_from((vertices, adjacency_matrix)).unwrap();
            // ... assert result.
            assert!(test_edges.into_iter().all(|(x, y)| g.is_neighbor(&x.into(), &y.into())));
        }
    }
}
