use crate::{
    graphs::{directions, IntoUndirectedGraph},
    prelude::{BaseGraph, UndirectedGraph},
};

/// Moral graph trait.
pub trait MoralGraph: IntoUndirectedGraph {
    /// Associated moral graph type.
    type MoralGraph: BaseGraph<Direction = directions::Undirected> + UndirectedGraph;

    /// Build a moral graph.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME: Add doc examples.
    /// ```
    ///
    fn moral(&self) -> Self::MoralGraph;
}
