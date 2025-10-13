/// Macro to implement `From`, `Into` and lock traits for a outer-inner pair.
#[macro_export]
macro_rules! impl_from_into_lock {
    ($outer:ty, $inner:ty) => {
        impl $outer {
            /// Returns a read lock on the inner field.
            #[inline]
            pub fn lock<'a>(&'a self) -> std::sync::RwLockReadGuard<'a, $inner> {
                self.inner.read().expect("Failed to acquire read lock.")
            }

            /// Returns a write lock on the inner field.
            #[inline]
            pub fn lock_mut<'a>(&'a mut self) -> std::sync::RwLockWriteGuard<'a, $inner> {
                self.inner.write().expect("Failed to acquire write lock.")
            }
        }

        impl From<$inner> for $outer {
            fn from(inner: $inner) -> Self {
                Self {
                    inner: std::sync::Arc::new(std::sync::RwLock::new(inner)),
                }
            }
        }

        impl From<$outer> for $inner {
            fn from(outer: $outer) -> Self {
                (&*outer.lock()).clone()
            }
        }
    };
}
