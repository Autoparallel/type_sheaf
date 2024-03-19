#![allow(dead_code)]

// TODO: It would be nice to use const generic expressions here, but they are
// not stable yet and tough to work with.
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::{Rc, Weak},
};

use crate::topology::{OpenSet, PreSheaf, Section, Sheaf, TopologicalSpace};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub id: String,
    pub dim: usize,
    attachments: Vec<String>,
}

impl Cell {
    pub fn new(id: &str, dim: usize) -> Self {
        Self {
            id: id.to_string(),
            dim,
            attachments: Vec::new(),
        }
    }

    pub fn get_attachments(&self) -> &Vec<String> {
        &self.attachments
    }
}

impl IntoIterator for Cell {
    type Item = Cell;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl<const TOP_DIM: usize> OpenSet for CellComplex<TOP_DIM> {
    type Element = Cell;

    fn intersect(&self, other: Self) -> Self {
        self.intersection(&other).cloned().collect()
    }

    fn union(&self, other: Self) -> Self {
        self.union(&other).cloned().collect()
    }
}

pub struct CellComplex<const TOP_DIM: usize> {
    skeletons: [HashMap<String, Cell>; TOP_DIM],
}

impl<const N: usize> CellComplex<N> {
    pub fn new() -> Self {
        Self {
            skeletons: core::array::from_fn(|_| HashMap::new()),
        }
    }

    pub const fn dim(&self) -> usize {
        N
    }

    pub fn add_cell(&mut self, cell: Cell) -> &mut Cell {
        let dim = cell.dim;
        assert!(dim <= N, "Cell dimension exceeds complex dimension");
        let id = cell.id.clone();
        self.skeletons[dim].insert(cell.id.clone(), cell).unwrap();
        self.skeletons[dim].get_mut(&id).unwrap()
    }

    pub fn get_cell(&self, dim: usize, id: &str) -> Option<&Cell> {
        self.skeletons.get(dim)?.get(id)
    }

    pub fn get_cell_mut(&mut self, dim: usize, id: &str) -> Option<&mut Cell> {
        self.skeletons.get_mut(dim)?.get_mut(id)
    }

    pub fn attach_cells(&mut self, k_cell: &mut Cell, k_minus_one_cell: &mut Cell) {
        k_cell.attachments.push(k_minus_one_cell.id.clone());
        k_minus_one_cell.attachments.push(k_cell.id.clone());
    }
}

impl<const N: usize> TopologicalSpace for CellComplex<N> {
    type Element = Cell;
    type OpenSet = HashSet<Cell>;

    fn elements(&self) -> HashSet<Self::Element> {
        let mut elements = HashSet::new();
        for skeleton in &self.skeletons {
            for cell in skeleton.values() {
                elements.insert(cell.id.clone());
            }
        }
        elements
    }

    fn neighborhood(&self, point: Self::Element) -> Self::OpenSet {
        let mut neighborhood = HashSet::new();
        for skeleton in &self.skeletons {
            for cell in skeleton.values() {
                if cell.attachments.contains(&point) {
                    neighborhood.insert(cell.id.clone());
                }
            }
        }
        neighborhood
    }

    fn is_open(&self, set: Self::OpenSet) -> bool {
        let mut status: Vec<bool> = Vec::new();
        for skeleton in &self.skeletons {
            let intersection: HashSet<_> = skeleton
                .values()
                .filter(|cell| set.contains(&cell.id))
                .map(|cell| cell.id.clone())
                .collect();
            if intersection.is_empty() {
                status.push(true);
            } else {
                status.push(intersection == set);
            }
        }
        status.into_iter().all(|x| x)
    }
}

// /// This struct of a cell complex contains the collect of cells, the set of
// /// points from each cell composing the complex, and the maximal dimension of
// /// the complex.
// pub struct CellComplex<T: Eq + Hash + Clone> {
//     #[allow(clippy::type_complexity)]
//     pub cells: Vec<Rc<dyn Cell<T, Element = Point<T>, OpenSet =
// HashSet<Point<T>>>>>,     pub points: HashSet<Point<T>>,
//     pub dim: usize,
// }

// impl<T: Eq + Hash + Clone> CellComplex<T> {
//     pub fn new(skeleton: Skeleton<T>, dim: usize) -> Self {
//         Self {
//             cells: skeleton.cells,
//             points: skeleton.points,
//             dim,
//         }
//     }
// }

// /// Implements OpenSets as HashSet<Point<T>> for the cell complex topology.
// impl<T: Eq + Hash + Clone> OpenSet for HashSet<Point<T>> {
//     type Element = Point<T>;
//     fn intersect(&self, other: Self) -> Self {
//         self.intersection(&other).cloned().collect()
//     }
//     fn union(&self, other: Self) -> Self {
//         self.union(&other).cloned().collect()
//     }
// }

// /// This implements the weak topology on the cell complex, where the open
// sets /// are the sets who's intersections are open in every cell.
// impl<T: Eq + Hash + Clone> TopologicalSpace for CellComplex<T> {
//     type Element = Point<T>;
//     type OpenSet = HashSet<Point<T>>;

//     fn elements(&self) -> HashSet<Self::Element> {
//         let mut points = HashSet::new();
//         for cell in &self.cells {
//             for point in cell.elements() {
//                 points.insert(point);
//             }
//         }
//         points
//     }

//     fn neighborhood(&self, point: Self::Element) -> Self::OpenSet {
//         let mut neighborhood = HashSet::new();
//         for cell in &self.cells {
//             for neighbor in cell.neighborhood(point.clone()) {
//                 neighborhood.insert(neighbor);
//             }
//         }
//         neighborhood
//     }

//     fn is_open(&self, set: HashSet<Point<T>>) -> bool {
//         let mut status: Vec<bool> = Vec::new();
//         for cell in &self.cells {
//             let intersection: HashSet<_> =
// cell.elements().intersection(&set).cloned().collect();             if
// intersection.is_empty() {                 status.push(true);
//             } else {
//                 status.push(cell.is_open(intersection));
//             }
//         }
//         status.into_iter().all(|x| x)
//     }
// }

// /// This implements the Presheaf conditions for the cell complex topology.
// impl<T: Eq + Hash + Clone, S: Section> PreSheaf<S> for CellComplex<T> {
//     type TopologicalSpace = CellComplex<T>;
//     // Defines the restriction of a section to an open set.
//     fn restriction(
//         &self,
//         set_to: &<<S as Section>::TopologicalSpace as
// TopologicalSpace>::OpenSet,         section: &S,
//     ) -> S {
//         section.restrict(set_to.clone())
//     }
// }

// /// This implements the Sheaf conditions for the cell complex topology.
// impl<T: Eq + Hash + Clone, S: Section> Sheaf<S> for CellComplex<T> {
//     // This function glues sections together on the intersection of their
// domains.     fn gluing(
//         &self,
//         sections: Vec<(
//             &<<S as Section>::TopologicalSpace as TopologicalSpace>::OpenSet,
//             S,
//         )>,
//     ) -> Option<S> {
//         for (i, (domain_i, section_i)) in sections.iter().enumerate() {
//             for (j, (domain_j, section_j)) in sections.iter().enumerate() {
//                 if i != j {
//                     // Compute the intersection of domain_i and domain_j
//                     let intersection =
// domain_i.intersect((*domain_j).clone());

//                     // Check if the sections are compatible on the
// intersection                     if !section_i.is_compatible(intersection,
// section_j.clone()) {                         return None;
//                     }
//                 }
//             }
//         }
//         let mut global_section: Option<S> = None;
//         for (domain, section) in sections {
//             if global_section.is_none() {
//                 global_section = Some(section);
//             } else {
//                 global_section = global_section?.glue(domain.clone(),
// section);             }
//         }
//         global_section
//     }
//     // This function checks if the sections are locally defined.
//     fn uniqueness(
//         &self,
//         sections: Vec<(
//             &<<S as Section>::TopologicalSpace as TopologicalSpace>::OpenSet,
//             S,
//             S,
//         )>,
//     ) -> bool {
//         let mut all_unique = true;

//         for (domain, section_i, section_j) in sections {
//             if section_i.restrict(domain.clone()) ==
// section_j.restrict(domain.clone()) {                 all_unique = false;
//             }
//         }
//         all_unique
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn build_basic_complex() {
//     //     struct CellStruct {
//     //         points: HashSet<Point<i32>>,
//     //         dim: usize,
//     //     }

//     //     impl TopologicalSpace for CellStruct {
//     //         type Element = Point<i32>;
//     //         type OpenSet = HashSet<Point<i32>>;
//     //         fn elements(&self) -> HashSet<Self::Element> {
//     //             self.points.clone()
//     //         }
//     //         fn neighborhood(&self, point: Self::Element) -> Self::OpenSet
// {     //             let mut neighborhood = HashSet::new();
//     //             for neighbor in self.points.clone() {
//     //                 if (neighbor.0 - point.0).abs() <= 1_i32 {
//     //                     // neighborhood is the set of points within 1 of
// the     // given point                     neighborhood.insert(neighbor);
//     //                 }
//     //             }
//     //             neighborhood
//     //         }
//     //         fn is_open(&self, set: Self::OpenSet) -> bool {
//     //             for point in set.clone() {
//     //                 if !self.points.contains(&point) {
//     //                     return false;
//     //                 }
//     //             }
//     //             if set == self.points {
//     //                 return true;
//     //             } else if set.is_empty() {
//     //                 return true;
//     //             } else {
//     //                 for points in &self.points {
//     //                     let mut is_neighborhood = true;
//     //                     for neighbor in &set {
//     //                         if (neighbor.0 - points.0).abs() > 1_i32 {
//     //                             is_neighborhood = false;
//     //                             break;
//     //                         }
//     //                     }
//     //                     if is_neighborhood {
//     //                         return true;
//     //                     }
//     //                 }
//     //                 false
//     //             }
//     //         }
//     //     }
//     //     impl Cell<i32> for CellStruct {
//     //         fn identification(&self, skeleton: &Skeleton<i32>) ->
//     // HashSet<Point<i32>> {             let mut identification =
//     // HashSet::new();             for point in skeleton.points.clone() {
//     //                 identification.insert(point);
//     //             }
//     //             identification
//     //         }
//     //     }
//     //     let mut skeleton_0: Skeleton<i32> = Skeleton::new();
//     //     let first_cell = Rc::new(CellStruct {
//     //         points: vec![Point(0), Point(1), Point(2), Point(3)]
//     //             .into_iter()
//     //             .collect(),
//     //         dim: 0,
//     //     });
//     //     skeleton_0.include_cell(first_cell);
//     // }

//     // Sheaf over a 2-sphere cell complex
//     // This cell complex has 2 0-cells, 2 1-cells, and 2 2-cell.
//     // Attachments:
//     // * e_1 -> v_1
//     // * e_1 -> v_2
//     // * e_2 -> v_1
//     // * e_2 -> v_2
//     // * f_1 -> e_1
//     // * f_1 -> e_2
//     // * f_2 -> e_1
//     // * f_2 -> e_2
//     // Fig. 3.5
//     #[test]
//     fn two_sphere() {}
// }
