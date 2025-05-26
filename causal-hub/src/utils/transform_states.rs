use crate::types::{FxIndexMap, FxIndexSet};

/// Utility function to collect states from an iterator.
pub fn collect_states<I, J, K, V>(states: I) -> FxIndexMap<String, FxIndexSet<String>>
where
    I: IntoIterator<Item = (K, J)>,
    J: IntoIterator<Item = V>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    // Initialize variables counter.
    let mut n = 0;
    // Get the states of the variables.
    let states: FxIndexMap<_, _> = states
        .into_iter()
        .inspect(|_| n += 1)
        .map(|(label, states)| {
            // Convert the variable label to a string.
            let label = label.as_ref().to_owned();

            // Initialize states counter.
            let mut n = 0;
            // Convert the variable states to a set of strings.
            let states: FxIndexSet<_> = states
                .into_iter()
                .inspect(|_| n += 1)
                .map(|x| x.as_ref().to_owned())
                .collect();
            // Assert unique states.
            assert_eq!(states.len(), n, "Variables states must be unique.");

            (label, states)
        })
        .collect();

    // Assert unique labels.
    assert_eq!(states.len(), n, "Variables labels must be unique.");

    states
}

/// Utility function to sort states and labels.
pub fn sort_states(
    mut states: FxIndexMap<String, FxIndexSet<String>>,
) -> (
    FxIndexMap<String, FxIndexSet<String>>,
    Vec<(usize, Vec<usize>)>,
) {
    // Get the indices to sort the labels and states labels.
    let mut sorted_indices: Vec<(_, Vec<_>)> = states
        .values()
        .enumerate()
        .map(|(label_idx, states)| {
            // Allocate the indices of the states labels.
            let mut sort_indices: Vec<_> = (0..states.len()).collect();
            // Sort the indices by the states labels.
            sort_indices.sort_by_key(|&i| &states[i]);

            (label_idx, sort_indices)
        })
        .collect();
    // Sort the indices by the states labels.
    sorted_indices.sort_by_key(|&(i, _)| states.get_index(i).unwrap().0);
    // Sort the states labels.
    states.values_mut().for_each(|states| states.sort());
    // Sort the labels.
    states.sort_keys();

    (states, sorted_indices)
}
