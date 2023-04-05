#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;
            use ndarray::prelude::*;

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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.get_vertex_index(x))));
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
                    assert_eq!(g.size(), (o * (o.saturating_sub(1))) / 2);
                    assert!(V!(g).eq(v.into_iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(e
                        .into_iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
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
                    let g = $G::from(k.clone());

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));

                    let e: EdgeList<_> = g.into();

                    assert!(e
                        .into_iter()
                        .eq(j.iter().map(|&(x, y)| (x.into(), y.into()))));
                }
            }

            #[test]
            fn from_adjacency_list() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![("0", vec!["0"])]),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1"], vec![("0", "1")], vec![("0", vec!["1"])]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("0", vec!["1"]), ("1", vec!["2"]), ("2", vec!["3"])],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["1", "3", "58", "71", "75"],
                        vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        vec![
                            ("71", vec!["1"]),
                            ("1", vec!["58"]),
                            ("58", vec!["3"]),
                            ("3", vec!["75"]),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let k: AdjacencyList<_> = k
                        .into_iter()
                        .map(|(x, ys)| (x, ys.into_iter().collect()))
                        .collect();
                    let g = $G::from(k);

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
                }
            }

            #[test]
            fn try_from_dense_adjacency_matrix() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    ((vec![], vec![]), (vec![], Default::default())),
                    // ... one vertex and one edge,
                    ((vec!["0"], vec![("0", "0")]), (vec!["0"], array![[true]])),
                    // ... multiple vertices and one edge,
                    (
                        (vec!["0", "1"], vec![("0", "1")]),
                        (vec!["0", "1"], array![[false, true], [true, false]]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                        (
                            vec!["0", "1", "2", "3"],
                            array![
                                [false, true, false, false],
                                [true, false, true, false],
                                [false, true, false, true],
                                [false, false, true, false]
                            ],
                        ),
                    ),
                    // ... random vertices and edges,
                    (
                        (
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                        (
                            vec!["1", "3", "58", "71", "75"],
                            array![
                                [false, false, true, true, false],
                                [false, false, true, false, true],
                                [true, true, false, false, false],
                                [true, false, false, false, false],
                                [false, true, false, false, false]
                            ],
                        ),
                    ),
                ];

                // Test for each scenario.
                for ((i, j), (v, a)) in data {
                    let g = $G::try_from((v.clone(), a.clone())).unwrap();

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));

                    let (u, b): (_, DenseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_dense_adjacency_matrix_should_panic() {
                $G::try_from((vec!["0", "1"], array![[false]])).unwrap();
            }
        };
    }

    mod undirected_dense_matrix {
        use causal_hub::graphs::structs::UndirectedDenseAdjacencyMatrixGraph;
        generic_tests!(UndirectedDenseAdjacencyMatrixGraph);
    }
}

#[cfg(test)]
mod directed {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;
            use ndarray::prelude::*;

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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.get_vertex_index(x))));
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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(e
                        .into_iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
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
                        vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let k: EdgeList<_> = k.into_iter().collect();
                    let g = $G::from(k);

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));

                    let e: EdgeList<_> = g.into();

                    assert!(e
                        .into_iter()
                        .eq(j.iter().map(|&(x, y)| (x.into(), y.into()))));
                }
            }

            #[test]
            fn from_adjacency_list() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![("0", vec!["0"])]),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1"], vec![("0", "1")], vec![("0", vec!["1"])]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("0", vec!["1"]), ("1", vec!["2"]), ("2", vec!["3"])],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["1", "3", "58", "71", "75"],
                        vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        vec![("1", vec!["58", "71"]), ("3", vec!["58", "75"])],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let k: AdjacencyList<_> = k
                        .into_iter()
                        .map(|(x, ys)| (x, ys.into_iter().collect()))
                        .collect();
                    let g = $G::from(k);

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
                }
            }

            #[test]
            fn try_from_dense_adjacency_matrix() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    ((vec![], vec![]), (vec![], Default::default())),
                    // ... one vertex and one edge,
                    ((vec!["0"], vec![("0", "0")]), (vec!["0"], array![[true]])),
                    // ... multiple vertices and one edge,
                    (
                        (vec!["0", "1"], vec![("0", "1")]),
                        (vec!["0", "1"], array![[false, true], [false, false]]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                        (
                            vec!["0", "1", "2", "3"],
                            array![
                                [false, true, false, false],
                                [false, false, true, false],
                                [false, false, false, true],
                                [false, false, false, false]
                            ],
                        ),
                    ),
                    // ... random vertices and edges,
                    (
                        (
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                        (
                            vec!["1", "3", "58", "71", "75"],
                            array![
                                [false, false, true, true, false],
                                [false, false, true, false, true],
                                [false, false, false, false, false],
                                [false, false, false, false, false],
                                [false, false, false, false, false]
                            ],
                        ),
                    ),
                ];

                // Test for each scenario.
                for ((i, j), (v, a)) in data {
                    let g = $G::try_from((v.clone(), a.clone())).unwrap();

                    assert!(V!(g).eq(i.iter().map(|x| g.get_vertex_index(x))));
                    assert!(E!(g).eq(j
                        .iter()
                        .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));

                    let (u, b): (_, DenseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_dense_adjacency_matrix_should_panic() {
                $G::try_from((vec!["0", "1"], array![[false]])).unwrap();
            }
        };
    }

    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}
