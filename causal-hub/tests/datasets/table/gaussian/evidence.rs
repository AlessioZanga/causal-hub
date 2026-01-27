#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{GaussEv, GaussEvT},
        labels,
        models::Labelled,
        set,
        types::Result,
    };

    mod gauss_ev_type {
        use super::*;

        #[test]
        fn event() -> Result<()> {
            let evidence = GaussEvT::CertainPositive {
                event: 5,
                value: 1.5,
            };
            assert_eq!(evidence.event(), 5);
            Ok(())
        }

        #[test]
        fn set_event() -> Result<()> {
            let mut evidence = GaussEvT::CertainPositive {
                event: 0,
                value: 2.0,
            };
            evidence.set_event(3);
            assert_eq!(evidence.event(), 3);
            Ok(())
        }

        #[test]
        fn clone() -> Result<()> {
            let evidence = GaussEvT::CertainPositive {
                event: 2,
                value: 3.5,
            };
            let cloned = evidence.clone();
            assert_eq!(cloned.event(), 2);
            Ok(())
        }
    }

    mod gauss_ev_creation {
        use super::*;

        #[test]
        fn new_empty() -> Result<()> {
            let labels = labels!["X", "Y", "Z"];
            let values: Vec<GaussEvT> = vec![];
            let ev = GaussEv::new(labels, values)?;

            assert_eq!(ev.labels(), &labels!["X", "Y", "Z"]);
            assert_eq!(ev.evidences().len(), 3);
            // All should be None.
            assert!(ev.evidences().iter().all(|e| e.is_none()));

            Ok(())
        }

        #[test]
        fn new_with_single_evidence() -> Result<()> {
            let labels = labels!["X", "Y", "Z"];
            let values = vec![GaussEvT::CertainPositive {
                event: 1, // Y
                value: 2.5,
            }];
            let ev = GaussEv::new(labels, values)?;

            assert_eq!(ev.labels().len(), 3);
            // Only Y (index 1) should have evidence.
            assert!(ev.evidences()[0].is_none());
            assert!(ev.evidences()[1].is_some());
            assert!(ev.evidences()[2].is_none());

            Ok(())
        }

        #[test]
        fn new_with_multiple_evidences() -> Result<()> {
            let labels = labels!["A", "B", "C"];
            let values = vec![
                GaussEvT::CertainPositive {
                    event: 0, // A
                    value: 1.0,
                },
                GaussEvT::CertainPositive {
                    event: 2, // C
                    value: 3.0,
                },
            ];
            let ev = GaussEv::new(labels, values)?;

            assert!(ev.evidences()[0].is_some()); // A
            assert!(ev.evidences()[1].is_none()); // B
            assert!(ev.evidences()[2].is_some()); // C

            Ok(())
        }

        #[test]
        fn new_out_of_bounds() -> Result<()> {
            let labels = labels!["X", "Y"];
            let values = vec![GaussEvT::CertainPositive {
                event: 5, // Out of bounds.
                value: 1.0,
            }];
            let res = GaussEv::new(labels, values);

            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("out of bounds"));

            Ok(())
        }

        #[test]
        fn new_with_unsorted_labels() -> Result<()> {
            let labels = labels!["Z", "A", "M"];
            let values = vec![
                GaussEvT::CertainPositive {
                    event: 0, // Originally Z, will become index 2 after sorting.
                    value: 26.0,
                },
                GaussEvT::CertainPositive {
                    event: 1, // Originally A, will become index 0 after sorting.
                    value: 1.0,
                },
            ];
            let ev = GaussEv::new(labels, values)?;

            // Labels should be sorted.
            assert!(ev.labels().iter().is_sorted());
            assert_eq!(ev.labels(), &labels!["A", "M", "Z"]);

            // Evidence should be reindexed according to sorted labels.
            // After sorting: A=0, M=1, Z=2
            // Original event 0 was Z -> now at index 2
            // Original event 1 was A -> now at index 0
            assert!(ev.evidences()[0].is_some()); // A has evidence.
            assert!(ev.evidences()[1].is_none()); // M has no evidence.
            assert!(ev.evidences()[2].is_some()); // Z has evidence.

            Ok(())
        }
    }

    mod gauss_ev_labelled {
        use super::*;

        #[test]
        fn labels() -> Result<()> {
            let labels = labels!["P", "Q", "R"];
            let ev = GaussEv::new(labels.clone(), vec![])?;

            // Labels should be sorted.
            assert_eq!(ev.labels(), &labels!["P", "Q", "R"]);

            Ok(())
        }

        #[test]
        fn labels_after_sorting() -> Result<()> {
            let labels = labels!["C", "A", "B"];
            let ev = GaussEv::new(labels, vec![])?;

            assert_eq!(ev.labels(), &labels!["A", "B", "C"]);

            Ok(())
        }
    }

    mod gauss_ev_select {
        use super::*;

        #[test]
        fn select_subset() -> Result<()> {
            let labels = labels!["A", "B", "C", "D"];
            let values = vec![
                GaussEvT::CertainPositive {
                    event: 0, // A
                    value: 1.0,
                },
                GaussEvT::CertainPositive {
                    event: 2, // C
                    value: 3.0,
                },
            ];
            let ev = GaussEv::new(labels, values)?;

            // Select only A and C.
            let selected = ev.select(&set![0, 2])?;

            // Selected evidence should have only 2 labels.
            assert_eq!(selected.labels().len(), 2);
            assert!(selected.labels().iter().eq(&["A", "C"]));

            // Both should have evidence.
            assert!(selected.evidences()[0].is_some()); // A -> new index 0
            assert!(selected.evidences()[1].is_some()); // C -> new index 1

            Ok(())
        }

        #[test]
        fn select_preserves_some_evidence() -> Result<()> {
            let labels = labels!["X", "Y", "Z"];
            let values = vec![GaussEvT::CertainPositive {
                event: 0, // X
                value: 5.0,
            }];
            let ev = GaussEv::new(labels, values)?;

            // Select Y and Z (X is not included).
            let selected = ev.select(&set![1, 2])?;

            // No evidence should be present (X was not selected).
            assert!(selected.evidences()[0].is_none()); // Y
            assert!(selected.evidences()[1].is_none()); // Z

            Ok(())
        }

        #[test]
        fn select_out_of_bounds() -> Result<()> {
            let labels = labels!["A", "B", "C"];
            let ev = GaussEv::new(labels, vec![])?;

            let res = ev.select(&set![0, 1, 10]);

            assert!(res.is_err());
            assert!(res.unwrap_err().to_string().contains("out of bounds"));

            Ok(())
        }

        #[test]
        fn select_single_variable() -> Result<()> {
            let labels = labels!["A", "B", "C"];
            let values = vec![GaussEvT::CertainPositive {
                event: 1, // B
                value: 2.0,
            }];
            let ev = GaussEv::new(labels, values)?;

            let selected = ev.select(&set![1])?;

            assert_eq!(selected.labels().len(), 1);
            assert!(selected.labels().iter().eq(&["B"]));
            assert!(selected.evidences()[0].is_some());

            Ok(())
        }

        #[test]
        fn select_reindexes_events() -> Result<()> {
            let labels = labels!["A", "B", "C", "D", "E"];
            let values = vec![
                GaussEvT::CertainPositive {
                    event: 2, // C
                    value: 3.0,
                },
                GaussEvT::CertainPositive {
                    event: 4, // E
                    value: 5.0,
                },
            ];
            let ev = GaussEv::new(labels, values)?;

            // Select B, C, E (indices 1, 2, 4).
            let selected = ev.select(&set![1, 2, 4])?;

            assert_eq!(selected.labels().len(), 3);
            // New labels: B=0, C=1, E=2
            assert!(selected.evidences()[0].is_none()); // B - no evidence.
            assert!(selected.evidences()[1].is_some()); // C - has evidence, event reindexed to 1.
            assert!(selected.evidences()[2].is_some()); // E - has evidence, event reindexed to 2.

            // Verify the event indices were updated correctly.
            if let Some(ev_c) = &selected.evidences()[1] {
                assert_eq!(ev_c.event(), 1);
            }
            if let Some(ev_e) = &selected.evidences()[2] {
                assert_eq!(ev_e.event(), 2);
            }

            Ok(())
        }
    }

    mod gauss_ev_accessors {
        use super::*;

        #[test]
        fn evidences_returns_reference() -> Result<()> {
            let labels = labels!["X", "Y"];
            let values = vec![GaussEvT::CertainPositive {
                event: 0,
                value: 1.0,
            }];
            let ev = GaussEv::new(labels, values)?;

            let evidences = ev.evidences();
            assert_eq!(evidences.len(), 2);

            Ok(())
        }

        #[test]
        fn clone_gaussian_evidence() -> Result<()> {
            let labels = labels!["A", "B"];
            let values = vec![GaussEvT::CertainPositive {
                event: 0,
                value: 42.0,
            }];
            let ev = GaussEv::new(labels, values)?;

            let cloned = ev.clone();

            assert_eq!(cloned.labels(), ev.labels());
            assert_eq!(cloned.evidences().len(), ev.evidences().len());

            Ok(())
        }
    }
}
