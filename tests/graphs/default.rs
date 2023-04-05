#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use std::ops::Deref;

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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

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

            #[test]
            fn try_from_sparse_adjacency_matrix() {
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
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };
                    let g = $G::try_from((v.clone(), a)).unwrap();

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

                    let a = g.deref();
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };

                    let (u, b): (_, SparseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_sparse_adjacency_matrix_should_panic() {
                $G::try_from((
                    vec!["0", "1"],
                    SparseAdjacencyMatrix::from_triplets((1, 1), vec![0], vec![0], vec![true]),
                ))
                .unwrap();
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
            use std::ops::Deref;

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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

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

            #[test]
            fn try_from_sparse_adjacency_matrix() {
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
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };
                    let g = $G::try_from((v.clone(), a)).unwrap();

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

                    let a = g.deref();
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };

                    let (u, b): (_, SparseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_sparse_adjacency_matrix_should_panic() {
                $G::try_from((
                    vec!["0", "1"],
                    SparseAdjacencyMatrix::from_triplets((1, 1), vec![0], vec![0], vec![true]),
                ))
                .unwrap();
            }
        };
    }

    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}

#[cfg(test)]
mod partially_directed{
    macro_rules! generic_tests {
        ($G: ident) => {
            use std::ops::Deref;

            use causal_hub::prelude::*;
            use ndarray::prelude::*;

            #[test]
            fn default() {
                let g = $G::default();

                assert_eq!(g.order(), 0);
                assert_eq!(g.size(), 0);
                assert!(V!(g).next().is_none());
                assert!(uE!(g).next().is_none());
                assert!(dE!(g).next().is_none());
                assert!(E!(g).next().is_none());
            }

            #[test]
            fn null() {
                let g = $G::null();

                assert_eq!(g.order(), 0);
                assert_eq!(g.size(), 0);
                assert!(V!(g).next().is_none());
                assert!(uE!(g).next().is_none());
                assert!(dE!(g).next().is_none());
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
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(uE!(g).next().is_none());
                    assert!(dE!(g).next().is_none());
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
                    let g = $G::complete(i.clone());
                    assert_eq!(g.order(), o);
                    assert_eq!(g.size(), o * (o.saturating_sub(1)) / 2);
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(uE!(g).eq(e.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                    assert!(dE!(g).next().is_none());
                    assert!(E!(g).eq(e.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(uE!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                    assert!(dE!(g).next().is_none());
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(uE!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                    assert!(dE!(g).next().is_none());
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn from_undirected_dense_adjacency_matrix_graph() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![]),
                    // ... zero vertices and one edge,
                    (vec![], vec![("0", "0")]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![]),
                    // ... zero vertices and multiple edges,
                    (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")]),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1", "2", "3"], vec![("0", "1")]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = UndirectedDenseAdjacencyMatrixGraph::new(i, j);
                    let g_partially_directed: PartiallyDenseAdjacencyMatrixGraph = g.clone().into();
                    // Test the order
                    assert_eq!(g.order(), g_partially_directed.order());
                    // Test the labels
                    assert_eq!(
                        g.labels().collect::<Vec<_>>(),
                        g_partially_directed.labels().collect::<Vec<_>>()
                    );
                    // Test matrices
                    assert_eq!(0, g_partially_directed.size_of_type('d'));
                    assert_eq!(g.deref(), g_partially_directed.deref_of_type('u'));
                    assert_eq!(g.deref(), g_partially_directed.deref());
                }
            }

            #[test]
            fn from_directed_dense_adjacency_matrix_graph() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![]),
                    // ... zero vertices and one edge,
                    (vec![], vec![("0", "0")]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![]),
                    // ... zero vertices and multiple edges,
                    (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = DirectedDenseAdjacencyMatrixGraph::new(i, j);
                    let g_partially_directed: PartiallyDenseAdjacencyMatrixGraph = g.clone().into();
                    // Test the order
                    assert_eq!(g.order(), g_partially_directed.order());
                    // Test the labels
                    assert_eq!(
                        g.labels().collect::<Vec<_>>(),
                        g_partially_directed.labels().collect::<Vec<_>>()
                    );
                    // Test matrices
                    assert_eq!(0, g_partially_directed.size_of_type('u'));
                    assert_eq!(g.deref(), g_partially_directed.deref_of_type('d'));
                    assert_eq!(g.to_undirected().deref(), g_partially_directed.deref());
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

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

                    let (u, b): (_, DenseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_dense_adjacency_matrix_should_panic_for_dimensions() {
                $G::try_from((vec!["0", "1"], array![[false]])).unwrap();
            }

            #[test]
            #[should_panic]
            fn try_from_dense_adjacency_matrix_should_panic_for_symmetry() {
                $G::try_from((
                    vec!["0", "1", "2", "3"],
                    array![
                        [false, false, false, false],
                        [true, false, true, false],
                        [false, true, false, true],
                        [false, false, true, false]
                    ],
                ))
                .unwrap();
            }

            #[test]
            fn try_from_sparse_adjacency_matrix() {
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
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };
                    let g = $G::try_from((v.clone(), a)).unwrap();

                    assert!(V!(g).eq(i.iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));

                    let a = g.deref();
                    let a = {
                        let (mut rows, mut cols) = (vec![], vec![]);
                        for ((i, j), &f) in a.indexed_iter() {
                            if f {
                                rows.push(i);
                                cols.push(j);
                            }
                        }
                        let data: Vec<_> = std::iter::repeat(true).take(rows.len()).collect();
                        SparseAdjacencyMatrix::from_triplets(a.dim(), rows, cols, data)
                    };

                    let (u, b): (_, SparseAdjacencyMatrix) = g.into();

                    assert!(u.into_iter().eq(v.into_iter()));
                    assert_eq!(b, a);
                }
            }

            #[test]
            #[should_panic]
            fn try_from_sparse_adjacency_matrix_should_panic() {
                $G::try_from((
                    vec!["0", "1"],
                    SparseAdjacencyMatrix::from_triplets((1, 1), vec![0], vec![0], vec![true]),
                ))
                .unwrap();
            }
        };
    }

    mod partially_dense_matrix {
        use causal_hub::graphs::structs::{
            DirectedDenseAdjacencyMatrixGraph, PartiallyDenseAdjacencyMatrixGraph,
            UndirectedDenseAdjacencyMatrixGraph,
        };
        generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
    }
}
