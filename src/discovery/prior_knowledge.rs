use std::collections::{BTreeSet, HashSet};

use bimap::BiHashMap;

/// Prior knowledge trait
pub trait PriorKnowledge: Sync {
    fn forbidden<'a>(&'a self) -> &HashSet<(usize, usize)>;

    fn has_forbidden(&self, x: usize, y: usize) -> bool;

    fn add_forbidden(&mut self, x: usize, y: usize) -> bool;

    fn del_forbidden(&mut self, x: usize, y: usize) -> bool;

    fn required<'a>(&'a self) -> &HashSet<(usize, usize)>;

    fn has_required(&self, x: usize, y: usize) -> bool;

    fn add_required(&mut self, x: usize, y: usize) -> bool;

    fn del_required(&mut self, x: usize, y: usize) -> bool;

    fn labels(&self) -> &BTreeSet<String>;
}

#[derive(Clone, Debug)]
pub struct ForbiddenRequired {
    forbidden: HashSet<(usize, usize)>,
    required: HashSet<(usize, usize)>,
    labels: BTreeSet<String>,
}

impl ForbiddenRequired {
    pub fn new<V, I, J, K>(vertices: I, forbidden: J, required: K) -> Self
    where
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>,
        K: IntoIterator<Item = (V, V)>,
        V: Into<String>,
    {
        // Remove duplicated vertices labels.
        let labels: BTreeSet<_> = vertices.into_iter().map(|x| x.into()).collect();
        // Map vertices labels to vertices indices.
        let labels_indices: BiHashMap<_, _> = labels
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect();
        // Map forbidden edges to vertices indices.
        let forbidden = forbidden
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    *labels_indices
                        .get_by_left(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{}`", x)),
                    *labels_indices
                        .get_by_left(&y)
                        .unwrap_or_else(|| panic!("No vertex with label `{}`", y)),
                )
            })
            .collect();
        // Map required edges to vertices indices.
        let required = required
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    *labels_indices
                        .get_by_left(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{}`", x)),
                    *labels_indices
                        .get_by_left(&y)
                        .unwrap_or_else(|| panic!("No vertex with label `{}`", y)),
                )
            })
            .collect();

        // FIXME: Check forbidden and required consistency.

        Self {
            forbidden,
            required,
            labels,
        }
    }
}

impl PriorKnowledge for ForbiddenRequired {
    #[inline]
    fn forbidden<'a>(&'a self) -> &HashSet<(usize, usize)> {
        &self.forbidden
    }

    #[inline]
    fn has_forbidden(&self, x: usize, y: usize) -> bool {
        self.forbidden.contains(&(x, y))
    }

    #[inline]
    fn add_forbidden(&mut self, x: usize, y: usize) -> bool {
        // FIXME: Check forbidden and required consistency.
        self.forbidden.insert((x, y))
    }

    #[inline]
    fn del_forbidden(&mut self, x: usize, y: usize) -> bool {
        self.forbidden.remove(&(x, y))
    }

    #[inline]
    fn required<'a>(&'a self) -> &HashSet<(usize, usize)> {
        &self.required
    }

    #[inline]
    fn has_required(&self, x: usize, y: usize) -> bool {
        self.required.contains(&(x, y))
    }

    #[inline]
    fn add_required(&mut self, x: usize, y: usize) -> bool {
        // FIXME: Check forbidden and required consistency.
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

pub type FR = ForbiddenRequired;
