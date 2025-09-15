use crate::{
    inference::{BNInference, BackdoorCriterion},
    models::{BN, CatBN, CatCPD},
    set,
    types::Set,
};

/// A causal inference engine.
pub struct CausalInference<'a, E> {
    engine: &'a E,
}

/// A trait for causal inference with Bayesian Networks.
pub trait BNCausalInference<T>
where
    T: BN,
{
    /// The output type.
    type Output;

    /// Estimate the average causal effect of `X` on `Y` as E(Y | do(X)).
    ///
    /// # Arguments
    ///
    /// * `x` - The treatment variables.
    /// * `y` - The outcome variables.
    ///
    /// # Returns
    ///
    /// The estimated average causal effect of `X` on `Y`.
    ///
    fn average_causal_effect(&self, x: &Set<usize>, y: &Set<usize>) -> Option<Self::Output> {
        self.conditional_average_causal_effect(x, y, &set![])
    }

    /// Estimate the conditional average causal effect of `X` on `Y` given `Z` as E(Y | do(X), Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The treatment variables.
    /// * `y` - The outcome variables.
    /// * `z` - The conditioning variables.
    ///
    /// # Returns
    ///
    /// The estimated conditional average causal effect of `X` on `Y` given `Z`.
    ///
    fn conditional_average_causal_effect(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
    ) -> Option<Self::Output>;
}

impl<E> BNCausalInference<CatBN> for CausalInference<'_, E>
where
    E: BNInference<CatBN, Output = CatCPD>,
{
    type Output = CatCPD;

    fn conditional_average_causal_effect(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
    ) -> Option<Self::Output> {
        // Get the graph.
        let g = self.engine.model().graph();
        // Find a minimal backdoor adjustment set Z \cup S, if any.
        let z_s = g.find_minimal_backdoor_set(x, y, Some(z), None);
        // Match on the backdoor adjustment set.
        match z_s {
            // If no backdoor adjustment set exists, return None.
            None => None,
            // If the backdoor adjustment set is empty ...
            Some(z_s) if z_s.is_empty() => {
                // ... estimate P(Y | do(X), Z) as P(Y | X, Z).
                Some(self.engine.predict(y, &(x | z)))
            }
            // If the backdoor adjustment set is non-empty ...
            Some(z_s) => {
                // Get the S part.
                let s = &(&z_s - z);
                // Estimate P(Y | X, Z, S) and P(S).
                let p_y_x_z_s = self.engine.predict(y, &(x | s));
                let p_s = self.engine.predict(s, &set![]);
                // TODO: Compute P(Y | X, Z, S) * P(S).
                let p_y_s_do_x_z = p_y_x_z_s; // FIXME: Placeholder.
                // TODO: Map global S indices to the local S indices.
                let s = s; // FIXME: Placeholder.
                // Marginalize over S.
                let p_y_do_x_z = p_y_s_do_x_z.marginalize(s, &set![]);
                // Return the result.
                Some(p_y_do_x_z)
            }
        }
    }
}
