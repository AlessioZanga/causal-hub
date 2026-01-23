#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::{
        datasets::CatTable,
        estimators::{BE, CPDEstimator, ParCPDEstimator},
        labels,
        models::{CPD, Labelled},
        set, states,
        types::{Error, Result},
    };
    use ndarray::prelude::*;

    mod cpd {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn fit() -> Result<()> {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 1, 0],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = BE::new(&dataset).with_prior(1);

                // P(A)
                let distribution = estimator.fit(&set![0], &set![])?;

                assert_eq!(distribution.labels(), &labels!["A"]);
                assert_eq!(distribution.states(), &states![("A", ["no", "yes"])]);
                assert_eq!(distribution.conditioning_labels(), &labels![]);
                assert!(
                    distribution
                        .conditioning_states()
                        .values()
                        .all(|x| x.iter().eq(["no", "yes"]))
                );

                assert_relative_eq!(
                    distribution.parameters(),
                    &array![
                        // A: no, yes
                        [0.5714285714285714, 0.42857142857142855]
                    ]
                );

                assert_eq!(distribution.parameters_size(), 1);
                assert_eq!(
                    distribution.sample_statistics().map(|s| s.sample_size()),
                    Some(5.)
                );
                assert_relative_eq!(
                    distribution
                        .sample_log_likelihood()
                        .ok_or(Error::IllegalArgument("no ll".into()))?,
                    -4.780356732903302
                );

                assert_eq!(
                    distribution.to_string(),
                    concat!(
                        "-----------------------\n",
                        "| A        |          |\n",
                        "| -------- | -------- |\n",
                        "| no       | yes      |\n",
                        "| -------- | -------- |\n",
                        "| 0.571429 | 0.428571 |\n",
                        "-----------------------\n",
                    )
                );

                // P(A | B, C)
                let distribution = estimator.fit(&set![0], &set![1, 2])?;

                assert_eq!(distribution.labels(), &labels!["A"]);
                assert_eq!(distribution.states(), &states![("A", ["no", "yes"])]);
                assert_eq!(distribution.conditioning_labels(), &labels!["B", "C"]);
                assert!(
                    distribution
                        .conditioning_states()
                        .values()
                        .all(|x| x.iter().eq(["no", "yes"]))
                );

                assert_relative_eq!(
                    distribution.parameters(),
                    &array![
                        // A: no, yes
                        [0.6666666666666666, 0.3333333333333333], // B: no, C: no
                        [0.6666666666666666, 0.3333333333333333], // B: no, C: yes
                        [0.3333333333333333, 0.6666666666666666], // B: yes, C: no
                        [0.5, 0.5]                                // B: yes, C: yes
                    ]
                );

                assert_eq!(distribution.parameters_size(), 4);
                assert_eq!(
                    distribution.sample_statistics().map(|s| s.sample_size()),
                    Some(5.)
                );
                assert_relative_eq!(
                    distribution
                        .sample_log_likelihood()
                        .ok_or(Error::IllegalArgument("no ll".into()))?,
                    -8.501216236893097
                );

                assert_eq!(
                    distribution.to_string(),
                    concat!(
                        "---------------------------------------------\n",
                        "|          |          | A        |          |\n",
                        "| -------- | -------- | -------- | -------- |\n",
                        "| B        | C        | no       | yes      |\n",
                        "| -------- | -------- | -------- | -------- |\n",
                        "| no       | no       | 0.666667 | 0.333333 |\n",
                        "| no       | yes      | 0.666667 | 0.333333 |\n",
                        "| yes      | no       | 0.333333 | 0.666667 |\n",
                        "| yes      | yes      | 0.500000 | 0.500000 |\n",
                        "---------------------------------------------\n",
                    )
                );

                Ok(())
            }

            #[test]
            fn par_fit() -> Result<()> {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 1, 0],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = BE::new(&dataset).with_prior(1);

                // P(A)
                let distribution = estimator.par_fit(&set![0], &set![])?;

                assert_relative_eq!(
                    distribution.parameters(),
                    &array![
                        // A: no, yes
                        [0.5714285714285714, 0.42857142857142855]
                    ]
                );

                Ok(())
            }

            #[test]
            #[should_panic(expected = "Variables and conditioning variables must be disjoint.")]
            fn unique_variables() {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 1, 0],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = BE::new(&dataset).with_prior(1);

                // P(A | A, C)
                let _ = estimator.fit(&set![0], &set![0, 2]).unwrap();
            }
        }

        mod gaussian {
            use causal_hub::datasets::{GaussIncTable, GaussTable, GaussWtdTable, MissingMethod};

            use super::*;

            mod complete {
                use super::*;

                #[test]
                fn fit() -> Result<()> {
                    let labels = labels!["X", "Y"];
                    let data = array![[1.0, 2.0], [2.0, 4.0], [3.0, 6.0]];
                    let dataset = GaussTable::new(labels.clone(), data);

                    let estimator = BE::new(&dataset).with_prior(1.0);

                    // P(X | Y)
                    let d = estimator.fit(&set![0], &set![1])?;

                    assert_eq!(d.labels(), &labels!["X"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Y"]);

                    // Verify sample_statistics reflects original data size
                    assert_eq!(d.sample_statistics().map(|s| s.sample_size()), Some(3.));

                    // a = 10 / 21 approx 0.47619
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.47619]],
                        epsilon = 1e-4
                    );
                    // b = 0.5 / 7 approx 0.0714
                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.071428],
                        epsilon = 1e-4
                    );
                    // s = 13 / 42 approx 0.3095
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.30952]],
                        epsilon = 1e-4
                    );

                    // P(Y | X)
                    // S_YX = S_XY^T = 10.
                    // S_XX_post = 6.
                    // a = 10 / 6 = 5/3 = 1.666
                    // b = mu_Y - a * mu_X = 3.0 - (5/3) * 1.5 = 3 - 2.5 = 0.5.
                    // s = (S_YY - a * S_YX) / N_post = (21 - (5/3)*10) / 4 = (21 - 50/3)/4 = (13/3)/4 = 13/12 = 1.0833
                    let d = estimator.fit(&set![1], &set![0])?;
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[1.66666]],
                        epsilon = 1e-4
                    );
                    assert_relative_eq!(d.parameters().intercept(), &array![0.5], epsilon = 1e-4);
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[1.08333]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }

                #[test]
                fn fit_informative_prior() -> Result<()> {
                    let labels = labels!["X", "Y"];
                    let data = array![[1.0, 2.0], [2.0, 4.0], [3.0, 6.0]];
                    let dataset = GaussTable::new(labels.clone(), data);

                    // Prior nu = 2.0
                    let estimator = BE::new(&dataset).with_prior(2.0);

                    // Fit P(X | Y)
                    let d = estimator.fit(&set![0], &set![1])?;

                    assert_eq!(d.labels(), &labels!["X"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Y"]);

                    // Verify sample_statistics reflects original data size still
                    assert_eq!(d.sample_statistics().map(|s| s.sample_size()), Some(3.));

                    // Expected values with prior=2.0 (nu=2):
                    // n = 3, nu = 2, n_post = 5.
                    // mu_X = 2, mu_Y = 4.
                    // S_XX = 2, S_YY = 8, S_XY = 4.
                    //
                    // Correction f = n*nu/n_post = 6/5 = 1.2
                    // mu_post = mu * (n/n_post) = mu * 0.6 => mu_X_post = 1.2, mu_Y_post = 2.4
                    //
                    // S_XX_post = S_XX + nu + f*mu_X^2 = 2 + 2 + 1.2*4 = 8.8
                    // S_YY_post = S_YY + nu + f*mu_Y^2 = 8 + 2 + 1.2*16 = 29.2
                    // S_XY_post = S_XY + 0 + f*mu_X*mu_Y = 4 + 1.2*8 = 13.6
                    //
                    // A = S_XY_post / S_YY_post = 13.6 / 29.2 approx 0.46575
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.46575]],
                        epsilon = 1e-4
                    );

                    // b = mu_X_post - A * mu_Y_post = 1.2 - 0.46575 * 2.4 approx 0.08219
                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.08219],
                        epsilon = 1e-4
                    );

                    // S = (S_XX_post - A * S_XY_post) / n_post
                    // S = (8.8 - 0.46575 * 13.6) / 5 = (8.8 - 6.3342) / 5 = 2.4658 / 5 approx 0.49316
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.49315]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }

                #[test]
                fn par_fit() -> Result<()> {
                    let labels = labels!["X", "Y"];
                    let data = array![[1.0, 2.0], [2.0, 4.0], [3.0, 6.0]];
                    let dataset = GaussTable::new(labels.clone(), data);

                    let estimator = BE::new(&dataset).with_prior(1.0);

                    // Fit P(X | Y) using parallel fit
                    let d = estimator.par_fit(&set![0], &set![1])?;

                    assert_eq!(d.labels(), &labels!["X"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Y"]);

                    // Results should be identical to sequential fit
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.47619]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }

                #[test]
                fn fit_multivariate() -> Result<()> {
                    let labels = labels!["X1", "X2", "Z1", "Z2"];
                    let data = array![
                        [1.0, 0.0, 1.0, 0.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ];
                    let dataset = GaussTable::new(labels.clone(), data);
                    let estimator = BE::new(&dataset).with_prior(1.0);

                    // P(X1, X2 | Z1, Z2)
                    let d = estimator.fit(&set![0, 1], &set![2, 3])?;

                    assert_eq!(d.labels(), &labels!["X1", "X2"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Z1", "Z2"]);

                    // Check correctness
                    // Coeffs A = 1/7 * Ones
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.142857, 0.142857], [0.142857, 0.142857]],
                        epsilon = 1e-4
                    );

                    // Intercept B = 2/7 * Ones
                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.285714, 0.285714],
                        epsilon = 1e-4
                    );

                    // Covariance S = [[3/7, 1/35], [1/35, 3/7]]
                    // 3/7 = 0.428571
                    // 1/35 = 0.028571
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.428571, 0.028571], [0.028571, 0.428571]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }
            }

            mod incomplete {
                use super::*;

                #[test]
                fn fit() -> Result<()> {
                    let labels = labels!["X", "Y"];
                    let data = array![[1.0, 2.0], [2.0, f64::NAN], [3.0, 6.0]];
                    let dataset = GaussIncTable::new(labels.clone(), data);

                    let estimator = BE::new(&dataset)
                        .with_prior(1.0)
                        .with_missing_method(Some(MissingMethod::LW), None);

                    let d = estimator.fit(&set![0], &set![1])?;
                    assert_eq!(d.labels(), &labels!["X"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Y"]);

                    // SSE with LW should have dropped one row
                    assert_eq!(d.sample_statistics().map(|s| s.sample_size()), Some(2.));

                    // Matching the manual calculation for N=2 (LW case)
                    // a = 28/59 approx 0.47457
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.47457]],
                        epsilon = 1e-4
                    );
                    // b = 4/59 approx 0.06779
                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.06779],
                        epsilon = 1e-4
                    );
                    // s = 73/177 approx 0.41243
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.41243]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }

                #[test]
                fn fit_multivariate() -> Result<()> {
                    let labels = labels!["X1", "X2", "Z1", "Z2"];
                    let data = array![
                        [1.0, 0.0, 1.0, 0.0],
                        [1.0, 1.0, 0.0, 1.0],
                        [0.0, 1.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                        [f64::NAN, 0.0, 0.0, 1.0] // Missing value, should be dropped
                    ];
                    let dataset = GaussIncTable::new(labels.clone(), data);
                    let estimator = BE::new(&dataset)
                        .with_prior(1.0)
                        .with_missing_method(Some(MissingMethod::LW), None);

                    // P(X1, X2 | Z1, Z2)
                    let d = estimator.fit(&set![0, 1], &set![2, 3])?;

                    assert_eq!(d.labels(), &labels!["X1", "X2"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Z1", "Z2"]);

                    // Should match complete case exactly (since incomplete row is dropped)
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.142857, 0.142857], [0.142857, 0.142857]],
                        epsilon = 1e-4
                    );

                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.285714, 0.285714],
                        epsilon = 1e-4
                    );

                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.428571, 0.028571], [0.028571, 0.428571]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }
            }

            mod weighted {
                use super::*;

                #[test]
                fn fit() -> Result<()> {
                    let labels = labels!["X", "Y"];
                    let data = array![
                        [1.0, 2.0], // w=1
                        [2.0, 4.0], // w=0
                        [3.0, 6.0]  // w=1
                    ];
                    let weights = array![1.0, 0.0, 1.0];
                    let dataset = GaussTable::new(labels.clone(), data);
                    let dataset = GaussWtdTable::new(dataset, weights);

                    let estimator = BE::new(&dataset).with_prior(1.0);

                    let d = estimator.fit(&set![0], &set![1])?;
                    assert_eq!(d.labels(), &labels!["X"]);
                    assert_eq!(d.conditioning_labels(), &labels!["Y"]);

                    // Effective sample size should be sum of weights = 2.0
                    assert_eq!(d.sample_statistics().map(|s| s.sample_size()), Some(2.));

                    // Should match incomplete case exactly
                    assert_relative_eq!(
                        d.parameters().coefficients(),
                        &array![[0.47457]],
                        epsilon = 1e-4
                    );
                    assert_relative_eq!(
                        d.parameters().intercept(),
                        &array![0.06779],
                        epsilon = 1e-4
                    );
                    assert_relative_eq!(
                        d.parameters().covariance(),
                        &array![[0.41243]],
                        epsilon = 1e-4
                    );

                    Ok(())
                }
            }
        }
    }
}
