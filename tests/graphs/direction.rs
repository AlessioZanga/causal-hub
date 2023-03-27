#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;
            use is_sorted::IsSorted;

            #[test]
            fn neighbors() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0, 2]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (2, vec![0, 1]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (3, vec![]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(Ne!(g, x).is_sorted());
                    assert!(Ne!(g, x).eq(f));
                }
            }

            #[test]
            #[should_panic]
            fn neighbors_should_panic() {
                let g = $G::null();

                Ne!(g, 0);
            }

            #[test]
            fn is_neighbor() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0, 2]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (2, vec![0, 1]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (3, vec![]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(f.iter().all(|&y| g.is_neighbor(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_neighbor_should_panic() {
                let g = $G::null();

                g.is_neighbor(0, 0);
            }

            #[test]
            fn degree() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], (0, 1)),
                    // ... multiple edges,
                    (vec![("0", "1"), ("1", "2"), ("2", "3")], (1, 2)),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (2, 2),
                    ),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let g = $G::new([], i);
                    assert_eq!(g.degree(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn degree_should_panic() {
                let g = $G::null();
                g.degree(0);
            }
        };
    }

    #[allow(unstable_name_collisions)]
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
            use is_sorted::IsSorted;

            #[test]
            fn ancestors() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (4, vec![0, 1, 2, 3]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(An!(g, x).is_sorted());
                    assert!(An!(g, x).eq(f));
                }
            }

            #[test]
            #[should_panic]
            fn ancestors_should_panic() {
                let g = $G::null();

                An!(g, 0);
            }

            #[test]
            fn is_ancestor() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (4, vec![0, 1, 2, 3]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(f.iter().all(|&y| g.is_ancestor(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_ancestor_should_panic() {
                let g = $G::null();

                g.is_ancestor(0, 0);
            }

            #[test]
            fn parents() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (4, vec![1]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(Pa!(g, x).is_sorted());
                    assert!(Pa!(g, x).eq(f));
                }
            }

            #[test]
            #[should_panic]
            fn parents_should_panic() {
                let g = $G::null();

                Pa!(g, 0);
            }

            #[test]
            fn is_parent() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (1, vec![0]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (4, vec![1]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(f.iter().all(|&y| g.is_parent(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_parent_should_panic() {
                let g = $G::null();

                g.is_parent(0, 0);
            }

            #[test]
            fn children() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (0, vec![1]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (1, vec![4]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![0, 8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(Ch!(g, x).is_sorted());
                    assert!(Ch!(g, x).eq(f));
                }
            }

            #[test]
            #[should_panic]
            fn children_should_panic() {
                let g = $G::null();

                Ch!(g, 0);
            }

            #[test]
            fn is_child() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (0, vec![1]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (1, vec![4]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![0, 8]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(f.iter().all(|&y| g.is_child(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_child_should_panic() {
                let g = $G::null();

                g.is_child(0, 0);
            }

            #[test]
            fn descendants() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (0, vec![1, 2, 3]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (1, vec![4]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![0, 4, 6, 8, 9]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(De!(g, x).is_sorted());
                    assert!(De!(g, x).eq(f));
                }
            }

            #[test]
            #[should_panic]
            fn descendants_should_panic() {
                let g = $G::null();

                De!(g, 0);
            }

            #[test]
            fn is_descendant() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, vec![])),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, vec![0])),
                    // ... multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (0, vec![1, 2, 3]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (1, vec![4]),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, vec![0, 4, 6, 8, 9]),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert!(f.iter().all(|&y| g.is_descendant(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_descendant_should_panic() {
                let g = $G::null();

                g.is_descendant(0, 0);
            }

            #[test]
            fn in_degree() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, 0)),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, 1)),
                    // ... multiple edges,
                    (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], (1, 1)),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (4, 1),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, 1),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert_eq!(g.in_degree(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn in_degree_should_panic() {
                let g = $G::null();

                g.in_degree(0);
            }

            #[test]
            fn out_degree() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices,
                    // (vec![], vec![], (0, vec![])),
                    // ... zero edges,
                    (vec!["0"], vec![], (0, 0)),
                    // ... one edge,
                    (vec![], vec![("0", "0")], (0, 1)),
                    // ... multiple edges,
                    (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], (0, 1)),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (1, 1),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![
                            ("71", "71"),
                            ("71", "1"),
                            ("1", "58"),
                            ("58", "3"),
                            ("3", "75"),
                        ],
                        (8, 2),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (x, f)) in data {
                    let g = $G::new(i, j);

                    assert_eq!(g.out_degree(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn out_degree_should_panic() {
                let g = $G::null();

                g.out_degree(0);
            }
        };
    }

    #[allow(unstable_name_collisions)]
    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}

#[cfg(test)]
mod partially_directed {} //TODO: test specific traits, Undirected trait and Directed Trait
