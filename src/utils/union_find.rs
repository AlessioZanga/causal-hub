/// Union-Find structure.
#[derive(Clone, Debug)]
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// assert!(!union_find.contains(0, 1));
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// assert_eq!(union_find.len(), 5);
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// assert!(!union_find.contains(0, 1));
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// for x in 0..union_find.len() {
    ///     assert_eq!(union_find.find(x), x);
    /// }
    /// ```
    ///
    #[inline]
    pub fn find(&self, x: usize) -> usize {
        // Make item mutable.
        let mut x = x;
        // While root is not found.
        while self.parents[x] != x {
            // Traverse the path backward.
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let mut union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// for x in 0..union_find.len() {
    ///     assert_eq!(union_find.find_mut(x), x);
    /// }
    /// ```
    ///
    #[inline]
    pub fn find_mut(&mut self, x: usize) -> usize {
        // Make item mutable.
        let mut x = x;
        // While root is not found.
        while self.parents[x] != x {
            // Compress the path backward.
            self.parents[x] = self.parents[self.parents[x]];
            // Update current root.
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
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let mut union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// for x in 0..union_find.len() {
    ///     assert_eq!(union_find.find(x), x);
    /// }
    ///
    /// // Merge item set 0 and 3.
    /// assert!(union_find.union(0, 3));
    ///
    /// // Now, items 0 and 3 are in the same set.
    /// assert!(union_find.contains(0, 3));
    /// ```
    ///
    #[inline]
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        // Get root of items.
        let (mut x, mut y) = (self.find_mut(x), self.find_mut(y));

        // If both items has the same root ...
        if x == y {
            // ... then, they are already in the same set.
            return false;
        }

        // Get items ranks.
        let (rank_x, rank_y) = (self.ranks[x], self.ranks[y]);
        // If first item rank is lower ...
        if rank_x < rank_y {
            // ... then, swaps the items.
            std::mem::swap(&mut x, &mut y);
        }

        // Merge two sets together by setting the root.
        self.parents[y] = x;

        // If both items has the same rank ...
        if rank_x == rank_y {
            // ... then, increment the rank of the root.
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
    /// # Examples
    ///
    /// ```
    /// use causal_hub::utils::UnionFind;
    ///
    /// // Initialize a new union-find.
    /// let mut union_find = UnionFind::new(5);
    ///
    /// // The new union-find contains only disjoint sets.
    /// for x in 0..union_find.len() {
    ///     assert_eq!(union_find.find(x), x);
    /// }
    ///
    /// // Merge items from 0 to 3.
    /// union_find.extend(0..4);
    ///
    /// // Now, items from 0 to 3 are in the same set.
    /// assert!(union_find.contains(0, 1));
    /// assert!(union_find.contains(0, 2));
    /// assert!(union_find.contains(0, 3));
    ///
    /// // Item 4 is is not in the same set
    /// assert!(!union_find.contains(0, 4));
    /// assert!(!union_find.contains(1, 4));
    /// assert!(!union_find.contains(2, 4));
    /// assert!(!union_find.contains(3, 4));
    /// ```
    ///
    #[inline]
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        // Get iterator.
        let mut iter = iter.into_iter();
        // Get first item as root.
        if let Some(x) = iter.next() {
            // For each consecutive pair of items ...
            for y in iter {
                // ... apply union algorithm.
                self.union(x, y);
            }
        }
    }
}
