#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;

            use std::collections::HashSet;
            use is_sorted::IsSorted;
            use ndarray::prelude::*;
            use regex::Regex;

            #[test]
            fn clone() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (vec![], Default::default()),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A"], array![[false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, false], [false, false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, true], [true, false]]),
                ];

                // Test for each scenario ...
                for (vertices, adjacency_matrix) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert_eq!(g, g.clone());
                }
            }

            #[test]
            fn debug() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ labels: \{\}, labels_indices: \{\}, adjacency_matrix: \[\[\]\], shape=\[0, 0\], strides=\[0, 0\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A"\}, labels_indices: \{"A" <> 0\}, adjacency_matrix: \[\[false\]\], shape=\[1, 1\], strides=\[1, 1\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A", "B"\}, labels_indices: \{("A" <> 0, "B" <> 1|"B" <> 1, "A" <> 0)\}, adjacency_matrix: \[\[false, false\],\n \[false, false\]\], shape=\[2, 2\], strides=\[2, 1\], layout=Cc \(0x5\), const ndim=2, size: 0 \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_debug) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_debug).unwrap().is_match(&*format!("{:?}", g)));
                }
            }

            #[test]
            fn display() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ V = \{\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A"\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A", "B"\}, E = \{\} \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_display) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_display).unwrap().is_match(&*format!("{}", g)));
                }
            }

            #[test]
            fn hash() {
                let g = $G::new(
                    vec!["0", "1", "2", "3"],
                    vec![("0", "1")]
                );
                let h = g.clone();

                let mut set = HashSet::new();
                set.insert(g);

                assert!(set.contains(&h));
            }

            #[test]
            fn new() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], (1, 0, vec!["0"], vec![])),
                    // ... zero vertices and one edge,
                    (
                        vec![],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        (4, 0, vec!["0", "1", "2", "3"], vec![]),
                    ),
                    // ... zero vertices and multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        (4, 1, vec!["0", "1", "2", "3"], vec![("0", "1")]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            5,
                            4,
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            11,
                            4,
                            vec![
                                "1", "100", "18", "29", "3", "35", "58", "62", "71", "75", "99",
                            ],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (o, s, v, e)) in data {
                    let g = $G::new(i, j);
                    assert_eq!(g.order(), o);
                    assert_eq!(g.size(), s);
                    assert!(V!(g).is_sorted());
                    assert!(E!(g).is_sorted());
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn clear() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![]),
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
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new(i, j);
                    g.clear();
                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }
            }

            #[test]
            fn labels() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(i, []);
                    assert!(L!(g).is_sorted());
                    assert!(L!(g).eq(L!(g)));
                    assert!(L!(g).eq(j));
                }
            }

            #[test]
            fn vertices() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(i, []);
                    assert!(V!(g).is_sorted());
                    assert!(V!(g).eq(j.iter().map(|x| g.vertex(x))));
                }
            }

            #[test]
            fn order() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], 0),
                    // ... one vertex,
                    (vec!["0"], 1),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], 4),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], 5),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.order(), j);
                }
            }

            #[test]
            fn has_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.has_vertex(x), f);
                }
            }

            #[test]
            fn add_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], ("0", 0)),
                    // ... one vertex,
                    (vec!["0"], ("0", 0)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], ("1", 1)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], ("2", 1)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.add_vertex(x), f);
                }
            }

            #[test]
            fn del_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.del_vertex(x), f);
                }
            }

            #[test]
            fn edges() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], vec![]),
                    // ... one edge,
                    (vec![("0", "0")], vec![("0", "0")]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(vec![], i);
                    assert!(E!(g).is_sorted());
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn size() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], 1),
                    // ... multiple edges,
                    (vec![("0", "1"), ("1", "2"), ("2", "3")], 3),
                    // ... random edges,
                    (vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")], 4),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    assert_eq!(g.size(), j);
                }
            }

            #[test]
            fn has_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), true), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), true),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.has_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn has_edge_should_panic() {
                let g = $G::null();
                g.has_edge(0, 0);
            }

            #[test]
            fn add_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), false)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), false), (("1", "0"), false), (("1", "3"), true)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), false),
                            (("1", "58"), false),
                            (("58", "1"), false),
                            (("71", "71"), true),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.add_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn add_edge_should_panic() {
                let mut g = $G::null();
                g.add_edge(0, 0);
            }

            #[test]
            fn del_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), false), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), false),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.del_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn del_edge_should_panic() {
                let mut g = $G::null();
                g.del_edge(0, 0);
            }

            #[test]
            fn adjacents() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![("0", vec![])]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![("0", vec!["0"])]),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![("0", vec![]), ("1", vec![]), ("2", vec![]), ("3", vec![])],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0"]),
                            ("2", vec![]),
                            ("3", vec![]),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0", "2"]),
                            ("2", vec!["1", "3"]),
                            ("3", vec!["2"]),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("58", vec!["1", "3"]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("18", vec![]),
                            ("29", vec![]),
                            ("35", vec![]),
                            ("58", vec!["1", "3"]),
                            ("62", vec![]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                            ("99", vec![]),
                            ("100", vec![]),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (x, ys) in k {
                        let x = g.vertex(x);
                        assert!(Adj!(g, x).is_sorted());
                        assert!(Adj!(g, x).eq(ys.into_iter().map(|y| g.vertex(y))));
                    }
                }
            }

            #[test]
            #[should_panic]
            fn adjacents_should_panic() {
                let g = $G::null();
                Adj!(g, 0);
            }

            #[test]
            fn is_adjacent() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), true), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), true),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.is_adjacent(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn is_adjacent_should_panic() {
                let g = $G::null();
                g.is_adjacent(0, 0);
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

            use std::collections::HashSet;
            use ndarray::prelude::*;
            use regex::Regex;

            #[test]
            fn clone() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (vec![], Default::default()),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A"], array![[false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, false], [false, false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, true], [true, false]]),
                ];

                // Test for each scenario ...
                for (vertices, adjacency_matrix) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert_eq!(g, g.clone());
                }
            }

            #[test]
            fn debug() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ labels: \{\}, labels_indices: \{\}, adjacency_matrix: \[\[\]\], shape=\[0, 0\], strides=\[0, 0\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A"\}, labels_indices: \{"A" <> 0\}, adjacency_matrix: \[\[false\]\], shape=\[1, 1\], strides=\[1, 1\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A", "B"\}, labels_indices: \{("A" <> 0, "B" <> 1|"B" <> 1, "A" <> 0)\}, adjacency_matrix: \[\[false, false\],\n \[false, false\]\], shape=\[2, 2\], strides=\[2, 1\], layout=Cc \(0x5\), const ndim=2, size: 0 \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_debug) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_debug).unwrap().is_match(&*format!("{:?}", g)));
                }
            }

            #[test]
            fn display() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ V = \{\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A"\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A", "B"\}, E = \{\} \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_display) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_display).unwrap().is_match(&*format!("{}", g)));
                }
            }

            #[test]
            fn hash() {
                let g = $G::new(
                    vec!["0", "1", "2", "3"],
                    vec![("0", "1")]
                );
                let h = g.clone();

                let mut set = HashSet::new();
                set.insert(g);

                assert!(set.contains(&h));
            }

            #[test]
            fn new() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], (1, 0, vec!["0"], vec![])),
                    // ... zero vertices and one edge,
                    (
                        vec![],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        (4, 0, vec!["0", "1", "2", "3"], vec![]),
                    ),
                    // ... zero vertices and multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            5,
                            4,
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("3", "75"), ("58", "3"), ("71", "1")],
                        ),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            11,
                            4,
                            vec![
                                "1", "100", "18", "29", "3", "35", "58", "62", "71", "75", "99",
                            ],
                            vec![("1", "58"), ("3", "75"), ("58", "3"), ("71", "1")],
                        ),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (o, s, v, e)) in data {
                    let g = $G::new(i, j);
                    assert_eq!(g.order(), o);
                    assert_eq!(g.size(), s);
                    assert!(V!(g).is_sorted());
                    assert!(E!(g).is_sorted());
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn clear() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![]),
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
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new(i, j);
                    g.clear();
                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }
            }

            #[test]
            fn labels() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(i, []);
                    assert!(L!(g).is_sorted());
                    assert!(L!(g).eq(L!(g)));
                    assert!(L!(g).eq(j));
                }
            }

            #[test]
            fn vertices() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::empty(i);
                    assert!(V!(g).is_sorted());
                    assert!(V!(g).eq(j.iter().map(|x| g.vertex(x))));
                }
            }

            #[test]
            fn order() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], 0),
                    // ... one vertex,
                    (vec!["0"], 1),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], 4),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], 5),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.order(), j);
                }
            }

            #[test]
            fn has_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.has_vertex(x), f);
                }
            }

            #[test]
            fn add_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], ("0", 0)),
                    // ... one vertex,
                    (vec!["0"], ("0", 0)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], ("1", 1)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], ("2", 1)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.add_vertex(x), f);
                }
            }

            #[test]
            fn del_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.del_vertex(x), f);
                }
            }

            #[test]
            fn edges() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], vec![]),
                    // ... one edge,
                    (vec![("0", "0")], vec![("0", "0")]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "58"), ("3", "75"), ("58", "3"), ("71", "1")],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(vec![], i);
                    assert!(E!(g).is_sorted());
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn size() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], 1),
                    // ... multiple edges,
                    (vec![("0", "1"), ("1", "2"), ("2", "3")], 3),
                    // ... random edges,
                    (vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")], 4),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    assert_eq!(g.size(), j);
                }
            }

            #[test]
            fn has_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), false), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), false),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.has_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn has_edge_should_panic() {
                let g = $G::null();
                g.has_edge(0, 0);
            }

            #[test]
            fn add_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), false)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), false), (("1", "0"), true), (("1", "3"), true)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), false),
                            (("1", "58"), false),
                            (("58", "1"), true),
                            (("71", "71"), true),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.add_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn add_edge_should_panic() {
                let mut g = $G::null();
                g.add_edge(0, 0);
            }

            #[test]
            fn del_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), false), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), false),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.del_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn del_edge_should_panic() {
                let mut g = $G::null();
                g.del_edge(0, 0);
            }

            #[test]
            fn adjacents() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![("0", vec![])]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![("0", vec!["0"])]),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![("0", vec![]), ("1", vec![]), ("2", vec![]), ("3", vec![])],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0"]),
                            ("2", vec![]),
                            ("3", vec![]),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0", "2"]),
                            ("2", vec!["1", "3"]),
                            ("3", vec!["2"]),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("58", vec!["1", "3"]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("18", vec![]),
                            ("29", vec![]),
                            ("35", vec![]),
                            ("58", vec!["1", "3"]),
                            ("62", vec![]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                            ("99", vec![]),
                            ("100", vec![]),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (x, ys) in k {
                        let x = g.vertex(x);
                        assert!(Adj!(g, x).is_sorted());
                        assert!(Adj!(g, x).eq(ys.into_iter().map(|y| g.vertex(y))));
                    }
                }
            }

            #[test]
            #[should_panic]
            fn adjacents_should_panic() {
                let g = $G::null();
                Adj!(g, 0);
            }

            #[test]
            fn is_adjacent() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            (("0", "1"), true),
                            (("1", "0"), true),
                            (("1", "3"), false)
                        ],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), true),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.is_adjacent(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn is_adjacent_should_panic() {
                let g = $G::null();

                g.is_adjacent(0, 0);
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
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::prelude::*;

            use std::collections::HashSet;
            use is_sorted::IsSorted;
            use ndarray::prelude::*;
            use regex::Regex;

            #[test]
            fn clone() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (vec![], Default::default()),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A"], array![[false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, false], [false, false]]),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (vec!["A", "B"], array![[false, true], [true, false]]),
                ];

                // Test for each scenario ...
                for (vertices, adjacency_matrix) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert_eq!(g, g.clone());
                }
            }

            #[test]
            fn debug() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ labels: \{\}, labels_indices: \{\}, adjacency_matrix: \[\[\]\], shape=\[0, 0\], strides=\[0, 0\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A"\}, labels_indices: \{"A" <> 0\}, adjacency_matrix: \[\[false\]\], shape=\[1, 1\], strides=\[1, 1\], layout=CFcf \(0xf\), const ndim=2, size: 0 \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ labels: \{"A", "B"\}, labels_indices: \{("A" <> 0, "B" <> 1|"B" <> 1, "A" <> 0)\}, adjacency_matrix: \[\[false, false\],\n \[false, false\]\], shape=\[2, 2\], strides=\[2, 1\], layout=Cc \(0x5\), const ndim=2, size: 0 \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_debug) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_debug).unwrap().is_match(&*format!("{:?}", g)));
                }
            }

            #[test]
            fn display() {
                // Test for ...
                let data = [
                    // Empty vertex set and adjacency matrix.
                    (
                        (vec![], Default::default()),
                        r#"[a-zA-Z]+Graph \{ V = \{\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A"], array![[false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A"\}, E = \{\} \}"#,
                    ),
                    // Non-empty vertex set and non-empty adjacency matrix.
                    (
                        (vec!["A", "B"], array![[false, false], [false, false]]),
                        r#"[a-zA-Z]+Graph \{ V = \{"A", "B"\}, E = \{\} \}"#,
                    ),
                ];

                // Test for each scenario ...
                for ((vertices, adjacency_matrix), test_display) in data {
                    // ... construct the graph ...
                    let g = $G::try_from((vertices, adjacency_matrix)).unwrap();
                    // ... assert result.
                    assert!(Regex::new(test_display).unwrap().is_match(&*format!("{}", g)));
                }
            }

            #[test]
            fn hash() {
                let g = $G::new(
                    vec!["0", "1", "2", "3"],
                    vec![("0", "1")]
                );
                let h = g.clone();

                let mut set = HashSet::new();
                set.insert(g);

                assert!(set.contains(&h));
            }

            #[test]
            fn new() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], (1, 0, vec!["0"], vec![])),
                    // ... zero vertices and one edge,
                    (
                        vec![],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... one vertex and one edge,
                    (
                        vec!["0"],
                        vec![("0", "0")],
                        (1, 1, vec!["0"], vec![("0", "0")]),
                    ),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        (4, 0, vec!["0", "1", "2", "3"], vec![]),
                    ),
                    // ... zero vertices and multiple edges,
                    (
                        vec![],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        (4, 1, vec!["0", "1", "2", "3"], vec![("0", "1")]),
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        (
                            4,
                            3,
                            vec!["0", "1", "2", "3"],
                            vec![("0", "1"), ("1", "2"), ("2", "3")],
                        ),
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            5,
                            4,
                            vec!["1", "3", "58", "71", "75"],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        (
                            11,
                            4,
                            vec![
                                "1", "100", "18", "29", "3", "35", "58", "62", "71", "75", "99",
                            ],
                            vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                        ),
                    ),
                ];

                // Test for each scenario.
                for (i, j, (o, s, v, e)) in data {
                    let g = $G::new(i, j);
                    assert_eq!(g.order(), o);
                    assert_eq!(g.size(), s);
                    assert!(V!(g).is_sorted());
                    assert!(E!(g).is_sorted());
                    assert!(V!(g).eq(v.into_iter().map(|x| g.vertex(x))));
                    assert!(E!(g).eq(e.into_iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn clear() {
                // Test for ...
                let data = [
                    // ... zero vertices and zero edges,
                    (vec![], vec![]),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")]),
                    // ... multiple vertices and zero edges,
                    (vec!["0", "1", "2", "3"], vec![]),
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
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new(i, j);
                    g.clear();
                    assert_eq!(g.order(), 0);
                    assert_eq!(g.size(), 0);
                    assert!(V!(g).next().is_none());
                    assert!(E!(g).next().is_none());
                }
            }

            #[test]
            fn labels() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(i, []);
                    assert!(L!(g).is_sorted());
                    assert!(L!(g).eq(L!(g)));
                    assert!(L!(g).eq(j));
                }
            }

            #[test]
            fn vertices() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], vec![]),
                    // ... one vertex,
                    (vec!["0"], vec!["0"]),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], vec!["0", "1", "2", "3"]),
                    // ... random vertices,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec!["1", "3", "58", "71", "75"],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(i, []);
                    assert!(V!(g).is_sorted());
                    assert!(V!(g).eq(j.iter().map(|x| g.vertex(x))));
                }
            }

            #[test]
            fn order() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], 0),
                    // ... one vertex,
                    (vec!["0"], 1),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], 4),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], 5),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.order(), j);
                }
            }

            #[test]
            fn has_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let g = $G::empty(i);
                    assert_eq!(g.has_vertex(x), f);
                }
            }

            #[test]
            fn add_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], ("0", 0)),
                    // ... one vertex,
                    (vec!["0"], ("0", 0)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], ("1", 1)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], ("2", 1)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.add_vertex(x), f);
                }
            }

            #[test]
            fn del_vertex() {
                // Test for ...
                let data = [
                    // ... zero vertices,
                    (vec![], (0, false)),
                    // ... one vertex,
                    (vec!["0"], (0, true)),
                    // ... multiple vertices,
                    (vec!["0", "1", "2", "3"], (1, true)),
                    // ... random vertices,
                    (vec!["71", "1", "58", "3", "75"], (5, false)),
                ];

                // Test for each scenario.
                for (i, (x, f)) in data {
                    let mut g = $G::empty(i);
                    assert_eq!(g.del_vertex(x), f);
                }
            }

            #[test]
            fn edges() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], vec![]),
                    // ... one edge,
                    (vec![("0", "0")], vec![("0", "0")]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![("1", "58"), ("1", "71"), ("3", "58"), ("3", "75")],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new(vec![], i);
                    assert!(E!(g).is_sorted());
                    assert!(E!(g).eq(j.iter().map(|(x, y)| (g.vertex(x), g.vertex(y)))));
                }
            }

            #[test]
            fn size() {
                // Test for ...
                let data = [
                    // ... zero edges,
                    (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], 1),
                    // ... multiple edges,
                    (vec![("0", "1"), ("1", "2"), ("2", "3")], 3),
                    // ... random edges,
                    (vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")], 4),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    assert_eq!(g.size(), j);
                }
            }

            #[test]
            fn has_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), true), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), true),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.has_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn has_edge_should_panic() {
                let g = $G::null();
                g.has_edge(0, 0);
            }

            #[test]
            fn add_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), false)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), false), (("1", "0"), false), (("1", "3"), true)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), false),
                            (("1", "58"), false),
                            (("58", "1"), false),
                            (("71", "71"), true),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.add_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn add_edge_should_panic() {
                let mut g = $G::null();
                g.add_edge(0, 0);
            }

            #[test]
            fn del_edge() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), false), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), false),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let mut g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.del_edge(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn del_edge_should_panic() {
                let mut g = $G::null();
                g.del_edge(0, 0);
            }

            #[test]
            fn adjacents() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero vertices and zero edges,
                    // (vec![], vec![], (0, 0, vec![], vec![])),
                    // ... one vertex and zero edges,
                    (vec!["0"], vec![], vec![("0", vec![])]),
                    // ... one vertex and one edge,
                    (vec!["0"], vec![("0", "0")], vec![("0", vec!["0"])]),
                    // ... multiple vertices and zero edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![],
                        vec![("0", vec![]), ("1", vec![]), ("2", vec![]), ("3", vec![])],
                    ),
                    // ... multiple vertices and one edge,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0"]),
                            ("2", vec![]),
                            ("3", vec![]),
                        ],
                    ),
                    // ... multiple vertices and multiple edges,
                    (
                        vec!["0", "1", "2", "3"],
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![
                            ("0", vec!["1"]),
                            ("1", vec!["0", "2"]),
                            ("2", vec!["1", "3"]),
                            ("3", vec!["2"]),
                        ],
                    ),
                    // ... random vertices and edges,
                    (
                        vec!["71", "1", "58", "3", "75"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("58", vec!["1", "3"]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                        ],
                    ),
                    // ... random non-overlapping vertices and edges,
                    (
                        vec!["35", "62", "99", "29", "100", "18"],
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            ("1", vec!["58", "71"]),
                            ("3", vec!["58", "75"]),
                            ("18", vec![]),
                            ("29", vec![]),
                            ("35", vec![]),
                            ("58", vec!["1", "3"]),
                            ("62", vec![]),
                            ("71", vec!["1"]),
                            ("75", vec!["3"]),
                            ("99", vec![]),
                            ("100", vec![]),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j, k) in data {
                    let g = $G::new(i, j);
                    for (x, ys) in k {
                        let x = g.vertex(x);
                        assert!(Adj!(g, x).is_sorted());
                        assert!(Adj!(g, x).eq(ys.into_iter().map(|y| g.vertex(y))));
                    }
                }
            }

            #[test]
            #[should_panic]
            fn adjacents_should_panic() {
                let g = $G::null();
                Adj!(g, 0);
            }

            #[test]
            fn is_adjacent() {
                // Test for ...
                let data = [
                    // NOTE: This would panic!
                    // ... zero edges,
                    // (vec![], 0),
                    // ... one edge,
                    (vec![("0", "0")], vec![(("0", "0"), true)]),
                    // ... multiple edges,
                    (
                        vec![("0", "1"), ("1", "2"), ("2", "3")],
                        vec![(("0", "1"), true), (("1", "0"), true), (("1", "3"), false)],
                    ),
                    // ... random edges,
                    (
                        vec![("71", "1"), ("1", "58"), ("58", "3"), ("3", "75")],
                        vec![
                            (("71", "1"), true),
                            (("1", "58"), true),
                            (("58", "1"), true),
                            (("71", "71"), false),
                        ],
                    ),
                ];

                // Test for each scenario.
                for (i, j) in data {
                    let g = $G::new([], i);
                    for ((x, y), f) in j {
                        let (x, y) = (g.vertex(x), g.vertex(y));
                        assert_eq!(g.is_adjacent(x, y), f);
                    }
                }
            }

            #[test]
            #[should_panic]
            fn is_adjacent_should_panic() {
                let g = $G::null();
                g.is_adjacent(0, 0);
            }
        };
    }

    #[allow(unstable_name_collisions)]
    mod partially_dense_matrix {
        use causal_hub::graphs::structs::PartiallyDenseAdjacencyMatrixGraph;
        generic_tests!(PartiallyDenseAdjacencyMatrixGraph);
    }
}