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

mod inference;
pub use inference::*;

mod moral;
pub use moral::*;
