use super::{Element, Group};

#[derive(Debug)]
pub enum Component {
    Element(Element),
    Group(Group),
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Component::Element(element) => {
                if let Component::Element(other_element) = other {
                    element == other_element
                } else {
                    false
                }
            }
            Component::Group(group) => {
                if let Component::Group(other_group) = other {
                    group == other_group
                } else {
                    false
                }
            }
        }
    }
}
