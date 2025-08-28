/// A struct representing the output of the expectation-maximization algorithm.
#[derive(Clone, Debug)]
pub struct EMOutput<M, E2M> {
    /// The models fitted during the expectation-maximization process.
    /// Each model is used to compute the expected sufficient statistics.
    pub models: Vec<M>,
    /// The expected sufficient statistics for each model.
    pub expectations: Vec<E2M>,
    /// The last model fitted by the maximization step.
    pub last_model: M,
    /// The number of iterations performed.
    pub iterations: usize,
}

/// A struct representing the expectation-maximization algorithm.
#[derive(Debug)]
pub struct EM<'a, M, E, EStep, E2M, MStep, Stop>
where
    M: Clone,
    E2M: Clone,
    EStep: Fn(&M, &E) -> E2M,
    MStep: Fn(&M, &E2M) -> M,
    Stop: Fn(&M, &M, usize) -> bool,
{
    /// The model to be fitted.
    initial_model: &'a M,
    /// The evidence to be used for fitting.
    evidence: &'a E,
    /// The expectation step.
    expectation: &'a EStep,
    /// The maximization step.
    maximization: &'a MStep,
    /// The stopping criteria.
    stop: &'a Stop,
}

impl<'a, M, E, EStep, E2M, MStep, Stop> EM<'a, M, E, EStep, E2M, MStep, Stop>
where
    M: Clone,
    E2M: Clone,
    EStep: Fn(&M, &E) -> E2M,
    MStep: Fn(&M, &E2M) -> M,
    Stop: Fn(&M, &M, usize) -> bool,
{
    /// Executes the expectation-maximization algorithm.
    ///
    /// # Returns
    ///
    /// The fitted model.
    ///
    pub fn fit(&self) -> EMOutput<M, E2M> {
        // Initialize the output.
        let mut output = EMOutput {
            models: Vec::new(),
            expectations: Vec::new(),
            last_model: self.initial_model.clone(),
            iterations: 0,
        };

        // Declare the previous model.
        let mut prev_model: M;
        // Set the current model to the initial model.
        let mut curr_model: M = self.initial_model.clone();

        // Do while ...
        while {
            // Set the previous model to the current model.
            prev_model = curr_model;
            // Store the current model in the output.
            output.models.push(prev_model.clone());
            // Expectation step.
            let expectation = (self.expectation)(&prev_model, self.evidence);
            // Store the expected sufficient statistics in the output.
            output.expectations.push(expectation.clone());
            // Maximization step.
            curr_model = (self.maximization)(&prev_model, &expectation);
            // Store the last model in the output.
            output.last_model = curr_model.clone();
            // Increment the counter.
            output.iterations += 1;
            // Check stopping criteria.
            !(self.stop)(&prev_model, &curr_model, output.iterations)
        } {}

        // Return the output.
        output
    }
}

/// A builder for the expectation-maximization algorithm.
pub struct EMBuilder<'a, M, E, EStep, E2M, MStep, Stop>
where
    M: Clone,
    E2M: Clone,
    EStep: Fn(&M, &E) -> E2M,
    MStep: Fn(&M, &E2M) -> M,
    Stop: Fn(&M, &M, usize) -> bool,
{
    initial_model: &'a M,
    evidence: &'a E,
    expectation: Option<&'a EStep>,
    maximization: Option<&'a MStep>,
    stop: Option<&'a Stop>,
}

impl<'a, M, E, EStep, E2M, MStep, Stop> EMBuilder<'a, M, E, EStep, E2M, MStep, Stop>
where
    M: Clone,
    E2M: Clone,
    EStep: Fn(&M, &E) -> E2M,
    MStep: Fn(&M, &E2M) -> M,
    Stop: Fn(&M, &M, usize) -> bool,
{
    /// Creates a new builder for the expectation-maximization algorithm.
    ///
    /// # Arguments
    ///
    /// * `initial_model` - The initial model to start the fitting process.
    /// * `evidence` - The evidence to be used for fitting.
    ///
    /// # Returns
    ///
    /// A new builder for the expectation-maximization algorithm.
    ///
    pub fn new(initial_model: &'a M, evidence: &'a E) -> Self {
        Self {
            initial_model,
            evidence,
            expectation: None,
            maximization: None,
            stop: None,
        }
    }

    /// Sets the expectation step.
    ///
    /// # Arguments
    ///
    /// * `expectation` - The expectation step function.
    ///
    /// # Notes
    ///
    /// The expectation step function takes a reference to the current model and the evidence,
    /// and returns the expected sufficient statistics.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    ///
    pub fn with_e_step(mut self, expectation: &'a EStep) -> Self {
        self.expectation = Some(expectation);
        self
    }

    /// Sets the maximization step.
    ///
    /// # Arguments
    ///
    /// * `maximization` - The maximization step function.
    ///
    /// # Notes
    ///
    /// The maximization step function takes a reference to the current model and the expected sufficient statistics,
    /// and returns the next model.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    ///
    pub fn with_m_step(mut self, maximization: &'a MStep) -> Self {
        self.maximization = Some(maximization);
        self
    }

    /// Sets the stopping criteria.
    ///
    /// # Arguments
    ///
    /// * `stop` - The stopping criteria function.
    ///
    /// # Notes
    ///
    /// The stopping criteria function takes a reference to the previous model, the current model,
    /// and the iteration counter, and returns a boolean indicating whether to stop the fitting process.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    ///
    pub fn with_stop(mut self, stop: &'a Stop) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Builds the expectation-maximization algorithm.
    ///
    /// # Panics
    ///
    /// Panics if any of the expectation, maximization, or stopping criteria steps are not set.
    ///
    /// # Returns
    ///
    /// The expectation-maximization algorithm.
    ///
    pub fn build(self) -> EM<'a, M, E, EStep, E2M, MStep, Stop> {
        EM {
            initial_model: self.initial_model,
            evidence: self.evidence,
            expectation: self.expectation.expect("Expectation step not set"),
            maximization: self.maximization.expect("Maximization step not set"),
            stop: self.stop.expect("Stopping criteria not set"),
        }
    }
}
