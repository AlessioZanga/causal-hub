mod categorical;
pub use categorical::*;

mod gaussian;
pub use gaussian::*;

mod missing;
pub use missing::*;
use pyo3::prelude::*;
use pyo3_stub_gen_derive::gen_stub_pyclass_enum;

/// A tabular dataset.
#[gen_stub_pyclass_enum]
#[pyclass(name = "Dataset", module = "causal_hub.datasets", skip_from_py_object)]
#[derive(Clone, Debug, FromPyObject)]
pub enum PyDataset {
    /// A categorical tabular dataset.
    Categorical(PyCatTable),
    /// A categorical incomplete tabular dataset.
    CategoricalIncomplete(PyCatIncTable),
    /// A Gaussian tabular dataset.
    Gaussian(PyGaussTable),
    /// A Gaussian incomplete tabular dataset.
    GaussianIncomplete(PyGaussIncTable),
}
