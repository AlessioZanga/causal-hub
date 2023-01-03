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

    /// Iterator over the ancestors set.
    // FIXME: Add docs.
    fn ancestors(&self, x: usize) -> Self::AncestorsIter<'_>;

    /// Checks if a vertex is ancestor of another vertex.
    // FIXME: Add docs.
    fn is_ancestor(&self, x: usize, y: usize) -> bool {
        self.ancestors(x).any(|z| z == y)
    }

    /// Iterator over the parents set.
    // FIXME: Add docs.
    fn parents(&self, x: usize) -> Self::ParentsIter<'_>;

    /// Checks if a vertex is parent of another vertex.
    // FIXME: Add docs.
    fn is_parent(&self, x: usize, y: usize) -> bool {
        self.parents(x).any(|z| z == y)
    }

    /// Iterator over the children set.
    // FIXME: Add docs.
    fn children(&self, x: usize) -> Self::ChildrenIter<'_>;

    /// Checks if a vertex is child of another vertex.
    // FIXME: Add docs.
    fn is_child(&self, x: usize, y: usize) -> bool {
        self.children(x).any(|z| z == y)
    }

    /// Iterator over the descendants set.
    // FIXME: Add docs.
    fn descendants(&self, x: usize) -> Self::DescendantsIter<'_>;

    /// Checks if a vertex is descendant of another vertex.
    // FIXME: Add docs.
    fn is_descendant(&self, x: usize, y: usize) -> bool {
        self.descendants(x).any(|z| z == y)
    }

    /// In-degree of a vertex.
    // FIXME: Add docs.
    fn in_degree(&self, x: usize) -> usize {
        self.parents(x).count()
    }

    /// Out-degree of a vertex.
    // FIXME: Add docs.
    fn out_degree(&self, x: usize) -> usize {
        self.children(x).count()
    }
}
