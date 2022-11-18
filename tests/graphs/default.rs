#[cfg(test)]
mod tests {
    use causal_hub::{
        graphs::{BaseGraph, DefaultGraph, DenseMatrixUndirectedGraph, ErrorGraph as E},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn default() {
        let g = DenseMatrixUndirectedGraph::default();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn null() {
        let g = DenseMatrixUndirectedGraph::null();

        assert_eq!(g.order(), 0);
        assert_eq!(g.size(), 0);
    }

    #[test]
    fn empty() {
        // Test database as (input, output) pairs.
        let data: [(Vec<&str>, Option<E>); 2] = [
            // Empty vertices set.
            (vec![], None),
            // Invalid vertex label.
            (vec![""], Some(E::EmptyVertexLabel)),
            // FIXME:
        ];

        // For each test case in the test database ...
        for (vertices, error) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::empty(vertices.clone());
            // ... assert result.
            match g {
                Ok(g) => {
                    assert!(g.vertices().eq(vertices.clone().into_iter()));
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
    #[ignore]
    fn complete() {
        todo!() // TODO:
    }

    #[test]
    fn with_adjacency_matrix() {
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
        for ((vertices, adjacency_matrix), error) in data.into_iter() {
            // ... assert result.
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix);

            assert_eq!(g.err(), error);
        }
    }
}
