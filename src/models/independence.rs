use std::fmt::Debug;

/// Independence trait.
pub trait Independence: Clone + Debug + Sync {
    /// Checks whether $\mathbf{X} \mathrlap{\thinspace\perp}{\perp} \mathbf{Y} \mid \mathbf{Z}$ holds or not.
    fn is_independent(&self, x: usize, y: usize, z: &[usize]) -> bool;
}

/// Generalized Independence trait.
pub trait GeneralizedIndependence: Clone + Debug + Sync {
    /// Checks whether $\mathbf{X} \mathrlap{\thinspace\perp}{\perp} \mathbf{Y} \mid \mathbf{Z}$ holds or not.
    fn are_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>;
}
