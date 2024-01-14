#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash, rc::Rc};

use crate::topology::TopologicalSpace;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Wrapper<T: Eq + Hash + Clone> {
    Point(T),
}

// Trait for an n-cell in a cell complex. Inherits from TopologicalSpace, and adds the cell's identification map.
pub trait Cell<T: Eq + Hash + Clone>:
    TopologicalSpace<Point = Wrapper<T>, OpenSet = HashSet<Wrapper<T>>>
{
    fn cell_points(&self) -> Vec<<Self as TopologicalSpace>::Point> {
        self.points().into_iter().collect()
    }
    fn cell_point_neighborhood(
        &self,
        point: <Self as TopologicalSpace>::Point,
    ) -> Vec<<Self as TopologicalSpace>::Point> {
        self.neighborhood(point).into_iter().collect()
    }
    fn identification(&self, skeleton: &Skeleton<T>) -> HashSet<<Self as TopologicalSpace>::Point>; //Should return a set of points in the cell identified with previous skeleton points, and thus shouldn't be included in the next skeleton.
}

// A skeleton is a collection of cells, glued together by their identification maps.
pub struct Skeleton<T: Eq + Hash + Clone> {
    pub cells: Vec<Rc<dyn Cell<T, Point = Wrapper<T>, OpenSet = HashSet<Wrapper<T>>>>>,
    pub points: HashSet<Wrapper<T>>,
    pub dim: usize,
    pub children: Vec<Skeleton<T>>,
}

impl<T: Eq + Hash + Clone> Skeleton<T> {
    pub fn new() -> Self {
        let cells = Vec::new();
        let points: HashSet<Wrapper<T>> = HashSet::new();
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
        cell: Rc<dyn Cell<T, Point = Wrapper<T>, OpenSet = HashSet<Wrapper<T>>>>,
    ) {
        for points in cell.cell_points() {
            if !cell.identification(&self).contains(&points) {
                self.points.insert(points);
            } else {
            }
        }
        self.cells.push(cell);
    }
}
// This struct of a cell complex contains the collect of cells, the set of points from each cell composing the complex, and the maximal dimension of the complex.
pub struct CellComplex<T: Eq + Hash + Clone> {
    pub cells: Vec<Rc<dyn Cell<T, Point = Wrapper<T>, OpenSet = HashSet<Wrapper<T>>>>>,
    pub points: HashSet<Wrapper<T>>,
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

// This implements the weak topology on the cell complex, where the open sets are the sets who's intersections are open in every cell.
impl<T: Eq + Hash + Clone> TopologicalSpace for CellComplex<T> {
    type Point = Wrapper<T>;
    type OpenSet = HashSet<Wrapper<T>>;

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

    fn is_open(&self, set: HashSet<Wrapper<T>>) -> bool {
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
