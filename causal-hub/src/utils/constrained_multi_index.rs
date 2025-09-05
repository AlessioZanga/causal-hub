use itertools::Itertools;
use ndarray::prelude::*;

use crate::types::Map;

/// CMI (Constrained Multi-dimensional Indexer) struct.
/// Stores shape, strides, and precomputed rules for the Principle of Inclusion-Exclusion.
pub struct CMI {
    shape: Array1<usize>,
    strides: Array1<usize>,
    constraints: Vec<Vec<Option<usize>>>,
    inclusion_exclusion: Vec<(Vec<Option<usize>>, i8)>,
    total_valid: usize,
}

impl CMI {
    /// Creates a new CMI instance.
    pub fn new(shape: Array1<usize>, constraints: Vec<Map<usize, usize>>) -> Self {
        // Convert constraints to Vec<Vec<Option<usize>>>
        let constraints: Vec<_> = constraints
            .into_iter()
            .map(|m| {
                let mut rule = vec![None; shape.len()];
                for (&axis, &val) in m.iter() {
                    rule[axis] = Some(val);
                }
                rule
            })
            .collect();

        let mut strides = Array1::ones(shape.len());
        for i in (0..shape.len() - 1).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }

        fn merge_constraints(rules: &[&Vec<Option<usize>>]) -> Option<Vec<Option<usize>>> {
            if rules.is_empty() {
                return None;
            }
            let mut merged = rules[0].clone();
            for rule in rules.iter().skip(1) {
                for (m, r) in merged.iter_mut().zip(rule.iter()) {
                    if let (Some(m_val), Some(r_val)) = (*m, *r) {
                        if m_val != r_val {
                            return None;
                        }
                    } else if r.is_some() {
                        *m = *r;
                    }
                }
            }
            Some(merged)
        }

        let mut inclusion_exclusion = Vec::new();
        for i in 1..=constraints.len() {
            let sign: i8 = if (i - 1) % 2 == 0 { 1 } else { -1 };
            for combo in constraints.iter().combinations(i) {
                if let Some(merged_rule) = merge_constraints(&combo) {
                    inclusion_exclusion.push((merged_rule, sign));
                }
            }
        }

        fn compute_total_skipped(
            inclusion_exclusion: &[(Vec<Option<usize>>, i8)],
            shape: &Array1<usize>,
        ) -> usize {
            let mut total_skipped: i64 = 0;
            for (rule, sign) in inclusion_exclusion {
                let matches = rule
                    .iter()
                    .zip(shape)
                    .map(|(r, s)| if r.is_none() { *s } else { 1 })
                    .product::<usize>();
                total_skipped += (matches as i64) * (*sign as i64);
            }
            total_skipped as usize
        }

        let total_multi_index = shape.product();
        let total_skipped = compute_total_skipped(&inclusion_exclusion, &shape);
        let total_valid = total_multi_index - total_skipped;

        CMI {
            shape,
            strides,
            constraints,
            inclusion_exclusion,
            total_valid,
        }
    }

    /// Return the number of dimensions.
    ///
    /// # Returns
    ///
    /// The number of dimensions.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.shape
    }

    pub fn ravel<I>(&self, multi_index: I) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        let multi_index: Vec<usize> = multi_index.into_iter().collect();
        assert_eq!(multi_index.len(), self.shape.len());
        for (&m, &c) in multi_index.iter().zip(&self.shape) {
            assert!(m < c, "Index `{multi_index:?}` out of bounds.");
        }
        if self.is_skipped(&multi_index) {
            panic!("Index `{multi_index:?}` not allowed.");
        }
        self.rank_of_multi_index(&multi_index)
    }

    pub fn unravel(&self, index: usize) -> Vec<usize> {
        if index >= self.total_valid {
            panic!("Index `{index}` out of bounds.");
        }

        let mut low = 0;
        let mut high = self.shape.product();
        let mut multi_index = vec![];

        while low < high {
            let mid_flat = low + (high - low) / 2;
            let mid_multi_index = self.unravel_flat(mid_flat);
            let mut rank = self.rank_of_multi_index(&mid_multi_index);
            if self.is_skipped(&mid_multi_index) {
                rank = rank.saturating_sub(1);
            }
            if rank < index {
                low = mid_flat + 1;
            } else {
                multi_index = mid_multi_index;
                high = mid_flat;
            }
        }

        multi_index
    }

    fn rank_of_multi_index(&self, multi_index: &[usize]) -> usize {
        let index = multi_index
            .iter()
            .zip(&self.strides)
            .map(|(c, s)| c * s)
            .sum::<usize>();
        let skipped = self.compute_skipped_before(multi_index);
        index - skipped
    }

    fn is_skipped(&self, multi_index: &[usize]) -> bool {
        self.constraints.iter().any(|rule| {
            rule.iter()
                .zip(multi_index)
                .all(|(&r, &c)| r.is_none() || r == Some(c))
        })
    }

    fn unravel_flat(&self, mut index: usize) -> Vec<usize> {
        self.strides
            .iter()
            .map(|stride| {
                let offset = index / stride;
                index %= stride;
                offset
            })
            .collect()
    }

    fn compute_skipped_before(&self, multi_index: &[usize]) -> usize {
        let mut skipped: i64 = 0;
        for (rule, sign) in &self.inclusion_exclusion {
            let matches = self.count_matches_before(multi_index, rule);
            skipped += (matches as i64) * (*sign as i64);
        }
        skipped as usize
    }

    fn count_matches_before(&self, multi_index: &[usize], rule: &[Option<usize>]) -> usize {
        let mut count = 0;
        let mut prefix = Vec::new();
        for (i, &target_val) in multi_index.iter().enumerate() {
            for val in 0..target_val {
                let current_prefix = {
                    let mut t = prefix.clone();
                    t.push(val);
                    t
                };
                if current_prefix
                    .iter()
                    .zip(rule)
                    .all(|(&p, &r)| r.is_none() || r == Some(p))
                {
                    count += (i + 1..self.shape.len())
                        .map(|k| if rule[k].is_none() { self.shape[k] } else { 1 })
                        .product::<usize>();
                }
            }
            prefix.push(target_val);
            if !prefix
                .iter()
                .zip(rule)
                .all(|(&p, &r)| r.is_none() || r == Some(p))
            {
                break;
            }
        }
        count
    }
}
