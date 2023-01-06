#[cfg(test)]
mod tests {
    mod undirected {
        macro_rules! generic_tests {
            ($G: ident) => {
                use causal_hub::prelude::*;

                #[test]
                fn subgraph() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], (vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], (vec![], vec![])),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], (vec![0], vec![])),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], (vec![0, 1], vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            (vec![0, 1, 2], vec![(0, 1)]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            (vec![0, 1, 2, 3, 4], vec![(0, 2), (0, 3)]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, (v, e)) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph(v.clone(), e.clone());

                        assert!(V!(h)
                            .into_iter()
                            .map(|x| h.label(x))
                            .eq(v.into_iter().map(|x| g.label(x))));
                        assert!(E!(h)
                            .into_iter()
                            .map(|(x, y)| (h.label(x), h.label(y)))
                            .eq(e.into_iter().map(|(x, y)| (g.label(x), g.label(y)))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_should_panic() {
                    let g = $G::null();

                    g.subgraph(vec![0], vec![(0, 0)]);
                }

                #[test]
                fn subgraph_by_vertices() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], vec![0]),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], vec![0, 1]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![0, 1, 2],
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![0, 1, 2, 3, 4],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, v) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph_by_vertices(v.clone());

                        assert!(V!(h)
                            .into_iter()
                            .map(|x| h.label(x))
                            .eq(v.into_iter().map(|x| g.label(x))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_by_vertices_should_panic() {
                    let g = $G::null();

                    g.subgraph_by_vertices(vec![0]);
                }

                #[test]
                fn subgraph_by_edges() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], vec![(0, 0)]),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], vec![]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![(0, 1)],
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![(0, 2), (0, 3)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, e) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph_by_edges(e.clone());

                        assert!(E!(h)
                            .into_iter()
                            .map(|(x, y)| (h.label(x), h.label(y)))
                            .eq(e.into_iter().map(|(x, y)| (g.label(x), g.label(y)))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_by_edges_should_panic() {
                    let g = $G::null();

                    g.subgraph_by_edges(vec![(0, 0)]);
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
                fn subgraph() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], (vec![], vec![])),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], (vec![], vec![])),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], (vec![0], vec![])),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], (vec![0, 1], vec![])),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            (vec![0, 1, 2], vec![(0, 1)]),
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            (vec![0, 1, 2, 3, 4], vec![(0, 2), (3, 0)]),
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, (v, e)) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph(v.clone(), e.clone());

                        assert!(V!(h)
                            .into_iter()
                            .map(|x| h.label(x))
                            .eq(v.into_iter().map(|x| g.label(x))));
                        assert!(E!(h)
                            .into_iter()
                            .map(|(x, y)| (h.label(x), h.label(y)))
                            .eq(e.into_iter().map(|(x, y)| (g.label(x), g.label(y)))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_should_panic() {
                    let g = $G::null();

                    g.subgraph(vec![0], vec![(0, 0)]);
                }

                #[test]
                fn subgraph_by_vertices() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], vec![0]),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], vec![0, 1]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![0, 1, 2],
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![0, 1, 2, 3, 4],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, v) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph_by_vertices(v.clone());

                        assert!(V!(h)
                            .into_iter()
                            .map(|x| h.label(x))
                            .eq(v.into_iter().map(|x| g.label(x))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_by_vertices_should_panic() {
                    let g = $G::null();

                    g.subgraph_by_vertices(vec![0]);
                }

                #[test]
                fn subgraph_by_edges() {
                    // Test for ...
                    let data = [
                        // ... zero vertices and zero edges,
                        (vec![], vec![], vec![]),
                        // ... one vertex and zero edges,
                        (vec!["0"], vec![], vec![]),
                        // ... one vertex and one edge,
                        (vec!["0"], vec![("0", "0")], vec![(0, 0)]),
                        // ... multiple vertices and zero edges,
                        (vec!["0", "1", "2", "3"], vec![], vec![]),
                        // ... multiple vertices and multiple edges,
                        (
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                            vec![(0, 1)],
                        ),
                        // ... random vertices and edges,
                        (
                            vec!["71", "1", "58", "3", "75"],
                            vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                            vec![(0, 2), (3, 0)],
                        ),
                    ];

                    // Test for each scenario.
                    for (i, j, e) in data {
                        let g = $G::new(i, j);

                        let h = g.subgraph_by_edges(e.clone());

                        assert!(E!(h)
                            .into_iter()
                            .map(|(x, y)| (h.label(x), h.label(y)))
                            .eq(e.into_iter().map(|(x, y)| (g.label(x), g.label(y)))));
                        assert!(h.is_subgraph(&g));
                        assert!(g.is_supergraph(&h));
                    }
                }

                #[test]
                #[should_panic]
                fn subgraph_by_edges_should_panic() {
                    let g = $G::null();

                    g.subgraph_by_edges(vec![(0, 0)]);
                }
            };
        }

        mod directed_dense_matrix {
            use causal_hub::graphs::DirectedDenseAdjacencyMatrixGraph;
            generic_tests!(DirectedDenseAdjacencyMatrixGraph);
        }
    }
}