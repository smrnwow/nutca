use crate::chemistry::element::Element;

#[derive(Debug)]
pub struct Component {
    pub element: Element,
    pub percent: f32,
}
