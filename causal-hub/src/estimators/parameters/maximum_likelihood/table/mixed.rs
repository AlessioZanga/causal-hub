use crate::{
    estimators::{CPDEstimator, MLE, ParCPDEstimator},
    models::{MixedCPD, MixedIncTable, MixedTable, MixedWtdTable},
    types::Set,
};

macro_rules! impl_mle_for_mixed {
    ($enum:ident) => {
        impl CPDEstimator<MixedCPD> for MLE<'_, $enum> {
            fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPD> {
                match self.dataset {
                    $enum::Categorical(t) => MLE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(MixedCPD::Categorical),
                    $enum::Gaussian(t) => MLE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(MixedCPD::Gaussian),
                }
            }
        }
        impl ParCPDEstimator<MixedCPD> for MLE<'_, $enum> {
            fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPD> {
                match self.dataset {
                    $enum::Categorical(t) => MLE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(MixedCPD::Categorical),
                    $enum::Gaussian(t) => MLE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(MixedCPD::Gaussian),
                }
            }
        }
    };
}

impl_mle_for_mixed!(MixedTable);
impl_mle_for_mixed!(MixedIncTable);
impl_mle_for_mixed!(MixedWtdTable);
