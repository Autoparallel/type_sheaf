use std::hash::Hash;

use super::*;
use crate::topology::{OpenSet, Section};

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

impl OpenSet for HashSet<usize> {
    type Point = usize;

    fn intersect(&self, other: Self) -> Self {
        self.intersection(&other).cloned().collect()
    }
    fn union(&self, other: Self) -> Self {
        self.union(&other).cloned().collect()
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

    fn is_open(&self, _set: Self::OpenSet) -> bool {
        true
    }
}

impl MetricSpace for UndirectedGraph {
    type Distance = Option<usize>;

    fn distance(
        &self,
        point_a: <Self as TopologicalSpace>::Point,
        point_b: <Self as TopologicalSpace>::Point,
    ) -> Self::Distance {
        let mut visited = HashSet::new();
        let mut queue = vec![(point_a, 0)];
        while let Some((point, distance)) = queue.pop() {
            if point == point_b {
                return Some(distance);
            }
            visited.insert(point);
            for neighbor in self.neighborhood(point) {
                if !visited.contains(&neighbor) {
                    queue.push((neighbor, distance + 1));
                }
            }
        }
        None
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Data<T: Eq + Hash + Clone>(T);

impl<T: Eq + Hash + Clone> Section for HashMap<usize, Data<T>> {
    type TopologicalSpace = UndirectedGraph;

    fn restrict(&self, set_to: std::collections::HashSet<usize>) -> Self {
        let mut restricted_section = HashMap::new();
        for point in set_to.clone() {
            if let Some(value) = self.get(&point) {
                restricted_section.insert(point, value.clone());
            }
        }
        restricted_section
    }
    fn glue(&self, domain: std::collections::HashSet<usize>, section: Self) -> Option<Self> {
        let mut glued_section = self.clone();
        for point in domain.clone() {
            if let Some(value) = section.get(&point) {
                glued_section.insert(point, value.clone());
            }
        }
        Some(glued_section)
    }
}

impl<T: Eq + Hash + Clone> PreSheaf<HashMap<usize, Data<T>>> for UndirectedGraph {
    type TopologicalSpace = Self;

    fn restriction(
        &self,
        set_to: &<Self::TopologicalSpace as TopologicalSpace>::OpenSet,
        section: &HashMap<<Self::TopologicalSpace as TopologicalSpace>::Point, Data<T>>,
    ) -> HashMap<<Self::TopologicalSpace as TopologicalSpace>::Point, Data<T>> {
        section.restrict(set_to.clone())
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
        vertices.insert(5);

        let mut edges = HashSet::new();
        edges.insert((1, 2));
        edges.insert((2, 3));
        edges.insert((3, 4));

        UndirectedGraph::new(vertices, edges)
    }

    #[test]
    fn graph_builds() {
        let graph = create_graph();
        assert_eq!(graph.vertices.len(), 5);
        assert_eq!(graph.edges.len(), 3);
    }

    #[test]
    fn graph_points() {
        let graph = create_graph();
        assert_eq!(graph.points(), graph.vertices);
    }

    #[test]
    fn neighborhood() {
        let graph = create_graph();
        assert_eq!(
            graph.neighborhood(1),
            vec![2].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(
            graph.neighborhood(2),
            vec![1, 3].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(
            graph.neighborhood(3),
            vec![2, 4].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(
            graph.neighborhood(4),
            vec![3].into_iter().collect::<HashSet<_>>()
        );
    }

    #[test]
    fn distance() {
        let graph = create_graph();
        assert_eq!(graph.distance(1, 1), Some(0));
        assert_eq!(graph.distance(1, 2), Some(1));
        assert_eq!(graph.distance(1, 3), Some(2));
        assert_eq!(graph.distance(1, 4), Some(3));
        assert_eq!(graph.distance(1, 5), None);
    }

    #[test]
    fn restriction() {
        let graph = create_graph();
        let mut section = HashMap::new();
        section.insert(1, Data::<i32>(1));
        section.insert(2, Data::<i32>(2));
        section.insert(3, Data::<i32>(3));

        let set_to = vec![1, 2].into_iter().collect::<HashSet<_>>();
        let restricted_section = graph.restriction(&set_to, &section);
        println!("{:?}", restricted_section);
        println!("{:?}", restricted_section.get(&1).unwrap());
    }
}
