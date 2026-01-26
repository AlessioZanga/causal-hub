use dry::macro_for;

use crate::{
    inference::{BNInference, BackdoorCriterion, Modelled, ParBNInference},
    models::{BN, CatBN, GaussBN, Labelled, Phi},
    set,
    types::{Error, Result, Set},
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
    /// Estimate the average causal effect of `X` on `Y` as E(Y | do(X)).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    ///
    /// # Errors
    ///
    /// * `EmptySet` if `X` is empty.
    /// * `EmptySet` if `Y` is empty.
    /// * `SetsNotDisjoint` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated average causal effect of `X` on `Y`.
    ///
    fn ace_estimate(&self, x: &Set<usize>, y: &Set<usize>) -> Result<Option<T::CPD>> {
        self.cace_estimate(x, y, &set![])
    }

    /// Estimate the conditional average causal effect of `X` on `Y` given `Z` as E(Y | do(X), Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    ///
    /// # Errors
    ///
    /// * `EmptySet` if `X` is empty.
    /// * `EmptySet` if `Y` is empty.
    /// * `SetsNotDisjoint` if `X` and `Y` are not disjoint.
    /// * `SetsNotDisjoint` if `X` and `Z` are not disjoint.
    /// * `SetsNotDisjoint` if `Y` and `Z` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated conditional average causal effect of `X` on `Y` given `Z`.
    ///
    fn cace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
    ) -> Result<Option<T::CPD>>;
}

macro_for!($type in [CatBN, GaussBN] {

    impl<E> BNCausalInference<$type> for CausalInference<'_, E>
    where
        E: Modelled<$type> + BNInference<$type>,
    {
        fn cace_estimate(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<Option<<$type as BN>::CPD>> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert Y is not empty.
            if y.is_empty() {
                return Err(Error::EmptySet("Y".into()));
            }
            // Assert X and Y are disjoint.
            if !x.is_disjoint(y) {
                return Err(Error::SetsNotDisjoint("X".into(), "Y".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert Y and Z are disjoint.
            if !y.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("Y".into(), "Z".into()));
            }

            /* Effect Identification */

            // Get the model.
            let m = self.engine.model();
            // Find a minimal backdoor adjustment set Z \cup S, if any.
            let z_s = m.graph().find_minimal_backdoor_set(x, y, Some(z), None)?;

            /* Effect Estimation */

            // Match on the backdoor adjustment set.
            match z_s {
                // If no backdoor adjustment set exists, return None.
                None => Ok(None),
                // If the backdoor adjustment set is empty ...
                Some(z_s) if z_s.is_empty() => {
                    // ... estimate P(Y | do(X)) as P(Y | X).
                    Ok(Some(self.engine.estimate(y, x)?))
                }
                // If the backdoor adjustment set is equal to Z ...
                Some(z_s) if z_s.eq(z) => {
                    // ... estimate P(Y | do(X), Z) as P(Y | X, Z).
                    Ok(Some(self.engine.estimate(y, &(x | z))?))
                }
                // If the backdoor adjustment set is not equal to Z ...
                Some(z_s) => {
                    // Get the S part.
                    let s = &(&z_s - z);
                    // Estimate P(Y | X, Z, S) and P(S).
                    let p_y_x_z_s = self.engine.estimate(y, &(x | &z_s))?;
                    let p_s = self.engine.estimate(s, &set![])?;
                    // Convert to potentials for aligned multiplication.
                    let p_y_x_z_s = p_y_x_z_s.into_phi()?;
                    let p_s = p_s.into_phi()?;
                    // Compute P(Y | X, Z, S) * P(S) using potentials.
                    let p_y_s_do_x_z = &p_y_x_z_s * &p_s;
                    // Map BN indices to the potential indices.
                    let s = p_y_s_do_x_z.indices_from(s, m.labels())?;
                    // Marginalize over S.
                    let p_y_do_x_z = p_y_s_do_x_z.marginalize(&s)?;
                    // Map BN indices to the potential indices.
                    let x = p_y_do_x_z.indices_from(x, m.labels())?;
                    let y = p_y_do_x_z.indices_from(y, m.labels())?;
                    let z = p_y_do_x_z.indices_from(z, m.labels())?;
                    // Convert back to CPD.
                    let p_y_do_x_z = p_y_do_x_z.into_cpd(&y, &(&x | &z))?;
                    // Return the result.
                    Ok(Some(p_y_do_x_z))
                }
            }
        }
    }

});

/// A trait for causal inference with Bayesian Networks in parallel.
pub trait ParBNCausalInference<T>
where
    T: BN,
{
    /// Estimate the average causal effect of `X` on `Y` as E(Y | do(X)) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if `X` is empty.
    /// * `IllegalArgument` if `Y` is empty.
    /// * `IllegalArgument` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated average causal effect of `X` on `Y`.
    ///
    fn par_ace_estimate(&self, x: &Set<usize>, y: &Set<usize>) -> Result<Option<T::CPD>> {
        self.par_cace_estimate(x, y, &set![])
    }

    /// Estimate the conditional average causal effect of `X` on `Y` given `Z` as E(Y | do(X), Z) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if `X` is empty.
    /// * `IllegalArgument` if `Y` is empty.
    /// * `IllegalArgument` if `X` and `Y` are not disjoint.
    /// * `IllegalArgument` if `X` and `Z` are not disjoint.
    /// * `IllegalArgument` if `Y` and `Z` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated conditional average causal effect of `X` on `Y` given `Z`.
    ///
    fn par_cace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
    ) -> Result<Option<T::CPD>>;
}

macro_for!($type in [CatBN, GaussBN] {

    impl<E> ParBNCausalInference<$type> for CausalInference<'_, E>
    where
        E: Modelled<$type> + ParBNInference<$type>,
    {
        fn par_cace_estimate(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<Option<<$type as BN>::CPD>> {
            // Assert X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X".into()));
            }
            // Assert Y is not empty.
            if y.is_empty() {
                return Err(Error::EmptySet("Y".into()));
            }
            // Assert X and Y are disjoint.
            if !x.is_disjoint(y) {
                return Err(Error::SetsNotDisjoint("X".into(), "Y".into()));
            }
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert Y and Z are disjoint.
            if !y.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("Y".into(), "Z".into()));
            }

            /* Effect Identification */

            // Get the model.
            let m = self.engine.model();
            // Find a minimal backdoor adjustment set Z \cup S, if any.
            let z_s = m.graph().find_minimal_backdoor_set(x, y, Some(z), None)?;

            /* Effect Estimation */

            // Match on the backdoor adjustment set.
            match z_s {
                // If no backdoor adjustment set exists, return None.
                None => Ok(None),
                // If the backdoor adjustment set is empty ...
                Some(z_s) if z_s.is_empty() => {
                    // ... estimate P(Y | do(X)) as P(Y | X).
                    Ok(Some(self.engine.par_estimate(y, x)?))
                }
                // If the backdoor adjustment set is equal to Z ...
                Some(z_s) if z_s.eq(z) => {
                    // ... estimate P(Y | do(X), Z) as P(Y | X, Z).
                    Ok(Some(self.engine.par_estimate(y, &(x | z))?))
                }
                // If the backdoor adjustment set is not equal to Z ...
                Some(z_s) => {
                    // Get the S part.
                    let s = &(&z_s - z);
                    // Estimate P(Y | X, Z, S) and P(S).
                    let p_y_x_z_s = self.engine.par_estimate(y, &(x | &z_s))?;
                    let p_s = self.engine.par_estimate(s, &set![])?;
                    // Convert to potentials for aligned multiplication.
                    let p_y_x_z_s = p_y_x_z_s.into_phi()?;
                    let p_s = p_s.into_phi()?;
                    // Compute P(Y | X, Z, S) * P(S) using potentials.
                    let p_y_s_do_x_z = &p_y_x_z_s * &p_s;
                    // Map BN indices to the potential indices.
                    let s = p_y_s_do_x_z.indices_from(s, m.labels())?;
                    // Marginalize over S.
                    let p_y_do_x_z = p_y_s_do_x_z.marginalize(&s)?;
                    // Map BN indices to the potential indices.
                    let x = p_y_do_x_z.indices_from(x, m.labels())?;
                    let y = p_y_do_x_z.indices_from(y, m.labels())?;
                    let z = p_y_do_x_z.indices_from(z, m.labels())?;
                    // Convert back to CPD.
                    let p_y_do_x_z = p_y_do_x_z.into_cpd(&y, &(&x | &z))?;
                    // Return the result.
                    Ok(Some(p_y_do_x_z))
                }
            }
        }
    }

});
