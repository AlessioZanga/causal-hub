use crate::distribution::Distribution;
use crate::estimator::Estimator;

/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BayesianEstimator<'a, P>
where
    P: Distribution,
{
    // Required fields.
    data: &'a P::Data,
    alpha: f64,
}

impl<'a, P> BayesianEstimator<'a, P>
where
    P: Distribution,
{
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    /// * `alpha` - The prior parameter.
    ///
    /// # Returns
    ///
    /// A new `BayesianEstimator` instance.
    ///
    #[inline]
    pub const fn new(data: &'a P::Data, alpha: f64) -> Self {
        Self { data, alpha }
    }

    /// Returns a reference to the data.
    ///
    /// # Returns
    ///
    /// A reference to the data.
    ///
    #[inline]
    pub const fn data(&self) -> &'a P::Data {
        self.data
    }

    /// Returns the prior parameter.
    ///
    /// # Returns
    ///
    /// The prior parameter.
    ///
    #[inline]
    pub const fn alpha(&self) -> f64 {
        self.alpha
    }
}

impl<'a> Estimator for BayesianEstimator<'a, CategoricalDistribution> {
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
        // Concat the variables to fit.
        let x_z: Vec<_> = [x].iter().chain(z).cloned().collect();

        // Assert X_Z does not contain duplicates.
        assert!(
            x_z.iter().unique().count() == x_z.len(),
            "Variables to fit must be unique."
        );

        // Get the reference to the labels, states and cardinality.
        let (labels, states, cards) = (
            self.data().labels(),
            self.data().states(),
            self.data().cardinality(),
        );

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

        // Compute the parameters by normalizing the counts with the prior.
        let parameters = (&n_xz + self.alpha()) / (n_z.insert_axis(Axis(1)) + self.alpha() * cards[x] as f64);
        // Compute the parameters size.
        let parameters_size = parameters.ncols().saturating_sub(1) * parameters.nrows();
        // Set the sample size.
        let sample_size = Some(n);
        // Compute the sample log-likelihood, avoiding ln(0).
        let sample_log_likelihood =
            Some((n_xz * (&parameters + f64::MIN_POSITIVE).mapv(f64::ln)).sum());

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
            sample_log_likelihood,
        }
    }
}
