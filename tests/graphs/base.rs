#[cfg(test)]
mod tests {
    use causal_hub::{
        graphs::{BaseGraph, DefaultGraph, DenseMatrixUndirectedGraph},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;

    #[test]
    fn clone() {
        // Test database as (input, output) pairs.
        let data: [(Vec<&str>, AdjacencyMatrix); 4] = [
            // Empty vertices set and adjacency matrix.
            (vec![], Default::default()),
            // Non-empty vertices set and non-empty adjacency matrix.
            (vec!["A"], array![[false]]),
            // Non-empty vertices set and non-empty adjacency matrix.
            (vec!["A", "B"], array![[false, false], [false, false]]),
            // Non-empty vertices set and non-empty adjacency matrix.
            (vec!["A", "B"], array![[false, true], [true, false]]),
        ];

        // For each test case in the test database ...
        for (vertices, adjacency_matrix) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g, g.clone());
        }
    }

    #[test]
    fn debug() {
        // Test database as (input, output) pairs.
        let data: [((Vec<&str>, AdjacencyMatrix), &str); 4] = [
            // Empty vertices set and adjacency matrix.
            (
                (vec![], Default::default()),
                "DenseMatrixUndirectedGraph { vertices: {}, vertices_indexes: {}, adjacency_matrix: [[]], shape=[0, 0], strides=[0, 0], layout=CFcf (0xf), const ndim=2, size: 0 }"
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                "DenseMatrixUndirectedGraph { vertices: {\"A\"}, vertices_indexes: {\"A\" <> 0}, adjacency_matrix: [[false]], shape=[1, 1], strides=[1, 1], layout=CFcf (0xf), const ndim=2, size: 0 }"
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                "DenseMatrixUndirectedGraph { vertices: {\"A\", \"B\"}, vertices_indexes: {\"A\" <> 0, \"B\" <> 1}, adjacency_matrix: [[false, false],\n [false, false]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2, size: 0 }"
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true ], [true , false]]),
                "DenseMatrixUndirectedGraph { vertices: {\"A\", \"B\"}, vertices_indexes: {\"A\" <> 0, \"B\" <> 1}, adjacency_matrix: [[false, true],\n [true, false]], shape=[2, 2], strides=[2, 1], layout=Cc (0x5), const ndim=2, size: 1 }"
            )
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), debug) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(format!("{:?}", g), debug);
        }
    }

    #[test]
    fn display() {
        // Test database as (input, output) pairs.
        let data: [((Vec<&str>, AdjacencyMatrix), &str); 4] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), "UndirectedGraph { V = {}, E = {} }"),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), "UndirectedGraph { V = {\"A\"}, E = {} }"),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                "UndirectedGraph { V = {\"A\", \"B\"}, E = {} }",
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, true], [true, false]]),
                "UndirectedGraph { V = {\"A\", \"B\"}, E = {(\"A\", \"B\")} }",
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), display) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(format!("{}", g), display);
        }
    }

    #[test]
    fn order() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), usize); 4] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), 0),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), 1),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, false], [false, false]]), 2),
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
                100,
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), order) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.order(), order);
            assert_eq!(g.vertices().len(), order);
        }
    }

    #[test]
    fn vertices() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), Vec<&str>); 5] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), vec!["A"]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, false], [false, false]]), vec!["A", "B"]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["B", "A"], array![[false, false], [false, false]]), vec!["A", "B"]),
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
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), test_vertices) in data.into_iter() {
            // ... construct the graph ...
            let g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(g.vertices().eq(test_vertices));
        }
    }

    #[test]
    #[ignore]
    fn has_vertex() {
        todo!() // TODO:
    }

    #[test]
    fn add_vertex() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [(
            (Vec<&str>, AdjacencyMatrix, &str),
            (Vec<&str>, AdjacencyMatrix),
        ); 5] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default(), "A"), (vec!["A"], array![[false]])),
            // Non-empty vertices set and adjacency matrix.
            (
                (vec!["A"], array![[true]], "B"),
                (
                    vec!["A", "B"],
                    array![
                        [true , false],
                        [false, false]
                    ],
                )
            ),
            // Non-empty vertices set and adjacency matrix.
            (
                (vec!["B"], array![[true]], "A"),
                (
                    vec!["A", "B"],
                    array![
                        [false, false],
                        [false, true ]
                    ],
                )
            ),
            // Non-empty vertices set and adjacency matrix.
            (
                (
                    vec!["A", "C"],
                    array![
                        [true , true ],
                        [true , true ]
                    ],
                    "B"
                ),
                (
                    vec!["A", "B", "C"],
                    array![
                        [true , false, true ],
                        [false, false, false],
                        [true , false, true ]
                    ],
                )
            ),
            // Large vertices set.
            (
                (
                    vec![
                        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
                        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
                        "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
                        "40", "41",       "43", "44", "45", "46", "47", "48", "49",
                        "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
                        "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                        "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                        "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
                        "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
                    ],
                    AdjacencyMatrix::from_elem((99, 99), true),
                    "42"
                ),
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
                    {
                        let mut adjacency_matrix = AdjacencyMatrix::from_elem((100, 100), true);
                        adjacency_matrix.slice_mut(s![42, ..]).fill(false);
                        adjacency_matrix.slice_mut(s![.., 42]).fill(false);
                        adjacency_matrix
                    }
                )
            )
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix, x), (test_vertices, test_adjacency_matrix)) in data.into_iter() {
            // ... construct the graph ...
            let mut g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            let x = g.add_vertex(x);

            assert!(g.has_vertex(&x));
            assert!(g.vertices().eq(test_vertices));
            assert_eq!(*g, test_adjacency_matrix);
        }
    }

    #[test]
    fn del_vertex() {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [(
            (Vec<&str>, AdjacencyMatrix, &str),
            (Vec<&str>, AdjacencyMatrix),
        ); 5] = [
            // Empty vertices set and adjacency matrix.
            ((vec!["A"], array![[false]], "A"), (vec![], Default::default())),
            // Non-empty vertices set and adjacency matrix.
            (
                (
                    vec!["A", "B"],
                    array![
                        [true , false],
                        [false, false]
                    ],
                    "B"
                ),
                (vec!["A"], array![[true]])
            ),
            // Non-empty vertices set and adjacency matrix.
            (
                (
                    vec!["A", "B"],
                    array![
                        [false, false],
                        [false, true ]
                    ],
                    "A"
                ),
                (vec!["B"], array![[true]])
            ),
            // Non-empty vertices set and adjacency matrix.
            (
                (
                    vec!["A", "B", "C"],
                    array![
                        [true , false, true ],
                        [false, false, false],
                        [true , false, true ]
                    ],
                    "B"
                ),
                (
                    vec!["A", "C"],
                    array![
                        [true , true ],
                        [true , true ]
                    ],
                )
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
                    {
                        let mut adjacency_matrix = AdjacencyMatrix::from_elem((100, 100), true);
                        adjacency_matrix.slice_mut(s![42, ..]).fill(false);
                        adjacency_matrix.slice_mut(s![.., 42]).fill(false);
                        adjacency_matrix
                    },
                    "42"
                ),
                (
                    vec![
                        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09",
                        "10", "11", "12", "13", "14", "15", "16", "17", "18", "19",
                        "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
                        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39",
                        "40", "41",       "43", "44", "45", "46", "47", "48", "49",
                        "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
                        "60", "61", "62", "63", "64", "65", "66", "67", "68", "69",
                        "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
                        "80", "81", "82", "83", "84", "85", "86", "87", "88", "89",
                        "90", "91", "92", "93", "94", "95", "96", "97", "98", "99",
                    ],
                    AdjacencyMatrix::from_elem((99, 99), true),
                )
            )
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix, x), (test_vertices, test_adjacency_matrix)) in data.into_iter() {
            // ... construct the graph ...
            let mut g = DenseMatrixUndirectedGraph::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            let x: String = x.into();

            g.del_vertex(&x);

            assert!(!g.has_vertex(&x));
            assert!(g.vertices().eq(test_vertices));
            assert_eq!(*g, test_adjacency_matrix);
        }
    }

    #[test]
    #[ignore]
    fn size() {
        todo!() // TODO:
    }

    #[test]
    #[ignore]
    fn edges() {
        todo!() // TODO:
    }

    #[test]
    #[ignore]
    fn has_edge() {
        todo!() // TODO:
    }

    #[test]
    #[ignore]
    fn add_edge() {
        todo!() // TODO:
    }

    #[test]
    #[ignore]
    fn del_edge() {
        todo!() // TODO:
    }

    #[test]
    #[ignore]
    fn is_adjacent() {
        todo!() // TODO:
    }
}
