use std::fmt::Display;

use approx::relative_eq;
use itertools::Itertools;
use ndarray::prelude::*;

use super::Distribution;
use crate::{
    data::{CategoricalData, Data},
    estimator::{BE, CPDEstimator, MLE},
    types::{FxIndexMap, FxIndexSet},
};

/// A struct representing a categorical distribution.
///
#[derive(Clone, Debug)]
pub struct CategoricalConditionalProbabilityDistribution {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    parameters: Array2<f64>,
    parameters_size: usize,
    // Fitted statistics.
    sample_size: Option<usize>,
    sample_log_likelihood: Option<f64>,
}

/// A type alias for a categorical conditional probability distribution.
pub type CategoricalCPD = CategoricalConditionalProbabilityDistribution;

impl CategoricalCPD {
    /// Creates a new categorical conditional probability distribution.
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
    /// A new `CategoricalCPD` instance.
    ///
    pub fn new(variables: Vec<(&str, Vec<&str>)>, parameters: Array2<f64>) -> Self {
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
        // Assert the probabilities sum to one by row.
        parameters
            .sum_axis(Axis(1))
            .iter()
            .enumerate()
            .for_each(|(i, &x)| {
                if !relative_eq!(x, 1.0, epsilon = 1e-8) {
                    panic!("Failed to sum probability to one: {}.", parameters.row(i));
                }
            });

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
            sample_log_likelihood: None,
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

    /// Returns the sample log-likelihood of the data given the distribution, if any.
    ///
    /// # Returns
    ///
    /// The sample log-likelihood of the data given the distribution.
    ///
    #[inline]
    pub const fn sample_log_likelihood(&self) -> Option<f64> {
        self.sample_log_likelihood
    }
}

impl Display for CategoricalCPD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Determine the maximum width for formatting based on the labels and states.
        let n = self
            .labels()
            .iter()
            .chain(self.states().values().flatten())
            .map(|x| x.len())
            .max()
            .unwrap_or(0)
            .max(8);
        // Get the number of variables to condition on.
        let z = self.labels().len().saturating_sub(1);
        // Get the number of states for the first variable.
        let s = self
            .states()
            .get_index(0)
            .map(|(_, i)| i.len())
            .unwrap_or(0);

        // Create a horizontal line for table formatting.
        let hline = "-".repeat((n + 3) * (z + s) + 1);
        writeln!(f, "{hline}")?;

        // Create the header row for the table.
        let header = std::iter::repeat_n("", z) // Empty columns for the conditioning variables.
            .chain([self.labels().get_index(0).map(|x| x.as_str()).unwrap_or("")]) // Label for the first variable.
            .chain(std::iter::repeat_n("", s.saturating_sub(1))) // Empty columns for remaining states.
            .map(|x| format!("{x:width$}", width = n)) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;

        // Create a separator row for the table.
        let separator = std::iter::repeat_n("-".repeat(n), z + s).join(" | ");
        writeln!(f, "| {separator} |")?;

        // Create the second header row with labels and states.
        let header = self
            .labels()
            .iter()
            .skip(1) // Skip the first label.
            .chain(self.states().get_index(0).unwrap().1) // Include states of the first variable.
            .map(|x| format!("{x:width$}", width = n)) // Format each column with fixed width.
            .join(" | ");
        writeln!(f, "| {header} |")?;
        writeln!(f, "| {separator} |")?;

        // Iterate over the Cartesian product of states and parameter rows.
        for (states, values) in self
            .states()
            .values()
            .skip(1)
            .multi_cartesian_product()
            .zip(self.parameters().rows())
        {
            // Format the states for the current row.
            let states = states.iter().map(|x| format!("{x:width$}", width = n));
            // Format the parameter values for the current row.
            let values = values.iter().map(|x| format!("{:width$.6}", x, width = n));
            // Join the states and values for the current row.
            let states_values = states.chain(values).join(" | ");
            writeln!(f, "| {states_values} |")?;
        }

        // Write the closing horizontal line for the table.
        writeln!(f, "{hline}")?;

        Ok(())
    }
}

impl Distribution for CategoricalCPD {
    type Labels = FxIndexSet<String>;
    type Parameters = Array2<f64>;

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

impl CPDEstimator<CategoricalData, CategoricalCPD> for MLE {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> CategoricalCPD {
        // Concat the variables to fit.
        let x_z: Vec<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert!(
            x_z.iter().unique().count() == x_z.len(),
            "Variables to fit must be unique."
        );

        // Get the reference to the labels, states and cardinality.
        let (labels, states, cards) = (data.labels(), data.states(), data.cardinality());

        // Assert the variables to fit are in the data.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the data."
        );

        // Get the cardinality of Z.
        let c_z: Array1<_> = z.iter().map(|&i| cards[i]).collect();
        // Allocate the strides of the parameters.
        let mut s = vec![1; c_z.len()];
        // Compute cumulative product in reverse order (row-major strides).
        for i in (0..c_z.len().saturating_sub(1)).rev() {
            s[i] = s[i + 1] * c_z[i + 1];
        }

        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((c_z.product(), cards[x]));

        // Count the occurrences of the states.
        data.values().rows().into_iter().for_each(|row| {
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
            "Failed to get non-zero counts for variable '{}'.",
            labels[x]
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
        // Set epsilon to avoid ln(0).
        let eps = f64::MIN_POSITIVE;
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood = Some((n_xz * (&parameters + eps).mapv(f64::ln)).sum());

        // Subset the labels, states and cardinality.
        let labels = x_z.iter().map(|&i| labels[i].clone()).collect();
        let states = x_z
            .iter()
            .map(|&i| states.get_index(i).unwrap())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let cardinality = x_z.iter().map(|&i| cards[i]).collect();

        CategoricalCPD {
            labels,
            states,
            cardinality,
            parameters,
            parameters_size,
            sample_size,
            sample_log_likelihood,
        }
    }
}

// NOTE: The prior is expressed as a scalar, which is the alpha for the Dirichlet distribution.
impl CPDEstimator<CategoricalData, CategoricalCPD> for BE<f64> {
    fn fit(&self, data: &CategoricalData, x: usize, z: &[usize]) -> CategoricalCPD {
        // Concat the variables to fit.
        let x_z: Vec<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert!(
            x_z.iter().unique().count() == x_z.len(),
            "Variables to fit must be unique."
        );

        // Get the reference to the labels, states and cardinality.
        let (labels, states, cards) = (data.labels(), data.states(), data.cardinality());

        // Assert the variables to fit are in the data.
        assert!(
            x_z.iter().all(|&i| i < labels.len()),
            "Variables to fit must be in the data."
        );

        // Get the cardinality of Z.
        let c_z: Array1<_> = z.iter().map(|&i| cards[i]).collect();
        // Allocate the strides of the parameters.
        let mut s = vec![1; c_z.len()];
        // Compute cumulative product in reverse order (row-major strides).
        for i in (0..c_z.len().saturating_sub(1)).rev() {
            s[i] = s[i + 1] * c_z[i + 1];
        }

        // Initialize the joint counts.
        let mut n_xz: Array2<usize> = Array::zeros((c_z.product(), cards[x]));

        // Count the occurrences of the states.
        data.values().rows().into_iter().for_each(|row| {
            // Get the value of X as index.
            let idx_x = row[x] as usize;
            // Get the value of Z as index using the strides.
            let idx_z = z.iter().zip(&s).map(|(&i, &j)| (row[i] as usize) * j).sum();
            // Increment the joint counts.
            n_xz[[idx_z, idx_x]] += 1;
        });

        // Marginalize the counts.
        let n_z = n_xz.sum_axis(Axis(1));
        // Compute the sample size.
        let n = n_z.sum();

        // Cast the counts to floating point.
        let n_xz = n_xz.mapv(|x| x as f64);
        let n_z = n_z.mapv(|x| x as f64);

        // Align the marginal counts axes.
        let n_z = n_z.insert_axis(Axis(1));

        // Get the prior, as the alpha of the Dirichlet distribution.
        let alpha = *self.prior();
        // Compute the parameters by normalizing the counts with the prior.
        let parameters = (&n_xz + alpha) / (n_z + alpha * cards[x] as f64);
        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();
        // Set the sample size.
        let sample_size = Some(n);
        // Compute the sample log-likelihood.
        let sample_log_likelihood = Some((n_xz * parameters.mapv(f64::ln)).sum());

        // Subset the labels, states and cardinality.
        let labels = x_z.iter().map(|&i| labels[i].clone()).collect();
        let states = x_z
            .iter()
            .map(|&i| states.get_index(i).unwrap())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let cardinality = x_z.iter().map(|&i| cards[i]).collect();

        CategoricalCPD {
            labels,
            states,
            cardinality,
            parameters,
            parameters_size,
            sample_size,
            sample_log_likelihood,
        }
    }
}
