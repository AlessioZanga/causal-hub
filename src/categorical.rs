use ndarray::Array2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Categorical {
    variables: Vec<(String, Vec<String>)>,
    probabilities: Array2<f64>,
}

impl Categorical {
    pub fn new(
        variables: Vec<(String, Vec<String>)>,
        probabilities: Array2<f64>,
    ) -> Self {
        Self {
            variables,
            probabilities,
        }
    }

    pub fn get_probability(&self, state: Vec<usize>) -> f64 {
        let mut index = 0;
        let mut multiplier = 1;

        for (i, &s) in state.iter().rev().enumerate() {
            index += s * multiplier;
            multiplier *= self.variables[i].1.len();
        }

        self.probabilities[[index / self.variables[0].1.len(), index % self.variables[0].1.len()]]
    }
}
