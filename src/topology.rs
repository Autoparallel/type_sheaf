use std::collections::HashSet;

pub trait TopologicalSpace {
    type Point;
    type OpenSet: IntoIterator<Item = Self::Point>;
    fn points(&self) -> HashSet<Self::Point>;
    fn neighborhood(&self, point: Self::Point) -> Self::OpenSet;
    fn is_open(&self, open_set: Self::OpenSet) -> bool;
}

pub trait MetricSpace: TopologicalSpace {
    type Distance;
    fn distance(
        &self,
        point_a: <Self as TopologicalSpace>::Point,
        point_b: <Self as TopologicalSpace>::Point,
    ) -> Self::Distance;
}

pub trait PreSheaf<S> {
    type TopologicalSpace: TopologicalSpace;
    fn restriction(
        &self,
        set_to: &<Self::TopologicalSpace as TopologicalSpace>::OpenSet,
        section: &S,
    ) -> S;
}

pub trait Sheaf<S>: PreSheaf<S> {
    fn gluing(
        &self,
        sections: Vec<(&<Self::TopologicalSpace as TopologicalSpace>::OpenSet, S)>,
        gluing_domain: &<Self::TopologicalSpace as TopologicalSpace>::OpenSet,
    ) -> Option<S>;

    fn is_locally_unique(&self, sections: Vec<(&<Self::TopologicalSpace as TopologicalSpace>::OpenSet, S)>) -> bool;
}
