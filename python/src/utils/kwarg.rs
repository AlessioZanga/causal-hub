/// A macro to extract an optional keyword argument from an `Option<&Bound<'_, PyDict>>` type,
/// propagating extraction errors as `PyResult`.
///
/// Extraction errors surface as native Python exceptions (e.g., `TypeError`),
/// consistent with the `?`-on-`extract` convention, rather than being wrapped
/// into the custom `causal_hub.Error` exception (see `crate::error::to_pyerr`).
#[macro_export]
macro_rules! kwarg {
    ($kwargs:ident, $key:expr, $type:ty) => {
        match $kwargs.and_then(|kwargs| kwargs.get_item($key).ok().flatten()) {
            // The keyword argument is not present.
            None => Ok(None),
            // An explicit `None` maps to `None`; any other value must have the expected type.
            Some(value) => value
                .extract::<Option<$type>>()
                .map_err(::pyo3::PyErr::from),
        }
    };
}
