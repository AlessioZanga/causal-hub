use itertools::Itertools;

/// Union-Find structure.
#[derive(Clone, Debug, Default)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    /// Build a new structure with `size` items.
    ///
    /// # Panics
    ///
    /// If `size` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    #[inline]
    pub fn new(size: usize) -> Self {
        // Check if size is valid.
        assert!(size > 0);

        Self {
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    /// Gets the number of items in the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    /// Checks if two items are in the same set.
    ///  
    /// # Panics
    ///
    /// At least one of the items does not exist in the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    #[inline]
    pub fn contains(&self, x: usize, y: usize) -> bool {
        // Check if x and y are in the same set.
        self.find(x) == self.find(y)
    }

    /// Gets the root of a given item.
    ///  
    /// # Panics
    ///
    /// The items does not exist in the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    pub fn find(&self, x: usize) -> usize {
        let mut x = x;

        while self.parents[x] != x {
            x = self.parents[x];
        }

        x
    }

    /// Gets the root of a given item, while compressing the paths.
    ///  
    /// # Panics
    ///
    /// The items does not exist in the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    pub fn find_mut(&mut self, x: usize) -> usize {
        let mut x = x;

        while self.parents[x] != x {
            self.parents[x] = self.parents[self.parents[x]];
            x = self.parents[x];
        }

        x
    }

    /// Make two items into the same set, if not already.
    ///
    /// # Panics
    ///
    /// At least one of the items does not exist in the structure.
    ///  
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut x, mut y) = (self.find_mut(x), self.find_mut(y));

        if x == y {
            return false;
        }

        let (rank_x, rank_y) = (self.ranks[x], self.ranks[y]);

        if rank_x < rank_y {
            let z = x;
            y = x;
            x = z;
        }

        self.parents[y] = x;

        if rank_x == rank_y {
            self.ranks[x] += 1;
        }

        true
    }
}

impl Extend<usize> for UnionFind {
    /// Union all the items from an iterator into the same set.
    ///
    /// # Panics
    ///
    /// At least one of the items does not exist in the structure.
    ///
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        // For each consecutive pair of items ...
        for (x, y) in iter.into_iter().tuple_windows() {
            // ... apply union algorithm.
            self.union(x, y);
        }
    }
}
