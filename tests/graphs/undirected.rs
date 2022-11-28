#[cfg(test)]
mod tests {
    use causal_hub::{
        graphs::{DefaultGraph, UndirectedDenseMatrixGraph, UndirectedGraph},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn neighbors() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), (&str, Vec<&str>)); 5] = [
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), ("A", vec![])),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]]), ("A", vec!["A"])),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]]), ("A", vec!["B"])),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, true], [true, false]]), ("B", vec!["A"])),
            // Large vertices set.
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
            let g = UndirectedDenseMatrixGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(g.neighbors(&test_x.to_string()).eq(test_neighbors.into_iter()));
        }
    }

    #[test]
    fn is_neighbor() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), Vec<(&str, &str)>); 6] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]]), vec![("A", "A")]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]]), vec![("A", "B")]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, true], [true, false]]), vec![("A", "B")]),
            // Large vertices set.
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
            let g = UndirectedDenseMatrixGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(test_edges
                .into_iter()
                .all(|(x, y)| g.is_neighbor(&x.to_string(), &y.to_string())));
        }
    }

    #[test]
    fn degree() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), (&str, usize)); 5] = [
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), ("A", 0)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]]), ("A", 1)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]]), ("A", 1)),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, true], [true, false]]), ("B", 1)),
            // Large vertices set.
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
                ("A", 2),
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), (test_x, test_degree)) in data {
            // ... construct the graph ...
            let g = UndirectedDenseMatrixGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.degree(&test_x.to_string()), test_degree);
        }
    }
}
