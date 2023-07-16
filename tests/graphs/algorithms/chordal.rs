#[cfg(test)]
mod tests {
    macro_rules! generic_tests {
        ($G: ident) => {
            use causal_hub::{graphs::algorithms::chordal::MCS, prelude::*};

            #[test]
            fn fill_in() {
                let data = vec![
                    (vec![], vec![], vec![], vec![]),
                    (vec!["A"], vec![], vec![0], vec![]),
                    (vec!["A", "B"], vec![], vec![0, 1], vec![]),
                    (vec!["A", "B"], vec![("A", "B")], vec![0, 1], vec![]),
                    (
                        vec!["A", "B", "C"],
                        vec![("A", "B"), ("B", "C")],
                        vec![0, 1, 2],
                        vec![],
                    ),
                    (
                        vec!["A", "B", "C", "D"],
                        vec![("A", "B"), ("B", "C"), ("C", "D"), ("D", "A")],
                        vec![0, 1, 2, 3],
                        vec![(0, 2)],
                    ),
                ];

                for (v, e, true_a, true_f) in data {
                    let g = $G::new(v, e);

                    let (pred_a, pred_f) = MCS::new(&g).fill_in();

                    assert_eq!(pred_a, true_a);
                    assert_eq!(pred_f, true_f);
                }
            }
        };
    }

    mod undirected_dense_matrix {
        use causal_hub::graphs::UndirectedDenseAdjacencyMatrixGraph;
        generic_tests!(UndirectedDenseAdjacencyMatrixGraph);
    }
}
