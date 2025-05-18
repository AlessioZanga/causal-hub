use std::sync::{Arc, RwLock};

use super::FxIndexMap;
use crate::estimators::CPDEstimator;

/// A cache for calling a function with a key and value.
#[derive(Clone, Debug)]
pub struct Cache<'a, C, K, V> {
    call: &'a C,
    cache: Arc<RwLock<FxIndexMap<K, V>>>,
}

impl<'a, E, P> Cache<'a, E, (usize, Vec<usize>), (E::SS, P)>
where
    E: CPDEstimator<P>,
    E::SS: Clone,
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
        let cache = Arc::new(RwLock::new(FxIndexMap::default()));

        Self { call, cache }
    }
}

impl<E, P> CPDEstimator<P> for Cache<'_, E, (usize, Vec<usize>), (E::SS, P)>
where
    E: CPDEstimator<P>,
    E::SS: Clone,
    P: Clone,
{
    type SS = E::SS;

    fn fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, P) {
        // Get the key.
        let key = (x, z.to_vec());
        // Check if the key is in the cache.
        if let Some(value) = self.cache.read().unwrap().get(&key) {
            // If it is, return the value.
            return value.clone();
        }
        // If it is not, call the function.
        let value = self.call.fit_transform(x, z);
        // Insert the value into the cache.
        self.cache.write().unwrap().insert(key, value.clone());
        // Return the value.
        value
    }
}
