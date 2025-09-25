/// A macro to extract a keyword argument from an `Option<&Bound<'_, PyDict>>` type.
#[macro_export]
macro_rules! kwarg {
    ($kwargs:ident, $key:expr, $type:ty) => {
        $kwargs.and_then(|kwargs| {
            kwargs
                .get_item($key)
                .ok()
                .flatten()
                .and_then(|v| v.extract::<$type>().ok())
        })
    };
}
