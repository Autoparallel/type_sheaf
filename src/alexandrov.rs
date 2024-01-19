
pub trait PreOrderedSet {
    type Element;
    fn compare(&self, element_a: Self::Element, element_b: Self::Element) -> bool;
    fn upper_set(&self, element: Self::Element) -> Vec<Self::Element>;
}
