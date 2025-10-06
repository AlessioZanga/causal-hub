use std::collections::BTreeMap;

use backend::{
    io::JsonIO,
    models::{BN, DiGraph, GaussBN, Labelled},
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
};
use pyo3::{prelude::*, types::PyType};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::PyGaussTable,
    impl_deref_from_into,
    models::{PyDiGraph, PyGaussCPD},
};

/// A Gaussian Bayesian network.
#[gen_stub_pyclass]
#[pyclass(name = "GaussBN", module = "causal_hub.models", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyGaussBN {
    inner: GaussBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyGaussBN, GaussBN);

#[gen_stub_pymethods]
#[pymethods]
impl PyGaussBN {
    /// Constructs a new Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cpds` - The conditional probability distributions.
    ///
    /// # Returns
    ///
    /// A new Bayesian network instance.
    ///
    #[new]
    pub fn new(graph: &Bound<'_, PyDiGraph>, cpds: &Bound<'_, PyAny>) -> PyResult<Self> {
        // Convert PyDiGraph to DiGraph.
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Convert PyAny to Vec<CatCPD>.
        let cpds: Vec<_> = cpds
            .try_iter()?
            .map(|x| x?.extract::<PyGaussCPD>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyGaussCPD> to Vec<GaussCPD>.
        let cpds = cpds.into_iter().map(|x| x.into());
        // Create a new GaussBN with the given parameters.
        Ok(GaussBN::new(graph, cpds).into())
    }

    /// Returns the name of the model, if any.
    ///
    /// # Returns
    ///
    /// The name of the model, if it exists.
    ///
    pub fn name(&self) -> PyResult<Option<&str>> {
        Ok(self.inner.name())
    }

    /// Returns the description of the model, if any.
    ///
    /// # Returns
    ///
    /// The description of the model, if it exists.
    ///
    pub fn description(&self) -> PyResult<Option<&str>> {
        Ok(self.inner.description())
    }

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    pub fn labels(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.labels().iter().map(AsRef::as_ref).collect())
    }

    /// Returns the underlying graph.
    ///
    /// # Returns
    ///
    /// A reference to the graph.
    ///
    pub fn graph(&self) -> PyResult<PyDiGraph> {
        Ok(self.inner.graph().clone().into())
    }

    /// Returns the a map labels-distributions.
    ///
    /// # Returns
    ///
    /// A reference to the CPDs.
    ///
    pub fn cpds(&self) -> PyResult<BTreeMap<&str, PyGaussCPD>> {
        Ok(self
            .inner
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CPD to a PyGaussCPD.
                let cpd = cpd.clone().into();
                // Return the label and CPD as a tuple.
                (label, cpd)
            })
            .collect())
    }

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    pub fn parameters_size(&self) -> PyResult<usize> {
        Ok(self.inner.parameters_size())
    }

    /// Fit the model to a dataset and a given graph.
    ///
    /// # Arguments
    ///
    /// * `dataset` - The dataset to fit the model to.
    /// * `graph` - The graph to fit the model to.
    /// * `method` - The method to use for fitting (default is `mle`).
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel fitting (default is `true`).
    ///
    /// # Returns
    ///
    /// A new fitted model.
    ///
    #[classmethod]
    #[pyo3(signature = (dataset, graph, method="mle", seed=31, parallel=true))]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: &Bound<'_, PyGaussTable>,
        graph: &Bound<'_, PyDiGraph>,
        method: &str,
        seed: u64,
        parallel: bool,
    ) -> PyResult<Self> {
        todo!() // FIXME:
    }

    /// Generate samples from the model.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of samples to generate.
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel sampling (default is `true`).
    ///
    /// # Returns
    ///
    /// A new dataset containing the samples.
    ///
    #[pyo3(signature = (n, seed=31, parallel=true))]
    pub fn sample(
        &self,
        py: Python<'_>,
        n: usize,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussTable> {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &self.inner);
        // Sample from the model.
        let dataset = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || sampler.par_sample_n(n))
        } else {
            // Sample sequentially.
            sampler.sample_n(n)
        };
        // Return the dataset.
        Ok(dataset.into())
    }

    /// Estimate a conditional probability distribution.
    ///
    /// # Arguments
    ///
    /// * `x` - A variable or an iterable of variables.
    /// * `z` - A conditioning variable or an iterable of conditioning variables.
    /// * `method` - The method to use for estimation (default is `approximate`).
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel estimation (default is `true`).
    ///
    /// # Returns
    ///
    /// A new conditional probability distribution.
    ///
    #[pyo3(signature = (x, z, method="approximate", seed=31, parallel=true))]
    pub fn estimate(
        &self,
        x: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        method: &str,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussCPD> {
        todo!() // FIXME:
    }

    /// Estimate a conditional causal effect (CACE).
    ///
    /// # Arguments
    ///
    /// * `x` - An intervention variable or an iterable of intervention variables.
    /// * `y` - An outcome variable or an iterable of outcome variables.
    /// * `z` - A conditioning variable or an iterable of conditioning variables.
    /// * `method` - The method to use for estimation (default is `approximate`).
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel estimation (default is `true`).
    ///
    /// # Returns
    ///
    /// A new conditional causal effect (CACE) distribution.
    ///
    #[pyo3(signature = (x, y, z, method="approximate", seed=31, parallel=true))]
    pub fn do_estimate(
        &self,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        method: &str,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyGaussCPD> {
        todo!() // FIXME:
    }

    /// Read class from a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: GaussBN::from_json(json),
        })
    }

    /// Write class to a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.inner.to_json())
    }

    /// Read class from a JSON file.
    #[classmethod]
    pub fn read_json(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: GaussBN::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
