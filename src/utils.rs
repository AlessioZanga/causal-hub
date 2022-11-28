/// Partial compares sets.
pub(crate) fn partial_cmp_sets<T: Ord>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> Option<Ordering> {
    if a.eq(b) {
        Some(Ordering::Equal)
    } else if a.is_subset(b) {
        Some(Ordering::Less)
    } else if a.is_superset(b) {
        Some(Ordering::Greater)
    } else {
        None
    }
}

use std::{cmp::Ordering, collections::BTreeSet};
