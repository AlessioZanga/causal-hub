use ndarray::prelude::*;

use super::CategoricalTrjEvT;
use crate::types::{FxIndexMap, FxIndexSet};

#[derive(Debug, Clone)]
pub enum CategoricalEvidenceType {
    CertainPositive { state: usize },
    CertainNegative { not_states: FxIndexSet<usize> },
    UncertainPositive { p_states: Array1<f64> },
    UncertainNegative { p_not_states: Array1<f64> },
}

pub type CategoricalEvT = CategoricalEvidenceType;

impl From<CategoricalTrjEvT> for CategoricalEvidenceType {
    fn from(evidence: CategoricalTrjEvT) -> Self {
        // Get shortened variable types.
        use CategoricalEvT as U;
        use CategoricalTrjEvT as T;
        // Match the evidence type discard the temporal information.
        match evidence {
            T::CertainPositiveInterval { state, .. } => U::CertainPositive { state },
            T::CertainNegativeInterval { not_states, .. } => U::CertainNegative { not_states },
            T::UncertainPositiveInterval { p_states, .. } => U::UncertainPositive { p_states },
            T::UncertainNegativeInterval { p_not_states, .. } => {
                U::UncertainNegative { p_not_states }
            }
        }
    }
}

pub struct CategoricalEvidence {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    evidences: FxIndexMap<String, Option<CategoricalEvT>>,
}

pub type CategoricalEv = CategoricalEvidence;

impl CategoricalEvidence {
    pub fn new<I, J, K, L, M, N>(states: I, values: M) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = L>,
        K: AsRef<str>,
        L: AsRef<str>,
        M: IntoIterator<Item = (N, CategoricalEvT)>,
        N: AsRef<str>,
    {
        todo!() // FIXME:
    }

    pub fn labels(&self) -> &FxIndexSet<String> {
        &self.labels
    }

    pub fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    pub fn evidences(&self) -> &FxIndexMap<String, Option<CategoricalEvT>> {
        &self.evidences
    }
}
