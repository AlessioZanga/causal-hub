#[generic_tests::define]
mod tests {
    use causal_hub::{
        graphs::{BaseGraph, DefaultGraph, DirectedDenseMatrixGraph, PartialOrdGraph, UndirectedDenseMatrixGraph},
        types::AdjacencyMatrix,
    };
    use ndarray::prelude::*;
    use regex::Regex;

    #[test]
    fn clone<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for (vertices, adjacency_matrix) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g, g.clone());
        }
    }

    #[test]
    fn debug<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        let data: [((Vec<&str>, AdjacencyMatrix), &str); 3] = [
            // Empty vertices set and adjacency matrix.
            (
                (vec![], Default::default()),
                r#"[a-zA-Z]+Graph \{ vertices: \{\}, vertices_indexes: \{\}, adjacency_matrix: \[\[\]\], shape=\[0, 0\], strides=\[0, 0\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                r#"[a-zA-Z]+Graph \{ vertices: \{"A"\}, vertices_indexes: \{"A" <> 0\}, adjacency_matrix: \[\[false\]\], shape=\[1, 1\], strides=\[1, 1\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                r#"[a-zA-Z]+Graph \{ vertices: \{"A", "B"\}, vertices_indexes: \{"A" <> 0, "B" <> 1\}, adjacency_matrix: \[\[false, false\],\n \[false, false\]\], shape=\[2, 2\], strides=\[2, 1\], layout=Cc \(0x5\), const ndim=2, size: 0 \}"#,
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), test_debug) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(Regex::new(test_debug).unwrap().is_match(&*format!("{:?}", g)));
        }
    }

    #[test]
    fn display<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        let data: [((Vec<&str>, AdjacencyMatrix), &str); 3] = [
            // Empty vertices set and adjacency matrix.
            (
                (vec![], Default::default()),
                r#"[a-zA-Z]+Graph \{ V = \{\}, E = \{\} \}"#,
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A"], array![[false]]),
                r#"[a-zA-Z]+Graph \{ V = \{"A"\}, E = \{\} \}"#,
            ),
            // Non-empty vertices set and non-empty adjacency matrix.
            (
                (vec!["A", "B"], array![[false, false], [false, false]]),
                r#"[a-zA-Z]+Graph \{ V = \{"A", "B"\}, E = \{\} \}"#,
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), test_display) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(Regex::new(test_display).unwrap().is_match(&*format!("{}", g)));
        }
    }

    #[test]
    fn order<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix), order) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.order(), order);
            assert_eq!(g.vertices().len(), order);
        }
    }

    #[test]
    fn vertices<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix), test_vertices) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(g.vertices().eq(test_vertices));
        }
    }

    #[test]
    fn has_vertex<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix), test_vertices) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(test_vertices.into_iter().all(|x| g.has_vertex(&x.to_string())));
        }
    }

    #[test]
    fn add_vertex<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix, x), (test_vertices, test_adjacency_matrix)) in data {
            // ... construct the graph ...
            let mut g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            let x = g.add_vertex(x);

            assert!(g.has_vertex(&x));
            assert!(g.vertices().eq(test_vertices));
            // FIXME: assert_eq!(*g, test_adjacency_matrix);
        }
    }

    #[test]
    fn del_vertex<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix, x), (test_vertices, test_adjacency_matrix)) in data {
            // ... construct the graph ...
            let mut g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            let x: String = x.into();

            g.del_vertex(&x);

            assert!(!g.has_vertex(&x));
            assert!(g.vertices().eq(test_vertices));
            // FIXME: assert_eq!(*g, test_adjacency_matrix);
        }
    }

    #[test]
    fn size<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix), usize); 4] = [
            // Empty vertices set and adjacency matrix.
            ((vec![], Default::default()), 0),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]]), 0),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, false], [false, false]]), 0),
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
                0,
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix), size) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert_eq!(g.size(), size);
            assert_eq!(g.edges().len(), size);
        }
    }

    #[test]
    fn edges<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            // FIXME: assert!(g.edges().zip(test_edges).all(|((x, y), (s, t))| x == s && y == t));
        }
    }

    #[test]
    fn has_edge<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(test_edges
                .into_iter()
                .all(|(x, y)| g.has_edge(&x.to_string(), &y.to_string())));
        }
    }

    #[test]
    #[ignore]
    fn add_edge<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix, (&str, &str)), Vec<(&str, &str)>); 5] = [
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]], ("A", "A")), vec![("A", "A")]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]], ("A", "A")), vec![("A", "A")]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, false], [false, false]], ("A", "B")), vec![("A", "B")]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, false], [false, false]], ("B", "A")), vec![("A", "B")]),
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
                    ].mapv(|x| x != 0),
                    ("I", "J")
                ),
                vec![("A", "A"), ("A", "C"), ("B", "D"), ("D", "G")],
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix, (x, y)), test_edges) in data {
            // ... construct the graph ...
            let mut g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            g.add_edge(&x.to_string(), &y.to_string());

            // FIXME: assert!(g.edges().zip(test_edges).all(|((x, y), (s, t))| x == s && y == t));
        }
    }

    #[test]
    fn del_edge<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
        // Test database as (input, output) pairs.
        #[rustfmt::skip]
        let data: [((Vec<&str>, AdjacencyMatrix, (&str, &str)), Vec<(&str, &str)>); 5] = [
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[false]], ("A", "A")), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A"], array![[true]], ("A", "A")), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]], ("A", "B")), vec![]),
            // Non-empty vertices set and non-empty adjacency matrix.
            ((vec!["A", "B"], array![[false, true], [true, false]], ("B", "A")), vec![]),
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
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    ].mapv(|x| x != 0),
                    ("I", "J")
                ),
                vec![("A", "A"), ("A", "C"), ("B", "D"), ("D", "G")],
            ),
        ];

        // For each test case in the test database ...
        for ((vertices, adjacency_matrix, (x, y)), test_edges) in data {
            // ... construct the graph ...
            let mut g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            g.del_edge(&x.to_string(), &y.to_string());

            // FIXME: assert!(g.edges().zip(test_edges).all(|((x, y), (s, t))| x == s && y == t));
        }
    }

    #[test]
    fn adjacents<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
        for ((vertices, adjacency_matrix), (test_x, test_adjacents)) in data {
            // ... construct the graph ...
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(g.adjacents(&test_x.to_string()).eq(test_adjacents.into_iter()));
        }
    }

    #[test]
    fn is_adjacent<T>()
    where
        T: BaseGraph<Vertex = String> + DefaultGraph + PartialOrdGraph,
    {
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
            let g = T::with_adjacency_matrix(vertices, adjacency_matrix).unwrap();
            // ... assert result.
            assert!(test_edges
                .into_iter()
                .all(|(x, y)| g.is_adjacent(&x.to_string(), &y.to_string())));
        }
    }

    #[instantiate_tests(<UndirectedDenseMatrixGraph>)]
    mod undirected_dense_matrix_graph {}

    #[instantiate_tests(<DirectedDenseMatrixGraph>)]
    mod directed_dense_matrix_graph {}
}
