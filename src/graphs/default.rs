use super::BaseGraph;

/// Default graph trait.
pub trait DefaultGraph: BaseGraph + Default {
    /// Null graph constructor.
    #[inline]
    fn null() -> Self {
        Default::default()
    }

    /// Empty graph constructor given vertices set.
    fn empty<I>(vertices: I) -> Self
    where
        I: IntoIterator<Item = String>;

    /// Complete graph constructor given vertices set.
    fn complete<I>(vertices: I) -> Self
    where
        I: IntoIterator<Item = String>;
}
