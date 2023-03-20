mod bayesian_network;
pub use bayesian_network::*;

/// Alias for discrete bayesian network.
pub type DiscreteBN = DiscreteBayesianNetwork;

mod factor;
pub use factor::*;

mod distribution_estimation;
pub use distribution_estimation::*;

mod graphical_separation;
pub use graphical_separation::*;

/// Alias for graphical independence.
pub type GSeparation<'a, G, D> = GraphicalSeparation<'a, G, D>;

mod independence;
pub use independence::*;

mod kullback_leibler;
pub use kullback_leibler::*;

/// Alias for Kullback-Leibler divergence.
pub type KL<'a, P, Q> = KullbackLeiblerDivergence<'a, P, Q>;

mod moral;
pub use moral::*;

mod parameter_estimation;
pub use parameter_estimation::*;

/// Alias for maximum likelihood estimation.
pub type MLE = MaximumLikelihoodEstimation;

/// Alias for bayesian estimation.
pub type BE = BayesianEstimation;

mod variable_elimination;
pub use variable_elimination::*;

/// Alias for variable elimination.
pub type VE<'a, M> = VariableElimination<'a, M>;
