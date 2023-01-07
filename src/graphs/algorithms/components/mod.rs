mod connected_components;
pub use connected_components::ConnectedComponents;

/// Alias for connected components.
pub type CC<'a, G> = ConnectedComponents<'a, G>;
