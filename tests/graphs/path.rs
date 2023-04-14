#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;

            #[test]
            fn has_path() {
                // Test for ...
                let data = [
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], "0", "0", false),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], "0", "0", true),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1"], vec![], "0", "1", false),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1", "2"], vec![("0", "1")], "0", "1", true),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2")],
                        "0",
                        "2",
                        true,
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2"), ("2", "0")],
                        "0",
                        "2",
                        true,
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("2", "2")],
                        "0",
                        "2",
                        false,
                    ),
                ];

                // Test for each scenario.
                for (v, e, x, y, f) in data {
                    let g = $G::new(v.clone(), e.clone());

                    assert_eq!(
                        g.has_path(g.vertex(x), g.vertex(y)),
                        f,
                        "(({:?}, {:?}, {}, {}), {})",
                        v,
                        e,
                        x,
                        y,
                        f
                    );
                }
            }

            #[test]
            fn is_acyclic() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], true),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], true),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], false),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1"], vec![], true),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1", "2"], vec![("0", "1")], true),
                    // ... multiple vertices and multiple edges,
                    (vec!["0", "1", "2"], vec![("0", "1"), ("1", "2")], true),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2"), ("2", "0")],
                        false,
                    ),
                    // ... multiple vertices and multiple edges,
                    (vec!["0", "1", "2"], vec![("0", "1"), ("2", "2")], false),
                ];

                // Test for each scenario.
                for (v, e, f) in data {
                    let g = $G::new(v.clone(), e.clone());

                    assert_eq!(g.is_acyclic(), f, "(({:?}, {:?}), {})", v, e, f);
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
            use causal_hub::prelude::*;

            #[test]
            fn has_path() {
                // Test for ...
                let data = [
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], "0", "0", false),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], "0", "0", true),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1"], vec![], "0", "1", false),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1", "2"], vec![("0", "1")], "0", "1", true),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2")],
                        "0",
                        "2",
                        true,
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2"), ("2", "0")],
                        "0",
                        "2",
                        true,
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("2", "2")],
                        "0",
                        "2",
                        false,
                    ),
                ];

                // Test for each scenario.
                for (v, e, x, y, f) in data {
                    let g = $G::new(v.clone(), e.clone());

                    assert_eq!(
                        g.has_path(g.vertex(x), g.vertex(y)),
                        f,
                        "(({:?}, {:?}, {}, {}), {})",
                        v,
                        e,
                        x,
                        y,
                        f
                    );
                }
            }

            #[test]
            fn is_acyclic() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], true),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], true),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], false),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1"], vec![], true),
                    // ... multiple vertices and one edge,
                    (vec!["0", "1", "2"], vec![("0", "1")], true),
                    // ... multiple vertices and multiple edges,
                    (vec!["0", "1", "2"], vec![("0", "1"), ("1", "2")], true),
                    (vec!["0", "1", "2"], vec![("0", "1"), ("1", "0")], false),
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2"), ("2", "1")],
                        false,
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2"],
                        vec![("0", "1"), ("1", "2"), ("2", "0")],
                        false,
                    ),
                    // ... multiple vertices and multiple edges,
                    (vec!["0", "1", "2"], vec![("0", "1"), ("2", "2")], false),
                ];

                // Test for each scenario.
                for (v, e, f) in data {
                    let g = $G::new(v.clone(), e.clone());

                    assert_eq!(g.is_acyclic(), f, "(({:?}, {:?}), {})", v, e, f);
                }
            }
        };
    }

    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}
