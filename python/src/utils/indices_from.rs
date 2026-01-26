/// A macro to extract indices from a string or an iterable of strings.
#[macro_export]
macro_rules! indices_from {
    ($x:expr, $labels:expr) => {
        if let Ok(x) = $x.extract::<String>() {
            let i = $labels
                .label_to_index(&x)
                .map_err(|e| $crate::error::Error::new_err(e.to_string()))?;
            Ok(backend::set![i])
        } else if let Ok(x) = $x.try_iter() {
            x.map(|x| {
                let x = x?.extract::<String>()?;
                $labels
                    .label_to_index(&x)
                    .map_err(|e| $crate::error::Error::new_err(e.to_string()))
            })
            .collect::<PyResult<_>>()
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected a string or an iterable of strings.",
            ))
        }
    };
}
