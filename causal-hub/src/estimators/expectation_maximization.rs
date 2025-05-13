pub struct ExpectationMaximization<'a, M, E> {
    /// The model to be fitted.
    initial_model: &'a M,
    /// The evidence to be used for fitting.
    evidence: &'a E,
}

pub type EM<'a, M, E> = ExpectationMaximization<'a, M, E>;

// Get the evidence.
// Get the initial model.

// Set convergence criteria.

// While not converged:

// E-Step: Compute the expected sufficient statistics.
// M-Step: Update the model parameters.

// Return the model.
