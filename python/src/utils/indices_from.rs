/// A macro to extract indices from a string or an iterable of strings.
#[macro_export]
macro_rules! indices_from {
    ($x:expr, $labels:expr) => {
        if let Ok(x) = $x.extract::<String>() {
            Ok(backend::set![$labels.label_to_index(&x)])
        } else if let Ok(x) = $x.try_iter() {
            x.map(|x| x?.extract::<String>().map(|x| $labels.label_to_index(&x)))
                .collect::<PyResult<_>>()
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected a string or an iterable of strings.",
            ))
        }
    };
}
