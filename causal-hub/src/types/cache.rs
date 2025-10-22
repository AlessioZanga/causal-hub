use std::sync::{Arc, RwLock};

use dry::macro_for;

use crate::{
    estimators::{CIMEstimator, CPDEstimator},
    models::{CIM, CPD, Labelled},
    types::{Labels, Map, Set},
};

/// A cache for calling a function with a key and value.
#[derive(Clone, Debug)]
pub struct Cache<'a, C, K, V> {
    call: &'a C,
    cache: Arc<RwLock<Map<K, V>>>,
}

impl<'a, E, P> Cache<'a, E, (Vec<usize>, Vec<usize>), P>
where
    P: Clone,
{
    /// Create a new cache.
    ///
    /// # Arguments
    ///
    /// * `call` - The function to call.
    ///
    /// # Returns
    ///
    /// A new cache.
    ///
    #[inline]
    pub fn new(call: &'a E) -> Self {
        // Create a new cache.
        let cache = Arc::new(RwLock::new(Map::default()));

        Self { call, cache }
    }
}

impl<C, K, V> Labelled for Cache<'_, C, K, V>
where
    C: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.call.labels()
    }
}

macro_for!($type in [CPD, CIM] {
    paste::paste! {

        impl<E, P> [<$type Estimator>]<P> for Cache<'_, E, (Vec<usize>, Vec<usize>), P>
        where
            E: [<$type Estimator>]<P>,
            P: $type + Clone,
            P::Statistics: Clone,
        {
            fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> P {
                // Get the key.
                let key: (Vec<_>, Vec<_>) = (
                    x.into_iter().cloned().collect(),
                    z.into_iter().cloned().collect(),
                );
                // Check if the key is in the cache.
                if let Some(value) = self.cache.read().unwrap().get(&key) {
                    // If it is, return the value.
                    return value.clone();
                }
                // If it is not, call the function.
                let value = self.call.fit(x, z);
                // Insert the value into the cache.
                self.cache.write().unwrap().insert(key, value.clone());
                // Return the value.
                value
            }
        }

    }
});
