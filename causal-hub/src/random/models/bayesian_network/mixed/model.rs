use rand::prelude::*;

use crate::{
    models::{BN, CatSupport, MixedBN, MixedCPD, MixedSupport},
    random::{Random, RngCatCPD, RngDag, RngGaussCPD},
    set,
    types::{Error, Labels, Map, Result},
};

/// A struct for random mixed Bayesian network generation.
pub struct RngMixedBN<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    labels: &'a Labels,
    support: &'a Map<String, MixedSupport>,
    alpha: f64,
    s_a: f64,
    s_b: f64,
    e: f64,
    p: f64,
}

impl<'a, R> RngMixedBN<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngMixedBN` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the variables.
    /// * `support` - The support of the variables (MixedSupport per variable).
    /// * `alpha` - The Dirichlet parameter for categorical CPDs (must be positive if any categorical).
    /// * `s_a` - The standard deviation of regression coefficients for Gaussian CPDs.
    /// * `s_b` - The standard deviation of the intercept for Gaussian CPDs.
    /// * `e` - A small positive constant for covariance regularization.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Errors
    ///
    /// * If `alpha` is not positive.
    /// * If `s_a`, `s_b`, or `e` are not positive.
    /// * If `p` is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `RngMixedBN` instance.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rng: &'a mut R,
        labels: &'a Labels,
        support: &'a Map<String, MixedSupport>,
        alpha: f64,
        s_a: f64,
        s_b: f64,
        e: f64,
        p: f64,
    ) -> Result<Self> {
        if alpha <= 0.0 {
            return Err(Error::InvalidParameter("alpha", "must be positive"));
        }
        if s_a <= 0.0 {
            return Err(Error::InvalidParameter("s_a", "must be positive"));
        }
        if s_b <= 0.0 {
            return Err(Error::InvalidParameter("s_b", "must be positive"));
        }
        if e <= 0.0 {
            return Err(Error::InvalidParameter("e", "must be positive"));
        }
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self {
            rng,
            labels,
            support,
            alpha,
            s_a,
            s_b,
            e,
            p,
        })
    }
}

impl<R> Random for RngMixedBN<'_, R>
where
    R: Rng,
{
    type Output = Result<MixedBN>;

    fn random(&mut self) -> Self::Output {
        let graph = RngDag::new(self.rng, self.labels, self.p)?.random()?;

        let cpds = self
            .labels
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let pa_i = graph.parents(&set![i])?;
                let mixed_support = &self.support[x];

                match mixed_support {
                    MixedSupport::Categorical(cat_support) => {
                        let mut support = CatSupport::default();
                        support.insert(x.clone(), cat_support[x].clone());

                        let conditioning_support: CatSupport = pa_i
                            .iter()
                            .map(|&j| {
                                let y = &self.labels[j];
                                match &self.support[y] {
                                    MixedSupport::Categorical(s) => (y.clone(), s[y].clone()),
                                    _ => unreachable!("parents must match CPD type"),
                                }
                            })
                            .collect();

                        let cpd =
                            RngCatCPD::new(self.rng, &support, &conditioning_support, self.alpha)?
                                .random()?;
                        Ok(MixedCPD::Categorical(cpd))
                    }
                    MixedSupport::Gaussian(_) => {
                        let v_labels = crate::labels![x.clone()];
                        let conditioning_labels: Labels =
                            pa_i.iter().map(|&j| self.labels[j].clone()).collect();

                        let cpd = RngGaussCPD::new(
                            self.rng,
                            &v_labels,
                            &conditioning_labels,
                            self.s_a,
                            self.s_b,
                            self.e,
                        )?
                        .random()?;
                        Ok(MixedCPD::Gaussian(cpd))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        MixedBN::new(graph, cpds)
    }
}
