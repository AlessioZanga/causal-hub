use thiserror::Error;

/// An enum representing custom errors.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Error indicating a mismatch between the number of columns and labels.
    #[error("Number of columns does not match number of labels.")]
    ColumnsLabelsMismatch {
        /// The number of columns.
        columns: usize,
        /// The number of labels.
        labels: usize,
    },
    /// Error indicating a mismatch between the number of columns and states.
    #[error(
        "Number of variables must be equal to the number of columns: \n\
        \t expected:    |states| == |values.columns()| , \n\
        \t found:       |states| == {states} and |values.columns()| == {columns} ."
    )]
    ColumnsStatesMismatch {
        /// The number of columns.
        columns: usize,
        /// The number of states.
        states: usize,
    },
    /// Error indicating the presence of non-finite values.
    #[error("Values contain non-finite numbers.")]
    NonFiniteValues,
    /// Error indicating too many states for a categorical variable.
    #[error(
        "Variable '{label}' should have less than {max_states} states: \n\
        \t expected:    |states| <  {max_states} , \n\
        \t found:       |states| == {states} ."
    )]
    TooManyStates {
        /// The label of the variable.
        label: String,
        /// The number of states.
        states: usize,
        /// The maximum allowed number of states.
        max_states: usize,
    },
}
