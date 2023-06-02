use std::{cell::RefCell, rc::{Weak, Rc}, collections::HashSet};


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
    type Element;
    type Operation;
    fn germ(&self) -> Self::Element;
    fn new(germ: Self::Element, preimage: Vec<Weak<Self>>, image: Vec<Rc<Self>>) -> Self;
}

pub trait PreSheaf {
    type Section;
    type Restriction;
    fn restriction(&self, section_from: Self::Section, section_to: Self::Section) -> Self::Restriction;
}

pub trait Sheaf: PreSheaf {
    type Stalk;
    type Basis;
    fn stalks(&self) -> Vec<Self::Stalk>;
    fn basis(&self) -> Vec<Self::Basis>;
    fn new(stalks: Vec<Self::Stalk>, basis: Vec<Self::Basis>) -> Self;
}

pub trait CellComplex {
    type Cell;
    fn attachment(&self, cell: Self::Cell) -> Self::Cell;
}

// pub struct Stalk<T> {
//     pub germ: Vec<T>,
//     pub preimage: RefCell<Vec<Weak<Stalk<T>>>>,
//     pub image: RefCell<Vec<Rc<Stalk<T>>>>, // TODO: Do we want to store maps of between the different stalks?
// }

// impl<T> Stalk<T> {
//     pub fn new(germ: Vec<T>) -> Self {
//         Self { germ, preimage: RefCell::new(vec![]), image: RefCell::new(vec![]) }
//     }
// }

// pub struct Sheaf<T> {
//     pub sections: HashSet<Box<<Self as Presheaf<T>>::Section>>,
//     pub stalks: HashSet<Stalk<T>>,
//     pub basis: HashSet<HashSet<Stalk<T>>>,
// }

// impl<T> Default for Sheaf {
//     fn default() -> Self {
//         Self { sections: HashSet::new(), stalks: HashSet::new(), basis: HashSet::new() }
//     }
// }

// impl<T> Sheaf<T> {
//     pub fn new(sections: HashSet<Box<<Self as Presheaf<T>>::Section>>, stalks: HashSet<Stalk<T>>, basis: HashSet<HashSet<Stalk<T>>>) -> Self {
//         Self { sections, stalks, basis }
//     }
//     pub fn add_stalk(&mut self, stalk: Stalk<T>) {
//         todo!()
//         // self.stalks.insert(stalk);
//         // TODO: Also needs to be added to the basis.
//     }
// }

// impl <T> Presheaf<T> for Sheaf<T> {
//     type Section = Box<dyn Fn(&HashSet<Stalk<T>>) -> T>;
//     fn restriction(&self, section_from: Box<Self::Section>, section_to: Box<Self::Section>) -> Box<Self::Section> {
//         todo!();
//     }
// }

// pub trait Presheaf<T> {
//     type Section:  Fn(&HashSet<Stalk<T>>) -> T;
//     fn restriction(&self, section_from: Box<Self::Section>, section_to: Box<Self::Section>) -> Box<Self::Section>;
// }

// Note to self: Would be cool to build a general tensor type so that we can build things like that. (Look at num-complex for inspiration)

#[cfg(test)]
mod tests {
    use super::*;

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