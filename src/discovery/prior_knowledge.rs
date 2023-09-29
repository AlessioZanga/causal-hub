use itertools::Itertools;

use crate::types::FxIndexSet;

pub trait PriorKnowledge {
    fn forbidden(&self) -> &FxIndexSet<(usize, usize)>;

    fn has_forbidden(&self, x: usize, y: usize) -> bool;

    fn add_forbidden(&mut self, x: usize, y: usize) -> bool;

    fn del_forbidden(&mut self, x: usize, y: usize) -> bool;

    fn required(&self) -> &FxIndexSet<(usize, usize)>;

    fn has_required(&self, x: usize, y: usize) -> bool;

    fn add_required(&mut self, x: usize, y: usize) -> bool;

    fn del_required(&mut self, x: usize, y: usize) -> bool;

    fn labels(&self) -> &FxIndexSet<String>;
}

#[derive(Clone, Debug)]
pub struct ForbiddenRequired {
    forbidden: FxIndexSet<(usize, usize)>,
    required: FxIndexSet<(usize, usize)>,
    labels: FxIndexSet<String>,
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
        let mut labels: FxIndexSet<_> = vertices.into_iter().map_into().collect();
        // Sort labels.
        labels.sort();
        // Map forbidden edges to vertices indices.
        let forbidden: FxIndexSet<_> = forbidden
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    labels
                        .get_index_of(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{x}`")),
                    labels
                        .get_index_of(&y)
                        .unwrap_or_else(|| panic!("No vertex with label `{y}`")),
                )
            })
            .collect();
        // Map required edges to vertices indices.
        let required: FxIndexSet<_> = required
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .map(|(x, y)| {
                (
                    labels
                        .get_index_of(&x)
                        .unwrap_or_else(|| panic!("No vertex with label `{x}`")),
                    labels
                        .get_index_of(&y)
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
    fn forbidden(&self) -> &FxIndexSet<(usize, usize)> {
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
    fn required(&self) -> &FxIndexSet<(usize, usize)> {
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
    fn labels(&self) -> &FxIndexSet<String> {
        &self.labels
    }
}

pub type FR = ForbiddenRequired;
