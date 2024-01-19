#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash, rc::Rc};

use crate::topology::{OpenSet, PreSheaf, Section, Sheaf, TopologicalSpace};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Point<T: Eq + Hash + Clone>(T);

/// Trait for an n-cell in a cell complex. Inherits from TopologicalSpace, and adds the cell's identification map.
pub trait Cell<T: Eq + Hash + Clone>:
    TopologicalSpace<Point = Point<T>, OpenSet = HashSet<Point<T>>>
{
    fn identification(&self, skeleton: &Skeleton<T>) -> HashSet<<Self as TopologicalSpace>::Point>; //Should return a set of points in the cell identified with previous skeleton points, and thus shouldn't be included in the next skeleton.
}

/// A skeleton is a collection of cells, glued together by their identification maps.
pub struct Skeleton<T: Eq + Hash + Clone> {
    pub cells: Vec<Rc<dyn Cell<T, Point = Point<T>, OpenSet = HashSet<Point<T>>>>>,
    pub points: HashSet<Point<T>>,
    pub dim: usize,
    pub children: Vec<Skeleton<T>>,
}

impl<T: Eq + Hash + Clone> Skeleton<T> {
    pub fn new() -> Self {
        let cells = Vec::new();
        let points: HashSet<Point<T>> = HashSet::new();
        let dim = 0;
        Self {
            cells,
            points,
            dim,
            children: Vec::new(),
        }
    }

    // This function adds a child skeleton to the current skeleton. This is helpful in tracking the filtration of the cell complex.
    pub fn add_child(&mut self, child: Skeleton<T>) {
        self.children.push(child);
        self.dim = self.children.len()
    }

    // This function decides which points from the n-cell to include in the next skeleton based on the identification map of the specific n-cell implementation.
    pub fn include_cell(
        &mut self,
        cell: Rc<dyn Cell<T, Point = Point<T>, OpenSet = HashSet<Point<T>>>>,
    ) {
        for points in cell.points() {
            if !cell.identification(&self).contains(&points) {
                self.points.insert(points);
            } else {
            }
        }
        self.cells.push(cell);
    }
}
/// This struct of a cell complex contains the collect of cells, the set of points from each cell composing the complex, and the maximal dimension of the complex.
pub struct CellComplex<T: Eq + Hash + Clone> {
    pub cells: Vec<Rc<dyn Cell<T, Point = Point<T>, OpenSet = HashSet<Point<T>>>>>,
    pub points: HashSet<Point<T>>,
    pub dim: usize,
}

impl<T: Eq + Hash + Clone> CellComplex<T> {
    pub fn new(skeleton: Skeleton<T>, dim: usize) -> Self {
        Self {
            cells: skeleton.cells,
            points: skeleton.points,
            dim,
        }
    }
}

/// Implements OpenSets as HashSet<Point<T>> for the cell complex topology.
impl<T: Eq + Hash + Clone> OpenSet for HashSet<Point<T>> {
    type Point = Point<T>;
    fn intersect(&self, other: Self) -> Self {
        self.intersection(&other).cloned().collect()
    }
    fn union(&self, other: Self) -> Self {
        self.union(&other).cloned().collect()
    }
}

/// This implements the weak topology on the cell complex, where the open sets are the sets who's intersections are open in every cell.
impl<T: Eq + Hash + Clone> TopologicalSpace for CellComplex<T> {
    type Point = Point<T>;
    type OpenSet = HashSet<Point<T>>;

    fn points(&self) -> HashSet<Self::Point> {
        let mut points = HashSet::new();
        for cell in &self.cells {
            for point in cell.points() {
                points.insert(point);
            }
        }
        points
    }

    fn neighborhood(&self, point: Self::Point) -> Self::OpenSet {
        let mut neighborhood = HashSet::new();
        for cell in &self.cells {
            for neighbor in cell.neighborhood(point.clone()) {
                neighborhood.insert(neighbor);
            }
        }
        neighborhood
    }

    fn is_open(&self, set: HashSet<Point<T>>) -> bool {
        let mut status: Vec<bool> = Vec::new();
        for cell in &self.cells {
            let intersection: HashSet<_> = cell.points().intersection(&set).cloned().collect();
            if intersection.is_empty() {
                status.push(true);
            } else {
                status.push(cell.is_open(intersection));
            }
        }
        if status.into_iter().all(|x| x == true) {
            true
        } else {
            false
        }
    }
}

/// This implements the Presheaf conditions for the cell complex topology.
impl<T: Eq + Hash + Clone, S: Section> PreSheaf<S> for CellComplex<T> {
    type TopologicalSpace = CellComplex<T>;
    // Defines the restriction of a section to an open set.
    fn restriction(
        &self,
        set_to: &<<S as Section>::TopologicalSpace as TopologicalSpace>::OpenSet,
        section: &S,
    ) -> S {
        section.restrict(set_to.clone())
    }
}

/// This implements the Sheaf conditions for the cell complex topology.
impl<T: Eq + Hash + Clone, S: Section> Sheaf<S> for CellComplex<T> {
    // This function glues sections together on the intersection of their domains.
    fn gluing(
        &self,
        sections: Vec<(
            &<<S as Section>::TopologicalSpace as TopologicalSpace>::OpenSet,
            S,
        )>,
    ) -> Option<S> {
        for (i, (domain_i, section_i)) in sections.iter().enumerate() {
            for (j, (domain_j, section_j)) in sections.iter().enumerate() {
                if i != j {
                    // Compute the intersection of domain_i and domain_j
                    let intersection = domain_i.intersect((*domain_j).clone());

                    // Check if the sections are compatible on the intersection
                    if !section_i.is_compatible(intersection, section_j.clone()) {
                        return None;
                    }
                }
            }
        }
        let mut global_section: Option<S> = None;
        for (domain, section) in sections {
            if global_section.is_none() {
                global_section = Some(section);
            } else {
                global_section = global_section?.glue(domain.clone(), section);
            }
        }
        global_section
    }
    // This function checks if the sections are locally defined.
    fn uniqueness(
        &self,
        sections: Vec<(
            &<<S as Section>::TopologicalSpace as TopologicalSpace>::OpenSet,
            S,
            S,
        )>,
    ) -> bool {
        let mut all_unique = true;

        for (domain, section_i, section_j) in sections {
            if section_i.restrict(domain.clone()) == section_j.restrict(domain.clone()) {
                all_unique = false;
            }
        }
        all_unique
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_basic_complex() {
        struct CellStruct {
            points: HashSet<Point<usize>>,
            dim: usize,
        }

        impl TopologicalSpace for CellStruct { // This is a horribly useless topology for testing purposes.
            type Point = Point<usize>;
            type OpenSet = HashSet<Point<usize>>;
            fn points(&self) -> HashSet<Self::Point> {
                self.points.clone()
            }
            fn neighborhood(&self, _point: Self::Point) -> Self::OpenSet {
                let mut neighborhood = HashSet::new();
                for neighbor in self.points.clone() {
                    neighborhood.insert(neighbor);
                }
                neighborhood
            }
            fn is_open(&self, _set: Self::OpenSet) -> bool {
                true
            }
        }
        impl Cell<usize> for CellStruct {
            fn identification(&self, skeleton: &Skeleton<usize>) -> HashSet<Point<usize>> {
                let mut identification = HashSet::new();
                for point in skeleton.points.clone() {
                    identification.insert(point);
                }
                identification
            }
        }
        let mut skeleton_0: Skeleton<usize> = Skeleton::new();
        let first_cell = Rc::new(CellStruct {
            points: vec![Point(0), Point(1)].into_iter().collect(),
            dim: 0,
        });
        skeleton_0.include_cell(first_cell);

    }
}