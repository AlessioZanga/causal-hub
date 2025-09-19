use crate::types::{States};

/// Utility function to sort states and labels.
pub fn sort_states(mut states: States) -> (States, Vec<(usize, Vec<usize>)>) {
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
