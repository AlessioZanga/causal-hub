#[generic_tests::define]
mod tests {
    use causal_hub::{
        graphs::{BaseGraph, DefaultGraph, DirectedDenseMatrixGraph, ErrorGraph as E, UndirectedDenseMatrixGraph},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn default<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph,
    {
        let g = T::default();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn null<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph,
    {
        let g = T::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn empty<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [(Vec<&str>, Option<E>); 5] = [
            // Empty vertices set.
            (vec![], None),
            // Invalid vertex label.
            (vec![""], Some(E::EmptyVertexLabel)),
            // Non-empty vertices set.
            (vec!["A"], None),
            // Non-empty vertices set.
            (vec!["A", "B"], None),
            // Large vertices set.
            (
                vec![
                    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
                    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
                    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
                    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
                    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
                    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
                    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
                ],
                None,
            ),
        ];

        // For each test case in the test database ...
        for (vertices, error) in data {
            // ... construct the graph ...
            let g = T::empty(vertices.clone());
            // ... assert result.
            match g {
                Ok(g) => {
                    assert!(g.vertices().eq(vertices.clone()));
                    assert_eq!(g.order(), vertices.len());
                    assert_eq!(g.size(), 0);
                }
                Err(e) => {
                    assert_eq!(Some(e), error);
                }
            }
        }
    }

    #[test]
    fn complete<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [(Vec<&str>, Option<E>); 5] = [
            // Empty vertices set.
            (vec![], None),
            // Invalid vertex label.
            (vec![""], Some(E::EmptyVertexLabel)),
            // Non-empty vertices set.
            (vec!["A"], None),
            // Non-empty vertices set.
            (vec!["A", "B"], None),
            // Large vertices set.
            (
                vec![
                    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
                    "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
                    "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                    "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
                    "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
                    "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
                    "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                    "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
                    "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
                ],
                None,
            ),
        ];

        // For each test case in the test database ...
        for (vertices, error) in data {
            // ... construct the graph ...
            let g = T::complete(vertices.clone());
            // ... assert result.
            match g {
                Ok(g) => {
                    let order = vertices.len();

                    assert!(g.vertices().eq(vertices.clone()));
                    assert_eq!(g.order(), order);
                    assert_eq!(g.size(), (order * (order.saturating_sub(1))) / 2);
                }
                Err(e) => {
                    assert_eq!(Some(e), error);
                }
            }
        }
    }

    #[test]
    fn with_adjacency_matrix<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), Option<E>); 8] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), None),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), None),
            // Empty vertices set but non-empty adjacency matrix.
            ((vec![], array![[false]]), Some(E::InconsistentMatrix)),
            // Invalid vertex label and adjacency matrix.
            ((vec![""], array![[false]]), Some(E::EmptyVertexLabel)),
            // Non-empty vertices set but empty adjacency matrix.
            ((vec!["A"], Default::default()), Some(E::InconsistentMatrix)),
            // Non-square adjacency matrix.
            ((vec!["A"], array![[false, false]]), Some(E::NonSquareMatrix)),
            // Non-symmetric adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [false, false]]),
                Some(E::NonSymmetricMatrix),
            ),
            // Large vertices set.
            (
                (
                    vec![
                        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
                        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
                        "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
                        "40", "41", "42", "43", "44", "45", "46", "47", "48", "49",
                        "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
                        "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                        "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                        "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
                        "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
                    ],
                    AdjacencyMatrix::default((100, 100)),
                ),
                None,
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), error) in data {
            // ... assert result.
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix);

            assert_eq!(g.err(), error);
        }
    }

    #[instantiate_tests(<UndirectedDenseMatrixGraph>)]
    mod undirected_dense_matrix_graph {}

    #[instantiate_tests(<DirectedDenseMatrixGraph>)]
    mod directed_dense_matrix_graph {}
}
