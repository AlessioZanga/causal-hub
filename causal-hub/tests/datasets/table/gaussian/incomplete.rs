#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{Dataset, GaussIncTable, IncDataset, MissingMethod as MM},
        labels,
        models::Labelled,
        set,
    };
    use ndarray::prelude::*;

    const M: <GaussIncTable as IncDataset>::Missing = GaussIncTable::MISSING;

    #[test]
    fn new() {
        // Set the labels.
        let labels = labels!("A", "B", "C");
        // Set the values, using M as missing value.
        let values = array![
            [0., 1., 2.], //
            [1., 0., 2.],
            [2., 1., 0.],
            [M, 0., 1.],
            [0., M, 3.],
            [1., 1., M],
            [M, M, M],
            [M, 1., 3.]
        ];
        // Create the gaussian incomplete table.
        let dataset = GaussIncTable::new(labels.clone(), values.clone());

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], dataset.labels());

        // Assert the values.
        let d_values = dataset.values();
        for (a, b) in values.iter().zip(d_values.iter()) {
            if a.is_nan() {
                assert!(b.is_nan());
            } else {
                assert_eq!(a, b);
            }
        }

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
                [1.00000000, 0.14907120, 0.14907120],
                [0.14907120, 1.00000000, 0.33333333],
                [0.14907120, 0.33333333, 1.00000000]
            ],
            dataset.missing().missing_correlation(),
            epsilon = 1e-6
        );

        // Assert the complete columns count.
        assert_eq!(0, dataset.missing().complete_cols_count());
        // Assert the complete rows count.
        assert_eq!(3, dataset.missing().complete_rows_count());
    }

    #[test]
    fn lw_deletion() {
        // Set the labels.
        let labels = labels!("A", "B", "C");
        // Set the values, using M as missing value.
        let values = array![
            [0., 1., 2.], //
            [1., 0., 2.],
            [2., 1., 0.],
            [M, 0., 1.],
            [0., M, 3.],
            [1., 1., M],
            [M, M, M],
            [M, 1., 3.]
        ];
        // Create the gaussian incomplete table.
        let dataset = GaussIncTable::new(labels.clone(), values.clone());

        // Apply list-wise deletion.
        let dataset = dataset.lw_deletion();

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], dataset.labels());
        // Assert the values.
        assert_eq!(
            &array![
                [0., 1., 2.], //
                [1., 0., 2.],
                [2., 1., 0.],
            ],
            dataset.values()
        );
    }

    #[test]
    fn pw_deletion() {
        // Set the labels.
        let labels = labels!("A", "B", "C");
        // Set the values, using M as missing value.
        let values = array![
            [0., 1., 2.], //
            [1., 0., 2.],
            [2., 1., 0.],
            [M, 0., 1.],
            [0., M, 3.],
            [1., 1., M],
            [M, M, M],
            [M, 1., 3.]
        ];
        // Create the gaussian incomplete table.
        let dataset = GaussIncTable::new(labels.clone(), values.clone());

        // Apply pw-wise deletion.
        let dataset = dataset.pw_deletion(&set![0, 1]);

        // Assert the labels.
        assert_eq!(&labels!["A", "B"], dataset.labels());
        // Assert the values.
        assert_eq!(
            &array![
                [0., 1.], //
                [1., 0.],
                [2., 1.],
                [1., 1.],
            ],
            dataset.values()
        );
    }

    #[test]
    #[should_panic]
    fn apply_missing_method_ipw() {
        // Set the labels.
        let labels = labels!("A", "B", "C");
        // Set the values, using M as missing value.
        let values = array![
            [0., 1., 2.], //
            [1., 0., 2.],
            [2., 1., 0.],
            [M, 0., 1.],
            [0., M, 3.],
            [1., 1., M],
            [M, M, M],
            [M, 1., 3.]
        ];
        // Create the gaussian incomplete table.
        let dataset = GaussIncTable::new(labels.clone(), values.clone());

        // Apply IPW deletion.
        dataset.apply_missing_method(&MM::IPW, None, None);
    }
}
