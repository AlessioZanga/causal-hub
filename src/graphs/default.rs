use super::{BaseGraph, ErrorGraph as E};

/// Default graph trait.
pub trait DefaultGraph: BaseGraph + Default {
    /// Null constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The null constructor of $\mathcal{G}$
    /// returns a null graph $G$ (i.e. both $V$ and $E$ are empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a null graph.
    /// let g = Graph::null();
    ///
    /// // The vertex set is empty.
    /// assert_eq!(g.order(), 0);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Empty constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The empty constructor of $\mathcal{G}$
    /// returns an empty graph $G$ (i.e. $E$ is empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Build an empty graph.
    /// let g = Graph::empty(["A", "B", "C"])?;
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn empty<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;

    /// Complete constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The complete constructor of $\mathcal{G}$
    /// returns an complete graph $G$ (i.e. $E$ is $V \times V$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// # fn main() -> Result<(), ErrorGraph> {
    /// // Build a complete graph.
    /// let g = DiGraph::complete(["A", "B", "C"])?;
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 6);
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn complete<I, V>(vertices: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = V>,
        V: Into<Self::Vertex>;
}
