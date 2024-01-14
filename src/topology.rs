use std::collections::HashSet;

pub trait OpenSet: IntoIterator<Item = Self::Point> + Clone {
    type Point;
    fn intersect(&self, other: Self) -> Self;
}

pub trait TopologicalSpace {
    type Point;
    type OpenSet: OpenSet<Point = Self::Point>;
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

pub trait PreSheaf<S: Section> {
    type TopologicalSpace: TopologicalSpace;
    fn restriction(
        &self,
        set_to: &<S::TopologicalSpace as TopologicalSpace>::OpenSet,
        section: &S,
    ) -> S {
        section.restrict(set_to.clone())
    }
}

pub trait Sheaf<S: Section>: PreSheaf<S> {
    fn gluing(
        &self,
        sections: Vec<(&<S::TopologicalSpace as TopologicalSpace>::OpenSet, S)>,
    ) -> Option<S> {
        for (i, (domain_i, section_i)) in sections.iter().enumerate() {
            for (j, (domain_j, section_j)) in sections.iter().enumerate() {
                if i != j {
                    // Compute the intersection of domain_i and domain_j
                    let intersection = domain_i.intersect(domain_j.clone().clone());
                    
                    // Check if the sections are compatible on the intersection
                    if !section_i.is_compatible(intersection, section_j.clone()) {
                        return None;
                    }
                }
            }
        };
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
    fn uniqueness(&self, sections: Vec<(&<S::TopologicalSpace as TopologicalSpace>::OpenSet, S, S)>) -> bool {
        let mut all_unique = true;
    
        for (domain, section_i, section_j) in sections {
            if section_i.restrict(domain.clone()) == section_j.restrict(domain.clone()) {
                all_unique = false;
                break;
            }
        }
        all_unique
    }
    
}

pub trait Section: Eq + PartialEq + Clone {
    type TopologicalSpace: TopologicalSpace;

    fn restrict(&self, domain: <Self::TopologicalSpace as TopologicalSpace>::OpenSet) -> Self;
    fn is_compatible(&self, domain: <Self::TopologicalSpace as TopologicalSpace>::OpenSet, section: Self) -> bool {
        if self.restrict(domain.clone()) == section.restrict(domain.clone()) {
            true
        } else {
            false
        }
    }
    fn glue(&self, domain: <Self::TopologicalSpace as TopologicalSpace>::OpenSet, section: Self) -> Option<Self>;
}
