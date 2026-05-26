use dry::macro_for;

use crate::{
    datasets::Dataset,
    inference::{BNInference, BackdoorCriterion, Modelled, ParBNInference},
    models::{BN, CatBN, GaussBN, Labelled, MixedBN, Phi},
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
    /// Estimate the population average causal effect of `X` on `Y` with
    /// optional evidence W = w as E(Y | do(X), W = w).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `w` - The evidence, if any.
    ///
    /// # Errors
    ///
    /// * `EmptySet` if `X` is empty.
    /// * `EmptySet` if `Y` is empty.
    /// * `SetsNotDisjoint` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated population average causal effect of `X` on `Y`,
    /// or `None` if the effect is not identifiable.
    ///
    #[inline]
    fn pace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        w: Option<&T::Evidence>,
    ) -> Result<Option<T::CPD>> {
        self.cpace_estimate(x, y, &set![], w)
    }

    /// Estimate the conditional population average causal effect of `X` on `Y`
    /// given `Z` with optional evidence W = w as E(Y | do(X), Z, W = w).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    /// * `w` - The evidence, if any.
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
    /// The estimated conditional population average causal effect of `X` on `Y` given `Z`,
    /// or `None` if the effect is not identifiable.
    ///
    fn cpace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&T::Evidence>,
    ) -> Result<Option<T::CPD>>;

    /// Estimate the sample average causal effect of `X` on `Y`
    /// with evidence `W = w` from data `D` as E(Y | do(X), W = w).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `d` - The data to use as evidence.
    ///
    /// # Errors
    ///
    /// * `EmptySet` if `X` is empty.
    /// * `EmptySet` if `Y` is empty.
    /// * `SetsNotDisjoint` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated sample average causal effect of `X` on `Y`,
    /// or `None` if the effect is not identifiable.
    ///
    #[inline]
    fn sace_estimate<D>(&self, x: &Set<usize>, y: &Set<usize>, d: D) -> Result<Option<Vec<T::CPD>>>
    where
        D: Dataset<Evidence = T::Evidence>,
    {
        self.csace_estimate(x, y, &set![], d)
    }

    /// Estimate the conditional sample average causal effect of `X` on `Y`
    /// given `Z` with evidence `W = w` from data `D` as E(Y | do(X), Z, W = w).
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    /// * `d` - The data to use as evidence.
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
    /// The estimated conditional sample average causal effect of `X` on `Y` given `Z`,
    /// or `None` if the effect is not identifiable.
    ///
    fn csace_estimate<D>(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        d: D,
    ) -> Result<Option<Vec<T::CPD>>>
    where
        D: Dataset<Evidence = T::Evidence>;
}

macro_for!($type in [CatBN, GaussBN] {

    impl<E> BNCausalInference<$type> for CausalInference<'_, E>
    where
        E: Modelled<$type> + BNInference<$type>,
    {
        fn cpace_estimate(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            w: Option<&<$type as BN>::Evidence>,
        ) -> Result<Option<<$type as BN>::CPD>> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X"));
            }
            // Check Y is not empty.
            if y.is_empty() {
                return Err(Error::EmptySet("Y"));
            }
            // Check X and Y are disjoint.
            if !x.is_disjoint(y) {
                return Err(Error::SetsNotDisjoint("X", "Y"));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X", "Z"));
            }
            // Check Y and Z are disjoint.
            if !y.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("Y", "Z"));
            }

            // Get the variables from the evidence.
            let w_ = &w.map_or(set![], |w| w.evidences().iter().filter_map(|e| e.clone().map(|e| e.event())).collect());

            // Check (X or Y or Z) and W are disjoint.
            if !x.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("X", "W"));
            }
            if !y.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("Y", "W"));
            }
            if !z.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("Z", "W"));
            }

            /* Effect Identification */

            // Get the model.
            let m = self.engine.model();
            // Get the union of Z and W.
            let z_w = &(z | w_);
            // Find a minimal backdoor adjustment set (Z \cup W) \cup S, if any.
            let z_w_s = m.graph().find_minimal_backdoor_set(x, y, Some(z_w), None)?;

            /* Effect Estimation */

            // Match on the backdoor adjustment set.
            match z_w_s {
                // If no backdoor adjustment set exists, return None.
                None => Ok(None),
                // If the backdoor adjustment set is empty ...
                Some(z_w_s) if z_w_s.is_empty() => {
                    // ... estimate P(Y | do(X), W = w) as P(Y | X, W = w).
                    Ok(Some(self.engine.estimate(y, x, w)?))
                }
                // If the backdoor adjustment set is equal to (Z \cup W) ...
                Some(z_w_s) if z_w_s.eq(z_w) => {
                    // ... estimate P(Y | do(X), Z, W = w) as P(Y | X, Z, W = w).
                    Ok(Some(self.engine.estimate(y, &(x | z), w)?))
                }
                // If the backdoor adjustment set is not equal to (Z \cup W) ...
                Some(z_w_s) => {
                    // Get the S part.
                    let s = &(&(&z_w_s - z) - w_);
                    // Get the Z \cup S part.
                    let z_s = &(&z_w_s - w_);
                    // Estimate P(Y | X, Z, W = w, S) and P(S).
                    let p_y_x_z_s = self.engine.estimate(y, &(x | z_s), w)?;
                    let p_s = self.engine.estimate(s, &set![], None)?;
                    // Convert to potentials for aligned multiplication.
                    let p_y_x_z_s = p_y_x_z_s.into_phi()?;
                    let p_s = p_s.into_phi()?;
                    // Compute P(Y | X, Z, W = w, S) * P(S) using potentials.
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

        fn csace_estimate<D>(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            d: D,
        ) -> Result<Option<Vec<<$type as BN>::CPD>>>
        where
            D: Dataset<Evidence = <$type as BN>::Evidence>,
        {
            // Check labels of the estimator model and the dataset are the same.
            if self.engine.model().labels() != d.labels() {
                return Err(Error::LabelMismatch(
                    &format!("{:?}", self.engine.model().labels()),
                    &format!("{:?}", d.labels()),
                ));
            }

            // Exclude the variable in X, Y and Z.
            let u = Set::from_iter(0..d.labels().len());
            let u = &(&(&u - x) - y) - z;
            // Restrict the data to the variables in U.
            let d_prime = d.select(&u)?;

            // For each evidence w in D ...
            d_prime
                .evidence_iter()
                // ... estimate the CPACE with evidence W = w ...
                .map(|w| w.and_then(|w| self.cpace_estimate(x, y, z, Some(&w))))
                // ... and collect the results.
                .collect()
        }
    }

});

/// A trait for causal inference with Bayesian Networks in parallel.
pub trait ParBNCausalInference<T>
where
    T: BN,
{
    /// Estimate the population average causal effect of `X` on `Y`
    /// with optional evidence W = w as E(Y | do(X), W = w) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `w` - The evidence, if any.
    ///
    /// # Errors
    ///
    /// * `InvalidParameter` if `X` is empty.
    /// * `InvalidParameter` if `Y` is empty.
    /// * `InvalidParameter` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated population average causal effect of `X` on `Y`.
    ///
    #[inline]
    fn par_pace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        w: Option<&T::Evidence>,
    ) -> Result<Option<T::CPD>> {
        self.par_cpace_estimate(x, y, &set![], w)
    }

    /// Estimate the conditional population average causal effect of `X` on `Y`
    /// given `Z` with optional evidence `W = w` as E(Y | do(X), Z, W = w) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    /// * `w` - The evidence, if any.
    ///
    /// # Errors
    ///
    /// * `InvalidParameter` if `X` is empty.
    /// * `InvalidParameter` if `Y` is empty.
    /// * `InvalidParameter` if `X` and `Y` are not disjoint.
    /// * `InvalidParameter` if `X` and `Z` are not disjoint.
    /// * `InvalidParameter` if `Y` and `Z` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated conditional population average causal effect of `X` on `Y` given `Z`.
    ///
    fn par_cpace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&T::Evidence>,
    ) -> Result<Option<T::CPD>>;

    /// Estimate the sample average causal effect of `X` on `Y`
    /// with evidence `W = w` from data `D` as E(Y | do(X), W = w) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `d` - The data to use as evidence.
    ///
    /// # Errors
    ///
    /// * `InvalidParameter` if `X` is empty.
    /// * `InvalidParameter` if `Y` is empty.
    /// * `InvalidParameter` if `X` and `Y` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated sample average causal effect of `X` on `Y`.
    ///
    #[inline]
    fn par_sace_estimate<D>(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        d: D,
    ) -> Result<Option<Vec<T::CPD>>>
    where
        D: Dataset<Evidence = T::Evidence>,
    {
        self.par_csace_estimate(x, y, &set![], d)
    }

    /// Estimate the conditional sample average causal effect of `X` on `Y`
    /// given `Z` with evidence `W = w` from data `D` as E(Y | do(X), Z, W = w) in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The cause variables.
    /// * `y` - The effect variables.
    /// * `z` - The conditioning variables.
    /// * `d` - The data to use as evidence.
    ///
    /// # Errors
    ///
    /// * `InvalidParameter` if `X` is empty.
    /// * `InvalidParameter` if `Y` is empty.
    /// * `InvalidParameter` if `X` and `Y` are not disjoint.
    /// * `InvalidParameter` if `X` and `Z` are not disjoint.
    /// * `InvalidParameter` if `Y` and `Z` are not disjoint.
    ///
    /// # Returns
    ///
    /// The estimated conditional sample average causal effect of `X` on `Y` given `Z`.
    ///
    fn par_csace_estimate<D>(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        d: D,
    ) -> Result<Option<Vec<T::CPD>>>
    where
        D: Dataset<Evidence = T::Evidence>;
}

macro_for!($type in [CatBN, GaussBN] {

    impl<E> ParBNCausalInference<$type> for CausalInference<'_, E>
    where
        E: Modelled<$type> + ParBNInference<$type>,
    {
        fn par_cpace_estimate(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            w: Option<&<$type as BN>::Evidence>
        ) -> Result<Option<<$type as BN>::CPD>> {
            // Check X is not empty.
            if x.is_empty() {
                return Err(Error::EmptySet("X"));
            }
            // Check Y is not empty.
            if y.is_empty() {
                return Err(Error::EmptySet("Y"));
            }
            // Check X and Y are disjoint.
            if !x.is_disjoint(y) {
                return Err(Error::SetsNotDisjoint("X", "Y"));
            }
            // Check X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X", "Z"));
            }
            // Check Y and Z are disjoint.
            if !y.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("Y", "Z"));
            }

            // Get the variables from the evidence.
            let w_ = &w.map_or(set![], |w| w.evidences().iter().filter_map(|e| e.clone().map(|e| e.event())).collect());

            // Check (X or Y or Z) and W are disjoint.
            if !x.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("X", "W"));
            }
            if !y.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("Y", "W"));
            }
            if !z.is_disjoint(w_) {
                return Err(Error::SetsNotDisjoint("Z", "W"));
            }

            /* Effect Identification */

            // Get the model.
            let m = self.engine.model();
            // Get the union of Z and W.
            let z_w = &(z | w_);
            // Find a minimal backdoor adjustment set (Z \cup W) \cup S, if any.
            let z_w_s = m.graph().find_minimal_backdoor_set(x, y, Some(z_w), None)?;

            // Match on the backdoor adjustment set.
            match z_w_s {
                // If no backdoor adjustment set exists, return None.
                None => Ok(None),
                // If the backdoor adjustment set is empty ...
                Some(z_w_s) if z_w_s.is_empty() => {
                    // ... estimate P(Y | do(X), W = w) as P(Y | X, W = w).
                    Ok(Some(self.engine.par_estimate(y, x, w)?))
                }
                // If the backdoor adjustment set is equal to (Z \cup W) ...
                Some(z_w_s) if z_w_s.eq(z_w) => {
                    // ... estimate P(Y | do(X), Z, W = w) as P(Y | X, Z, W = w).
                    Ok(Some(self.engine.par_estimate(y, &(x | z), w)?))
                }
                // If the backdoor adjustment set is not equal to (Z \cup W) ...
                Some(z_w_s) => {
                    // Get the S part.
                    let s = &(&(&z_w_s - z) - w_);
                    // Get the Z \cup S part.
                    let z_s = &(&z_w_s - w_);
                    // Estimate P(Y | X, Z, W = w, S) and P(S).
                    let p_y_x_z_s = self.engine.par_estimate(y, &(x | z_s), w)?;
                    let p_s = self.engine.par_estimate(s, &set![], None)?;
                    // Convert to potentials for aligned multiplication.
                    let p_y_x_z_s = p_y_x_z_s.into_phi()?;
                    let p_s = p_s.into_phi()?;
                    // Compute P(Y | X, Z, W = w, S) * P(S) using potentials.
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

        fn par_csace_estimate<D>(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            d: D,
        ) -> Result<Option<Vec<<$type as BN>::CPD>>>
        where
            D: Dataset<Evidence = <$type as BN>::Evidence>,
        {
            // Check labels of the estimator model and the dataset are the same.
            if self.engine.model().labels() != d.labels() {
                return Err(Error::LabelMismatch(
                    &format!("{:?}", self.engine.model().labels()),
                    &format!("{:?}", d.labels()),
                ));
            }

            // Exclude the variable in X, Y and Z.
            let u = Set::from_iter(0..d.labels().len());
            let u = &(&(&u - x) - y) - z;
            // Restrict the data to the variables in U.
            let d_prime = d.select(&u)?;

            // For each evidence w in D ...
            d_prime
                .evidence_iter()
                // ... estimate the CPACE with evidence W = w ...
                .map(|w| w.and_then(|w| self.par_cpace_estimate(x, y, z, Some(&w))))
                // ... and collect the results.
                .collect()
        }
    }

});

// ── MixedBN Causal Inference (phi conversion not yet supported) ───

impl<E> BNCausalInference<MixedBN> for CausalInference<'_, E>
where
    E: Modelled<MixedBN> + BNInference<MixedBN>,
{
    fn cpace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&<MixedBN as BN>::Evidence>,
    ) -> Result<Option<<MixedBN as BN>::CPD>> {
        if x.is_empty() {
            return Err(Error::EmptySet("X"));
        }
        if y.is_empty() {
            return Err(Error::EmptySet("Y"));
        }
        if !x.is_disjoint(y) {
            return Err(Error::SetsNotDisjoint("X", "Y"));
        }
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint("X", "Z"));
        }
        if !y.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint("Y", "Z"));
        }

        let w_ = &w.map_or(set![], |w| w.events());

        if !x.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("X", "W"));
        }
        if !y.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("Y", "W"));
        }
        if !z.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("Z", "W"));
        }

        let m = self.engine.model();
        let z_w = &(z | w_);
        let z_w_s = m.graph().find_minimal_backdoor_set(x, y, Some(z_w), None)?;

        match z_w_s {
            None => Ok(None),
            Some(z_w_s) if z_w_s.is_empty() => Ok(Some(self.engine.estimate(y, x, w)?)),
            Some(z_w_s) if z_w_s.eq(z_w) => Ok(Some(self.engine.estimate(y, &(x | z), w)?)),
            Some(_) => Err(Error::InvalidParameter(
                "backdoor_set",
                "Phi conversion not yet supported for MixedBN",
            )),
        }
    }

    fn csace_estimate<D>(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        d: D,
    ) -> Result<Option<Vec<<MixedBN as BN>::CPD>>>
    where
        D: Dataset<Evidence = <MixedBN as BN>::Evidence>,
    {
        if self.engine.model().labels() != d.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", self.engine.model().labels()),
                &format!("{:?}", d.labels()),
            ));
        }

        let u = Set::from_iter(0..d.labels().len());
        let u = &(&(&u - x) - y) - z;
        let d_prime = d.select(&u)?;

        d_prime
            .evidence_iter()
            .map(|w| w.and_then(|w| self.cpace_estimate(x, y, z, Some(&w))))
            .collect()
    }
}

impl<E> ParBNCausalInference<MixedBN> for CausalInference<'_, E>
where
    E: Modelled<MixedBN> + ParBNInference<MixedBN>,
{
    fn par_cpace_estimate(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&<MixedBN as BN>::Evidence>,
    ) -> Result<Option<<MixedBN as BN>::CPD>> {
        if x.is_empty() {
            return Err(Error::EmptySet("X"));
        }
        if y.is_empty() {
            return Err(Error::EmptySet("Y"));
        }
        if !x.is_disjoint(y) {
            return Err(Error::SetsNotDisjoint("X", "Y"));
        }
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint("X", "Z"));
        }
        if !y.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint("Y", "Z"));
        }

        let w_ = &w.map_or(set![], |w| w.events());

        if !x.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("X", "W"));
        }
        if !y.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("Y", "W"));
        }
        if !z.is_disjoint(w_) {
            return Err(Error::SetsNotDisjoint("Z", "W"));
        }

        let m = self.engine.model();
        let z_w = &(z | w_);
        let z_w_s = m.graph().find_minimal_backdoor_set(x, y, Some(z_w), None)?;

        match z_w_s {
            None => Ok(None),
            Some(z_w_s) if z_w_s.is_empty() => Ok(Some(self.engine.par_estimate(y, x, w)?)),
            Some(z_w_s) if z_w_s.eq(z_w) => Ok(Some(self.engine.par_estimate(y, &(x | z), w)?)),
            Some(_) => Err(Error::InvalidParameter(
                "backdoor_set",
                "Phi conversion not yet supported for MixedBN",
            )),
        }
    }

    fn par_csace_estimate<D>(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        d: D,
    ) -> Result<Option<Vec<<MixedBN as BN>::CPD>>>
    where
        D: Dataset<Evidence = <MixedBN as BN>::Evidence>,
    {
        if self.engine.model().labels() != d.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", self.engine.model().labels()),
                &format!("{:?}", d.labels()),
            ));
        }

        let u = Set::from_iter(0..d.labels().len());
        let u = &(&(&u - x) - y) - z;
        let d_prime = d.select(&u)?;

        d_prime
            .evidence_iter()
            .map(|w| w.and_then(|w| self.par_cpace_estimate(x, y, z, Some(&w))))
            .collect()
    }
}
