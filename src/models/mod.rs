mod independence;
pub use independence::*;

/// Alias for graphical independence.
pub type GIndependence<'a, G, D> = GraphicalIndependence<'a, G, D>;

mod moral;
pub use moral::*;
