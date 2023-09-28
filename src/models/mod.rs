mod bayesian_network;
pub use bayesian_network::*;

pub type CategoricalBN = CategoricalBayesianNetwork;

mod factor;
pub use factor::*;

mod distribution_estimation;
pub use distribution_estimation::*;

mod graphical_separation;
pub use graphical_separation::*;

pub type GSeparation<'a, G, D> = GraphicalSeparation<'a, G, D>;

mod conditional_independence;
pub use conditional_independence::*;

mod kullback_leibler;
pub use kullback_leibler::*;

pub type KL<'a, P, Q> = KullbackLeiblerDivergence<'a, P, Q>;

mod moral;
pub use moral::*;

mod parameter_estimation;
pub use parameter_estimation::*;

pub type MLE = MaximumLikelihoodEstimation<false>;

pub type ParallelMLE = MaximumLikelihoodEstimation<true>;

pub type BE = BayesianEstimation<false>;

pub type ParallelBE = BayesianEstimation<true>;

mod variable_elimination;
pub use variable_elimination::*;

pub type VE<'a, M> = VariableElimination<'a, M, false>;

pub type ParallelVE<'a, M> = VariableElimination<'a, M, true>;
