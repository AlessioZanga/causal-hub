/// Traversal enumerator.
pub enum Traversal {
    /// Tree variant, i.e. only vertices reachable from the source vertex are visited.
    Tree,
    /// Forest variant, i.e. any vertex in the graph is visited.
    Forest,
}

mod breadth_first_search;
pub use breadth_first_search::*;

mod depth_first_search;
pub use depth_first_search::*;

mod depth_first_search_edges;
pub use depth_first_search_edges::*;

mod lexicographic_breadth_first_search;
pub use lexicographic_breadth_first_search::*;

mod lexicographic_depth_first_search;
pub use lexicographic_depth_first_search::*;

mod topological_sort;
pub use topological_sort::*;
