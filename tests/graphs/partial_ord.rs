#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use std::cmp::Ordering;

            use causal_hub::prelude::*;

            #[test]
            fn eq() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], true),
                            (vec!["0"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], true),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], true),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], true),
                            (vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], true),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], true),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                true,
                            ),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (i, j, f) in k {
                        let h = $G::new(i, j);
                        assert_eq!(g.eq(&h), f);
                    }
                }
            }

            #[test]
            fn partial_cmp() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Equal)),
                            (vec!["0"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], Some(Ordering::Less)),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Equal)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], Some(Ordering::Less)),
                            (vec![], vec![("0", "1")], Some(Ordering::Less)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], None),
                            (vec![], vec![("0", "0")], Some(Ordering::Equal)),
                            (vec![], vec![("0", "1")], None),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Equal)),
                            (vec!["0", "1", "2", "3", "4"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], None),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["71"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["71", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["71", "1", "58", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["71", "1", "58", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("71", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("71", "71")], None),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (i, j, f) in k {
                        let h = $G::new(i, j);
                        assert!(g.partial_cmp(&h).eq(&f));
                    }
                }
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
            use std::cmp::Ordering;

            use causal_hub::prelude::*;

            #[test]
            fn eq() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], true),
                            (vec!["0"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], true),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], true),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], true),
                            (vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], true),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], true),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (vec![], vec![], false),
                            (vec!["0"], vec![], false),
                            (vec!["0", "1"], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], false),
                            (vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], false),
                            (vec![], vec![("0", "1"), ("1", "2"), ("2", "3")], false),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                true,
                            ),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (i, j, f) in k {
                        let h = $G::new(i, j);
                        assert_eq!(g.eq(&h), f);
                    }
                }
            }

            #[test]
            fn partial_cmp() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Equal)),
                            (vec!["0"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], Some(Ordering::Less)),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Equal)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], Some(Ordering::Less)),
                            (vec![], vec![("0", "1")], Some(Ordering::Less)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], None),
                            (vec![], vec![("0", "0")], Some(Ordering::Equal)),
                            (vec![], vec![("0", "1")], None),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Equal)),
                            (vec!["0", "1", "2", "3", "4"], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], None),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["0", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("1", "1")], None),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (vec![], vec![], Some(Ordering::Greater)),
                            (vec!["71"], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], None),
                            (vec!["71", "1"], vec![], Some(Ordering::Greater)),
                            (vec!["71", "1", "58", "3"], vec![], Some(Ordering::Greater)),
                            (vec!["71", "1", "58", "3", "4"], vec![], None),
                            (vec![], vec![("0", "0")], None),
                            (vec![], vec![("71", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("71", "71")], None),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (i, j, f) in k {
                        let h = $G::new(i, j);
                        assert!(g.partial_cmp(&h).eq(&f));
                    }
                }
            }
        };
    }

    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}
