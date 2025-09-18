use crate::{
    inference::{BNInference, BackdoorCriterion},
    models::{BN, CatBN, CatCPD, Labelled},
    set,
    types::Set,
};

/// A causal inference engine.
#[derive(Clone, Debug)]
pub struct CausalInference<'a, E> {
    engine: &'a E,
}

impl<'a, E> CausalInference<'a, E> {
    /// Create a new causal inference engine.
    ///
    /// # Arguments
    ///
    /// * `engine` - The underlying inference engine.
    ///
    /// # Returns
    ///
    /// The causal inference engine.
    ///
    pub fn new(engine: &'a E) -> Self {
        Self { engine }
    }
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
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    ///
    /// # Panics
    ///
    /// * If `X` is empty.
    /// * If `Y` is empty.
    /// * If `X` and `Y` are not disjoint.
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
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    ///
    /// # Panics
    ///
    /// * If `X` is empty.
    /// * If `Y` is empty.
    /// * If `X` and `Y` are not disjoint.
    /// * If `X` and `Z` are not disjoint.
    /// * If `Y` and `Z` are not disjoint.
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
        // Assert X is not empty.
        assert!(!x.is_empty(), "Variables X must not be empty.");
        // Assert Y is not empty.
        assert!(!y.is_empty(), "Variables Y must not be empty.");
        // Assert X and Y are disjoint.
        assert!(x.is_disjoint(y), "Variables X and Y must be disjoint.");
        // Assert X and Z are disjoint.
        assert!(x.is_disjoint(z), "Variables X and Z must be disjoint.");
        // Assert Y and Z are disjoint.
        assert!(y.is_disjoint(z), "Variables Y and Z must be disjoint.");

        /* Effect Identification */

        // Get the model.
        let m = self.engine.model();
        // Find a minimal backdoor adjustment set Z \cup S, if any.
        let z_s = m.graph().find_minimal_backdoor_set(x, y, Some(z), None);

        /* Effect Estimation */

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
                // Convert to potentials for aligned multiplication.
                let p_y_x_z_s = p_y_x_z_s.into_phi();
                let p_s = p_s.into_phi();
                // Compute P(Y | X, Z, S) * P(S) using potentials.
                let p_y_s_do_x_z = &p_y_x_z_s * &p_s;
                // Map BN indices to the potential indices.
                let s = p_y_s_do_x_z.indices_from(s, m.labels());
                // Marginalize over S.
                let p_y_do_x_z = p_y_s_do_x_z.marginalize(&s);
                // Map BN indices to the potential indices.
                let x = p_y_do_x_z.indices_from(x, m.labels());
                let y = p_y_do_x_z.indices_from(y, m.labels());
                let z = p_y_do_x_z.indices_from(z, m.labels());
                // Convert back to CPD.
                let p_y_do_x_z = p_y_do_x_z.into_cpd(&y, &(&x | &z));
                // Return the result.
                Some(p_y_do_x_z)
            }
        }
    }
}
