use std::borrow::Cow;

use approx::{AbsDiffEq, RelativeEq};

use crate::{
    datasets::{
        CatEv, CatIncTable, CatTable, CatWtdTable, GaussEv, GaussIncTable, GaussTable,
        GaussWtdTable,
    },
    inference::TopologicalOrder,
    models::{BN, CPD, DiGraph, Graph, Labelled, MixedCPD, MixedSample, MixedSupport},
    set,
    types::{Error, Labels, Map, Result, Set},
};

/// A unified evidence type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MixedEv {
    /// Categorical evidence.
    Categorical(CatEv),
    /// Gaussian evidence.
    Gaussian(GaussEv),
}

/// A unified complete dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MixedTable {
    /// Categorical table.
    Categorical(CatTable),
    /// Gaussian table.
    Gaussian(GaussTable),
}

/// A unified incomplete dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MixedIncTable {
    /// Categorical incomplete table.
    Categorical(CatIncTable),
    /// Gaussian incomplete table.
    Gaussian(GaussIncTable),
}

/// A unified weighted dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MixedWtdTable {
    /// Categorical weighted table.
    Categorical(CatWtdTable),
    /// Gaussian weighted table.
    Gaussian(GaussWtdTable),
}

impl From<CatEv> for MixedEv {
    #[inline]
    fn from(ev: CatEv) -> Self {
        Self::Categorical(ev)
    }
}

impl From<GaussEv> for MixedEv {
    #[inline]
    fn from(ev: GaussEv) -> Self {
        Self::Gaussian(ev)
    }
}

impl From<CatTable> for MixedTable {
    #[inline]
    fn from(table: CatTable) -> Self {
        Self::Categorical(table)
    }
}

impl From<GaussTable> for MixedTable {
    #[inline]
    fn from(table: GaussTable) -> Self {
        Self::Gaussian(table)
    }
}

impl From<CatIncTable> for MixedIncTable {
    #[inline]
    fn from(table: CatIncTable) -> Self {
        Self::Categorical(table)
    }
}

impl From<GaussIncTable> for MixedIncTable {
    #[inline]
    fn from(table: GaussIncTable) -> Self {
        Self::Gaussian(table)
    }
}

impl From<CatWtdTable> for MixedWtdTable {
    #[inline]
    fn from(table: CatWtdTable) -> Self {
        Self::Categorical(table)
    }
}

impl From<GaussWtdTable> for MixedWtdTable {
    #[inline]
    fn from(table: GaussWtdTable) -> Self {
        Self::Gaussian(table)
    }
}

/// A mixed Bayesian network.
#[derive(Clone, Debug)]
pub struct MixedBN {
    /// The name of the model.
    name: Option<String>,
    /// The description of the model.
    description: Option<String>,
    /// The labels of the variables.
    labels: Labels,
    /// The graph of the model.
    graph: DiGraph,
    /// The parameters of the model.
    cpds: Map<String, MixedCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

impl PartialEq for MixedBN {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self.cpds.eq(&other.cpds)
    }
}

impl AbsDiffEq for MixedBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self
                .cpds
                .iter()
                .zip(&other.cpds)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.abs_diff_eq(other_cpd, epsilon)
                })
    }
}

impl RelativeEq for MixedBN {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.labels.eq(&other.labels)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self
                .cpds
                .iter()
                .zip(&other.cpds)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.relative_eq(other_cpd, epsilon, max_relative)
                })
    }
}

impl Labelled for MixedBN {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl BN for MixedBN {
    type CPD = MixedCPD;
    type Support = Map<String, MixedSupport>;
    type Evidence = MixedEv;
    type Sample = MixedSample;
    type Samples = MixedTable;
    type IncSamples = MixedIncTable;
    type WtdSamples = MixedWtdTable;

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Owned(
            self.cpds
                .iter()
                .map(|(label, cpd)| (label.clone(), cpd.support().into_owned()))
                .collect(),
        )
    }

    fn new<I>(graph: DiGraph, cpds: I) -> Result<Self>
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Collect the CPDs into a map.
        let mut cpds: Map<_, _> = cpds
            .into_iter()
            .map(|x| {
                if x.labels().len() != 1 {
                    return Err(Error::InvalidParameter(
                        "cpd",
                        "CPD must contain exactly one label.",
                    ));
                }
                Ok((x.labels()[0].to_owned(), x))
            })
            .collect::<Result<_>>()?;
        // Sort the CPDs by their labels.
        cpds.sort_keys();

        // Check same number of graph labels and CPDs.
        if !graph.labels().iter().eq(cpds.keys()) {
            return Err(Error::LabelMismatch("graph labels", "distributions labels"));
        }

        // Get the labels of the variables.
        let labels: Labels = graph.labels().clone();

        // Check if all vertices have the same labels as their parents.
        graph.vertices().into_iter().try_for_each(|i| {
            // Get the parents of the vertex.
            let pa_i = graph.parents(&set![i])?.into_iter();
            let pa_i: &Labels = &pa_i.map(|j| labels[j].to_owned()).collect();
            // Get the conditioning labels of the CPD.
            let pa_j = cpds[&labels[i]].conditioning_labels();
            // Check they are the same.
            if pa_i != pa_j {
                return Err(Error::LabelMismatch(
                    &format!("{pa_i:?}"),
                    &format!("{pa_j:?}"),
                ));
            }
            Ok(())
        })?;

        // Check the graph is acyclic.
        let topological_order = graph.topological_order().ok_or_else(|| Error::NotADag())?;

        Ok(Self {
            name: None,
            description: None,
            labels,
            graph,
            cpds,
            topological_order,
        })
    }

    #[inline]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[inline]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[inline]
    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    #[inline]
    fn cpds(&self) -> &Map<String, Self::CPD> {
        &self.cpds
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        self.cpds.iter().map(|(_, x)| x.parameters_size()).sum()
    }

    fn select(&self, x: &Set<usize>) -> Result<Self>
    where
        Self: Sized,
    {
        // Check that the variables are in bounds.
        x.iter().try_for_each(|&i| {
            if i >= self.labels.len() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;

        // Sort the indices.
        let mut x = x.clone();
        x.sort();

        // Construct the subgraph.
        let graph = self.graph.select(&x)?;
        // Select the CPDs.
        let cpds = x.iter().map(|&i| self.cpds[i].clone());

        // Construct the submodel.
        Self::with_optionals(
            // Clone the optionals.
            self.name.clone(),
            self.description.clone(),
            graph,
            cpds,
        )
    }

    #[inline]
    fn topological_order(&self) -> &[usize] {
        &self.topological_order
    }

    fn with_optionals<I>(
        name: Option<String>,
        description: Option<String>,
        graph: DiGraph,
        cpds: I,
    ) -> Result<Self>
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Check name is not empty string.
        if let Some(name) = &name
            && name.is_empty()
        {
            return Err(Error::InvalidParameter("name", "cannot be empty"));
        }
        // Check description is not empty string.
        if let Some(description) = &description
            && description.is_empty()
        {
            return Err(Error::InvalidParameter("description", "cannot be empty"));
        }

        // Construct the BN.
        let mut bn = Self::new(graph, cpds)?;

        // Set the optional fields.
        bn.name = name;
        bn.description = description;

        Ok(bn)
    }
}
