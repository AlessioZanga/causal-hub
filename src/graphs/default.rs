use super::BaseGraph;

/// Default graph trait.
pub trait DefaultGraph: BaseGraph + Default {
    /// Null constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The null constructor of $\mathcal{G}$
    /// returns a null graph $\mathcal{G}$ (i.e. both $\mathbf{V}$ and $\mathbf{E}$ are empty).
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
    /// returns an empty graph $\mathcal{G}$ (i.e. $\mathbf{E}$ is empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build an empty graph.
    /// let g = Graph::empty(["A", "B", "C"]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also empty.
    /// assert_eq!(g.size(), 0);
    /// ```
    ///
    fn empty<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;

    /// Complete constructor.
    ///
    /// Let be $\mathcal{G}$ a graph type. The complete constructor of $\mathcal{G}$
    /// returns an complete graph $\mathcal{G}$ (i.e. $\mathbf{E}$ is $V \times V$).
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a complete graph.
    /// let g = DiGraph::complete(["A", "B", "C"]);
    ///
    /// // The vertex set is not empty.
    /// assert_eq!(g.order(), 3);
    ///
    /// // The edge set is also not empty.
    /// assert_eq!(g.size(), 6);
    /// ```
    ///
    fn complete<V, I>(vertices: I) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>;
}
