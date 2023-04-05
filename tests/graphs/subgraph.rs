#[cfg(test)]
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
        use causal_hub::graphs::structs::DirectedDenseAdjacencyMatrixGraph;
        generic_tests!(DirectedDenseAdjacencyMatrixGraph);
    }
}

#[cfg(test)]
mod partially_directed{
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;

            #[test]
            fn subgraph() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], vec![], (vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![], (vec![], vec![])),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![], (vec![0], vec![])),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![], vec![("0", "0")], (vec![0], vec![])),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![],
                        (vec![0, 1], vec![]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        (vec![0, 1, 2], vec![(0, 1)]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        (vec![0, 1, 2, 3], vec![(3, 0)]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        (vec![0, 1, 2], vec![(1, 2)]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        (vec![0, 1, 2, 3, 4], vec![(1, 2), (4, 0)]),
                    ),
                    // ... random vertices and edges,
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        (vec![0, 1, 2, 3, 4], vec![(1, 2)]),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        (vec![0, 1, 2, 3, 4], vec![(4, 0)]),
                    ),
                ];

                // Test for each scenario.
                for (i, ue, de, (v, e)) in data {
                    let g = $G::new_partial(i.clone(), ue, de).unwrap();

                    let h = g.subgraph(v.clone(), e.clone());
                    dbg!(i.clone());
                    assert!(V!(h)
                        .into_iter()
                        .map(|x| h.label(x))
                        .eq(v.into_iter().map(|x| g.label(x))));
                    assert!(iter_set::union(uE!(h), dE!(h))
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
                    (vec![], vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![], vec![0]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![], vec![("0", "0")], vec![0]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![], vec![], vec![0, 1]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        vec![0, 1, 2],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![0, 1, 2, 3, 4],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![0, 1, 2, 3],
                    ),
                ];

                // Test for each scenario.
                for (i, ue, de, v) in data {
                    let g = $G::new_partial(i, ue, de).unwrap();

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
                    (vec![], vec![], vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![], vec![("0", "0")], vec![]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![], vec![], vec![]),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        vec![(0, 1)],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("3", "0")],
                        vec![(3, 0)],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![(1, 2), (4, 0)],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![(1, 2)],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "1"), ("75", "1")],
                        vec![(4, 0)],
                    ),
                ];

                // Test for each scenario.
                for (i, ue, de, e) in data {
                    let g = $G::new_partial(i, ue, de).unwrap();

                    let h = g.subgraph_by_edges(e.clone());
                    dbg!(iter_set::union(uE!(h), dE!(h))
                        .into_iter()
                        .map(|(x, y)| (h.label(x), h.label(y)))
                        .collect::<Vec<_>>());
                    dbg!(e
                        .clone()
                        .into_iter()
                        .map(|(x, y)| (g.label(x), g.label(y)))
                        .collect::<Vec<_>>());
                    assert!(iter_set::union(uE!(h), dE!(h))
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

    mod partially_dense_matrix {
        use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
        generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
    }
}
