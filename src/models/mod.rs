mod bayesian_network;
pub use bayesian_network::*;

/// Alias for discrete bayesian network.
pub type DiscreteBN = DiscreteBayesianNetwork;

mod factor;
pub use factor::*;

mod graphical_separation;
pub use graphical_separation::*;

/// Alias for graphical independence.
pub type GSeparation<'a, G, D> = GraphicalSeparation<'a, G, D>;

mod independence;
pub use independence::*;

mod moral;
pub use moral::*;

mod parameter_estimator;
pub use parameter_estimator::*;

/// Alias for maximum likelihood estimator.
pub type MLE = MaximumLikelihoodEstimator;

/// Alias for bayesian estimator.
pub type BE = BayesianEstimator;

mod variable_elimination;
pub use variable_elimination::*;
