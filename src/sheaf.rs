use std::{cell::RefCell, rc::{Weak, Rc}, collections::HashSet};

pub struct Stalk<T> {
    pub germ: Vec<T>,
    pub preimage: RefCell<Vec<Weak<Stalk<T>>>>,
    pub image: RefCell<Vec<Rc<Stalk<T>>>>, // TODO: Do we want to store maps of between the different stalks?
}

impl<T> Stalk<T> {
    pub fn new(germ: Vec<T>) -> Self {
        Self { germ, preimage: RefCell::new(vec![]), image: RefCell::new(vec![]) }
    }
}

pub struct Sheaf<T> {
    pub sections: HashSet<Box<<Self as Presheaf<T>>::Section>>,
    pub stalks: HashSet<Stalk<T>>,
    pub basis: HashSet<HashSet<Stalk<T>>>,
}

impl <T> Presheaf<T> for Sheaf<T> {
    type Section = Box<dyn Fn(&HashSet<Stalk<T>>) -> T>;
    fn restriction(&self, section_from: Box<Self::Section>, section_to: Box<Self::Section>) -> Box<Self::Section> {
        todo!();
    }
}

pub trait Presheaf<T> {
    type Section:  Fn(&HashSet<Stalk<T>>) -> T;
    fn restriction(&self, section_from: Box<Self::Section>, section_to: Box<Self::Section>) -> Box<Self::Section>;
}

// Note to self: Would be cool to build a general tensor type so that we can build things like that. (Look at num-complex for inspiration)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stalk = Stalk::new(vec![1, 2, 3]);
        assert_eq!(stalk.germ, vec![1, 2, 3]);
    }

    #[test]
    fn compact_lattice() {
        todo!();
    }
}