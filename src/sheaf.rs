use std::collections::HashMap;
use std::{any::Any, collections::HashSet};

pub trait TopologicalSpace {
    type Point;
    type OpenSet: IntoIterator<Item = Self::Point>;
    fn points(&self) -> HashSet<Self::Point>;
    fn neighborhood(&self, point: Self::Point) -> Self::OpenSet;
}

pub trait PreSheaf {
    type TopologicalSpace: TopologicalSpace;
    type Section: Fn(
        <Self::TopologicalSpace as TopologicalSpace>::OpenSet,
    )
        -> HashMap<<Self::TopologicalSpace as TopologicalSpace>::Point, Box<dyn Any>>;
    fn restriction(&self, section_from: Self::Section, section_to: Self::Section) -> Self::Section;
}

pub trait MetricSpace: TopologicalSpace {
    type Distance;
    fn distance(
        &self,
        point_a: <Self as TopologicalSpace>::Point,
        point_b: <Self as TopologicalSpace>::Point,
    ) -> Self::Distance;
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
