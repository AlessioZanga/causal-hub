#[derive(Clone, Debug)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    #[inline]
    pub fn new(size: usize) -> Self {
        // Check if size is valid.
        assert!(size > 0);

        Self {
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.parents.len()
    }

    #[inline]
    pub fn contains(&self, x: usize, y: usize) -> bool {
        // Check if x and y are in the same set.
        self.find(x) == self.find(y)
    }

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
