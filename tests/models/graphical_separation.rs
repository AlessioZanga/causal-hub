#[cfg(test)]
mod undirected {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::{
                graphs::Graph,
                models::GSeparation,
                stats::{ConditionalIndependenceTest, GeneralizedConditionalIndependenceTest},
            };

            #[test]
            fn call() {
                let e = vec![
                    ("A", "B"),
                    ("A", "C"),
                    ("B", "C"),
                    ("C", "D"),
                    ("C", "F"),
                    ("D", "E"),
                ];

                let g = $G::new([], e);

                let q = GSeparation::from(&g);

                // Check not( A _||_ B | { } )
                assert!(!ConditionalIndependenceTest::call(&q, 0, 1, &[]));
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [1],
                    []
                ));
                // Check      A _||_ B | {D}
                assert!(!ConditionalIndependenceTest::call(&q, 0, 1, &[3]));
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [1],
                    [3]
                ));
                // Check not( A _||_ E | { } )
                assert!(!ConditionalIndependenceTest::call(&q, 0, 1, &[3]));
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [4],
                    []
                ));
                // Check      A _||_ E | {D}
                assert!(ConditionalIndependenceTest::call(&q, 0, 4, &[3]));
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [4],
                    [3]
                ));
            }
        };
    }

    mod undirected_dense_matrix {
        use causal_hub::graphs::structs::UGraph;
        generic_tests!(UGraph);
    }
}

#[cfg(test)]
mod directed {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::{
                graphs::Graph, models::GSeparation, stats::GeneralizedConditionalIndependenceTest,
            };

            #[test]
            fn call() {
                let e = vec![("A", "E"), ("A", "C"), ("B", "C"), ("B", "F"), ("C", "D")];

                let g = $G::new([], e);

                let q = GSeparation::from(&g);

                // Check      E _||_ F | { }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    []
                ));
                // Check      E _||_ B | { }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [1],
                    []
                ));
                // Check      E _||_ F | { B }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    [1]
                ));
                // Check      E _||_ B | { F }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [1],
                    [5]
                ));
                // Check      E _||_ F | { A, C }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    [0, 2]
                ));
                // Check      E _||_ F | { B, D }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    [1, 3]
                ));
                // Check      A _||_ B | { }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [1],
                    []
                ));
                // Check      A _||_ F | { }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [5],
                    []
                ));
                // Check      A _||_ F | { E }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [5],
                    [4]
                ));
                // Check      D _||_ F | { C }
                assert!(GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [3],
                    [5],
                    [2]
                ));

                // Check not( E _||_ A | { } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [0],
                    []
                ));
                // Check not( E _||_ C | { } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [2],
                    []
                ));
                // Check not( E _||_ F | { C } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    [2]
                ));
                // Check not( E _||_ F | { D } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [5],
                    [3]
                ));
                // Check not( E _||_ A | { C, D } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [4],
                    [0],
                    [2, 3]
                ));
                // Check not( A _||_ C | { } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [2],
                    []
                ));
                // Check not( A _||_ D | { } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [3],
                    []
                ));
                // Check not( A _||_ B | { C } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [1],
                    [2]
                ));
                // Check not( A _||_ B | { C, D } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [0],
                    [1],
                    [2, 3]
                ));
                // Check not( B _||_ F | { } )
                assert!(!GeneralizedConditionalIndependenceTest::call(
                    &q,
                    [1],
                    [5],
                    []
                ));
            }
        };
    }

    mod directed_dense_matrix {
        use causal_hub::graphs::structs::DGraph;
        generic_tests!(DGraph);
    }
}
