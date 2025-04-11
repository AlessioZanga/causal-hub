use ndarray::prelude::*;

use crate::types::{FxIndexMap, FxIndexSet};

/// A multivariate trajectory.
pub struct Trajectory {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    transitions: Array1<(usize, usize, f64)>,
    // TODO: ending_time: f64
}

impl Trajectory {
    /// Construct a new trajectory.
    ///
    /// # Arguments
    ///
    /// * `states` - A collection of states, where each state is a tuple of a label and a collection of states.
    /// * `transitions` - A collection of transitions, where each transition is a tuple of:
    ///     - `x` - The variable that is transitioning.
    ///     - `y` - The state that is transitioning to.
    ///     - `t` - The time of the transition.
    ///
    /// # Panics
    ///
    /// Panics when:
    ///
    /// * The transitions are not sorted by increasing time.
    ///
    /// # Returns
    ///
    /// A new trajectory.
    ///
    pub fn new<I, J, K, L, M>(states: I, transitions: K) -> Self
    where
        I: IntoIterator<Item = (L, J)>,
        J: IntoIterator<Item = M>,
        K: IntoIterator<Item = (usize, usize, f64)>,
        L: Into<String>,
        M: Into<String>,
    {
        // Collect the states.
        let states: FxIndexMap<_, FxIndexSet<_>> = states
            .into_iter()
            .map(|(label, states)| (label.into(), states.into_iter().map(|s| s.into()).collect()))
            .collect();
        // Collect the variables labels.
        let labels: FxIndexSet<_> = states.keys().cloned().collect();
        // Collect the transitions.
        let mut transitions: Vec<_> = transitions.into_iter().collect();
        // Sort transitions by time.
        transitions.sort_by(|(_, _, t_a), (_, _, t_b)| {
            t_a.partial_cmp(t_b)
                .unwrap_or_else(|| panic!("Failed to sort transitions times `{t_a}` and `{t_b}`."))
        });
        // Cast to array.
        let transitions = Array::from_vec(transitions);

        // TODO: Debug assert sorted labels, states and transitions (by time and value).
        debug_assert!(
            transitions.iter().map(|(_, _, t)| t).is_sorted(),
            "Transitions must be sorted by increasing time."
        );

        Trajectory {
            labels,
            states,
            transitions,
        }
    }

    /// Returns the labels of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the labels of the trajectory.
    ///
    #[inline]
    pub const fn labels(&self) -> &FxIndexSet<String> {
        &self.labels
    }

    /// Returns the states of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectory.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the transitions of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the transitions of the trajectory.
    ///
    #[inline]
    pub const fn transitions(&self) -> &Array1<(usize, usize, f64)> {
        &self.transitions
    }

    /// Returns the times of the trajectory.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * The trajectory does not contain at least one transition for each variable.
    ///
    /// # Returns
    ///
    /// A reference to the times of the trajectory.
    ///
    pub fn initial_state(&self) -> Vec<usize> {
        // Initialize the counter of the observed variables.
        let mut counter = 0;
        // Allocate the initial state.
        let mut initial_state = vec![Option::<usize>::None; self.labels.len()];

        // Iterate over the transition and stop when all variables are observed at least once.
        self.transitions.iter().any(|&(x, y, _)| {
            // If variable has not been observed before ...
            if initial_state[x].is_none() {
                // Set the state of the observed variable.
                initial_state[x] = Some(y);
                // Increment observed variables counter.
                counter += 1;
            }
            // Check if we observed all the variables.
            self.labels.len().eq(&counter)
        });

        initial_state
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                x.unwrap_or_else(|| {
                    panic!("Failed to get initial state for variable `{i}`.");
                })
            })
            .collect()
    }
}

/// A type alias for a multivariate trajectory.
pub type Trj = Trajectory;

/* FIXME:
pub fn sufficient_statistics(&self) -> (Array1<f64>, Array2<usize>) {
    // Get the trajectory components.
    let (n, s_, t_, tn) = (
        self.states().len(),
        self.states(),
        self.times(),
        self.ending_time(),
    );

    // Compute the number of states from the trajectory.
    let c = s_.iter().max().unwrap() + 1;

    // Initialize the total time spent in each state.
    let mut t = Array1::<f64>::zeros(c);
    // Compute the total time spent in each state.
    t_.windows(2).into_iter().enumerate().for_each(|(i, ti)| {
        let si = s_[i];
        let dt = ti[1] - ti[0];
        t[si] += dt;
    });
    // Sum the ending time.
    t[s_[n - 1]] += tn - t_[n - 1];

    // Initialize the sum of the log-rates of the transitions.
    let mut n = Array2::<usize>::zeros((c, c));
    // Compute the sum of the log-rates of the transitions.
    s_.windows(2).into_iter().for_each(|si| {
        n[(si[0], si[1])] += 1;
    });

    (t, n)
}
*/
