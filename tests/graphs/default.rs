#[cfg(test)]
mod tests {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use causal_hub::prelude::*;

                #[test]
                fn default() {
                    let g = $G::default();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }

                #[test]
                fn null() {
                    let g = $G::null();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }

                #[test]
                fn empty() {
                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, vec![])),
                        // ... one vertex,
                        (vec!["0"], (1, vec!["0"])),
                        // ... multiple vertices,
                        (vec!["0", "1", "2", "3"], (4, vec!["0", "1", "2", "3"])),
                        // ... random vertices,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            (5, vec!["1", "3", "58", "71", "75"]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, (o, v)) in data {
                        let g = $G::empty(i);
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), 0);
                        assert!(V!(g).eq(v.into_iter().map(|x| g.index(x))));
                        assert!(E!(g).next().is_none());
                    }
                }

                #[test]
                fn complete() {
                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, vec![], vec![])),
                        // ... one vertex,
                        (vec!["0"], (1, vec!["0"], vec![])),
                        // ... multiple vertices,
                        (
                            vec!["0", "1", "2", "3"],
                            (
                                4,
                                vec!["0", "1", "2", "3"],
                                vec![
                                    ("0", "1"),
                                    ("0", "2"),
                                    ("0", "3"),
                                    ("1", "2"),
                                    ("1", "3"),
                                    ("2", "3"),
                                ],
                            ),
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            (
                                5,
                                vec!["1", "3", "58", "71", "75"],
                                vec![
                                    ("1", "3"),
                                    ("1", "58"),
                                    ("1", "71"),
                                    ("1", "75"),
                                    ("3", "58"),
                                    ("3", "71"),
                                    ("3", "75"),
                                    ("58", "71"),
                                    ("58", "75"),
                                    ("71", "75"),
                                ],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, (o, v, e)) in data {
                        let g = $G::complete(i);
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), (o * (o + 1)) / 2);
                        assert!(V!(g).eq(v.into_iter().map(|x| g.index(x))));
                        assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.index(x), g.index(y)))));
                    }
                }

                #[test]
                fn from_edge_list() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], vec![("0", "0")]),
                        // ... multiple vertices and one edge,
                        (vec!["0", "1"], vec![("0", "1")], vec![("0", "1")]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k) in data {
                        let k: EdgeList<_> = k.into_iter().collect();
                        let g = $G::from(k);
                        assert!(V!(g).eq(i.into_iter().map(|x| g.index(x))));
                        assert!(E!(g).eq(j.into_iter().map(|(x, y)| (g.index(x), g.index(y)))));
                    }
                }

                #[test]
                #[ignore]
                fn from_adjacency_list() {
                    todo!() // FIXME:
                }

                #[test]
                #[ignore]
                fn try_from_dense_adjacency_matrix() {
                    todo!() // FIXME:
                }

                #[test]
                #[should_panic]
                #[ignore]
                fn try_from_dense_adjacency_matrix_should_panic() {
                    todo!() // FIXME:
                }

                #[test]
                #[ignore]
                fn try_from_sparse_adjacency_matrix() {
                    todo!() // FIXME:
                }

                #[test]
                #[should_panic]
                #[ignore]
                fn try_from_sparse_adjacency_matrix_should_panic() {
                    todo!() // FIXME:
                }
            };
        }

        mod undirected_dense_matrix {
            use causal_hub::graphs::UndirectedDenseAdjacencyMatrixGraph;
            generic_tests!(UndirectedDenseAdjacencyMatrixGraph);
        }
    }

    mod directed {
        macro_rules! generic_tests {
            ($G: ident) => {
                use causal_hub::prelude::*;

                #[test]
                fn default() {
                    let g = $G::default();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }

                #[test]
                fn null() {
                    let g = $G::null();

                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }

                #[test]
                fn empty() {
                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, vec![])),
                        // ... one vertex,
                        (vec!["0"], (1, vec!["0"])),
                        // ... multiple vertices,
                        (vec!["0", "1", "2", "3"], (4, vec!["0", "1", "2", "3"])),
                        // ... random vertices,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            (5, vec!["1", "3", "58", "71", "75"]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, (o, v)) in data {
                        let g = $G::empty(i);
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), 0);
                        assert!(V!(g).eq(v.into_iter().map(|x| g.index(x))));
                        assert!(E!(g).next().is_none());
                    }
                }

                #[test]
                fn complete() {
                    // Test for ...
                    let data = [
                        // ... zero vertices,
                        (vec![], (0, vec![], vec![])),
                        // ... one vertex,
                        (vec!["0"], (1, vec!["0"], vec![])),
                        // ... multiple vertices,
                        (
                            vec!["0", "1", "2", "3"],
                            (
                                4,
                                vec!["0", "1", "2", "3"],
                                vec![
                                    ("0", "1"),
                                    ("0", "2"),
                                    ("0", "3"),
                                    ("1", "0"),
                                    ("1", "2"),
                                    ("1", "3"),
                                    ("2", "0"),
                                    ("2", "1"),
                                    ("2", "3"),
                                    ("3", "0"),
                                    ("3", "1"),
                                    ("3", "2"),
                                ],
                            ),
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            (
                                5,
                                vec!["1", "3", "58", "71", "75"],
                                vec![
                                    ("1", "3"),
                                    ("1", "58"),
                                    ("1", "71"),
                                    ("1", "75"),
                                    ("3", "1"),
                                    ("3", "58"),
                                    ("3", "71"),
                                    ("3", "75"),
                                    ("58", "1"),
                                    ("58", "3"),
                                    ("58", "71"),
                                    ("58", "75"),
                                    ("71", "1"),
                                    ("71", "3"),
                                    ("71", "58"),
                                    ("71", "75"),
                                    ("75", "1"),
                                    ("75", "3"),
                                    ("75", "58"),
                                    ("75", "71"),
                                ],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, (o, v, e)) in data {
                        let g = $G::complete(i);
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), o * (o.saturating_sub(1)));
                        assert!(V!(g).eq(v.into_iter().map(|x| g.index(x))));
                        assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.index(x), g.index(y)))));
                    }
                }

                #[test]
                #[ignore]
                fn from_edge_list() {
                    todo!() // FIXME:
                }

                #[test]
                #[ignore]
                fn from_adjacency_list() {
                    todo!() // FIXME:
                }

                #[test]
                #[ignore]
                fn try_from_dense_adjacency_matrix() {
                    todo!() // FIXME:
                }

                #[test]
                #[should_panic]
                #[ignore]
                fn try_from_dense_adjacency_matrix_should_panic() {
                    todo!() // FIXME:
                }

                #[test]
                #[ignore]
                fn try_from_sparse_adjacency_matrix() {
                    todo!() // FIXME:
                }

                #[test]
                #[should_panic]
                #[ignore]
                fn try_from_sparse_adjacency_matrix_should_panic() {
                    todo!() // FIXME:
                }
            };
        }

        mod directed_dense_matrix {
            use causal_hub::graphs::DirectedDenseAdjacencyMatrixGraph;
            generic_tests!(DirectedDenseAdjacencyMatrixGraph);
        }
    }
}
