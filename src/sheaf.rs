use std::{cell::RefCell, rc::{Weak, Rc}, collections::HashSet, any::Any};


// Thinking out loud here with more advanced types:
pub trait TopologicalSpace {
    type Point;
    type Neighborhood;
    fn neighborhood(&self, point: Self::Point) -> Self::Neighborhood;
}

pub trait MetricSpace: TopologicalSpace {
    type Distance;
    fn distance(&self, point_a: <Self as TopologicalSpace>::Point, point_b: <Self as TopologicalSpace>::Point) -> Self::Distance;
}

pub trait Fiber {
    type Element;
    type Operation;
    fn projection(element: Self::Element) -> Self::Element;
    fn new(elements: Self::Element, op: Self::Operation) -> Self;
}

pub trait Stalk {
    type Data: 'static + std::any::Any;
    fn germ(&self) -> Self::Data;
}

pub trait PreSheaf {
    type Stalks;
    type Section;
    fn restriction(&self, section_from: Self::Section, section_to: Self::Section) -> Self::Section;
}

pub struct CellularSheaf {
    pub sections: HashSet<Box<dyn Fn(&HashSet<Box<dyn Stalk<Data = Box<dyn Any>>>>) -> HashSet<Box<dyn Any>>>>,
    pub stalks: HashSet<Box<dyn Stalk<Data = Box<dyn Any>>>>,
    pub basis: HashSet<HashSet<Box<dyn Stalk<Data = dyn Any>>>>,
}

impl PreSheaf for CellularSheaf {
    type Stalks = HashSet<Box<dyn Stalk<Data = Box<dyn Any>>>>;
    type Section = Box<dyn Fn(&HashSet<Box<dyn Stalk<Data = Box<dyn Any>>>>) -> Box<dyn Any>>;

    fn restriction(&self, section_from: Self::Section, section_to: Self::Section) -> Self::Section {
        todo!()
    }
}


pub trait CellComplex {
    type Cell;
    fn attachment(&self, cell: Self::Cell) -> Self::Cell;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStalk {
        data: i32,

    }

    // #[test]
    // fn it_works() {
    //     let stalk = Stalk::new(vec![1, 2, 3]);
    //     assert_eq!(stalk.germ, vec![1, 2, 3]);
    // }

    // #[test]
    // fn finite_set() {
    //     Sheaf::<dyn algebra::structure::GroupAbelian>::default();
    // }
}