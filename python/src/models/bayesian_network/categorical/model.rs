use std::collections::BTreeMap;

use backend::{
    datasets::CatTable,
    estimation::{BE, BNEstimator, MLE, ParBNEstimator},
    inference::{
        ApproximateInference, BNCausalInference, BNInference, CausalInference,
        ParBNCausalInference, ParBNInference,
    },
    io::{BifIO, JsonIO},
    models::{BN, CatBN, DiGraph, Labelled},
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
    set,
};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyDictMethods, PyType},
};
use pyo3_stub_gen::derive::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    datasets::PyCatTable,
    impl_deref_from_into, indices_from, kwarg,
    models::{PyCatCPD, PyDiGraph},
};

/// A categorical Bayesian network (BN).
#[gen_stub_pyclass]
#[pyclass(name = "CatBN", eq)]
#[derive(Clone, Debug, PartialEq)]
pub struct PyCatBN {
    inner: CatBN,
}

// Implement `Deref`, `From` and `Into` traits.
impl_deref_from_into!(PyCatBN, CatBN);

#[gen_stub_pymethods]
#[pymethods]
impl PyCatBN {
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
            .map(|x| x?.extract::<PyCatCPD>())
            .collect::<PyResult<_>>()?;
        // Convert Vec<PyCatCPD> to Vec<CatCPD>.
        let cpds = cpds.into_iter().map(|x| x.into());
        // Create a new CatBN with the given parameters.
        Ok(CatBN::new(graph, cpds).into())
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
    pub fn cpds(&self) -> PyResult<BTreeMap<&str, PyCatCPD>> {
        Ok(self
            .inner
            .cpds()
            .iter()
            .map(|(label, cpd)| {
                // Convert the label to a string slice.
                let label = label.as_ref();
                // Convert the CPD to a PyCatCPD.
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
    /// * `parallel` - The flag to enable parallel fitting (default is `true`).
    /// * `kwargs` - Optional keyword arguments:
    ///     - `alpha` - The prior of the Bayesian estimator.
    ///
    /// # Returns
    ///
    /// A new fitted model.
    ///
    #[classmethod]
    #[pyo3(signature = (
        dataset,
        graph,
        method="mle",
        parallel=true,
        **kwargs
    ))]
    pub fn fit(
        _cls: &Bound<'_, PyType>,
        py: Python<'_>,
        dataset: &Bound<'_, PyCatTable>,
        graph: &Bound<'_, PyDiGraph>,
        method: &str,
        parallel: bool,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        // Get the dataset and the graph.
        let dataset: CatTable = dataset.extract::<PyCatTable>()?.into();
        let graph: DiGraph = graph.extract::<PyDiGraph>()?.into();
        // Define a trait for the estimator.
        trait PyBNEstimator<T>: BNEstimator<T> + ParBNEstimator<T> + Send {}
        impl<E, T> PyBNEstimator<T> for E where E: BNEstimator<T> + ParBNEstimator<T> + Send {}
        // Initialize the estimator.
        let estimator: Box<dyn PyBNEstimator<CatBN>> = match method {
            // Initialize the maximum likelihood estimator.
            "mle" => Box::new(MLE::new(&dataset)),
            // Initialize the Bayesian estimator.
            "be" => {
                // Initialize the Bayesian estimator.
                let estimator = BE::new(&dataset);
                // Set the prior `alpha`, if any.
                match kwarg!(kwargs, "alpha", usize) {
                    None => Box::new(estimator),
                    Some(alpha) => Box::new(estimator.with_prior(alpha)),
                }
            }
            // Raise an error if the method is unknown.
            method => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Unknown method: '{}', choose one of the following: \n\
                    \t- 'mle' - Maximum likelihood estimator, \n\
                    \t- 'be' - Bayesian estimator.",
                    method
                )));
            }
        };
        // Fit the model.
        let model = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || estimator.par_fit(graph))
        } else {
            // Execute sequentially.
            estimator.fit(graph)
        };
        // Return the fitted model.
        Ok(model.into())
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
    ) -> PyResult<PyCatTable> {
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the sampler.
        let sampler = ForwardSampler::new(&mut rng, &self.inner);
        // Sample from the model.
        let dataset = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || sampler.par_sample_n(n))
        } else {
            // Execute sequentially.
            sampler.sample_n(n)
        };
        // Return the dataset.
        Ok(dataset.into())
    }

    /// Estimate a conditional probability distribution (CPD).
    ///
    /// # Arguments
    ///
    /// * `x` - A variable or an iterable of variables.
    /// * `z` - A conditioning variable or an iterable of conditioning variables.
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel estimation (default is `true`).
    ///
    /// # Returns
    ///
    /// A new conditional probability distribution.
    ///
    #[pyo3(signature = (x, z, seed=31, parallel=true))]
    pub fn estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<PyCatCPD> {
        // Get the set of variables.
        let x = indices_from!(x, self)?;
        let z = indices_from!(z, self)?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let estimator = ApproximateInference::new(&mut rng, &self.inner);
        // Estimate from the model.
        let estimate = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || estimator.par_estimate(&x, &z))
        } else {
            // Execute sequentially.
            estimator.estimate(&x, &z)
        };
        // Return the dataset.
        Ok(estimate.into())
    }

    /// Estimate a conditional causal effect (CACE).
    ///
    /// # Arguments
    ///
    /// * `x` - An intervention variable or an iterable of intervention variables.
    /// * `y` - An outcome variable or an iterable of outcome variables.
    /// * `z` - A conditioning variable or an iterable of conditioning variables.
    /// * `seed` - The seed of the random number generator (default is `31`).
    /// * `parallel` - The flag to enable parallel estimation (default is `true`).
    ///
    /// # Returns
    ///
    /// A new conditional causal effect (CACE) distribution.
    ///
    #[pyo3(signature = (x, y, z, seed=31, parallel=true))]
    pub fn do_estimate(
        &self,
        py: Python<'_>,
        x: &Bound<'_, PyAny>,
        y: &Bound<'_, PyAny>,
        z: &Bound<'_, PyAny>,
        seed: u64,
        parallel: bool,
    ) -> PyResult<Option<PyCatCPD>> {
        // Get the set of variables.
        let x = indices_from!(x, self)?;
        let y = indices_from!(y, self)?;
        let z = indices_from!(z, self)?;
        // Initialize the random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        // Initialize the inference engine.
        let estimator = ApproximateInference::new(&mut rng, &self.inner);
        // Estimate from the model.
        let estimate = if parallel {
            // Release the GIL to allow parallel execution.
            py.detach(move || CausalInference::new(&estimator).par_cace_estimate(&x, &y, &z))
        } else {
            // Execute sequentially.
            CausalInference::new(&estimator).cace_estimate(&x, &y, &z)
        };
        // Return the dataset.
        Ok(estimate.map(|e| e.into()))
    }

    /// Read class from a BIF string.
    #[classmethod]
    pub fn from_bif(_cls: &Bound<'_, PyType>, bif: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatBN::from_bif(bif),
        })
    }

    /// Write class to a BIF string.
    pub fn to_bif(&self) -> PyResult<String> {
        Ok(self.inner.to_bif())
    }

    /// Read class from a BIF file.
    #[classmethod]
    pub fn read_bif(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatBN::read_bif(path),
        })
    }

    /// Write class to a BIF file.
    pub fn write_bif(&self, path: &str) -> PyResult<()> {
        self.inner.write_bif(path);
        Ok(())
    }

    /// Read class from a JSON string.
    #[classmethod]
    pub fn from_json(_cls: &Bound<'_, PyType>, json: &str) -> PyResult<Self> {
        Ok(Self {
            inner: CatBN::from_json(json),
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
            inner: CatBN::read_json(path),
        })
    }

    /// Write class to a JSON file.
    pub fn write_json(&self, path: &str) -> PyResult<()> {
        self.inner.write_json(path);
        Ok(())
    }
}
