use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{CatSample, CatTrj, CatTrjs},
    impl_json_io,
    models::{BN, CIM, CTBN, CatBN, CatCIM, CatCPD, DiGraph, Graph, Labelled},
    set,
    types::{Labels, Map, Set, States},
};

/// A categorical continuous time Bayesian network.
#[derive(Clone, Debug)]
pub struct CatCTBN {
    /// The name of the model.
    name: Option<String>,
    /// The description of the model.
    description: Option<String>,
    /// The labels of the variables.
    labels: Labels,
    /// The states of the variables.
    states: States,
    /// The shape of the variables.
    shape: Array1<usize>,
    /// The initial distribution.
    initial_distribution: CatBN,
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional intensity matrices.
    cims: Map<String, CatCIM>,
}

impl CatCTBN {
    /// Returns the name of the model, if any.
    ///
    /// # Returns
    ///
    /// The name of the model, if it exists.
    ///
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the description of the model, if any.
    ///
    /// # Returns
    ///
    /// The description of the model, if it exists.
    ///
    #[inline]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the states of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the states of the variables.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        self.initial_distribution.states()
    }
}

impl PartialEq for CatCTBN {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.initial_distribution.eq(&other.initial_distribution)
            && self.graph.eq(&other.graph)
            && self.cims.eq(&other.cims)
    }
}

impl AbsDiffEq for CatCTBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.initial_distribution.eq(&other.initial_distribution)
            && self.graph.eq(&other.graph)
            && self
                .cims
                .iter()
                .zip(&other.cims)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.abs_diff_eq(other_cpd, epsilon)
                })
    }
}

impl RelativeEq for CatCTBN {
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
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.initial_distribution.eq(&other.initial_distribution)
            && self.graph.eq(&other.graph)
            && self
                .cims
                .iter()
                .zip(&other.cims)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.relative_eq(other_cpd, epsilon, max_relative)
                })
    }
}

impl Labelled for CatCTBN {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CTBN for CatCTBN {
    type CIM = CatCIM;
    type InitialDistribution = CatBN;
    type Event = (f64, CatSample);
    type Trajectory = CatTrj;
    type Trajectories = CatTrjs;

    fn new<I>(graph: DiGraph, cims: I) -> Self
    where
        I: IntoIterator<Item = Self::CIM>,
    {
        // Collect the CPDs into a map.
        let mut cims: Map<_, _> = cims
            .into_iter()
            // Assert CIM contains exactly one label.
            // TODO: Refactor code and remove this assumption.
            .inspect(|x| {
                assert_eq!(x.labels().len(), 1, "CPD must contain exactly one label.");
            })
            .map(|x| (x.labels()[0].to_owned(), x))
            .collect();
        // Sort the CPDs by their labels.
        cims.sort_keys();

        // Allocate the states of the variables.
        let mut states: States = Default::default();
        // Insert the states of the variables into the map to check if they are the same.
        for cim in cims.values() {
            cim.states()
                .iter()
                .chain(cim.conditioning_states())
                .for_each(|(l, s)| {
                    // Check if the states are already in the map.
                    if let Some(existing_states) = states.get(l) {
                        // Check if the states are the same.
                        assert_eq!(
                            existing_states, s,
                            "States of `{l}` must be the same across CIMs.",
                        );
                    } else {
                        // Insert the states into the map.
                        states.insert(l.to_owned(), s.clone());
                    }
                });
        }
        // Sort the states of the variables.
        states.sort_keys();

        // Get the labels of the variables.
        let labels: Labels = states.keys().cloned().collect();
        // Get the shape of the variables.
        let shape = Array::from_iter(states.values().map(Set::len));

        // Assert same number of graph labels and CIMs.
        assert!(
            graph.labels().iter().eq(cims.keys()),
            "Graph labels and distributions labels must be the same."
        );

        // Check if all vertices have the same labels as their parents.
        graph.vertices().iter().for_each(|&i| {
            // Get the parents of the vertex.
            let pa_i = graph.parents(&set![i]).into_iter();
            let pa_i: &Labels = &pa_i.map(|j| labels[j].to_owned()).collect();
            // Get the conditioning labels of the CIM.
            let pa_j = cims[&labels[i]].conditioning_labels();
            // Assert they are the same.
            assert_eq!(
                pa_i, pa_j,
                "Graph parents labels and CIM conditioning labels must be the same:\n\
                \t expected:    {:?} ,\n\
                \t found:       {:?} .",
                pa_i, pa_j
            );
        });

        // Initialize an empty graph for the uniform initial distribution.
        let initial_graph = DiGraph::empty(graph.labels());
        // Initialize the CPDs as uniform distributions.
        let initial_cpds = cims.values().map(|cim| {
            // Get label and states of the CIM.
            let states = cim.states().clone();
            // Set empty conditioning states.
            let conditioning_states = States::default();
            // Set uniform parameters.
            let alpha = cim.shape().product();
            let parameters = Array::from_vec(vec![1. / alpha as f64; alpha]);
            let parameters = parameters.insert_axis(Axis(0));
            // Construct the CPD.
            CatCPD::new(states, conditioning_states, parameters)
        });
        // Initialize a uniform initial distribution.
        let initial_distribution = CatBN::new(initial_graph, initial_cpds);

        Self {
            name: None,
            description: None,
            labels,
            states,
            shape,
            initial_distribution,
            graph,
            cims,
        }
    }

    fn initial_distribution(&self) -> &Self::InitialDistribution {
        &self.initial_distribution
    }

    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    fn cims(&self) -> &Map<String, Self::CIM> {
        &self.cims
    }

    fn parameters_size(&self) -> usize {
        // Parameters size of the initial distribution.
        self.initial_distribution.parameters_size()
            // Parameters size of the CIMs.
            + self
                .cims
                .values()
                .map(|x| x.parameters_size())
                .sum::<usize>()
    }

    fn with_optionals<I>(
        name: Option<String>,
        description: Option<String>,
        initial_distribution: Self::InitialDistribution,
        graph: DiGraph,
        cims: I,
    ) -> Self
    where
        I: IntoIterator<Item = Self::CIM>,
    {
        // Assert name is not empty string.
        if let Some(name) = &name {
            assert!(!name.is_empty(), "Name cannot be an empty string.");
        }
        // Assert description is not empty string.
        if let Some(description) = &description {
            assert!(
                !description.is_empty(),
                "Description cannot be an empty string."
            );
        }

        // Construct the categorical CTBN.
        let mut ctbn = Self::new(graph, cims);

        // Assert the initial distribution has same labels.
        assert!(
            initial_distribution.labels().eq(ctbn.labels()),
            "Initial distribution labels must be the same as the CIMs labels."
        );
        // Assert the initial distribution has same states.
        assert!(
            initial_distribution
                .cpds()
                .into_iter()
                .zip(ctbn.cims())
                .all(|((_, cpd), (_, cim))| cpd.states().eq(cim.states())),
            "Initial distribution states must be the same as the CIMs states."
        );

        // Set the optional fields.
        ctbn.name = name;
        ctbn.description = description;
        ctbn.initial_distribution = initial_distribution;

        ctbn
    }
}

impl Serialize for CatCTBN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Count the elements to serialize.
        let mut size = 4;
        size += self.name.is_some() as usize;
        size += self.description.is_some() as usize;

        // Allocate the map.
        let mut map = serializer.serialize_map(Some(size))?;

        // Convert the CIMs to a flat format.
        let cims: Vec<_> = self.cims.values().cloned().collect();

        // Serialize name, if any.
        if let Some(name) = &self.name {
            map.serialize_entry("name", name)?;
        }
        // Serialize description, if any.
        if let Some(description) = &self.description {
            map.serialize_entry("description", description)?;
        }
        // Serialize initial distribution.
        map.serialize_entry("initial_distribution", &self.initial_distribution)?;
        // Serialize graph.
        map.serialize_entry("graph", &self.graph)?;
        // Serialize CIMs.
        map.serialize_entry("cims", &cims)?;
        // Serialize type.
        map.serialize_entry("type", "catctbn")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatCTBN {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Name,
            Description,
            InitialDistribution,
            Graph,
            Cims,
            Type,
        }

        struct CatCTBNVisitor;

        impl<'de> Visitor<'de> for CatCTBNVisitor {
            type Value = CatCTBN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatCTBN")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CatCTBN, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate fields
                let mut name = None;
                let mut description = None;
                let mut initial_distribution = None;
                let mut graph = None;
                let mut cims = None;
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
                        Field::InitialDistribution => {
                            if initial_distribution.is_some() {
                                return Err(E::duplicate_field("initial_distribution"));
                            }
                            initial_distribution = Some(map.next_value()?);
                        }
                        Field::Graph => {
                            if graph.is_some() {
                                return Err(E::duplicate_field("graph"));
                            }
                            graph = Some(map.next_value()?);
                        }
                        Field::Cims => {
                            if cims.is_some() {
                                return Err(E::duplicate_field("cims"));
                            }
                            cims = Some(map.next_value()?);
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
                let initial_distribution =
                    initial_distribution.ok_or_else(|| E::missing_field("initial_distribution"))?;
                let graph = graph.ok_or_else(|| E::missing_field("graph"))?;
                let cims = cims.ok_or_else(|| E::missing_field("cims"))?;

                // Assert type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                assert_eq!(type_, "catctbn", "Invalid type for CatCTBN.");

                // Set helper types.
                let cims: Vec<_> = cims;

                Ok(CatCTBN::with_optionals(
                    name,
                    description,
                    initial_distribution,
                    graph,
                    cims,
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "name",
            "description",
            "initial_distribution",
            "graph",
            "cims",
            "type",
        ];

        deserializer.deserialize_struct("CatCTBN", FIELDS, CatCTBNVisitor)
    }
}

// Implement `JsonIO` for `CatCTBN`.
impl_json_io!(CatCTBN);
