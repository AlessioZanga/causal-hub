use crate::{
    estimators::{BE, CPDEstimator, ParCPDEstimator},
    models::{MixedCPD, MixedIncTable, MixedTable, MixedWtdTable},
    types::Set,
};

macro_rules! impl_be_for_mixed {
    ($enum:ident) => {
        impl CPDEstimator<MixedCPD> for BE<'_, $enum, ()> {
            fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPD> {
                match self.dataset {
                    $enum::Categorical(t) => BE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(MixedCPD::Categorical),
                    $enum::Gaussian(t) => BE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(MixedCPD::Gaussian),
                }
            }
        }
        impl ParCPDEstimator<MixedCPD> for BE<'_, $enum, ()> {
            fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPD> {
                match self.dataset {
                    $enum::Categorical(t) => BE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(MixedCPD::Categorical),
                    $enum::Gaussian(t) => BE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(MixedCPD::Gaussian),
                }
            }
        }
    };
}

impl_be_for_mixed!(MixedTable);
impl_be_for_mixed!(MixedIncTable);
impl_be_for_mixed!(MixedWtdTable);
