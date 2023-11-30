#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;
            use itertools::Itertools;

            #[test]
            fn connected_components() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    ((vec![], vec![]), vec![]),
                    // ... one vertex and zero edges,
                    ((vec!["0"], vec![]), vec![vec![0]]),
                    // ... one vertex and one edge,
                    ((vec!["0"], vec![("0", "0")]), vec![vec![0]]),
                    // ... multiple vertices and zero edges,
                    (
                        (vec!["0", "1", "2", "3"], vec![]),
                        vec![vec![0], vec![1], vec![2], vec![3]],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        (vec!["0", "1", "2", "3"], vec![("0", "1")]),
                        vec![vec![0, 1], vec![2], vec![3]],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                        vec![vec![0, 1, 2, 3]],
                    ),
                    // ... random vertices and edges,
                    (
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("3", "75")],
                        ),
                        vec![vec![0, 2, 3], vec![1, 4]],
                    ),
                ];

                // Test for each scenario.
                for ((i, j), ccs) in data {
                    let g = $G::new(i, j);

                    let cc = CC::from(&g);

                    assert!(cc.eq(ccs.into_iter().map(|c| c.into_iter().collect_vec())));
                }
            }
        };
    }

    mod undirected_dense_matrix {
        use causal_hub::graphs::structs::UGraph;
        generic_tests!(UGraph);
    }
}
