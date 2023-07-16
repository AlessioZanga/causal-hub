#[cfg(test)]
mod tests {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::{graphs::algorithms::clique::BK, prelude::*};

            #[test]
            fn call() {
                let data = vec![
                    (vec![], vec![], vec![]),
                    (vec!["A"], vec![], vec![vec![0]]),
                    (vec!["A", "B"], vec![], vec![vec![0], vec![1]]),
                    (vec!["A", "B"], vec![("A", "B")], vec![vec![0, 1]]),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B")],
                        vec![vec![0, 1], vec![2]],
                    ),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B"), ("B", "C")],
                        vec![vec![0, 1], vec![1, 2]],
                    ),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B"), ("B", "C"), ("C", "A")],
                        vec![vec![0, 1, 2]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A")],
                        vec![vec![0, 1, 2], vec![3]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A"), ("C", "D")],
                        vec![vec![0, 1, 2], vec![2, 3]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A"), ("C", "D"), ("D", "B")],
                        vec![vec![0, 1, 2], vec![1, 2, 3]],
                    ),
                    (
                        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "_10"],
                        vec![
                            ("0", "1"),
                            ("0", "3"),
                            ("0", "5"),
                            ("1", "3"),
                            ("1", "4"),
                            ("1", "6"),
                            ("2", "3"),
                            ("2", "5"),
                            ("3", "4"),
                            ("3", "5"),
                            ("3", "6"),
                            ("4", "7"),
                            ("5", "6"),
                            ("7", "8"),
                            ("7", "9"),
                            ("7", "_10"),
                            ("8", "9"),
                            ("8", "_10"),
                            ("9", "_10"),
                        ],
                        vec![
                            vec![0, 1, 3],
                            vec![0, 3, 5],
                            vec![1, 3, 4],
                            vec![1, 3, 6],
                            vec![2, 3, 5],
                            vec![3, 5, 6],
                            vec![4, 7],
                            vec![7, 8, 9, 10],
                        ],
                    ),
                ];

                for (v, e, true_c) in data {
                    let g = $G::new(v, e);

                    let pred_c = BK::new(&g).call();

                    assert_eq!(pred_c, true_c);
                }
            }

            #[test]
            fn par_call() {
                let data = vec![
                    (vec![], vec![], vec![]),
                    (vec!["A"], vec![], vec![vec![0]]),
                    (vec!["A", "B"], vec![], vec![vec![0], vec![1]]),
                    (vec!["A", "B"], vec![("A", "B")], vec![vec![0, 1]]),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B")],
                        vec![vec![0, 1], vec![2]],
                    ),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B"), ("B", "C")],
                        vec![vec![0, 1], vec![1, 2]],
                    ),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B"), ("B", "C"), ("C", "A")],
                        vec![vec![0, 1, 2]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A")],
                        vec![vec![0, 1, 2], vec![3]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A"), ("C", "D")],
                        vec![vec![0, 1, 2], vec![2, 3]],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "A"), ("C", "D"), ("D", "B")],
                        vec![vec![0, 1, 2], vec![1, 2, 3]],
                    ),
                    (
                        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "_10"],
                        vec![
                            ("0", "1"),
                            ("0", "3"),
                            ("0", "5"),
                            ("1", "3"),
                            ("1", "4"),
                            ("1", "6"),
                            ("2", "3"),
                            ("2", "5"),
                            ("3", "4"),
                            ("3", "5"),
                            ("3", "6"),
                            ("4", "7"),
                            ("5", "6"),
                            ("7", "8"),
                            ("7", "9"),
                            ("7", "_10"),
                            ("8", "9"),
                            ("8", "_10"),
                            ("9", "_10"),
                        ],
                        vec![
                            vec![0, 1, 3],
                            vec![0, 3, 5],
                            vec![1, 3, 4],
                            vec![1, 3, 6],
                            vec![2, 3, 5],
                            vec![3, 5, 6],
                            vec![4, 7],
                            vec![7, 8, 9, 10],
                        ],
                    ),
                ];

                for (v, e, true_c) in data {
                    let g = $G::new(v, e);

                    let pred_c = BK::new(&g).par_call();

                    assert_eq!(pred_c, true_c);
                }
            }
        };
    }

    mod undirected_dense_matrix {
        use causal_hub::graphs::UndirectedDenseAdjacencyMatrixGraph;
        generic_tests!(UndirectedDenseAdjacencyMatrixGraph);
    }
}
