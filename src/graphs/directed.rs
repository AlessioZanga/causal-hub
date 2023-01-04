use super::BaseGraph;

/// Ancestors iterator.
///
/// Return the vertex iterator representing $An(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! An {
    ($g:expr, $x:expr) => {
        $g.ancestors($x)
    };
}

/// Parents iterator.
///
/// Return the vertex iterator representing $Pa(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Pa {
    ($g:expr, $x:expr) => {
        $g.parents($x)
    };
}

/// Children iterator.
///
/// Return the vertex iterator representing $Ch(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! Ch {
    ($g:expr, $x:expr) => {
        $g.children($x)
    };
}

/// Descendants iterator.
///
/// Return the vertex iterator representing $De(\mathcal{G}, X)$.
///
#[macro_export]
macro_rules! De {
    ($g:expr, $x:expr) => {
        $g.descendants($x)
    };
}

/// Directed graph trait.
pub trait DirectedGraph: BaseGraph {
    /// Ancestors iterator type.
    type AncestorsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Parents iterator type.
    type ParentsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Children iterator type.
    type ChildrenIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Descendants iterator type.
    type DescendantsIter<'a>: Iterator<Item = usize>
    where
        Self: 'a;

    /// Ancestors iterator.
    ///
    /// Iterates over the vertex set $An(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.index("B");
    ///
    /// // Use the ancestors iterator.
    /// assert!(g.ancestors(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'An!'.
    /// assert!(g.ancestors(x).eq(An!(g, x)));
    /// ```
    ///
    fn ancestors(&self, x: usize) -> Self::AncestorsIter<'_>;

    /// Checks ancestor vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is ancestor of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_ancestor(x, y));
    /// assert!(An!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        self.ancestors(x).any(|z| z == y)
    }

    /// Parents iterator.
    ///
    /// Iterates over the vertex set $Pa(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.index("A");
    ///
    /// // Use the parents iterator.
    /// assert!(g.parents(x).eq([0, 2]));
    ///
    /// // Use the associated macro 'Pa!'.
    /// assert!(g.parents(x).eq(Pa!(g, x)));
    /// ```
    ///
    fn parents(&self, x: usize) -> Self::ParentsIter<'_>;

    /// Checks parent vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is parent of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("A"), g.index("C"));
    ///
    /// // Check edge.
    /// assert!(g.is_parent(x, y));
    /// assert!(Pa!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_parent(&self, x: usize, y: usize) -> bool {
        self.parents(x).any(|z| z == y)
    }

    /// Children iterator.
    ///
    /// Iterates over the vertex set $Ch(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.index("A");
    ///
    /// // Use the children iterator.
    /// assert!(g.children(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'Ch!'.
    /// assert!(g.children(x).eq(Ch!(g, x)));
    /// ```
    ///
    fn children(&self, x: usize) -> Self::ChildrenIter<'_>;

    /// Checks children vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is child of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("C"), g.index("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_child(x, y));
    /// assert!(Ch!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_child(&self, x: usize, y: usize) -> bool {
        self.children(x).any(|z| z == y)
    }

    /// Descendants iterator.
    ///
    /// Iterates over the vertex set $De(\mathcal{G}, X)$ of a given vertex $X$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose vertex.
    /// let x = g.index("C");
    ///
    /// // Use the descendants iterator.
    /// assert!(g.descendants(x).eq([0, 1]));
    ///
    /// // Use the associated macro 'De!'.
    /// assert!(g.descendants(x).eq(De!(g, x)));
    /// ```
    ///
    fn descendants(&self, x: usize) -> Self::DescendantsIter<'_>;

    /// Checks descendant vertices in the graph.
    ///
    /// Checks whether a vertex $Y$ is descendant of another vertex $X$ or not.
    ///
    /// # Panics
    ///
    /// At least one of the vertex indexes does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let g = DiGraph::from(e);
    ///
    /// // Choose an edge.
    /// let (x, y) = (g.index("C"), g.index("A"));
    ///
    /// // Check edge.
    /// assert!(g.is_descendant(x, y));
    /// assert!(De!(g, x).any(|z| z == y))
    /// ```
    ///
    fn is_descendant(&self, x: usize, y: usize) -> bool {
        self.descendants(x).any(|z| z == y)
    }

    /// In-degree of a given vertex.
    ///
    /// Computes the in-degree of a given vertex, i.e. $|Pa(\mathcal{G}, X)|$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = DiGraph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.in_degree(x), 2);
    /// assert_eq!(g.in_degree(x), Pa!(g, x).count());
    /// ```
    ///
    fn in_degree(&self, x: usize) -> usize {
        self.parents(x).count()
    }

    /// Out-degree of a given vertex.
    ///
    /// Computes the out-degree of a given vertex, i.e. $|Ch(\mathcal{G}, X)|$.
    ///
    /// # Panics
    ///
    /// Panics if the vertex identifier does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([("A", "B"), ("C", "A"), ("A", "A")]);
    ///
    /// // Build a graph.
    /// let mut g = DiGraph::from(e);
    ///
    /// // Choose a vertex.
    /// let x = g.index("A");
    ///
    /// // Check degree.
    /// assert_eq!(g.out_degree(x), 2);
    /// assert_eq!(g.out_degree(x), Ch!(g, x).count());
    /// ```
    ///
    fn out_degree(&self, x: usize) -> usize {
        self.children(x).count()
    }
}
