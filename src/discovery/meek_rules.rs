use itertools::Itertools;

use crate::{
    graphs::{PartiallyDirected, PartiallyDirectedGraph},
    Adj, Ch, Ne, Pa, V,
};

pub struct MeekRules;

impl MeekRules {
    pub fn apply_1<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ne!(g, x).collect_vec() {
                if iter_set::intersection(Adj!(g, z), Pa!(g, x))
                    .next()
                    .is_none()
                {
                    g.set_directed_edge(x, z);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn apply_2<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ch!(g, x).collect_vec() {
                for y in iter_set::intersection(Ne!(g, z), Pa!(g, x)).collect_vec() {
                    g.set_directed_edge(y, z);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn apply_3<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            for z in Ne!(g, x).collect_vec() {
                let intersection = iter_set::intersection(Ne!(g, z), Pa!(g, x));
                // Look for a non-adjacent couple of parents of `x`
                if intersection
                    .combinations(2)
                    .any(|ab| !g.is_adjacent(ab[0], ab[1]))
                {
                    g.set_directed_edge(z, x);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn apply_4<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ne!(g, x).collect_vec() {
                if iter_set::intersection(
                    Ne!(g, z),
                    Pa!(g, x).flat_map(|parent| Pa!(g, parent).filter(|&y| !g.is_adjacent(y, x))),
                )
                .next()
                .is_some()
                {
                    g.set_directed_edge(z, x);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    #[inline]
    pub fn apply_until_3<P>(mut g: P) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        let mut is_closed = false;

        let (mut is_closed_1, mut is_closed_2, mut is_closed_3);
        while !is_closed {
            (g, is_closed_1) = Self::apply_1(g);
            (g, is_closed_2) = Self::apply_2(g);
            (g, is_closed_3) = Self::apply_3(g);
            is_closed = is_closed_1 && is_closed_2 && is_closed_3;
        }

        g
    }

    #[inline]
    pub fn apply_until_4<P>(mut g: P) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        let mut is_closed = false;

        let (mut is_closed_1, mut is_closed_2, mut is_closed_3, mut is_closed_4);
        while !is_closed {
            (g, is_closed_1) = Self::apply_1(g);
            (g, is_closed_2) = Self::apply_2(g);
            (g, is_closed_3) = Self::apply_3(g);
            (g, is_closed_4) = Self::apply_4(g);
            is_closed = is_closed_1 && is_closed_2 && is_closed_3 && is_closed_4;
        }

        g
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn apply_1_base_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C"]);
        // Set edges.
        g.add_directed_edge(0, 1);
        g.add_directed_edge(1, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_1(g);

        // Assert the graph is closed.
        assert!(is_closed);
        // Test for directed edges.
        assert!(g.has_directed_edge(0, 1));
        assert!(g.has_directed_edge(1, 2));
    }

    #[test]
    fn apply_1_general_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F"]);
        // Set edges.
        g.add_undirected_edge(0, 3);
        g.add_undirected_edge(0, 4);
        g.add_undirected_edge(1, 2);
        g.add_undirected_edge(1, 4);
        g.add_undirected_edge(3, 5);
        g.add_undirected_edge(4, 5);
        g.add_directed_edge(1, 0);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_1(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges.
        assert!(g.has_undirected_edge(0, 4));
        assert!(g.has_undirected_edge(1, 2));
        assert!(g.has_undirected_edge(1, 4));
        // Test for directed edges.
        assert!(g.has_directed_edge(0, 3));
        assert!(g.has_directed_edge(3, 5));
        assert!(g.has_directed_edge(5, 4));
    }

    #[test]
    fn apply_2_base_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C"]);
        // Set edges.
        g.add_undirected_edge(0, 2);
        g.add_directed_edge(0, 1);
        g.add_directed_edge(1, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_2(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for directed edges.
        assert!(g.has_directed_edge(0, 1));
        assert!(g.has_directed_edge(0, 2));
        assert!(g.has_directed_edge(1, 2));
    }

    #[test]
    fn apply_2_general_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F"]);
        // Set edges.
        g.add_undirected_edge(0, 4);
        g.add_undirected_edge(1, 2);
        g.add_undirected_edge(1, 3);
        g.add_directed_edge(0, 2);
        g.add_directed_edge(1, 0);
        g.add_directed_edge(2, 3);
        g.add_directed_edge(4, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_2(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges.
        assert!(g.has_undirected_edge(0, 4));
        // Test for directed edges.
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(1, 3));
    }

    #[test]
    fn apply_3_base_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F"]);
        // Set edges.
        g.add_undirected_edge(0, 1);
        g.add_undirected_edge(0, 2);
        g.add_undirected_edge(0, 3);
        g.add_directed_edge(1, 2);
        g.add_directed_edge(3, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_3(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges.
        assert!(g.has_undirected_edge(0, 1));
        assert!(g.has_undirected_edge(0, 3));
        // Test for directed edges.
        assert!(g.has_directed_edge(0, 2));
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(3, 2));
    }

    #[test]
    fn apply_3_general_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F", "G", "H"]);
        // Set edges.
        g.add_undirected_edge(0, 1);
        g.add_undirected_edge(0, 4);
        g.add_undirected_edge(0, 5);
        g.add_undirected_edge(1, 2);
        g.add_undirected_edge(1, 3);
        g.add_undirected_edge(1, 4);
        g.add_undirected_edge(2, 5);
        g.add_undirected_edge(2, 6);
        g.add_undirected_edge(4, 6);
        g.add_undirected_edge(5, 6);
        g.add_directed_edge(2, 0);
        g.add_directed_edge(3, 0);
        g.add_directed_edge(6, 0);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_3(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges
        assert!(g.has_undirected_edge(0, 5));
        // Test for directed edges
        assert!(g.has_directed_edge(1, 0));
        assert!(g.has_directed_edge(4, 0));
    }

    #[test]
    fn apply_4_base_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F", "G", "H"]);
        // Set edges.
        g.add_undirected_edge(0, 3);
        g.add_undirected_edge(1, 3);
        g.add_undirected_edge(2, 3);
        g.add_directed_edge(0, 1);
        g.add_directed_edge(1, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_4(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges
        assert!(g.has_undirected_edge(0, 3));
        // Test for directed edges
        assert!(g.has_directed_edge(0, 1));
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(3, 2));

        // Initialize another empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F", "G", "H"]);
        // Set edges.
        g.add_undirected_edge(0, 3);
        g.add_undirected_edge(2, 3);
        g.add_directed_edge(0, 1);
        g.add_directed_edge(1, 2);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_4(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges
        assert!(g.has_undirected_edge(0, 3));
        // Test for directed edges
        assert!(g.has_directed_edge(0, 1));
        assert!(g.has_directed_edge(1, 2));
        assert!(g.has_directed_edge(3, 2));
    }

    #[test]
    fn apply_4_general_case() {
        // Initialize empty PGraph.
        let mut g = PGraph::empty(["A", "B", "C", "D", "E", "F", "G", "H"]);
        // Set edges.
        g.add_undirected_edge(0, 2);
        g.add_undirected_edge(0, 3);
        g.add_undirected_edge(0, 5);
        g.add_undirected_edge(0, 7);
        g.add_undirected_edge(2, 5);
        g.add_undirected_edge(3, 4);
        g.add_undirected_edge(6, 7);
        g.add_directed_edge(1, 0);
        g.add_directed_edge(2, 1);
        g.add_directed_edge(3, 7);
        g.add_directed_edge(4, 1);
        g.add_directed_edge(6, 3);

        // Apply Meek's rules.
        let (g, is_closed) = MeekRules::apply_4(g);

        // Assert the graph is not closed.
        assert!(!is_closed);
        // Test for undirected edges.
        assert!(g.has_undirected_edge(0, 5));
        // Test for directed edges.
        assert!(g.has_directed_edge(7, 0));
        assert!(g.has_directed_edge(3, 0));
    }
}
