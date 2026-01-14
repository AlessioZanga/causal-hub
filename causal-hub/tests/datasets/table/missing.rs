#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{datasets::MissingTable, labels, models::Labelled, set};
    use ndarray::prelude::*;

    #[test]
    fn new() {
        // Set the labels.
        let labels = labels!("A", "B", "C");
        // Set the missing mask.
        let missing_mask = array![
            [true, false, false],
            [false, false, false],
            [false, true, false],
            [false, false, true]
        ];
        // Create the missing table.
        let missing_table = MissingTable::new(labels.clone(), missing_mask.clone());

        // Check labels.
        assert_eq!(
            missing_table.labels(), //
            &labels
        );

        // Check fully observed columns.
        assert_eq!(
            missing_table.fully_observed(), //
            &set![]
        );

        // Check partially observed columns.
        assert_eq!(
            missing_table.partially_observed(), //
            &set![0, 1, 2]
        );

        // Check missing mask.
        assert_eq!(
            missing_table.missing_mask(), //
            missing_mask
        );
        // Check missing mask by columns.
        assert_eq!(
            missing_table.missing_mask_by_cols(),
            array![true, true, true]
        );
        // Check missing mask by rows.
        assert_eq!(
            missing_table.missing_mask_by_rows(),
            array![true, false, true, true]
        );

        // Check missing count.
        assert_eq!(
            missing_table.missing_count(), //
            3
        );
        // Check missing count by columns.
        assert_eq!(
            missing_table.missing_count_by_cols(), //
            array![1, 1, 1]
        );
        // Check missing count by rows.
        assert_eq!(
            missing_table.missing_count_by_rows(), //
            array![1, 0, 1, 1]
        );

        // Check missing rate.
        assert_relative_eq!(
            missing_table.missing_rate(), //
            0.25
        );
        // Check missing rate by columns.
        assert_relative_eq!(
            missing_table.missing_rate_by_cols(), //
            &array![0.25, 0.25, 0.25]
        );
        // Check missing rate by rows.
        assert_relative_eq!(
            missing_table.missing_rate_by_rows(), //
            &array![
                0.3333333333333333,
                0.0,
                0.3333333333333333,
                0.3333333333333333
            ]
        );

        // Check missing correlation.
        assert_relative_eq!(
            missing_table.missing_correlation(), //
            &array![
                [1.0, -0.3333333333333333, -0.3333333333333333], //
                [-0.3333333333333333, 1.0, -0.3333333333333333], //
                [-0.3333333333333333, -0.3333333333333333, 1.0]
            ]
        );

        // Check missing covariance.
        assert_relative_eq!(
            missing_table.missing_covariance(), //
            &array![
                [0.25, -0.0833333333333333, -0.0833333333333333], //
                [-0.0833333333333333, 0.25, -0.0833333333333333], //
                [-0.0833333333333333, -0.0833333333333333, 0.25]
            ]
        );

        // Check complete columns count.
        assert_eq!(
            missing_table.complete_cols_count(), //
            0
        );
        // Check complete rows count.
        assert_eq!(
            missing_table.complete_rows_count(), //
            1
        );
    }

    #[test]
    fn new_unordered_labels() {
        // Set the labels.
        let labels = labels!("C", "A", "B");
        // Set the missing mask.
        let missing_mask = array![
            [false, true, false],
            [false, false, false],
            [false, false, true],
            [true, false, false]
        ];
        // Create the missing table.
        let missing_table = MissingTable::new(labels.clone(), missing_mask.clone());

        // Check labels.
        assert_eq!(
            missing_table.labels(), //
            &labels!("A", "B", "C")
        );

        // Check fully observed columns.
        assert_eq!(
            missing_table.fully_observed(), //
            &set![]
        );

        // Check partially observed columns.
        assert_eq!(
            missing_table.partially_observed(), //
            &set![0, 1, 2]
        );

        // Check missing mask.
        assert_eq!(
            missing_table.missing_mask(), //
            &array![
                [true, false, false],
                [false, false, false],
                [false, true, false],
                [false, false, true]
            ]
        );
        // Check missing mask by columns.
        assert_eq!(
            missing_table.missing_mask_by_cols(),
            array![true, true, true]
        );
        // Check missing mask by rows.
        assert_eq!(
            missing_table.missing_mask_by_rows(),
            array![true, false, true, true]
        );

        // Check missing count.
        assert_eq!(
            missing_table.missing_count(), //
            3
        );
        // Check missing count by columns.
        assert_eq!(
            missing_table.missing_count_by_cols(), //
            array![1, 1, 1]
        );
        // Check missing count by rows.
        assert_eq!(
            missing_table.missing_count_by_rows(), //
            array![1, 0, 1, 1]
        );

        // Check missing rate.
        assert_relative_eq!(
            missing_table.missing_rate(), //
            0.25
        );
        // Check missing rate by columns.
        assert_relative_eq!(
            missing_table.missing_rate_by_cols(), //
            &array![0.25, 0.25, 0.25]
        );
        // Check missing rate by rows.
        assert_relative_eq!(
            missing_table.missing_rate_by_rows(), //
            &array![
                0.3333333333333333,
                0.0,
                0.3333333333333333,
                0.3333333333333333
            ]
        );

        // Check missing correlation.
        assert_relative_eq!(
            missing_table.missing_correlation(), //
            &array![
                [1.0, -0.3333333333333333, -0.3333333333333333], //
                [-0.3333333333333333, 1.0, -0.3333333333333333], //
                [-0.3333333333333333, -0.3333333333333333, 1.0]
            ]
        );

        // Check missing covariance.
        assert_relative_eq!(
            missing_table.missing_covariance(), //
            &array![
                [0.25, -0.0833333333333333, -0.0833333333333333], //
                [-0.0833333333333333, 0.25, -0.0833333333333333], //
                [-0.0833333333333333, -0.0833333333333333, 0.25]
            ]
        );

        // Check complete columns count.
        assert_eq!(
            missing_table.complete_cols_count(), //
            0
        );
        // Check complete rows count.
        assert_eq!(
            missing_table.complete_rows_count(), //
            1
        );
    }
}
