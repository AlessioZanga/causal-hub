use crate::{datasets::GaussType, models::Labelled, types::Labels};

/// Gaussian evidence type.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum GaussEvT {
    /// Certain positive evidence.
    CertainPositive {
        /// The observed event of the evidence.
        event: usize,
        /// The value of the evidence.
        value: GaussType,
    },
}

impl GaussEvT {
    /// Return the observed event of the evidence.
    ///
    /// # Returns
    ///
    /// The observed event of the evidence.
    ///
    pub const fn event(&self) -> usize {
        match self {
            Self::CertainPositive { event, .. } => *event,
        }
    }
}

/// Gaussian evidence structure.
#[derive(Clone, Debug)]
pub struct GaussEv {
    labels: Labels,
    evidences: Vec<Option<GaussEvT>>,
}

impl Labelled for GaussEv {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussEv {
    /// Create a new Gaussian evidence structure.
    ///
    /// # Arguments
    ///
    /// * `labels` - The labels of the evidence structure.
    /// * `values` - An iterator over the evidence values.
    ///
    /// # Returns
    ///
    /// A new Gaussian evidence structure.
    ///
    pub fn new<I>(mut labels: Labels, values: I) -> Self
    where
        I: IntoIterator<Item = GaussEvT>,
    {
        // Get shortened variable type.
        use GaussEvT as E;

        // Allocate evidences.
        let mut evidences = vec![None; labels.len()];

        // Fill the evidences.
        values.into_iter().for_each(|e| {
            // Get the event of the evidence.
            let event = e.event();
            // Push the value into the variable events.
            evidences[event] = Some(e);
        });

        // Sort labels, if necessary.
        if !labels.is_sorted() {
            // Clone the labels.
            let mut new_labels = labels.clone();
            // Sort the labels.
            new_labels.sort();

            // Create new evidences.
            let mut new_evidences = vec![None; new_labels.len()];

            // Sort the evidences.
            evidences.into_iter().flatten().for_each(|e| {
                // Get the event of the evidence.
                let event = labels
                    .get_index(e.event())
                    .expect("Failed to get label of evidence.");
                // Sort the event index.
                let event = new_labels
                    .get_index_of(event)
                    .expect("Failed to get index of evidence.");

                // Sort the variable events.
                let e = match e {
                    E::CertainPositive { value, .. } => E::CertainPositive { event, value },
                };

                // Push the value into the variable events.
                new_evidences[event] = Some(e);
            });

            // Update the labels.
            labels = new_labels;
            // Update the evidences.
            evidences = new_evidences;
        }

        Self { labels, evidences }
    }

    /// The evidences of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the evidence.
    ///
    #[inline]
    pub const fn evidences(&self) -> &Vec<Option<GaussEvT>> {
        &self.evidences
    }
}
