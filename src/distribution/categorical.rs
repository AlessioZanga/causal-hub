use approx::relative_eq;
use ndarray::prelude::*;

use crate::{
    data::{CategoricalData, Data},
    estimator::{Estimator, MaximumLikelihoodEstimator},
    types::{FxIndexMap, FxIndexSet},
};

use super::Distribution;

/// A struct representing a categorical distribution.
///
#[derive(Clone, Debug)]
pub struct CategoricalDistribution {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    parameters: Array2<f64>,
    parameters_size: usize,
    // Fitted statistics.
    sample_size: Option<usize>,
    log_likelihood: Option<f64>,
}

impl Distribution for CategoricalDistribution {
    type Labels = FxIndexSet<String>;
    type Parameters = Array2<f64>;
    type Data = CategoricalData;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        self.parameters_size
    }
}

impl CategoricalDistribution {
    /// Creates a new (conditional) categorical distribution.
    ///
    /// # Arguments
    ///
    /// * `variables` - The variables and their states.
    /// * `parameters` - The probabilities of the states.
    ///
    /// # Notes
    ///
    /// The first variable is the one conditioned on as P(X | Z).
    ///
    /// # Panics
    ///
    /// * If the variable labels are not unique.
    /// * If the variable states are not unique.
    /// * If the number of states of the first variable does not match the number of columns.
    /// * If the product of the number of states of the remaining variables does not match the number of rows.
    /// * If the probabilities do not sum to one by row, unless empty.
    ///
    ///
    /// # Returns
    ///
    /// A new `Categorical` instance.
    ///
    pub fn new(variables: &[(&str, Vec<&str>)], parameters: Array2<f64>) -> Self {
        // Get the states of the variables.
        let states: FxIndexMap<_, FxIndexSet<_>> = variables
            .iter()
            .map(|(i, j)| {
                (
                    // Convert the variable label to a string.
                    i.to_string(),
                    // Convert the variable states to a set of strings.
                    j.iter().map(|k| k.to_string()).collect(),
                )
            })
            .collect();
        // Get the labels of the variables.
        let labels: FxIndexSet<_> = states.keys().cloned().collect();
        // Get the cardinality of the set of states.
        let cardinality: Array1<_> = states.values().map(|i| i.len()).collect();
        // Check variables labels are unique.
        assert_eq!(
            states.len(),
            variables.len(),
            "Variable labels must be unique."
        );
        // Check variables states are unique.
        assert_eq!(
            cardinality,
            Array1::from_iter(variables.iter().map(|(_, i)| i.len())),
            "Variable states must be unique."
        );

        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            parameters.ncols(),
            states.get_index(0).map(|(_, i)| i.len()).unwrap_or(0),
            "Number of states of the first variable does not match the number of columns."
        );
        // Check if the product of the number of states of the remaining variables matches the number of rows.
        assert_eq!(
            parameters.nrows(),
            cardinality.iter().skip(1).product(),
            "Product of the number of states of the remaining variables does not match the number of rows."
        );
        // Assert the probabilities sum to one by row, unless empty.
        assert!(
            parameters.is_empty()
                || parameters
                    .sum_axis(Axis(1))
                    .iter()
                    .all(|&i| relative_eq!(i, 1.0)),
            "Probabilities must sum to one by row."
        );

        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();

        Self {
            labels,
            states,
            cardinality,
            parameters,
            parameters_size,
            // No estimated statistics, the distribution is not fitted.
            sample_size: None,
            log_likelihood: None,
        }
    }

    /// Returns the states of the variables in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the vector of states.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the cardinality of the set of states in the categorical distribution.
    ///
    /// # Returns
    ///
    /// A reference to the array of cardinality.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    /// Returns the sample size of the data used to fit the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample size of the data used to fit the distribution.
    ///
    #[inline]
    pub const fn sample_size(&self) -> Option<usize> {
        self.sample_size
    }

    /// Returns the log-likelihood of the data given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The log-likelihood of the data given the distribution.
    ///
    #[inline]
    pub const fn log_likelihood(&self) -> Option<f64> {
        self.log_likelihood
    }
}

/// A type alias for a maximum likelihood estimator.
pub type MLE<'a, P> = MaximumLikelihoodEstimator<'a, P>;

impl<'a> Estimator for MLE<'a, CategoricalDistribution> {
    type Distribution = CategoricalDistribution;

    /// Fits the distribution to the data.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit.
    /// * `z` - The variables to condition on.
    ///
    /// # Panics
    ///
    /// * If the variables to fit are not in the data.
    /// * If the any of the marginal counts are zero.
    ///
    /// # Returns
    ///
    /// A new `CategoricalDistribution` instance.
    ///
    fn fit(&self, x: usize, z: &[usize]) -> Self::Distribution {
        // Get the reference to the labels, states and cardinality.
        let (labels, states, cards) = (
            self.data().labels(),
            self.data().states(),
            self.data().cardinality(),
        );

        // Order the variables to fit.
        let x_z: Vec<_> = [x].iter().chain(z).cloned().collect();

        // Assert the variables to fit are in the data.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the data."
        );

        // Get the cardinality of Z.
        let c_z: Array1<_> = z.iter().map(|&i| cards[i]).collect();
        // Compute the strides of the parameters.
        let mut s = vec![1; c_z.len()];
        // Compute cumulative product (column-major strides).
        for i in 1..c_z.len() {
            s[i] = s[i - 1] * c_z[i - 1];
        }

        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((c_z.product(), cards[x]));

        // Count the occurrences of the states.
        self.data().values().rows().into_iter().for_each(|row| {
            // Get the value of X as index.
            let idx_x = row[x] as usize;
            // Get the value of Z as index using the strides.
            let idx_z = z.iter().zip(&s).map(|(&i, &j)| (row[i] as usize) * j).sum();
            // Increment the joint counts.
            n_xz[[idx_z, idx_x]] += 1;
        });

        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1));
        // Assert the marginal counts are not zero.
        assert!(
            n_z.iter().all(|&i| i > 0),
            "Marginal counts must be non-zero."
        );
        // Compute the sample size.
        let n = n_z.sum();

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n_z = n_z.mapv(|x| x as f64);

        // Compute the parameters by normalizing the counts.
        let parameters = &n_xz / n_z.insert_axis(Axis(1));
        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();
        // Set the sample size.
        let sample_size = Some(n);
        // Compute the log-likelihood, avoiding ln(0).
        let log_likelihood = Some((n_xz * (&parameters + f64::MIN_POSITIVE).mapv(f64::ln)).sum());

        // Subset the labels, states and cardinality.
        let labels = x_z.iter().map(|&i| labels[i].clone()).collect();
        let states = x_z
            .iter()
            .map(|&i| states.get_index(i).unwrap())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let cardinality = x_z.iter().map(|&i| cards[i]).collect();

        CategoricalDistribution {
            labels,
            states,
            cardinality,
            parameters,
            parameters_size,
            sample_size,
            log_likelihood,
        }
    }
}
