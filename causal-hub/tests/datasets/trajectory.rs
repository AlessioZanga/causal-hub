#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTrj, CatTrjEv, CatTrjEvT as E, Dataset},
        labels,
        models::Labelled,
        support,
        types::Result,
    };
    use ndarray::prelude::*;

    mod categorical {

        use causal_hub::datasets::CatTrjs;

        use super::*;

        #[test]
        fn new_trajectory() -> Result<()> {
            // Set the support.
            let support = support![
                ("A", ["0", "1", "2"]), //
                ("B", ["0", "1"]),      //
                ("C", ["0", "1"])       //
            ];
            // Set the events.
            let events = array![
                [0, 0, 0],
                [1, 0, 0],
                [1, 1, 0],
                [1, 0, 0],
                [2, 0, 0],
                [2, 0, 1]
            ];
            // Set the times.
            let times = array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
            // Construct a new trajectory.
            let trj = CatTrj::new(support, events, times)?;

            // Check the labels.
            assert_eq!(trj.labels(), &labels!["A", "B", "C"]);
            // Check the support.
            assert_eq!(
                trj.support(),
                &support![
                    ("A", ["0", "1", "2"]), //
                    ("B", ["0", "1"]),      //
                    ("C", ["0", "1"])       //
                ]
            );
            // Check the events.
            assert_eq!(
                trj.values(),
                &array![
                    [0, 0, 0],
                    [1, 0, 0],
                    [1, 1, 0],
                    [1, 0, 0],
                    [2, 0, 0],
                    [2, 0, 1]
                ]
            );
            // Check the times.
            assert_eq!(
                trj.times(),
                &array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5] //
            );

            Ok(())
        }

        #[test]
        fn new_trajectory_unordered_states() -> Result<()> {
            // Set the support.
            let support = support![
                ("B", ["0", "1"]),      //
                ("C", ["1", "0"]),      //
                ("A", ["0", "1", "2"]), //
            ];
            // Set the events.
            let events = array![
                [0, 1, 0],
                [0, 1, 1],
                [1, 1, 1],
                [0, 1, 1],
                [0, 1, 2],
                [0, 0, 2]
            ];
            // Set the times.
            let times = array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
            // Construct a new trajectory.
            let trj = CatTrj::new(support, events, times)?;

            // Check the labels.
            assert_eq!(trj.labels(), &labels!["A", "B", "C"]);
            // Check the support.
            assert_eq!(
                trj.support(),
                &support![
                    ("A", ["0", "1", "2"]), //
                    ("B", ["0", "1"]),      //
                    ("C", ["0", "1"])       //
                ]
            );
            // Check the events.
            assert_eq!(
                trj.values(),
                &array![
                    [0, 0, 0],
                    [1, 0, 0],
                    [1, 1, 0],
                    [1, 0, 0],
                    [2, 0, 0],
                    [2, 0, 1]
                ]
            );
            // Check the times.
            assert_eq!(
                trj.times(),
                &array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5] //
            );

            Ok(())
        }

        #[test]
        fn new_trajectory_unordered_times() -> Result<()> {
            // Set the support.
            let support = support![
                ("B", ["0", "1"]),      //
                ("C", ["1", "0"]),      //
                ("A", ["0", "1", "2"]), //
            ];
            // Set the events.
            let events = array![
                [0, 1, 1],
                [1, 1, 1],
                [0, 1, 1],
                [0, 1, 2],
                [0, 0, 2],
                [0, 1, 0]
            ];
            // Set the times.
            let times = array![0.1, 0.2, 0.3, 0.4, 0.5, 0.0];
            // Construct a new trajectory.
            let trj = CatTrj::new(support, events, times)?;

            // Check the labels.
            assert_eq!(trj.labels(), &labels!["A", "B", "C"]);
            // Check the support.
            assert_eq!(
                trj.support(),
                &support![
                    ("A", ["0", "1", "2"]), //
                    ("B", ["0", "1"]),      //
                    ("C", ["0", "1"])       //
                ]
            );
            // Check the events.
            assert_eq!(
                &array![
                    [0, 0, 0],
                    [1, 0, 0],
                    [1, 1, 0],
                    [1, 0, 0],
                    [2, 0, 0],
                    [2, 0, 1]
                ],
                trj.values()
            );
            // Check the times.
            assert_eq!(
                &array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5], //
                trj.times()
            );

            Ok(())
        }

        #[test]
        fn new_trajectories() -> Result<()> {
            // Initialize the first trajectory.
            let trj_0 = CatTrj::new(
                support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                array![0., 1., 2., 3., 4.],
            )?;
            // Initialize the second trajectory.
            let trj_1 = CatTrj::new(
                support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                array![0., 1., 2., 3., 4.],
            )?;
            // Construct a new set of trajectories.
            let trjs = CatTrjs::new([trj_0, trj_1])?;

            // Check the labels.
            assert_eq!(&labels!["A", "B"], trjs.labels());
            // Check the support.
            assert_eq!(
                &support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                trjs.support()
            );
            // Check the events of the first trajectory.
            assert_eq!(
                &array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                trjs.values()[0].values()
            );
            // Check the times of the first trajectory.
            assert_eq!(
                &array![0., 1., 2., 3., 4.], //
                trjs.values()[0].times()
            );
            // Check the events of the second trajectory.
            assert_eq!(
                &array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                trjs.values()[1].values()
            );
            // Check the times of the second trajectory.
            assert_eq!(
                &array![0., 1., 2., 3., 4.], //
                trjs.values()[1].times()
            );

            Ok(())
        }

        #[test]
        fn new_trajectories_unordered_states() -> Result<()> {
            // Initialize the first trajectory.
            let trj_0 = CatTrj::new(
                support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["1", "2", "3", "0"]), //
                ],
                array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                array![0., 1., 2., 3., 4.],
            )?;
            // Initialize the second trajectory.
            let trj_1 = CatTrj::new(
                support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                array![0., 1., 2., 3., 4.],
            )?;

            // Construct a new set of trajectories.
            let trjs = CatTrjs::new([trj_0, trj_1])?;

            // Check the labels.
            assert_eq!(&labels!["A", "B"], trjs.labels());
            // Check the support.
            assert_eq!(
                &support![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                trjs.support()
            );
            // Check the events of the first trajectory.
            assert_eq!(
                &array![
                    [0, 1], //
                    [0, 2], //
                    [1, 2], //
                    [2, 2], //
                    [2, 3]  //
                ],
                trjs.values()[0].values()
            );
            // Check the times of the first trajectory.
            assert_eq!(
                &array![0., 1., 2., 3., 4.], //
                trjs.values()[0].times()
            );
            // Check the events of the second trajectory.
            assert_eq!(
                &array![
                    [0, 0], //
                    [0, 1], //
                    [1, 1], //
                    [2, 1], //
                    [2, 2]  //
                ],
                trjs.values()[1].values()
            );
            // Check the times of the second trajectory.
            assert_eq!(
                &array![0., 1., 2., 3., 4.], //
                trjs.values()[1].times()
            );

            Ok(())
        }

        #[test]
        fn new_evidence() -> Result<()> {
            // Initialize the model.
            let support = support![
                ("B", ["0", "1"]),      //
                ("A", ["0", "1", "2"]), //
                ("C", ["0", "1"])       //
            ];

            // Initialize evidence.
            let _evidence = CatTrjEv::new(
                support,
                [
                    E::CertainPositiveInterval {
                        event: 2,
                        state: 0,
                        start_time: 0.,
                        end_time: 0.2,
                    },
                    E::CertainNegativeInterval {
                        event: 0,
                        not_states: [0].into_iter().collect(),
                        start_time: 0.,
                        end_time: 0.2,
                    },
                    E::UncertainPositiveInterval {
                        event: 1,
                        p_states: array![0.3, 0.7, 0.],
                        start_time: 0.1,
                        end_time: 0.2,
                    },
                    E::UncertainNegativeInterval {
                        event: 2,
                        p_not_states: array![0.9, 0.1],
                        start_time: 0.3,
                        end_time: 0.5,
                    },
                ],
            )?;

            Ok(())
        }
    }
}
