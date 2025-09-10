use crate::types::{Labels, States};

/// Utility function to collect labels from an iterator.
pub fn collect_labels<I, J>(labels: I) -> Labels
where
    I: IntoIterator<Item = J>,
    J: AsRef<str>,
{
    // Initialize labels counter.
    let mut n = 0;
    // Convert the variable labels to a set of strings.
    let labels: Labels = labels
        .into_iter()
        .inspect(|_| n += 1)
        .map(|x| x.as_ref().to_owned())
        .collect();
    // Assert unique labels.
    assert_eq!(
        labels.len(),
        n,
        "Variable labels must be unique: \n\
        \t expected:    |labels.unique()| == {} , \n\
        \t found:       |labels.unique()| == {} .",
        n,
        labels.len(),
    );

    labels
}

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
