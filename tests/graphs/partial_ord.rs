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

#[cfg(test)]
mod partially {
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
                        vec![],
                        vec![
                            (vec![], vec![], vec![], true),
                            (vec!["0"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec!["0"], vec![], vec![], true),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], vec![], false),
                            (vec![], vec![], vec![("0", "1")], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                            (
                                vec![],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                false,
                            ),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("0", "3")],
                                false,
                            ),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], true),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec![], vec![("0", "1")], vec![], false),
                            (vec![], vec![], vec![("0", "1")], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], true),
                            (vec![], vec![("0", "1")], vec![], false),
                            (vec![], vec![], vec![("0", "1")], false),
                            (vec![], vec![("3", "2")], vec![("0", "1")], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], true),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![("0", "1")], vec![], false),
                            (vec![], vec![], vec![("0", "1")], false),
                            (vec![], vec![("1", "2")], vec![("0", "1")], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], vec![], true),
                            (vec!["0", "1", "2", "3"], vec![], vec![("0", "1")], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                            (
                                vec![],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                false,
                            ),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("3", "1")],
                                false,
                            ),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![("0", "1")], true),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                            (
                                vec![],
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                false,
                            ),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("3", "1")],
                                false,
                            ),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3", "4"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("2", "4"), ("4", "1")],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], vec![], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("2", "4"), ("4", "1")],
                                true,
                            ),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                            (vec![], vec![], vec![("2", "4"), ("4", "1")], false),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1")],
                        vec![
                            (vec![], vec![], vec![], false),
                            (vec!["0"], vec![], vec![], false),
                            (vec!["0", "1"], vec![], vec![], false),
                            (vec!["0", "1", "2", "3"], vec![], vec![], false),
                            (vec![], vec![("0", "0")], vec![], false),
                            (vec![], vec![], vec![("0", "0")], false),
                            (vec!["0", "1", "2", "3"], vec![("0", "1")], vec![], false),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![],
                                false,
                            ),
                            (
                                vec![],
                                vec![("0", "1"), ("1", "2"), ("2", "3")],
                                vec![("1", "1")],
                                false,
                            ),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                vec![],
                                false,
                            ),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                vec![("2", "1")],
                                false,
                            ),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![],
                                vec![("1", "1")],
                                false,
                            ),
                            (
                                vec!["71", "1", "58", "3", "75"],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                vec![("1", "1")],
                                true,
                            ),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, ue, de, k) in data {
                    dbg!(i.clone());
                    let g = $G::new_spec(i, ue, de).unwrap();
                    for (i, ue, de, f) in k {
                        let h = $G::new_spec(i, ue, de).unwrap();
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
                        vec![],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Equal)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Less)),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], vec![], Some(Ordering::Less)),
                            (vec![], vec![], vec![("0", "0")], Some(Ordering::Less)),
                            (
                                vec![],
                                vec![("0", "3")],
                                vec![("0", "1")],
                                Some(Ordering::Less),
                            ),
                        ],
                    ),
                    // ... one vertex and zero edges,
                    (
                        vec!["0"],
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Equal)),
                            (vec!["0"], vec![], vec![("0", "1")], Some(Ordering::Less)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Less)),
                            (vec![], vec![("0", "0")], vec![], Some(Ordering::Less)),
                            (vec![], vec![], vec![("0", "0")], Some(Ordering::Less)),
                            (vec![], vec![("0", "1")], vec![], Some(Ordering::Less)),
                            (vec![], vec![], vec![("0", "1")], Some(Ordering::Less)),
                            (vec![], vec![], vec![("0", "1")], Some(Ordering::Less)),
                            (
                                vec![],
                                vec![("0", "3")],
                                vec![("0", "1")],
                                Some(Ordering::Less),
                            ),
                            (vec![], vec![("1", "3")], vec![("4", "1")], None),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], None),
                            (vec![], vec![("0", "0")], vec![], Some(Ordering::Equal)),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], vec![], None),
                            (vec![], vec![], vec![("0", "1")], None),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![],
                        vec![("0", "0")],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], None),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], Some(Ordering::Equal)),
                            (vec![], vec![("0", "1")], vec![], None),
                            (vec![], vec![], vec![("0", "1")], None),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Greater)),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![],
                                Some(Ordering::Equal),
                            ),
                            (
                                vec!["0", "1", "2", "3", "4"],
                                vec![],
                                vec![],
                                Some(Ordering::Less),
                            ),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], vec![], None),
                            (vec![], vec![], vec![("0", "1")], None),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Greater)),
                            (
                                vec!["0", "1", "2"],
                                vec![("0", "1")],
                                vec![],
                                Some(Ordering::Greater),
                            ),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![("0", "1")],
                                vec![("0", "2")],
                                Some(Ordering::Less),
                            ),
                            (vec!["0", "1", "2"], vec![], vec![("0", "1")], None),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![],
                                Some(Ordering::Greater),
                            ),
                            (vec!["0", "1", "2", "3", "4"], vec![], vec![], None),
                            (vec!["0", "1", "2", "3"], vec![], vec![("0", "1")], None),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], vec![], Some(Ordering::Greater)),
                            (vec![], vec![], vec![("0", "1")], None),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![("0", "1")],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0", "1", "2"], vec![("0", "1")], vec![], None),
                            (
                                vec!["0", "1", "2"],
                                vec![],
                                vec![("0", "1")],
                                Some(Ordering::Greater),
                            ),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![("0", "2")],
                                vec![("0", "1")],
                                Some(Ordering::Less),
                            ),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![],
                                Some(Ordering::Greater),
                            ),
                            (vec!["0", "1", "2", "3", "4"], vec![], vec![], None),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![("0", "1")],
                                Some(Ordering::Equal),
                            ),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], vec![], None),
                            (vec![], vec![], vec![("0", "1")], Some(Ordering::Greater)),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3", "4"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("1", "3"), ("4", "2")],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["0"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["5"], vec![], vec![], None),
                            (vec!["0", "1"], vec![], vec![], Some(Ordering::Greater)),
                            (
                                vec!["0", "1", "2", "3"],
                                vec![],
                                vec![],
                                Some(Ordering::Greater),
                            ),
                            (vec!["0", "1", "2", "3", "4", "5"], vec![], vec![], None),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("0", "1")], vec![], Some(Ordering::Greater)),
                            (vec![], vec![], vec![("0", "1")], None),
                            (vec![], vec![], vec![("4", "2")], Some(Ordering::Greater)),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![
                            (vec![], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["71"], vec![], vec![], Some(Ordering::Greater)),
                            (vec!["4"], vec![], vec![], None),
                            (vec!["71", "1"], vec![], vec![], Some(Ordering::Greater)),
                            (
                                vec!["71", "1", "58", "3"],
                                vec![],
                                vec![],
                                Some(Ordering::Greater),
                            ),
                            (vec!["71", "1", "58", "3", "4"], vec![], vec![], None),
                            (vec![], vec![("0", "0")], vec![], None),
                            (vec![], vec![], vec![("0", "0")], None),
                            (vec![], vec![("1", "71")], vec![], Some(Ordering::Greater)),
                            (vec![], vec![], vec![("75", "1")], Some(Ordering::Greater)),
                            (vec![], vec![], vec![("1", "75")], None),
                            (vec![], vec![("71", "71")], vec![], None),
                            (vec![], vec![], vec![("71", "71")], None),
                            (vec![], vec![("0", "3")], vec![("0", "1")], None),
                            (
                                vec![],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                vec![("1", "1"), ("75", "1")],
                                Some(Ordering::Equal),
                            ),
                            (
                                vec![],
                                vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                                vec![("1", "1"), ("75", "1"), ("58", "75")],
                                Some(Ordering::Less),
                            ),
                            (
                                vec![],
                                vec![
                                    ("71", "1"),
                                    ("1", "58"),
                                    ("58", "3"),
                                    ("3", "75"),
                                    ("58", "75"),
                                ],
                                vec![("1", "1"), ("75", "1")],
                                Some(Ordering::Less),
                            ),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, ue, de, k) in data {
                    dbg!(i.clone());
                    let g = $G::new_spec(i, ue, de).unwrap();
                    for (i, ue, de, f) in k {
                        let h = $G::new_spec(i, ue, de).unwrap();
                        assert!(g.partial_cmp(&h).eq(&f));
                    }
                }
            }
        };
    }

    mod partially_dense_matrix {
        use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
        generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
    }
}
