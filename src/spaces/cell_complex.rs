use std::collections::HashSet;

use crate::topology::TopologicalSpace;

// Trait for cells of any dimension with any identification function
pub trait Cell<T>: TopologicalSpace<Point = Vec<T>, OpenSet = HashSet<Vec<T>>> {
    type BoundaryPoint;
    fn cell_points(&self) -> Vec<<Self as TopologicalSpace>::Point> {
        self.points().into_iter().collect()
    }
    fn cell_neighborhood(&self, point: <Self as TopologicalSpace>::Point) -> Vec<<Self as TopologicalSpace>::Point> {
        self.neighborhood(point).into_iter().collect()
    }
    fn identification(&self, skeleton: Skeleton<T>);
}

pub struct Skeleton<T> {
    pub cells: HashSet<Box<dyn Cell<T, Point = Vec<T>, BoundaryPoint = Vec<T>>>>,
    pub dim: usize,
    pub children : Vec<Skeleton<T>>,
}

pub struct CellComplex<T> {
    pub cells: HashSet<Box<dyn Cell<T, Point = Vec<T>, BoundaryPoint = Vec<T>>>>,
    pub dim: usize,
}

impl<T> CellComplex<T> {
    pub fn new(cells: HashSet<Box<dyn Cell<T, Point = Vec<T>, BoundaryPoint = Vec<T>>>>, dim: usize) -> Self {
        Self { cells, dim }
    }
}