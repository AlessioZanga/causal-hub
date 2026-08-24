use crate::{
    datasets::{CatIncTable, IncDataset, MissingMethod},
    estimators::{CSSEstimator, ParCSSEstimator, SSE},
    models::{CatCPDS, HasLabels},
    types::{Result, Set},
};

impl CSSEstimator<CatCPDS> for SSE<'_, CatIncTable> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPDS> {
        // Get the union of X and Z.
        let x_z = Some(&(x | z));
        // Get the missing method or default to PW.
        let model = self.missing_method.as_ref().unwrap_or(&MissingMethod::PW);
        // Get the missing mechanism or default to None.
        let r = self.missing_mechanism.as_ref();

        // Apply the missing handling method.
        let data = self.dataset.apply_missing_method(model, x_z, r)?;

        // Get the labels of the original dataset.
        let labels = self.dataset.labels();
        // Map the indices from the original dataset to the new one.
        let x = data.indices_from(x, labels)?;
        let z = data.indices_from(z, labels)?;

        // Estimate based on the resulting dataset.
        data.map_either(
            |data| SSE::new(&data).fit(&x, &z), // Complete case.
            |data| SSE::new(&data).fit(&x, &z), // Weighted case.
        )
        .into_inner()
    }
}

impl ParCSSEstimator<CatCPDS> for SSE<'_, CatIncTable> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> Result<CatCPDS> {
        // Get the union of X and Z.
        let x_z = Some(&(x | z));
        // Get the missing method or default to PW.
        let model = self.missing_method.as_ref().unwrap_or(&MissingMethod::PW);
        // Get the missing mechanism or default to None.
        let r = self.missing_mechanism.as_ref();

        // Apply the missing handling method.
        let data = self.dataset.apply_missing_method(model, x_z, r)?;

        // Get the labels of the original dataset.
        let labels = self.dataset.labels();
        // Map the indices from the original dataset to the new one.
        let x = data.indices_from(x, labels)?;
        let z = data.indices_from(z, labels)?;

        // Estimate based on the resulting dataset.
        data.map_either(
            |data| SSE::new(&data).par_fit(&x, &z), // Complete case.
            |data| SSE::new(&data).par_fit(&x, &z), // Weighted case.
        )
        .into_inner()
    }
}
