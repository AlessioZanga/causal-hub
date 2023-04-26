#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;
            use is_sorted::IsSorted;

            #[test]
            fn get_neighbors_by_index() {
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
            fn is_neighbor_by_index() {
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

                    assert!(f.iter().all(|&y| g.is_neighbor_by_index(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_neighbor_should_panic() {
                let g = $G::null();

                g.is_neighbor_by_index(0, 0);
            }

            #[test]
            fn get_degree_by_index() {
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
                    assert_eq!(g.get_degree_by_index(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn get_degree_by_index_should_panic() {
                let g = $G::null();
                g.get_degree_by_index(0);
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
            fn get_ancestors_by_index() {
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
            fn get_ancestors_by_index_should_panic() {
                let g = $G::null();

                An!(g, 0);
            }

            #[test]
            fn is_ancestor_by_index() {
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

                    assert!(f.iter().all(|&y| g.is_ancestor_by_index(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_ancestor_should_panic() {
                let g = $G::null();

                g.is_ancestor_by_index(0, 0);
            }

            #[test]
            fn get_parents_by_index() {
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
            fn get_parents_by_index_should_panic() {
                let g = $G::null();

                Pa!(g, 0);
            }

            #[test]
            fn is_parent_by_index() {
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

                    assert!(f.iter().all(|&y| g.is_parent_by_index(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_parent_should_panic() {
                let g = $G::null();

                g.is_parent_by_index(0, 0);
            }

            #[test]
            fn get_children_by_index() {
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
            fn is_child_by_index() {
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

                    assert!(f.iter().all(|&y| g.is_child_by_index(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_child_should_panic() {
                let g = $G::null();

                g.is_child_by_index(0, 0);
            }

            #[test]
            fn get_descendants_by_index() {
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
            fn is_descendant_by_index() {
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

                    assert!(f.iter().all(|&y| g.is_descendant_by_index(x, y)));
                }
            }

            #[test]
            #[should_panic]
            fn is_descendant_should_panic() {
                let g = $G::null();

                g.is_descendant_by_index(0, 0);
            }

            #[test]
            fn get_in_degree_by_index() {
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

                    assert_eq!(g.get_in_degree_by_index(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn in_degree_should_panic() {
                let g = $G::null();

                g.get_in_degree_by_index(0);
            }

            #[test]
            fn get_out_degree_by_index() {
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

                    assert_eq!(g.get_out_degree_by_index(x), f);
                }
            }

            #[test]
            #[should_panic]
            fn out_degree_should_panic() {
                let g = $G::null();

                g.get_out_degree_by_index(0);
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
mod partially_directed {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use causal_hub::prelude::*;
                use is_sorted::IsSorted;

                #[test]
                fn get_neighbors_by_index() {
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
                fn is_neighbor_by_index() {
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

                        assert!(f.iter().all(|&y| g.is_neighbor_by_index(x, y)));
                    }
                }

                #[test]
                #[should_panic]
                fn is_neighbor_by_index_should_panic() {
                    let g = $G::null();

                    g.is_neighbor_by_index(0, 0);
                }

                #[test]
                fn get_degree_by_index() {
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
                        assert_eq!(g.get_degree_by_index(x), f);
                    }
                }

                #[test]
                #[should_panic]
                fn get_degree_by_index_should_panic() {
                    let g = $G::null();
                    g.get_degree_by_index(0);
                }
            };
        }

        #[allow(unstable_name_collisions)]
        mod undirected_dense_matrix {
            use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
            generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
        }
    }

    mod directed {
        macro_rules! generic_tests {
            ($G: ident) => {
                use causal_hub::prelude::*;
                use is_sorted::IsSorted;

                #[test]
                fn get_ancestors_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert!(An!(g, x).is_sorted());
                        assert!(An!(g, x).eq(f));
                    }
                }

                #[test]
                #[should_panic]
                fn get_ancestors_by_index_should_panic() {
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
                        let g = $G::new_partial(i, [], j);

                        assert!(f.iter().all(|&y| g.is_ancestor_by_index(x, y)));
                    }
                }

                #[test]
                #[should_panic]
                fn is_ancestor_should_panic() {
                    let g = $G::null();

                    g.is_ancestor_by_index(0, 0);
                }

                #[test]
                fn get_parents_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert!(Pa!(g, x).is_sorted());
                        assert!(Pa!(g, x).eq(f));
                    }
                }

                #[test]
                #[should_panic]
                fn get_parents_by_index_should_panic() {
                    let g = $G::null();

                    Pa!(g, 0);
                }

                #[test]
                fn is_parent_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert!(f.iter().all(|&y| g.is_parent_by_index(x, y)));
                    }
                }

                #[test]
                #[should_panic]
                fn is_parent_by_index_should_panic() {
                    let g = $G::null();

                    g.is_parent_by_index(0, 0);
                }

                #[test]
                fn get_children_by_index() {
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
                        let g = $G::new_partial(i, [], j);

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
                fn is_child_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert!(f.iter().all(|&y| g.is_child_by_index(x, y)));
                    }
                }

                #[test]
                #[should_panic]
                fn is_child_should_panic() {
                    let g = $G::null();

                    g.is_child_by_index(0, 0);
                }

                #[test]
                fn get_descendants_by_index() {
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
                        let g = $G::new_partial(i, [], j);

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
                        let g = $G::new_partial(i, [], j);

                        assert!(f.iter().all(|&y| g.is_descendant_by_index(x, y)));
                    }
                }

                #[test]
                #[should_panic]
                fn is_descendant_should_panic() {
                    let g = $G::null();

                    g.is_descendant_by_index(0, 0);
                }

                #[test]
                fn get_in_degree_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert_eq!(g.get_in_degree_by_index(x), f);
                    }
                }

                #[test]
                #[should_panic]
                fn in_degree_should_panic() {
                    let g = $G::null();

                    g.get_in_degree_by_index(0);
                }

                #[test]
                fn get_out_degree_by_index() {
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
                        let g = $G::new_partial(i, [], j);

                        assert_eq!(g.get_out_degree_by_index(x), f);
                    }
                }

                #[test]
                #[should_panic]
                fn out_degree_should_panic() {
                    let g = $G::null();

                    g.get_out_degree_by_index(0);
                }
            };
        }

        #[allow(unstable_name_collisions)]
        mod directed_dense_matrix {
            use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
            generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
        }
    }

    mod partially_directed {
        macro_rules! generic_tests {
            ($G: ident) => {
                use std::ops::Deref;

                use causal_hub::prelude::*;
                use is_sorted::IsSorted;
                use ndarray::prelude::*;

                // Test for `new_partial`, `edges_of_type`, `size_of_type` functions in `PartiallyDirectedGraph` trait
                #[test]
                fn new_edges_size() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (
                            vec![],
                            vec![],
                            vec![],
                            (0, 0, vec![], vec![], vec![], vec![]),
                        ),
                        // ... one vertex and zero edges,
                        (
                            vec!["0"],
                            vec![],
                            vec![],
                            (1, 0, vec!["0"], vec![], vec![], vec![]),
                        ),
                        // ... zero vertices and one undirected edge,
                        (
                            vec![],
                            vec![("0", "0")],
                            vec![],
                            (1, 1, vec!["0"], vec![("0", "0")], vec![], vec![("0", "0")]),
                        ),
                        // ... zero vertices and one directed edge,
                        (
                            vec![],
                            vec![],
                            vec![("0", "0")],
                            (1, 1, vec!["0"], vec![], vec![("0", "0")], vec![("0", "0")]),
                        ),
                        // ... one vertex and one undirected edge,
                        (
                            vec!["0"],
                            vec![("0", "0")],
                            vec![],
                            (1, 1, vec!["0"], vec![("0", "0")], vec![], vec![("0", "0")]),
                        ),
                        // ... one vertex and one directed edge,
                        (
                            vec!["0"],
                            vec![],
                            vec![("0", "0")],
                            (1, 1, vec!["0"], vec![], vec![("0", "0")], vec![("0", "0")]),
                        ),
                        // ... multiple vertices and zero edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![],
                            vec![],
                            (4, 0, vec!["0", "1", "2", "3"], vec![], vec![], vec![]),
                        ),
                        // ... zero vertices and multiple undirected edges,
                        (
                            vec![],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![],
                            (
                                4,
                                3,
                                vec!["0", "1", "2", "3"],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                            ),
                        ),
                        // ... zero vertices and multiple directed edges,
                        (
                            vec![],
                            vec![],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            (
                                4,
                                3,
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                            ),
                        ),
                        // ... zero vertices and multiple edges of different types,
                        (
                            vec![],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![("6", "5"), ("4", "5")],
                            (
                                7,
                                5,
                                vec!["0", "1", "2", "3", "4", "5", "6"],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("4", "5"), ("6", "5")],
                                vec![("0", "1"), ("1", "2"), ("2", "3"), ("4", "5"), ("5", "6")],
                            ),
                        ),
                        // ... multiple vertices and one undirected edge,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1")],
                            vec![],
                            (
                                4,
                                1,
                                vec!["0", "1", "2", "3"],
                                vec![("0", "1")],
                                vec![],
                                vec![("0", "1")],
                            ),
                        ),
                        // ... multiple vertices and one directed edge,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![],
                            vec![("0", "1")],
                            (
                                4,
                                1,
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![("0", "1")],
                                vec![("0", "1")],
                            ),
                        ),
                        // ... multiple vertices and multiple undirected edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![],
                            (
                                4,
                                3,
                                vec!["0", "1", "2", "3"],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                            ),
                        ),
                        // ... multiple vertices and multiple directed edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            (
                                4,
                                3,
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                            ),
                        ),
                        // ... multiple vertices and multiple edges of different types,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![("1", "3"), ("3", "0")],
                            (
                                4,
                                5,
                                vec!["0", "1", "2", "3"],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("1", "3"), ("3", "0")],
                                vec![("0", "1"), ("0", "3"), ("1", "2"), ("1", "3"), ("2", "3")],
                            ),
                        ),
                        // ... random vertices and undirected edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![],
                            (
                                5,
                                4,
                                vec!["1", "3", "58", "71", "75"],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                                vec![],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                            ),
                        ),
                        // ... random vertices and directed edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            (
                                5,
                                4,
                                vec!["1", "3", "58", "71", "75"],
                                vec![],
                                vec![("1", "58"), ("3", "75"), ("58", "3"), ("71", "1")],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                            ),
                        ),
                        // ... random vertices and edges of different types,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![("75", "1"), ("1", "3")],
                            (
                                5,
                                6,
                                vec!["1", "3", "58", "71", "75"],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                                vec![("1", "3"), ("75", "1")],
                                vec![
                                    ("1", "3"),
                                    ("1", "58"),
                                    ("1", "71"),
                                    ("1", "75"),
                                    ("3", "58"),
                                    ("3", "75"),
                                ],
                            ),
                        ),
                        // ... random non-overlapping vertices and undirected edges,
                        (
                            vec!["35", "62", "99", "29", "100", "18"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![],
                            (
                                11,
                                4,
                                vec![
                                    "1", "100", "18", "29", "3", "35", "58", "62", "71", "75", "99",
                                ],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                                vec![],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                            ),
                        ),
                        // ... random non-overlapping vertices and directed edges,
                        (
                            vec!["35", "62", "99", "29", "100", "18"],
                            vec![],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            (
                                11,
                                4,
                                vec![
                                    "1", "100", "18", "29", "3", "35", "58", "62", "71", "75", "99",
                                ],
                                vec![],
                                vec![("1", "58"), ("3", "75"), ("58", "3"), ("71", "1")],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                            ),
                        ),
                        // ... random non-overlapping vertices and edges of different types,
                        (
                            vec!["35", "62", "99", "29", "100", "18"],
                            vec![
                                ("71", "1"),
                                ("75", "3"),
                                ("1", "58"),
                                ("58", "3"),
                                ("3", "75"),
                            ],
                            vec![
                                ("62", "99"),
                                ("18", "36"),
                                ("101", "42"),
                                ("1", "60"),
                                ("1", "60"),
                            ],
                            (
                                15,
                                8,
                                vec![
                                    "1", "100", "101", "18", "29", "3", "35", "36", "42", "58",
                                    "60", "62", "71", "75", "99",
                                ],
                                vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                                vec![("1", "60"), ("101", "42"), ("18", "36"), ("62", "99")],
                                vec![
                                    ("1", "58"),
                                    ("1", "60"),
                                    ("1", "71"),
                                    ("101", "42"),
                                    ("18", "36"),
                                    ("3", "58"),
                                    ("3", "75"),
                                    ("62", "99"),
                                ],
                            ),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, k, (o, s, v, ue, de, e)) in data {
                        // Test for `new_partial` and `edges_of_type` (in `uE` and `dE` macros) function
                        let g = $G::new_partial(i, j, k);
                        assert_eq!(g.order(), o);
                        assert_eq!(g.size(), s);
                        assert!(V!(g).is_sorted());
                        assert!(uE!(g).is_sorted());
                        assert!(dE!(g).is_sorted());
                        assert!(E!(g).is_sorted());
                        assert!(V!(g).eq(v.into_iter().map(|x| g.get_vertex_index(x))));
                        assert!(uE!(g).eq(ue
                            .iter()
                            .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
                        assert!(dE!(g).eq(de
                            .iter()
                            .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
                        assert!(E!(g).eq(e
                            .into_iter()
                            .map(|(x, y)| (g.get_vertex_index(x), g.get_vertex_index(y)))));
                        // Test for `size_of_type` function
                        assert!(g.size_of_undirected_subgraph() == ue.len());
                        assert!(g.size_of_directed_subgraph() == de.len());
                    }
                }

                #[test]
                fn deref_of_type() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], Default::default()),
                        // ... one vertex and one edge,
                        (vec!["0"], array![[true]]),
                        // ... multiple vertices and one edge,
                        (vec!["0", "1"], array![[false, true], [true, false]]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            array![
                                [false, true, false, false],
                                [true, false, true, false],
                                [false, true, false, true],
                                [false, false, true, false]
                            ],
                        ),
                        // ... random vertices and edges,
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
                    ];

                    // Test for each scenario.
                    for (v, a) in data {
                        let order = v.len();
                        let empty_matrix = DenseAdjacencyMatrix::from_elem((order, order), false);

                        // Test for undirected graph
                        let g_undirected = $G::from((v.clone(), a.clone(), empty_matrix.clone()));
                        assert!(g_undirected.deref_of_type('u') == &a);
                        assert!(g_undirected.deref_of_type('d') == &empty_matrix);
                        assert!(g_undirected.deref() == &a);

                        // Test for directed graph
                        let g_directed = $G::from((v.clone(), empty_matrix.clone(), a.clone()));
                        assert!(g_directed.deref_of_type('u') == &empty_matrix);
                        assert!(g_directed.deref_of_type('d') == &a);
                        assert!(g_directed.deref() == &a);
                    }
                }

                #[test]
                #[should_panic]
                fn new_partial_should_panic() {
                    let (i, j, k) = (
                        vec!["0", "1", "2"],
                        vec![("1", "2")],
                        vec![("1", "0"), ("1", "2")],
                    );

                    $G::new_partial(i, j, k);
                }

                #[test]
                fn type_of_edge() {
                    let (i, j, k) = (vec!["0", "1", "2"], vec![("1", "2")], vec![("0", "1")]);
                    let g = $G::new_partial(i, j, k);
                    // Test for undirected edges
                    assert!(g.has_undirected_edge_by_index(1, 2) == true);
                    assert!(g.has_undirected_edge_by_index(2, 1) == true);
                    assert!(g.has_directed_edge_by_index(1, 2) == false);
                    assert!(g.has_directed_edge_by_index(2, 1) == false);
                    // Test for directed edges
                    assert!(g.has_directed_edge_by_index(0, 1) == true);
                    assert!(g.has_directed_edge_by_index(1, 0) == false);
                    assert!(g.has_undirected_edge_by_index(0, 1) == false);
                    // Test for non-present edges
                    assert!(g.has_undirected_edge_by_index(0, 2) == false);
                    assert!(g.has_undirected_edge_by_index(2, 0) == false);
                    assert!(g.has_directed_edge_by_index(0, 2) == false);
                    assert!(g.has_directed_edge_by_index(2, 0) == false);
                }

                #[test]
                fn add_edge_of_type() {
                    let (i, j, k) = (vec!["0", "1", "2", "3"], vec![("1", "2")], vec![("0", "1")]);
                    let mut g = $G::new_partial(i, j, k);
                    // Test for added edges
                    g.add_undirected_edge_by_index(0, 3);
                    g.add_directed_edge_by_index(3, 2);
                    assert!(g.has_undirected_edge_by_index(0, 3) == true);
                    assert!(g.has_undirected_edge_by_index(3, 0) == true);
                    assert!(g.has_directed_edge_by_index(3, 2) == true);
                    assert!(g.has_undirected_edge_by_index(2, 3) == false);
                    assert!(g.size() == 4);
                    // Test for already present edges
                    assert!(g.add_undirected_edge_by_index(0, 1) == false);
                    assert!(g.add_directed_edge_by_index(0, 1) == false);
                    assert!(g.add_directed_edge_by_index(1, 0) == false);
                    assert!(g.size_of_undirected_subgraph() == 2);
                    assert!(g.size_of_directed_subgraph() == 2);
                    assert!(g.size() == 4);
                }

                #[test]
                #[should_panic]
                fn add_edge_of_type_should_panic() {
                    let (i, j, k) = (vec!["0", "1", "2", "3"], vec![("1", "2")], vec![("0", "1")]);
                    let mut g = $G::new_partial(i, j, k);
                    // Test with a non-present vertex
                    g.add_undirected_edge_by_index(0, 4);
                }

                #[test]
                fn orient_edge() {
                    let (i, j, k) = (
                        vec!["0", "1", "2", "3", "4"],
                        vec![("1", "2"), ("1", "4")],
                        vec![("0", "1"), ("0", "3")],
                    );
                    let mut g = $G::new_partial(i, j, k);

                    g.orient_edge(0, 1);
                    g.orient_edge(3, 0);
                    g.orient_edge(2, 1);
                    // Test for type of edges
                    assert!(g.has_directed_edge_by_index(0, 1) == true);
                    assert!(g.has_undirected_edge_by_index(1, 0) == false);
                    assert!(g.has_directed_edge_by_index(3, 0) == true);
                    assert!(g.has_undirected_edge_by_index(0, 3) == false);
                    assert!(g.has_directed_edge_by_index(2, 1) == true);
                    assert!(g.has_undirected_edge_by_index(1, 2) == false);
                    assert!(g.has_undirected_edge_by_index(1, 4) == true);
                    assert!(g.has_undirected_edge_by_index(4, 1) == true);
                    // Test for sizes
                    assert!(g.size_of_undirected_subgraph() == 1);
                    assert!(g.size_of_directed_subgraph() == 3);
                    assert!(g.size() == 4);
                    // Test when orienting a non-existing edge
                    assert!(g.orient_edge(2, 3) == false);
                }
            };
        }

        #[allow(unstable_name_collisions)]
        mod partially_dense_matrix {
            use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
            generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
        }
    }
}
