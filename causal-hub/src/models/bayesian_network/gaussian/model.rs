use approx::{AbsDiffEq, RelativeEq};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{GaussEv, GaussSample, GaussTable, GaussWtdTable},
    impl_json_io,
    inference::TopologicalOrder,
    models::{BN, CPD, DiGraph, GaussCPD, Graph, Labelled},
    set,
    types::{Error, Labels, Map, Result},
};

/// A Gaussian Bayesian network.
#[derive(Clone, Debug)]
pub struct GaussBN {
    /// The name of the model.
    name: Option<String>,
    /// The description of the model.
    description: Option<String>,
    /// The labels of the variables.
    labels: Labels,
    /// The graph of the model.
    graph: DiGraph,
    /// The parameters of the model.
    cpds: Map<String, GaussCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

impl PartialEq for GaussBN {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self.cpds.eq(&other.cpds)
    }
}

impl AbsDiffEq for GaussBN {
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

impl RelativeEq for GaussBN {
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

impl Labelled for GaussBN {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl BN for GaussBN {
    type CPD = GaussCPD;
    type Evidence = GaussEv;
    type Sample = GaussSample;
    type Samples = GaussTable;
    type WeightedSamples = GaussWtdTable;

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
                        "cpd".to_string(),
                        "CPD must contain exactly one label.".to_string(),
                    ));
                }
                Ok((x.labels()[0].to_owned(), x))
            })
            .collect::<Result<_>>()?;
        // Sort the CPDs by their labels.
        cpds.sort_keys();

        // Assert same number of graph labels and CPDs.
        if !graph.labels().iter().eq(cpds.keys()) {
            return Err(Error::LabelMismatch(
                "graph labels".to_string(),
                "distributions labels".to_string(),
            ));
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
            // Assert they are the same.
            if pa_i != pa_j {
                return Err(Error::LabelMismatch(
                    format!("{pa_i:?}"),
                    format!("{pa_j:?}"),
                ));
            }
            Ok(())
        })?;

        // Assert the graph is acyclic.
        let topological_order = graph.topological_order().ok_or(Error::NotADag)?;

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
        // Assert name is not empty string.
        if let Some(name) = &name
            && name.is_empty()
        {
            return Err(Error::InvalidParameter(
                "name".to_string(),
                "cannot be empty".to_string(),
            ));
        }
        // Assert description is not empty string.
        if let Some(description) = &description
            && description.is_empty()
        {
            return Err(Error::InvalidParameter(
                "description".to_string(),
                "cannot be empty".to_string(),
            ));
        }

        // Construct the BN.
        let mut bn = Self::new(graph, cpds)?;

        // Set the optional fields.
        bn.name = name;
        bn.description = description;

        Ok(bn)
    }
}

impl Serialize for GaussBN {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the number of fields.
        let mut size = 3;
        // Add optional fields, if any.
        size += self.name.is_some() as usize;
        size += self.description.is_some() as usize;
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Serialize the name, if any.
        if let Some(name) = &self.name {
            map.serialize_entry("name", name)?;
        }
        // Serialize the description, if any.
        if let Some(description) = &self.description {
            map.serialize_entry("description", description)?;
        }

        // Serialize the graph.
        map.serialize_entry("graph", &self.graph)?;

        // Convert the CPDs to a flat format.
        let cpds: Vec<_> = self.cpds.values().cloned().collect();
        // Serialize CPDs.
        map.serialize_entry("cpds", &cpds)?;

        // Serialize type.
        map.serialize_entry("type", "gaussbn")?;

        // Finalize the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussBN {
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

        struct GaussBNVisitor;

        impl<'de> Visitor<'de> for GaussBNVisitor {
            type Value = GaussBN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussBN")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GaussBN, V::Error>
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

                // Assert type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                assert_eq!(type_, "gaussbn", "Invalid type for GaussBN.");

                // Set helper types.
                let cpds: Vec<_> = cpds;

                GaussBN::with_optionals(name, description, graph, cpds)
                    .map_err(serde::de::Error::custom)
            }
        }

        const FIELDS: &[&str] = &["name", "description", "graph", "cpds", "type"];

        deserializer.deserialize_struct("GaussBN", FIELDS, GaussBNVisitor)
    }
}

// Implement `JsonIO` for `GaussBN`.
impl_json_io!(GaussBN);
