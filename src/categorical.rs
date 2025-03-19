use ndarray::Array2;

/// A struct representing a categorical distribution.
///
pub struct Categorical {
    labels: Vec<String>,
    states: Vec<Vec<String>>,
    probabilities: Array2<f64>,
}

impl Categorical {
    /// Creates a new categorical distribution.
    ///
    /// # Arguments
    ///
    /// * `variables` - The variables and their states.
    /// * `probabilities` - The probabilities of the states.
    ///
    /// # Returns
    ///
    /// A new `Categorical` instance.
    ///
    pub fn new(variables: &[(&str, Vec<&str>)], probabilities: Array2<f64>) -> Self {
        // Convert the array of string slices to a vector of strings.
        let (labels, states): (Vec<_>, Vec<Vec<_>>) = {
            (
                // Get the labels of the variables.
                variables.iter().map(|(i, _)| i.to_string()).collect(),
                // Get the states of the variables.
                variables
                    .iter()
                    .map(|(_, i)| i.iter().map(|s| s.to_string()).collect())
                    .collect(),
            )
        };

        // Check if the number of states of the first variable matches the number of columns.
        assert_eq!(
            probabilities.ncols(),
            states.get(0).map(|i| i.len()).unwrap_or(0),
            "Number of states of the first variable does not match the number of columns."
        );
        // Check if the product of the number of states of the remaining variables matches the number of rows.
        assert_eq!(
            probabilities.nrows(),
            states.iter().skip(1).map(|i| i.len()).product(),
            "Product of the number of states of the remaining variables does not match the number of rows."
        );

        Self {
            labels,
            states,
            probabilities,
        }
    }

    /// Returns the labels of the variables in the categorical distribution.
    /// 
    /// # Returns
    /// 
    /// A reference to the vector of labels.
    /// 
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Returns the states of the variables in the categorical distribution.
    /// 
    /// # Returns
    /// 
    /// A reference to the vector of states.
    /// 
    pub fn states(&self) -> &Vec<Vec<String>> {
        &self.states
    }

    /// Returns the probabilities of the states in the categorical distribution.
    /// 
    /// # Returns
    /// 
    /// A reference to the array of probabilities.
    /// 
    pub fn probabilities(&self) -> &Array2<f64> {
        &self.probabilities
    }
}
