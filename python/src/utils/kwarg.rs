use pyo3::{exceptions::PyTypeError, prelude::*, types::PyDict};

/// A macro to extract an optional keyword argument from an `Option<&Bound<'_, PyDict>>` type,
/// propagating extraction errors as `PyResult`.
///
/// Extraction errors surface as native Python exceptions (e.g., `TypeError`),
/// consistent with the `?`-on-`extract` convention, rather than being wrapped
/// into the custom `causal_hub.Error` exception (see `crate::error::to_pyerr`).
///
/// Consumed keys are removed from the dictionary, so that any leftover key
/// can be rejected as unknown (see [`ensure_kwargs_consumed`]).
///
#[macro_export]
macro_rules! kwarg {
    ($kwargs:ident, $key:expr, $type:ty) => {
        match $kwargs.and_then(|kwargs| kwargs.get_item($key).ok().flatten()) {
            // The keyword argument is not present.
            None => Ok(None),
            // An explicit `None` maps to `None`; any other value must have the expected type.
            Some(value) => {
                // Mark the key as consumed.
                if let Some(kwargs) = $kwargs {
                    ::pyo3::PyResult::map(kwargs.del_item($key), |_| {})?;
                }
                value
                    .extract::<Option<$type>>()
                    .map_err(::pyo3::PyErr::from)
            }
        }
    };
}

/// Rejects any keyword argument that was not consumed by [`kwarg`].
///
/// This makes stale or misspelled keyword arguments fail fast with a
/// `TypeError`, instead of being silently ignored.
///
pub fn ensure_kwargs_consumed(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<()> {
    // Get the dictionary, if any.
    let Some(kwargs) = kwargs else {
        return Ok(());
    };
    // Collect the names of the leftover (unknown) keyword arguments.
    let unknown: Vec<String> = kwargs
        .keys()
        .iter()
        .map(|key| key.extract())
        .collect::<PyResult<_>>()?;
    // Match on the leftover keyword arguments.
    match unknown.as_slice() {
        [] => Ok(()),
        [name] => Err(PyTypeError::new_err(format!(
            "Unexpected keyword argument '{name}'"
        ))),
        names => Err(PyTypeError::new_err(format!(
            "Unexpected keyword arguments: {}",
            names.join(", ")
        ))),
    }
}
