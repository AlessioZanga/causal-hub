use std::collections::{BTreeSet, HashSet};

use bimap::BiHashMap;
use itertools::Itertools;

/// Prior knowledge trait
pub trait PriorKnowledge: Sync {
    /// Get the set of forbidden edges.
    fn forbidden(&self) -> &HashSet<(usize, usize)>;

    /// Checks if edge is forbidden.
    fn has_forbidden(&self, x: usize, y: usize) -> bool;

    /// Add edge to the forbidden set.
    fn add_forbidden(&mut self, x: usize, y: usize) -> bool;

    /// Delete edge from the forbidden set.
    fn del_forbidden(&mut self, x: usize, y: usize) -> bool;

    /// Get the set of required edges.
    fn required(&self) -> &HashSet<(usize, usize)>;

    /// Checks if edge is required.
    fn has_required(&self, x: usize, y: usize) -> bool;

    /// Add edge to the required set.
    fn add_required(&mut self, x: usize, y: usize) -> bool;

    /// Delete edge from the required set.
    fn del_required(&mut self, x: usize, y: usize) -> bool;

    /// Get the set of varibles labels.
    fn labels(&self) -> &BTreeSet<String>;
}

/// Forbidden and required sets.
#[derive(Clone, Debug)]
pub struct ForbiddenRequired {
    forbidden: HashSet<(usize, usize)>,
    required: HashSet<(usize, usize)>,
    labels: BTreeSet<String>,
}

impl ForbiddenRequired {
    /// Constructor for the forbidden and required prior knowledge sets.
    pub fn new<V, I, J, K>(vertices: I, forbidden: J, required: K) -> Self
    where
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>,
        V: Into<String>,
    {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = vertices.into_iter().map_into().collect();
        // Map vertices labels to vertices indices.
        let labels_indices: BiHashMap<_, _> = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        // Map forbidden edges to vertices indices.
        let forbidden: HashSet<_> = forbidden
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    *labels_indices
                        .get_by_left(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{x}`")),
                    *labels_indices
                        .get_by_left(&y)
                        .unwrap_or_else(|| panic!("No vertex with label `{y}`")),
                )
            })
            .collect();
        // Map required edges to vertices indices.
        let required: HashSet<_> = required
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    *labels_indices
                        .get_by_left(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{x}`")),
                    *labels_indices
                        .get_by_left(&y)
                        .unwrap_or_else(|| panic!("No vertex with label `{y}`")),
                )
            })
            .collect();

        // Check forbidden and required consistency.
        assert!(
            forbidden.is_disjoint(&required),
            "Forbidden and required sets must be disjoint"
        );

        Self {
            forbidden,
            required,
            labels,
        }
    }
}

impl PriorKnowledge for ForbiddenRequired {
    #[inline]
    fn forbidden(&self) -> &HashSet<(usize, usize)> {
        &self.forbidden
    }

    #[inline]
    fn has_forbidden(&self, x: usize, y: usize) -> bool {
        self.forbidden.contains(&(x, y))
    }

    #[inline]
    fn add_forbidden(&mut self, x: usize, y: usize) -> bool {
        // Check forbidden and required consistency.
        assert!(
            !self.required.contains(&(x, y)),
            "Failed to add edge as forbidden since it is in the required set"
        );

        self.forbidden.insert((x, y))
    }

    #[inline]
    fn del_forbidden(&mut self, x: usize, y: usize) -> bool {
        self.forbidden.remove(&(x, y))
    }

    #[inline]
    fn required(&self) -> &HashSet<(usize, usize)> {
        &self.required
    }

    #[inline]
    fn has_required(&self, x: usize, y: usize) -> bool {
        self.required.contains(&(x, y))
    }

    #[inline]
    fn add_required(&mut self, x: usize, y: usize) -> bool {
        // Check forbidden and required consistency.
        assert!(
            !self.forbidden.contains(&(x, y)),
            "Failed to add edge as required since it is in the forbidden set"
        );

        self.required.insert((x, y))
    }

    #[inline]
    fn del_required(&mut self, x: usize, y: usize) -> bool {
        self.required.remove(&(x, y))
    }

    #[inline]
    fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }
}

/// Alias for the forbidden and required sets.
pub type FR = ForbiddenRequired;
