/// Macro to implement `Deref`, `From` and `Into` traits for a outer-inner pair.
#[macro_export]
macro_rules! impl_deref_from_into {
    ($outer:ty, $inner:ty) => {
        impl std::ops::Deref for $outer {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl From<$inner> for $outer {
            fn from(inner: $inner) -> Self {
                Self {
                    inner: std::sync::Arc::new(inner),
                }
            }
        }

        impl From<$outer> for $inner {
            fn from(outer: $outer) -> Self {
                outer.inner.as_ref().clone()
            }
        }
    };
}
