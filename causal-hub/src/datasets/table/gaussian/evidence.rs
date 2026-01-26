use crate::{
    datasets::GaussType,
    models::Labelled,
    types::{Error, Labels, Result},
};

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
    pub fn new<I>(mut labels: Labels, values: I) -> Result<Self>
    where
        I: IntoIterator<Item = GaussEvT>,
    {
        // Get shortened variable type.
        use GaussEvT as E;

        // Fill the evidences.
        let mut evidences = values.into_iter().try_fold(
            vec![None; labels.len()],
            |mut evidences, e| -> Result<_> {
                // Get the event of the evidence.
                let event = e.event();
                // Check if event is in bounds.
                if event >= evidences.len() {
                    return Err(Error::VertexOutOfBounds(event));
                }
                // Push the value into the variable events.
                evidences[event] = Some(e);

                Ok(evidences)
            },
        )?;

        // Sort labels, if necessary.
        if !labels.is_sorted() {
            // Clone the labels.
            let mut new_labels = labels.clone();
            // Sort the labels.
            new_labels.sort();

            // Sort the evidences.
            let new_evidences = evidences.into_iter().flatten().try_fold(
                vec![None; new_labels.len()],
                |mut new_evidences, e| -> Result<_> {
                    // Get the event of the evidence.
                    let event_name = labels
                        .get_index(e.event())
                        .ok_or_else(|| Error::VertexOutOfBounds(e.event()))?;
                    // Sort the event index.
                    let event = new_labels
                        .get_index_of(event_name)
                        .ok_or_else(|| Error::MissingLabel(event_name.clone()))?;

                    // Sort the variable events.
                    let e = match e {
                        E::CertainPositive { value, .. } => E::CertainPositive { event, value },
                    };

                    // Push the value into the variable events.
                    new_evidences[event] = Some(e);

                    Ok(new_evidences)
                },
            )?;

            // Update the labels.
            labels = new_labels;
            // Update the evidences.
            evidences = new_evidences;
        }

        Ok(Self { labels, evidences })
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
