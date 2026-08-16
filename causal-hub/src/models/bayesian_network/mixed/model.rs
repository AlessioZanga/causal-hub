use std::borrow::Cow;

use approx::{AbsDiffEq, RelativeEq};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{
        CatEv, CatIncTable, CatTable, CatWtdTable, GaussEv, GaussIncTable, GaussTable,
        GaussWtdTable,
    },
    impl_json_io,
    inference::TopologicalOrder,
    models::{BN, CPD, DiGraph, Graph, Labelled, MixedCPD, MixedSample, MixedSupport},
    set,
    types::{Error, Labels, Map, Result, Set},
};

/// A unified evidence type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MixedEv {
    /// Categorical evidence.
    Categorical(CatEv),
    /// Gaussian evidence.
    Gaussian(GaussEv),
}

/// A unified complete dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MixedTable {
    /// Categorical table.
    Categorical(CatTable),
    /// Gaussian table.
    Gaussian(GaussTable),
}

/// A unified incomplete dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MixedIncTable {
    /// Categorical incomplete table.
    Categorical(CatIncTable),
    /// Gaussian incomplete table.
    Gaussian(GaussIncTable),
}

/// A unified weighted dataset type for mixed Bayesian networks.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MixedWtdTable {
    /// Categorical weighted table.
    Categorical(CatWtdTable),
    /// Gaussian weighted table.
    Gaussian(GaussWtdTable),
}

impl MixedEv {
    /// Return the events indices where evidence is present.
    pub fn events(&self) -> Set<usize> {
        match self {
            Self::Categorical(ev) => ev
                .evidences()
                .iter()
                .flatten()
                .map(|evidence| evidence.event())
                .collect(),
            Self::Gaussian(ev) => ev
                .evidences()
                .iter()
                .flatten()
                .map(|evidence| evidence.event())
                .collect(),
        }
    }
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

impl From<MixedTable> for MixedWtdTable {
    #[inline]
    fn from(table: MixedTable) -> Self {
        match table {
            MixedTable::Categorical(t) => MixedWtdTable::Categorical(t.into()),
            MixedTable::Gaussian(t) => MixedWtdTable::Gaussian(t.into()),
        }
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
            && self.cpds.iter().zip(&other.cpds).all(
                |((label, distribution), (other_label, other_cpd))| {
                    label.eq(other_label) && distribution.abs_diff_eq(other_cpd, epsilon)
                },
            )
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
            && self.cpds.iter().zip(&other.cpds).all(
                |((label, distribution), (other_label, other_cpd))| {
                    label.eq(other_label)
                        && distribution.relative_eq(other_cpd, epsilon, max_relative)
                },
            )
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
                .map(|(label, distribution)| (label.clone(), distribution.support().into_owned()))
                .collect(),
        )
    }

    fn new<I>(graph: DiGraph, cpds: I) -> Result<Self>
    where
        I: IntoIterator<Item = Self::CPD>,
    {
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
        cpds.sort_keys();

        if !graph.labels().iter().eq(cpds.keys()) {
            return Err(Error::LabelMismatch("graph labels", "distributions labels"));
        }

        let labels: Labels = graph.labels().clone();

        graph.vertices().into_iter().try_for_each(|i| {
            let pa_i = graph.parents(&set![i])?.into_iter();
            let pa_i: &Labels = &pa_i.map(|j| labels[j].to_owned()).collect();
            let pa_j = cpds[&labels[i]].conditioning_labels();
            if pa_i != pa_j {
                return Err(Error::LabelMismatch(
                    &format!("{pa_i:?}"),
                    &format!("{pa_j:?}"),
                ));
            }
            Ok(())
        })?;

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
        x.iter().try_for_each(|&i| {
            if i >= self.labels.len() {
                return Err(Error::IndexOutOfBounds(i));
            }
            Ok(())
        })?;

        let mut x = x.clone();
        x.sort();

        let graph = self.graph.select(&x)?;
        let cpds = x.iter().map(|&i| self.cpds[i].clone());

        Self::with_optionals(self.name.clone(), self.description.clone(), graph, cpds)
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
        if let Some(name) = &name
            && name.is_empty()
        {
            return Err(Error::InvalidParameter("name", "cannot be empty"));
        }
        if let Some(description) = &description
            && description.is_empty()
        {
            return Err(Error::InvalidParameter("description", "cannot be empty"));
        }

        let mut bayesian_network = Self::new(graph, cpds)?;
        bayesian_network.name = name;
        bayesian_network.description = description;

        Ok(bayesian_network)
    }
}

// ── Serde for MixedBN ──────────────────────────────────────────

impl Serialize for MixedBN {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut size = 3usize;
        size += self.name.is_some() as usize;
        size += self.description.is_some() as usize;

        let mut map = serializer.serialize_map(Some(size))?;

        if let Some(name) = &self.name {
            map.serialize_entry("name", name)?;
        }
        if let Some(description) = &self.description {
            map.serialize_entry("description", description)?;
        }
        map.serialize_entry("graph", &self.graph)?;

        let cpds: Vec<_> = self.cpds.values().cloned().collect();
        map.serialize_entry("cpds", &cpds)?;

        map.serialize_entry("type", "mixedbn")?;

        map.end()
    }
}

impl<'de> Deserialize<'de> for MixedBN {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Name,
            Description,
            Graph,
            Cpds,
            Type,
        }

        struct MixedBNVisitor;

        impl<'de> Visitor<'de> for MixedBNVisitor {
            type Value = MixedBN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct MixedBN")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MixedBN, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                let mut name = None;
                let mut description = None;
                let mut graph = None;
                let mut cpds = None;
                let mut type_ = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(E::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Description => {
                            if description.is_some() {
                                return Err(E::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        Field::Graph => {
                            if graph.is_some() {
                                return Err(E::duplicate_field("graph"));
                            }
                            graph = Some(map.next_value()?);
                        }
                        Field::Cpds => {
                            if cpds.is_some() {
                                return Err(E::duplicate_field("cpds"));
                            }
                            cpds = Some(map.next_value::<Vec<MixedCPD>>()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                let graph = graph.ok_or_else(|| E::missing_field("graph"))?;
                let cpds = cpds.ok_or_else(|| E::missing_field("cpds"))?;

                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                if type_ != "mixedbn" {
                    return Err(E::custom(format!(
                        "Invalid type for MixedBN: expected 'mixedbn', found '{type_}'"
                    )));
                }

                MixedBN::with_optionals(name, description, graph, cpds)
                    .map_err(serde::de::Error::custom)
            }
        }

        const FIELDS: &[&str] = &["name", "description", "graph", "cpds", "type"];

        deserializer.deserialize_struct("MixedBN", FIELDS, MixedBNVisitor)
    }
}

impl_json_io!(MixedBN);
