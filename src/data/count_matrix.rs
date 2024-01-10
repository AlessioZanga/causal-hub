use ndarray::prelude::*;

use super::{CategoricalDataSet, DataSet};

#[derive(Clone, Debug)]
struct RavelMultiIndex {
    ravel: Vec<usize>,
    size: usize,
}

impl RavelMultiIndex {
    #[inline]
    fn new<I>(cardinality: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Collect items into array.
        let cardinality = Vec::from_iter(cardinality);

        // Assert non-empty.
        assert!(
            !cardinality.is_empty(),
            "Ravel multi index must not be empty"
        );
        // Assert all strictly positive.
        assert!(
            cardinality.iter().all(|&x| x > 0),
            "Ravel multi index must not be empty"
        );

        // Compute max size.
        let size = cardinality.iter().product();

        // Make ravel mutable.
        let mut ravel = vec![1; cardinality.len()];

        // From the end to the beginning of ravel ...
        for i in (1..ravel.len()).rev() {
            // ... compute the cumulative product.
            ravel[i - 1] = ravel[i] * cardinality[i];
        }

        Self { ravel, size }
    }

    #[inline]
    fn call<I>(&self, multi_index: I) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        self.ravel.iter().zip(multi_index).map(|(i, j)| i * j).sum()
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    fn len(&self) -> usize {
        self.size
    }
}

/// Marginal counts of categorical variable.
pub struct MarginalCountMatrix {
    counts: Array1<usize>,
}

impl MarginalCountMatrix {
    /// Construct marginal counts from data set and given variable.
    ///
    /// # Arguments
    ///
    /// * `data_set` - Data set.
    /// * `x` - Index of variable.
    ///
    /// # Panics
    ///
    /// Panics if `x` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use causal_hub::prelude::*;
    /// use ndarray::prelude::*;
    ///
    /// // Initialize data.
    /// let data = array![
    ///     [0, 0, 0],
    ///     [0, 0, 1],
    ///     [0, 1, 0],
    ///     [0, 1, 1],
    ///     [1, 0, 0],
    ///     [1, 0, 1],
    ///     [1, 1, 0],
    ///     [1, 1, 1],
    /// ];
    ///
    /// // Initialize states.
    /// let states = [
    ///     ("X", ["no", "yes"]),
    ///     ("Y", ["no", "yes"]),
    ///     ("Z", ["no", "yes"]),
    /// ];
    /// // Map states to states structure.
    /// let states = states
    ///     .into_iter()
    ///     .map(|(label, states)| {(
    ///         label.into(),
    ///         states.into_iter().map(|s| s.into()).collect()
    ///     )})
    ///     .collect();
    ///
    /// // Initialize data set.
    /// let data_set = CategoricalDataSet::with_data_labels(data, states);
    ///
    /// // Get marginal counts of variable `X`.
    /// let counts = MarginalCountMatrix::new(&data_set, 0);
    ///
    /// // Check marginal counts.
    /// assert_eq!(counts.data(), &array![4, 4]);
    /// ```
    ///
    #[inline]
    pub fn new(data_set: &CategoricalDataSet, x: usize) -> Self {
        // Get cardinalities.
        let cards = data_set.cardinality();

        // Set count matrix shape.
        let shape = (cards[x] as usize,);

        // Allocate count matrix.
        let mut counts = Array1::zeros(shape);
        // Fill count matrix.
        for row in data_set.data().rows() {
            // Increment at given index.
            counts[row[x] as usize] += 1;
        }

        Self { counts }
    }

    /// Get underlying marginal counts.
    #[inline]
    pub const fn data(&self) -> &Array1<usize> {
        &self.counts
    }
}

impl From<MarginalCountMatrix> for Array1<usize> {
    #[inline]
    fn from(other: MarginalCountMatrix) -> Array1<usize> {
        other.counts
    }
}

/// Conditional counts of categorical variable.
pub struct ConditionalCountMatrix {
    counts: Array2<usize>,
}

impl ConditionalCountMatrix {
    /// Construct conditional counts from data set and given variable.
    ///
    /// # Arguments
    ///
    /// * `data_set` - Data set.
    /// * `x` - Index of variable.
    /// * `z` - Index of conditional set.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `z` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use causal_hub::prelude::*;
    /// use ndarray::prelude::*;
    ///
    /// // Initialize data.
    /// let data = array![
    ///     [0, 0, 0],
    ///     [0, 0, 1],
    ///     [0, 1, 0],
    ///     [0, 1, 1],
    ///     [1, 0, 0],
    ///     [1, 0, 1],
    ///     [1, 1, 0],
    ///     [1, 1, 1],
    /// ];
    ///
    /// // Initialize states.
    /// let states = [
    ///     ("X", ["no", "yes"]),
    ///     ("Y", ["no", "yes"]),
    ///     ("Z", ["no", "yes"]),
    /// ];
    /// // Map states to states structure.
    /// let states = states
    ///     .into_iter()
    ///     .map(|(label, states)| {(
    ///         label.into(),
    ///         states.into_iter().map(|s| s.into()).collect()
    ///     )})
    ///     .collect();
    ///
    /// // Initialize data set.
    /// let data_set = CategoricalDataSet::with_data_labels(data, states);
    ///
    /// // Get conditional counts of variable `X` given `Z`.
    /// let counts = ConditionalCountMatrix::new(&data_set, 0, &[2]);
    ///
    /// // Check conditional counts.
    /// assert_eq!(counts.data(), &array![[2, 2], [2, 2]]);
    /// ```
    ///
    #[inline]
    pub fn new(data_set: &CategoricalDataSet, x: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = data_set.cardinality();
        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z] as usize));
        // Set count matrix shape.
        let shape = (rmi.len(), cards[x] as usize);

        // Allocate count matrix.
        let mut counts = Array2::zeros(shape);
        // Fill count matrix.
        for row in data_set.data().rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z] as usize);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            counts[[row_z, row[x] as usize]] += 1;
        }

        Self { counts }
    }

    /// Get underlying conditional counts.
    #[inline]
    pub const fn data(&self) -> &Array2<usize> {
        &self.counts
    }
}

impl From<ConditionalCountMatrix> for Array2<usize> {
    #[inline]
    fn from(other: ConditionalCountMatrix) -> Array2<usize> {
        other.counts
    }
}

/// Joint counts of categorical variables.
pub struct JointCountMatrix {
    counts: Array2<usize>,
}

impl JointCountMatrix {
    /// Construct joint counts from data set and given variables.
    ///
    /// # Arguments
    ///
    /// * `data_set` - Data set.
    /// * `x` - Index of variable.
    /// * `y` - Index of variable.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use causal_hub::prelude::*;
    /// use ndarray::prelude::*;
    ///
    /// // Initialize data.
    /// let data = array![
    ///     [0, 0, 0],
    ///     [0, 0, 1],
    ///     [0, 1, 0],
    ///     [0, 1, 1],
    ///     [1, 0, 0],
    ///     [1, 0, 1],
    ///     [1, 1, 0],
    ///     [1, 1, 1],
    /// ];
    ///
    /// // Initialize states.
    /// let states = [
    ///     ("X", ["no", "yes"]),
    ///     ("Y", ["no", "yes"]),
    ///     ("Z", ["no", "yes"]),
    /// ];
    /// // Map states to states structure.
    /// let states = states
    ///     .into_iter()
    ///     .map(|(label, states)| {(
    ///         label.into(),
    ///         states.into_iter().map(|s| s.into()).collect()
    ///     )})
    ///     .collect();
    ///
    /// // Initialize data set.
    /// let data_set = CategoricalDataSet::with_data_labels(data, states);
    ///
    /// // Get joint counts of variables `X` and `Y`.
    /// let counts = JointCountMatrix::new(&data_set, 0, 1);
    ///
    /// // Check joint counts.
    /// assert_eq!(counts.data(), &array![[2, 2], [2, 2]]);
    /// ```
    ///
    #[inline]
    pub fn new(data_set: &CategoricalDataSet, x: usize, y: usize) -> Self {
        // Get cardinalities.
        let cards = data_set.cardinality();

        // Set count matrix shape.
        let shape = (cards[x] as usize, cards[y] as usize);

        // Allocate count matrix.
        let mut counts = Array2::zeros(shape);
        // Fill count matrix.
        for row in data_set.data().rows() {
            // Increment at given index.
            counts[[row[x] as usize, row[y] as usize]] += 1;
        }

        Self { counts }
    }

    /// Get underlying joint counts.
    #[inline]
    pub const fn data(&self) -> &Array2<usize> {
        &self.counts
    }
}

impl From<JointCountMatrix> for Array2<usize> {
    #[inline]
    fn from(other: JointCountMatrix) -> Array2<usize> {
        other.counts
    }
}

/// Joint conditional counts of categorical variables.
pub struct JointConditionalCountMatrix {
    counts: Array3<usize>,
}

impl JointConditionalCountMatrix {
    /// Construct joint conditional counts from data set and given variables.
    ///
    /// # Arguments
    ///
    /// * `data_set` - Data set.
    /// * `x` - Index of variable.
    /// * `y` - Index of variable.
    /// * `z` - Index of conditional set.
    ///
    /// # Panics
    ///
    /// Panics if `x`, `y` or `z` is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use causal_hub::prelude::*;
    /// use ndarray::prelude::*;
    ///
    /// // Initialize data.
    /// let data = array![
    ///     [0, 0, 0],
    ///     [0, 0, 1],
    ///     [0, 1, 0],
    ///     [0, 1, 1],
    ///     [1, 0, 0],
    ///     [1, 0, 1],
    ///     [1, 1, 0],
    ///     [1, 1, 1],
    /// ];
    ///
    /// // Initialize states.
    /// let states = [
    ///     ("X", ["no", "yes"]),
    ///     ("Y", ["no", "yes"]),
    ///     ("Z", ["no", "yes"]),
    /// ];
    /// // Map states to states structure.
    /// let states = states
    ///     .into_iter()
    ///     .map(|(label, states)| {(
    ///         label.into(),
    ///         states.into_iter().map(|s| s.into()).collect()
    ///     )})
    ///     .collect();
    ///
    /// // Initialize data set.
    /// let data_set = CategoricalDataSet::with_data_labels(data, states);
    ///
    /// // Get joint conditional counts of variables `X` and `Y` given `Z`.
    /// let counts = JointConditionalCountMatrix::new(&data_set, 0, 1, &[2]);
    ///
    /// // Check joint conditional counts.
    /// assert_eq!(counts.data(), &array![
    ///     [[1, 1], [1, 1]],
    ///     [[1, 1], [1, 1]],
    /// ]);
    /// ```
    ///
    #[inline]
    pub fn new(data_set: &CategoricalDataSet, x: usize, y: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = data_set.cardinality();

        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z] as usize));

        // Set count matrix shape.
        let shape = (rmi.len(), cards[x] as usize, cards[y] as usize);

        // Allocate count matrix.
        let mut counts = Array3::zeros(shape);
        // Fill count matrix.
        for row in data_set.data().rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z] as usize);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            counts[[row_z, row[x] as usize, row[y] as usize]] += 1;
        }

        Self { counts }
    }

    /// Get underlying joint conditional counts.
    #[inline]
    pub const fn data(&self) -> &Array3<usize> {
        &self.counts
    }
}

impl From<JointConditionalCountMatrix> for Array3<usize> {
    #[inline]
    fn from(other: JointConditionalCountMatrix) -> Array3<usize> {
        other.counts
    }
}
