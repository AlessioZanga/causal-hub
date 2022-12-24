use std::{cmp::Ordering, collections::BTreeSet};

/// Checks if iterator is sorted.
pub fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

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
