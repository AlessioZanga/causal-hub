use itertools::Either;

use crate::{
    datasets::{CatIncTable, CatTable, CatWtdTable, IncDataset, MissingMethod as MM},
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::{CatCPDS, Labelled},
    types::Set,
};

impl SSE<'_, CatIncTable> {
    /// Apply the missing data handling method to obtain a complete or weighted dataset.
    fn apply_missing_method(
        d: &CatIncTable,
        m: &Option<MM>,
        x: &Set<usize>,
        z: &Set<usize>,
    ) -> Either<CatTable, CatWtdTable> {
        // Compute the union of X and Z indices.
        let x_z = &(x | z);
        // Apply the missing handling method if any, or default to pair-wise deletion.
        match m.as_ref().unwrap_or(&MM::PW) {
            MM::LW => Either::Left(d.lw_deletion()),
            MM::PW => Either::Left(d.pw_deletion(x_z)),
            MM::IPW(r) => Either::Right(d.ipw_deletion(x_z, *r)),
            MM::AIPW(r) => Either::Right(d.aipw_deletion(x_z, *r)),
        }
    }
}

impl CSSEstimator<CatCPDS> for SSE<'_, CatIncTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
        // Get dataset and missing method.
        let (d, m) = (self.dataset, &self.missing_method);
        // Apply the missing handling method.
        let dataset = Self::apply_missing_method(d, m, x, z);
        // Estimate based on the resulting dataset.
        match dataset {
            Either::Left(complete) => {
                // Create a new estimator for the complete dataset.
                let sse = SSE::new(&complete);
                // Map the original indices to the new dataset.
                let x = &sse.indices_from(x, d.labels());
                let z = &sse.indices_from(z, d.labels());
                // Estimate on the complete dataset.
                sse.fit(x, z)
            }
            Either::Right(weighed) => {
                // Create a new estimator for the weighted dataset.
                let sse = SSE::new(&weighed);
                // Map the original indices to the new dataset.
                let x = &sse.indices_from(x, d.labels());
                let z = &sse.indices_from(z, d.labels());
                // Estimate on the weighted dataset.
                sse.fit(x, z)
            }
        }
    }
}

impl ParCSSEstimator<CatCPDS> for SSE<'_, CatIncTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCPDS {
        // Get dataset and missing method.
        let (d, m) = (self.dataset, &self.missing_method);
        // Apply the missing handling method.
        let dataset = Self::apply_missing_method(d, m, x, z);
        // Estimate based on the resulting dataset.
        match dataset {
            Either::Left(complete) => {
                // Create a new estimator for the complete dataset.
                let sse = SSE::new(&complete);
                // Map the original indices to the new dataset.
                let x = &sse.indices_from(x, d.labels());
                let z = &sse.indices_from(z, d.labels());
                // Estimate on the complete dataset in parallel.
                sse.par_fit(x, z)
            }
            Either::Right(weighted) => {
                // Create a new estimator for the weighted dataset.
                let sse = SSE::new(&weighted);
                // Map the original indices to the new dataset.
                let x = &sse.indices_from(x, d.labels());
                let z = &sse.indices_from(z, d.labels());
                // Estimate on the weighted dataset in parallel.
                sse.par_fit(x, z)
            }
        }
    }
}
