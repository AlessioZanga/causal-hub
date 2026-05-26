use crate::{
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::{MixedCPDS, MixedIncTable, MixedTable, MixedWtdTable},
    types::Set,
};

macro_rules! impl_css_for_mixed {
    ($enum:ident) => {
        impl CSSEstimator<MixedCPDS> for SSE<'_, $enum> {
            fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPDS> {
                match self.dataset {
                    $enum::Categorical(t) => SSE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(MixedCPDS::Categorical),
                    $enum::Gaussian(t) => SSE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .fit(x, z)
                        .map(|s| MixedCPDS::Gaussian(Box::new(s))),
                }
            }
        }
        impl ParCSSEstimator<MixedCPDS> for SSE<'_, $enum> {
            fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> crate::types::Result<MixedCPDS> {
                match self.dataset {
                    $enum::Categorical(t) => SSE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(MixedCPDS::Categorical),
                    $enum::Gaussian(t) => SSE::new(t)
                        .with_missing_method(self.missing_method, self.missing_mechanism.clone())?
                        .par_fit(x, z)
                        .map(|s| MixedCPDS::Gaussian(Box::new(s))),
                }
            }
        }
    };
}

impl_css_for_mixed!(MixedTable);
impl_css_for_mixed!(MixedIncTable);
impl_css_for_mixed!(MixedWtdTable);
