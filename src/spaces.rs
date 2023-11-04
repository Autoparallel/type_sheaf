use std::{collections::HashSet, hash::Hash};

use crate::sheaf::TopologicalSpace;

pub trait CellComplex {
    type Cell;
    fn attachment(&self, cell: Self::Cell) -> Self::Cell;
}

pub struct UndirectedGraph {
    pub vertices: HashSet<usize>,
    pub edges: HashSet<(usize, usize)>,
}

impl UndirectedGraph {
    pub fn new(vertices: HashSet<usize>, edges: HashSet<(usize, usize)>) -> Self {
        let edges = edges
            .into_iter()
            .map(|(a, b)| if a <= b { (a, b) } else { (b, a) })
            .collect::<HashSet<_>>();

        assert!(
            edges
                .iter()
                .all(|(a, b)| vertices.contains(a) && vertices.contains(b)),
            "All edges must be between vertices",
        );
        Self { vertices, edges }
    }
}

impl TopologicalSpace for UndirectedGraph {
    type Point = usize;

    type OpenSet = HashSet<Self::Point>;

    fn points(&self) -> HashSet<Self::Point> {
        self.vertices.clone()
    }

    fn neighborhood(&self, point: Self::Point) -> Self::OpenSet {
        self.edges
            .iter()
            .filter_map(|(a, b)| {
                if *a == point {
                    Some(*b)
                } else if *b == point {
                    Some(*a)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_graph() -> UndirectedGraph {
        let mut vertices = HashSet::new();
        vertices.insert(1);
        vertices.insert(2);
        vertices.insert(3);
        vertices.insert(4);

        let mut edges = HashSet::new();
        edges.insert((1, 2));
        edges.insert((2, 3));
        edges.insert((3, 4));

        UndirectedGraph::new(vertices, edges)
    }
}
