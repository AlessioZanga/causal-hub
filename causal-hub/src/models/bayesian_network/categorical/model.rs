use std::{borrow::Cow, fmt::Write};

use approx::{AbsDiffEq, RelativeEq};
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{CatEv, CatIncTable, CatSample, CatTable, CatWtdTable},
    impl_json_io,
    inference::TopologicalOrder,
    io::{BifIO, BifParser},
    models::{BN, CPD, CatCPD, CatSupport, DiGraph, Graph, Labelled},
    set,
    types::{Error, Labels, Map, Result, Set},
};

/// A categorical Bayesian network.
#[derive(Clone, Debug)]
pub struct CatBN {
    /// The name of the model.
    name: Option<String>,
    /// The description of the model.
    description: Option<String>,
    /// The labels of the variables.
    labels: Labels,
    /// The support of the variables.
    support: CatSupport,
    /// The shape of the variables.
    shape: Array1<usize>,
    /// The graph of the model.
    graph: DiGraph,
    /// The parameters of the model.
    cpds: Map<String, CatCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

impl CatBN {
    /// Returns the support of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the support of the variables.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        &self.support
    }

    /// Returns the shape of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the variables.
    ///
    #[inline]
    pub fn shape(&self) -> &Array1<usize> {
        &self.shape
    }
}

impl PartialEq for CatBN {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self.cpds.eq(&other.cpds)
    }
}

impl AbsDiffEq for CatBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self.cpds.iter().zip(&other.cpds).all(
                |((label, distribution), (other_label, other_cpd))| {
                    label.eq(other_label) && distribution.abs_diff_eq(other_cpd, epsilon)
                },
            )
    }
}

impl RelativeEq for CatBN {
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
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
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

impl Labelled for CatBN {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl BN for CatBN {
    type CPD = CatCPD;
    type Support = CatSupport;
    type Evidence = CatEv;
    type Sample = CatSample;
    type Samples = CatTable;
    type IncSamples = CatIncTable;
    type WtdSamples = CatWtdTable;

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
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

        // Allocate the support of the variables.
        let mut support: CatSupport = Default::default();
        // Insert the support of the variables into the map to check if they are the same.
        cpds.values().try_for_each(|distribution| {
            distribution
                .support()
                .iter()
                .chain(distribution.conditioning_support())
                .try_for_each(|(l, stats)| {
                    // Check if the support are already in the map.
                    if let Some(existing_states) = support.get(l) {
                        // Check if the support are the same.
                        if existing_states != stats {
                            return Err(Error::InvalidParameter(
                                "cpds",
                                &format!("CatSupport of `{l}` must be the same across CPDs."),
                            ));
                        }
                    } else {
                        // Insert the support into the map.
                        support.insert(l.to_owned(), stats.clone());
                    }
                    Ok(())
                })
        })?;
        // Sort the support of the variables.
        support.sort_keys();

        // Get the labels of the variables.
        let labels: Labels = support.keys().cloned().collect();
        // Get the shape of the variables.
        let shape: Array1<usize> = support.values().map(|stats| stats.len()).collect();

        // Check if all vertices have the same labels as their parents.
        graph.vertices().into_iter().try_for_each(|i| {
            // Get the parents of the vertex.
            let pa_i = graph.parents(&set![i])?.into_iter();
            let pa_i: &Labels = &pa_i.map(|j| labels[j].to_owned()).collect(); // FIXME: Use references to avoid clones.
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
            support,
            shape,
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
        let mut bayesian_network = Self::new(graph, cpds)?;

        // Set the optional fields.
        bayesian_network.name = name;
        bayesian_network.description = description;

        Ok(bayesian_network)
    }
}

impl Serialize for CatBN {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 3;
        size += self.name.is_some() as usize;
        size += self.description.is_some() as usize;

        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Serialize name, if any.
        if let Some(name) = &self.name {
            map.serialize_entry("name", name)?;
        }
        // Serialize description, if any.
        if let Some(description) = &self.description {
            map.serialize_entry("description", description)?;
        }
        // Serialize graph.
        map.serialize_entry("graph", &self.graph)?;

        // Convert the CPDs to a flat format.
        let cpds: Vec<_> = self.cpds.values().cloned().collect();
        // Serialize CPDs.
        map.serialize_entry("cpds", &cpds)?;

        // Serialize type.
        map.serialize_entry("type", "catbn")?;

        // Finalize the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatBN {
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

        struct CatBNVisitor;

        impl<'de> Visitor<'de> for CatBNVisitor {
            type Value = CatBN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatBN")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CatBN, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut name = None;
                let mut description = None;
                let mut graph = None;
                let mut cpds = None;
                let mut type_ = None;

                // Parse the map.
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
                            cpds = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let graph = graph.ok_or_else(|| E::missing_field("graph"))?;
                let cpds = cpds.ok_or_else(|| E::missing_field("cpds"))?;

                // Check type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                if type_ != "catbn" {
                    return Err(E::custom(format!(
                        "Invalid type for CatBN: expected 'catbn', found '{type_}'"
                    )));
                }

                // Set helper types.
                let cpds: Vec<_> = cpds;

                CatBN::with_optionals(name, description, graph, cpds)
                    .map_err(serde::de::Error::custom)
            }
        }

        const FIELDS: &[&str] = &["name", "description", "graph", "cpds", "type"];

        deserializer.deserialize_struct("CatBN", FIELDS, CatBNVisitor)
    }
}

// Implement `JsonIO` for `CatBN`.
impl_json_io!(CatBN);

impl BifIO for CatBN {
    fn from_bif_string(bif: &str) -> Result<Self> {
        BifParser::parse_str(bif)
    }

    fn to_bif_string(&self) -> Result<String> {
        let mut f = String::new();

        // Write network name.
        writeln!(
            f,
            "network {} {{",
            self.name.as_deref().unwrap_or("Network")
        )
        .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        // Write network description, if any.
        if let Some(description) = &self.description {
            writeln!(f, "  property description \"{}\";", description)
                .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        }
        writeln!(f, "}}").map_err(|evidence| Error::Parsing(&evidence.to_string()))?;

        // Write variables.
        for label in self.labels() {
            writeln!(f, "variable {} {{", label)
                .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
            let support = &self.support()[label];
            let states_str = support.iter().map(|x| x.to_string()).join(", ");
            writeln!(
                f,
                "  type discrete [ {} ] {{ {} }};",
                support.len(),
                states_str
            )
            .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
            writeln!(f, "}}").map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        }

        // Write probabilities.
        for (label, distribution) in &self.cpds {
            let parents = distribution.conditioning_labels();
            if parents.is_empty() {
                writeln!(f, "probability ( {} ) {{", label)
                    .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
            } else {
                let parents_str = parents.iter().map(|x| x.to_string()).join(", ");
                writeln!(f, "probability ( {} | {} ) {{", label, parents_str)
                    .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
            }

            if parents.is_empty() {
                // Write flat table.
                let values = distribution
                    .parameters()
                    .iter()
                    .map(|x| x.to_string())
                    .join(", ");
                writeln!(f, "  table {};", values)
                    .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
            } else {
                // Write conditional table.
                let conditioning_support = distribution.conditioning_support();
                let combinations = parents
                    .iter()
                    .map(|probability| &conditioning_support[probability])
                    .multi_cartesian_product();

                for (i, support) in combinations.enumerate() {
                    let states_str = support.iter().map(|x| x.to_string()).join(", ");
                    let probs_str = distribution
                        .parameters()
                        .row(i)
                        .iter()
                        .map(|x| x.to_string())
                        .join(", ");
                    writeln!(f, "  ({}) {};", states_str, probs_str)
                        .map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
                }
            }
            writeln!(f, "}}").map_err(|evidence| Error::Parsing(&evidence.to_string()))?;
        }

        Ok(f)
    }

    fn from_bif_file(path: &str) -> Result<Self> {
        Self::from_bif_string(&std::fs::read_to_string(path).map_err(Error::from)?)
    }

    fn to_bif_file(&self, path: &str) -> Result<()> {
        std::fs::write(path, self.to_bif_string()?).map_err(Error::from)
    }
}
