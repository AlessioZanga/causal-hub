use rand::{
    Rng,
    seq::{IndexedRandom, IteratorRandom, SliceRandom},
};

use crate::{
    datasets::{MissingMechanism, MissingType},
    inference::VStructures,
    models::{DiGraph, Graph, Labelled},
    random::Random,
    set,
    types::{Error, Map, Result, Set},
};

/// A struct representing a random missingness mechanism generator.
pub struct RngMissingMechanism<'a, R> {
    rng: &'a mut R,
    graph: &'a DiGraph,
    missing: MissingType,
    p: f64,
}

impl<'a, R> RngMissingMechanism<'a, R> {
    /// Creates a new `RngMissingMechanism` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `graph` - The graph on which to generate the missingness mechanism.
    /// * `missing` - The type of missingness mechanism to generate.
    /// * `p` - The ratio of missing variables.
    ///
    /// # Returns
    ///
    /// A new `RngMissingMechanism` instance.
    ///
    pub fn new(rng: &'a mut R, graph: &'a DiGraph, missing: MissingType, p: f64) -> Result<Self> {
        // Check if the ratio of missing variables is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self {
            rng,
            graph,
            missing,
            p,
        })
    }
}

impl<R: Rng> RngMissingMechanism<'_, R> {
    /// Generates a random missingness mechanism of type MCAR.
    ///
    /// # Returns
    ///
    /// A map where keys are missing variable indices and values are empty sets (no causes).
    ///
    pub fn random_mcar(&mut self) -> Result<MissingMechanism> {
        // Get the number of vertices.
        let v = self.graph.vertices();
        // Calculate the total number of missing variables.
        let n = (v.len() as f64 * self.p).round() as usize;
        // Randomly select n variables to be missing.
        let m = v.into_iter().sample(self.rng, n);
        // Create the missingness mechanism with empty cause sets.
        let pr = MissingMechanism::new(
            self.graph.labels().clone(),
            m.into_iter().map(|x| (x, set![])).collect(),
        )?;

        Ok(pr)
    }

    /// Generates a random missingness mechanism of type MAR.
    ///
    /// # Returns
    ///
    /// A map where keys are missing variable indices and values are sets of observed variable indices causing the missingness.
    ///
    pub fn random_mar(&mut self) -> Result<MissingMechanism> {
        // Get the number of vertices.
        let v = self.graph.vertices();
        // Calculate the total number of missing variables.
        let n = (v.len() as f64 * self.p).round() as usize;
        // Initialize the cause dictionary.
        let mut pr = MissingMechanism::new(self.graph.labels().clone(), Map::default())?;

        // Precompute v-structures.
        let v_structs = self.graph.v_structures()?;

        let mut m = Set::default();
        let mut o = Set::default();

        // 1. Prefer v-structures
        for (x, z, y) in v_structs {
            if m.len() >= n {
                break;
            }

            for &u in &[x, y] {
                if !m.contains(&u) && !o.contains(&u) {
                    m.insert(u);
                    o.insert(z);
                    pr.insert(u, set![z]);

                    if m.len() >= n {
                        break;
                    }
                }
            }
        }

        // 2. Fill remaining missing variables
        if m.len() < n {
            let mut remaining: Vec<_> = v
                .iter()
                .copied()
                .filter(|&u| !m.contains(&u) && !o.contains(&u))
                .collect();
            remaining.shuffle(self.rng);
            let extra_count = (n - m.len()).min(remaining.len());
            for &u in &remaining[..extra_count] {
                m.insert(u);
            }
            o = v.iter().copied().filter(|u| !m.contains(u)).collect();
        }

        // 3. Assign MAR causes
        let vars_obs_vec: Vec<_> = o.iter().copied().collect();
        for &x in &m {
            if pr.contains_key(&x) {
                continue;
            }

            let predecessors = self.graph.parents(&set![x])?;
            let successors = self.graph.children(&set![x])?;
            let neighbors = predecessors.union(&successors).copied().collect::<Set<_>>();
            let candidates: Vec<_> = neighbors.intersection(&o).copied().collect();

            if let Some(&z) = candidates.choose(self.rng) {
                pr.insert(x, set![z]);
            } else if let Some(&z) = vars_obs_vec.choose(self.rng) {
                pr.insert(x, set![z]);
            }
        }

        Ok(pr)
    }

    /// Generates a random missingness mechanism of type MNAR.
    ///
    /// # Returns
    ///
    /// A map where keys are missing variable indices and values are sets of observed variable indices causing the missingness.
    ///
    pub fn random_mnar(&mut self) -> Result<MissingMechanism> {
        // Get the number of vertices.
        let v = self.graph.vertices();
        // Calculate the total number of missing variables.
        let n = (v.len() as f64 * self.p).round() as usize;

        // Initialize the cause dictionary.
        let mut pr = MissingMechanism::new(self.graph.labels().clone(), Map::default())?;

        // Precompute v-structures.
        let v_structs = self.graph.v_structures()?;

        let p_mnar = (n as f64 / 2.0).round() as usize;

        let mut vars_miss_mnar = Set::default();
        let mut m = Set::default();

        // 1. Assign MNAR variables via v-structures
        for (x, z, y) in v_structs {
            if vars_miss_mnar.len() >= p_mnar {
                break;
            }

            for &u in &[x, y] {
                if !m.contains(&u) {
                    vars_miss_mnar.insert(u);
                    m.insert(u);
                    m.insert(z);
                    pr.insert(u, set![z]);

                    if vars_miss_mnar.len() >= p_mnar {
                        break;
                    }
                }
            }
        }

        // 2. MAR part
        let vars_miss_mar: Vec<_> = m.difference(&vars_miss_mnar).copied().collect();
        let o: Set<_> = v.iter().copied().filter(|u| !m.contains(u)).collect();
        let vars_obs_vec: Vec<_> = o.iter().copied().collect();

        for &x in &vars_miss_mar {
            let predecessors = self.graph.parents(&set![x])?;
            let successors = self.graph.children(&set![x])?;
            let neighbors = predecessors.union(&successors).copied().collect::<Set<_>>();
            let candidates: Vec<_> = neighbors.intersection(&o).copied().collect();

            if let Some(&z) = candidates.choose(self.rng) {
                pr.insert(x, set![z]);
            } else if let Some(&z) = vars_obs_vec.choose(self.rng) {
                pr.insert(x, set![z]);
            }
        }

        // 3. Fill remaining missing variables if needed
        while m.len() < n {
            let remaining: Vec<_> = v.iter().copied().filter(|u| !m.contains(u)).collect();
            if remaining.is_empty() {
                break;
            }

            if let Some(&x) = remaining.choose(self.rng) {
                // Z = random.choice(list(set(V) - m))
                // Note: remaining still contains x at this point in Python logic if it's the same set.
                if let Some(&z) = remaining.choose(self.rng) {
                    m.insert(x);
                    pr.insert(x, set![z]);
                }
            }
        }

        Ok(pr)
    }
}

impl<R: Rng> Random for RngMissingMechanism<'_, R> {
    type Output = Result<MissingMechanism>;

    fn random(&mut self) -> Self::Output {
        // Generate the missingness mechanism based on the specified type.
        match self.missing {
            MissingType::MCAR => self.random_mcar(),
            MissingType::MAR => self.random_mar(),
            MissingType::MNAR => self.random_mnar(),
        }
    }
}
