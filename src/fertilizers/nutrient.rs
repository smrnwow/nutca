use crate::chemistry::element::Element;

#[derive(Clone, Debug)]
pub struct Nutrient {
    pub element: Element,
    pub percent: f32,
}
