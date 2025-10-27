#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{CatIncTable, Dataset, IncDataset},
        labels,
        models::Labelled,
        states,
    };
    use ndarray::prelude::*;

    const M: <CatIncTable as IncDataset>::Missing = CatIncTable::MISSING;

    #[test]
    fn new() {
        // Set the states.
        let states = states!(
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2", "c3", "c4"])
        );
        // Set the values, using M as missing value.
        let values = array![
            [0, 1, 2], //
            [1, 0, 2],
            [2, 1, 0],
            [M, 0, 1],
            [0, M, 3],
            [1, 1, M],
            [M, M, M],
            [M, 1, 3]
        ];
        // Create the categorical incomplete table.
        let dataset = CatIncTable::new(states.clone(), values.clone());

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], dataset.labels());
        // Assert the states.
        assert_eq!(&states, dataset.states());
        // Assert the shape.
        assert_eq!(&array![3, 2, 4], dataset.shape());
        // Assert the values.
        assert_eq!(&values, dataset.values());
        // Assert the sample size.
        assert_eq!(
            8., //
            dataset.sample_size()
        );

        // Assert the missing mask.
        assert_eq!(
            &array![
                [false, false, false], //
                [false, false, false],
                [false, false, false],
                [true, false, false],
                [false, true, false],
                [false, false, true],
                [true, true, true],
                [true, false, false]
            ],
            dataset.missing().missing_mask()
        );
        // Assert the missing mask by columns.
        assert_eq!(
            &array![true, true, true],
            dataset.missing().missing_mask_by_cols()
        );
        // Assert the missing mask by rows.
        assert_eq!(
            &array![false, false, false, true, true, true, true, true],
            dataset.missing().missing_mask_by_rows()
        );
        // Assert the missing count.
        assert_eq!(7, dataset.missing().missing_count());
        // Assert the missing count by columns.
        assert_eq!(
            &array![3, 2, 2], //
            dataset.missing().missing_count_by_cols()
        );
        // Assert the missing count by rows.
        assert_eq!(
            &array![0, 0, 0, 1, 1, 1, 3, 1],
            dataset.missing().missing_count_by_rows()
        );
        // Assert the missing rate.
        assert_relative_eq!(
            7. / 24., //
            dataset.missing().missing_rate()
        );
        // Assert the missing rate by columns.
        assert_relative_eq!(
            &array![3. / 8., 2. / 8., 2. / 8.], //
            dataset.missing().missing_rate_by_cols()
        );
        // Assert the missing rate by rows.
        assert_relative_eq!(
            &array![0., 0., 0., 1. / 3., 1. / 3., 1. / 3., 1., 1. / 3.],
            dataset.missing().missing_rate_by_rows()
        );
        // Assert the missing correlation.
        assert_relative_eq!(
            &array![
                [1.0, 0.1490711984999860, 0.1490711984999860],
                [0.1490711984999860, 1.0, 0.3333333333333333],
                [0.1490711984999860, 0.3333333333333333, 1.0]
            ],
            dataset.missing().missing_correlation()
        );
        // Assert the missing covariance.
        assert_relative_eq!(
            &array![
                [0.2678571428571429, 0.0357142857142857, 0.0357142857142857],
                [0.0357142857142857, 0.2142857142857143, 0.0714285714285714],
                [0.0357142857142857, 0.0714285714285714, 0.2142857142857143]
            ],
            dataset.missing().missing_covariance()
        );
        // Assert the complete columns count.
        assert_eq!(
            0, //
            dataset.missing().complete_cols_count()
        );
        // Assert the complete rows count.
        assert_eq!(
            3, //
            dataset.missing().complete_rows_count()
        );
    }

    #[test]
    fn new_unordered_labels() {
        // Set the states.
        let states = states!(
            ("C", ["c1", "c2", "c3", "c4"]),
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"])
        );
        // Set the values, using M as missing value.
        let values = array![
            [2, 0, 1], //
            [2, 1, 0],
            [0, 2, 1],
            [1, M, 0],
            [3, 0, M],
            [M, 1, 1],
            [M, M, M],
            [3, M, 1]
        ];
        // Create the categorical incomplete table.
        let dataset = CatIncTable::new(states.clone(), values.clone());

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], dataset.labels());
        // Assert the states.
        assert_eq!(
            &states![
                ("A", ["a1", "a2", "a3"]),
                ("B", ["b1", "b2"]),
                ("C", ["c1", "c2", "c3", "c4"])
            ],
            dataset.states()
        );
        // Assert the shape.
        assert_eq!(&array![3, 2, 4], dataset.shape());
        // Assert the values.
        assert_eq!(
            &array![
                [0, 1, 2], //
                [1, 0, 2],
                [2, 1, 0],
                [M, 0, 1],
                [0, M, 3],
                [1, 1, M],
                [M, M, M],
                [M, 1, 3]
            ],
            dataset.values()
        );
        // Assert the sample size.
        assert_eq!(
            8., //
            dataset.sample_size()
        );

        // Assert the missing mask.
        assert_eq!(
            &array![
                [false, false, false], //
                [false, false, false],
                [false, false, false],
                [true, false, false],
                [false, true, false],
                [false, false, true],
                [true, true, true],
                [true, false, false]
            ],
            dataset.missing().missing_mask()
        );
        // Assert the missing mask by columns.
        assert_eq!(
            &array![true, true, true],
            dataset.missing().missing_mask_by_cols()
        );
        // Assert the missing mask by rows.
        assert_eq!(
            &array![false, false, false, true, true, true, true, true],
            dataset.missing().missing_mask_by_rows()
        );
        // Assert the missing count.
        assert_eq!(7, dataset.missing().missing_count());
        // Assert the missing count by columns.
        assert_eq!(
            &array![3, 2, 2], //
            dataset.missing().missing_count_by_cols()
        );
        // Assert the missing count by rows.
        assert_eq!(
            &array![0, 0, 0, 1, 1, 1, 3, 1],
            dataset.missing().missing_count_by_rows()
        );
        // Assert the missing rate.
        assert_relative_eq!(
            7. / 24., //
            dataset.missing().missing_rate()
        );
        // Assert the missing rate by columns.
        assert_relative_eq!(
            &array![3. / 8., 2. / 8., 2. / 8.], //
            dataset.missing().missing_rate_by_cols()
        );
        // Assert the missing rate by rows.
        assert_relative_eq!(
            &array![0., 0., 0., 1. / 3., 1. / 3., 1. / 3., 1., 1. / 3.],
            dataset.missing().missing_rate_by_rows()
        );
        // Assert the missing correlation.
        assert_relative_eq!(
            &array![
                [1.0, 0.1490711984999860, 0.1490711984999860],
                [0.1490711984999860, 1.0, 0.3333333333333333],
                [0.1490711984999860, 0.3333333333333333, 1.0]
            ],
            dataset.missing().missing_correlation()
        );
        // Assert the missing covariance.
        assert_relative_eq!(
            &array![
                [0.2678571428571429, 0.0357142857142857, 0.0357142857142857],
                [0.0357142857142857, 0.2142857142857143, 0.0714285714285714],
                [0.0357142857142857, 0.0714285714285714, 0.2142857142857143]
            ],
            dataset.missing().missing_covariance()
        );
        // Assert the complete columns count.
        assert_eq!(
            0, //
            dataset.missing().complete_cols_count()
        );
        // Assert the complete rows count.
        assert_eq!(
            3, //
            dataset.missing().complete_rows_count()
        );
    }

    #[test]
    fn new_unordered_states() {
        // Set the states.
        let states = states!(
            ("C", ["c1", "c2", "c3", "c4"]),
            ("A", ["a1", "a3", "a2"]),
            ("B", ["b1", "b2"])
        );
        // Set the values, using M as missing value.
        let values = array![
            [2, 0, 1], //
            [2, 2, 0],
            [0, 1, 1],
            [1, M, 0],
            [3, 0, M],
            [M, 2, 1],
            [M, M, M],
            [3, M, 1]
        ];
        // Create the categorical incomplete table.
        let dataset = CatIncTable::new(states.clone(), values.clone());

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], dataset.labels());
        // Assert the states.
        assert_eq!(
            &states![
                ("A", ["a1", "a2", "a3"]),
                ("B", ["b1", "b2"]),
                ("C", ["c1", "c2", "c3", "c4"])
            ],
            dataset.states()
        );
        // Assert the shape.
        assert_eq!(&array![3, 2, 4], dataset.shape());
        // Assert the values.
        assert_eq!(
            &array![
                [0, 1, 2], //
                [1, 0, 2],
                [2, 1, 0],
                [M, 0, 1],
                [0, M, 3],
                [1, 1, M],
                [M, M, M],
                [M, 1, 3]
            ],
            dataset.values()
        );
        // Assert the sample size.
        assert_eq!(
            8., //
            dataset.sample_size()
        );

        // Assert the missing mask.
        assert_eq!(
            &array![
                [false, false, false], //
                [false, false, false],
                [false, false, false],
                [true, false, false],
                [false, true, false],
                [false, false, true],
                [true, true, true],
                [true, false, false]
            ],
            dataset.missing().missing_mask()
        );
        // Assert the missing mask by columns.
        assert_eq!(
            &array![true, true, true],
            dataset.missing().missing_mask_by_cols()
        );
        // Assert the missing mask by rows.
        assert_eq!(
            &array![false, false, false, true, true, true, true, true],
            dataset.missing().missing_mask_by_rows()
        );
        // Assert the missing count.
        assert_eq!(7, dataset.missing().missing_count());
        // Assert the missing count by columns.
        assert_eq!(
            &array![3, 2, 2], //
            dataset.missing().missing_count_by_cols()
        );
        // Assert the missing count by rows.
        assert_eq!(
            &array![0, 0, 0, 1, 1, 1, 3, 1],
            dataset.missing().missing_count_by_rows()
        );
        // Assert the missing rate.
        assert_relative_eq!(
            7. / 24., //
            dataset.missing().missing_rate()
        );
        // Assert the missing rate by columns.
        assert_relative_eq!(
            &array![3. / 8., 2. / 8., 2. / 8.], //
            dataset.missing().missing_rate_by_cols()
        );
        // Assert the missing rate by rows.
        assert_relative_eq!(
            &array![0., 0., 0., 1. / 3., 1. / 3., 1. / 3., 1., 1. / 3.],
            dataset.missing().missing_rate_by_rows()
        );
        // Assert the missing correlation.
        assert_relative_eq!(
            &array![
                [1.0, 0.1490711984999860, 0.1490711984999860],
                [0.1490711984999860, 1.0, 0.3333333333333333],
                [0.1490711984999860, 0.3333333333333333, 1.0]
            ],
            dataset.missing().missing_correlation()
        );
        // Assert the missing covariance.
        assert_relative_eq!(
            &array![
                [0.2678571428571429, 0.0357142857142857, 0.0357142857142857],
                [0.0357142857142857, 0.2142857142857143, 0.0714285714285714],
                [0.0357142857142857, 0.0714285714285714, 0.2142857142857143]
            ],
            dataset.missing().missing_covariance()
        );
        // Assert the complete columns count.
        assert_eq!(
            0, //
            dataset.missing().complete_cols_count()
        );
        // Assert the complete rows count.
        assert_eq!(
            3, //
            dataset.missing().complete_rows_count()
        );
    }

    #[ignore]
    #[test]
    fn lw_deletion() {
        todo!() // FIXME:
    }

    #[ignore]
    #[test]
    fn pw_deletion() {
        todo!() // FIXME:
    }

    #[ignore]
    #[test]
    fn ipw_deletion() {
        todo!() // FIXME:
    }

    #[ignore]
    #[test]
    fn aipw_deletion() {
        todo!() // FIXME:
    }
}
