use super::compound::Compound;
use super::element::Element;
use super::group::Group;
use super::hydrate::Hydrate;
use core::iter::Peekable;
use core::str::Chars;

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(formula: &str) -> Tokenizer {
        Tokenizer {
            chars: formula.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Compound {
        self.compound()
    }

    fn compound(&mut self) -> Compound {
        let mut compound = Compound::new();

        while let Some(char) = self.chars.peek() {
            match char {
                '0'..='9' if compound.empty() => compound.add_coefficient(self.coefficient()),
                'A'..='Z' => compound.add_element(self.element()),
                '(' => compound.add_group(self.group()),
                '*' => compound.add_hydrate(self.hydrate()),
                _ => break,
            }
        }

        compound
    }

    fn group(&mut self) -> Group {
        self.chars.next();

        let mut group = Group::new();

        while let Some(char) = self.chars.peek() {
            match char {
                'A'..='Z' => group.add_element(self.element()),
                '(' => group.add_group(self.group()),
                ')' => {
                    self.chars.next();

                    group.add_subscript(self.subscript());

                    break;
                }
                _ => break,
            }
        }

        group
    }

    fn hydrate(&mut self) -> Hydrate {
        self.chars.next();

        let mut hydrate = Hydrate::new();

        while let Some(char) = self.chars.peek() {
            match char {
                '0'..='9' if hydrate.empty() => hydrate.add_coefficient(self.coefficient()),
                'A'..='Z' => hydrate.add_element(self.element()),
                _ => break,
            }
        }

        hydrate
    }

    fn element(&mut self) -> Element {
        let mut element = Element::new();

        element.add_symbol(self.symbol());

        element.add_subscript(self.subscript());

        element
    }

    fn coefficient(&mut self) -> i32 {
        let mut coefficient: String = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_digit(10) {
                coefficient.push(char);

                self.chars.next();
            } else {
                break;
            }
        }

        coefficient.parse().unwrap_or(1)
    }

    fn symbol(&mut self) -> String {
        let mut symbol = String::new();

        while let Some(char) = self.chars.next() {
            if char.is_alphabetic() {
                symbol.push(char);

                match self.chars.peek() {
                    Some(next) => {
                        if !next.is_alphabetic() {
                            break;
                        }

                        if next.is_uppercase() {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        symbol
    }

    fn subscript(&mut self) -> i32 {
        let mut subscript = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_digit(10) {
                subscript.push(char);

                self.chars.next();
            } else {
                break;
            }
        }

        subscript.parse().unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::formula::tokenization::component::Component;
    use crate::formula::tokenization::compound::Compound;
    use crate::formula::tokenization::element::Element;
    use crate::formula::tokenization::group::Group;
    use crate::formula::tokenization::hydrate::Hydrate;

    #[test]
    fn single_element() {
        assert_eq!(
            Tokenizer::new("N").tokenize(),
            Compound::from(1, vec![Component::Element(Element::from("N", 1))], None)
        );

        assert_eq!(
            Tokenizer::new("Mg").tokenize(),
            Compound::from(1, vec![Component::Element(Element::from("Mg", 1))], None)
        );

        assert_eq!(
            Tokenizer::new("Mg3").tokenize(),
            Compound::from(1, vec![Component::Element(Element::from("Mg", 3))], None)
        );
    }

    #[test]
    fn multiple_elements() {
        assert_eq!(
            Tokenizer::new("KNO3").tokenize(),
            Compound::from(
                1,
                vec![
                    Component::Element(Element::from("K", 1)),
                    Component::Element(Element::from("N", 1)),
                    Component::Element(Element::from("O", 3)),
                ],
                None,
            )
        );
    }

    #[test]
    fn group() {
        assert_eq!(
            Tokenizer::new("Ca(NO3)2").tokenize(),
            Compound::from(
                1,
                vec![
                    Component::Element(Element::from("Ca", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("O", 3)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );

        assert_eq!(
            Tokenizer::new("C14H18N3O10Fe(NH4)2").tokenize(),
            Compound::from(
                1,
                vec![
                    Component::Element(Element::from("C", 14)),
                    Component::Element(Element::from("H", 18)),
                    Component::Element(Element::from("N", 3)),
                    Component::Element(Element::from("O", 10)),
                    Component::Element(Element::from("Fe", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("H", 4)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );
    }

    #[test]
    fn coefficient() {
        assert_eq!(
            Tokenizer::new("2C14H18N3O10Fe(NH4)2").tokenize(),
            Compound::from(
                2,
                vec![
                    Component::Element(Element::from("C", 14)),
                    Component::Element(Element::from("H", 18)),
                    Component::Element(Element::from("N", 3)),
                    Component::Element(Element::from("O", 10)),
                    Component::Element(Element::from("Fe", 1)),
                    Component::Group(Group::from(
                        vec![
                            Component::Element(Element::from("N", 1)),
                            Component::Element(Element::from("H", 4)),
                        ],
                        2,
                    )),
                ],
                None,
            )
        );
    }

    #[test]
    fn hydrate() {
        assert_eq!(
            Tokenizer::new("MgSO4*7H2O").tokenize(),
            Compound::from(
                1,
                vec![
                    Component::Element(Element::from("Mg", 1)),
                    Component::Element(Element::from("S", 1)),
                    Component::Element(Element::from("O", 4)),
                ],
                Some(Hydrate::from(
                    7,
                    vec![Element::from("H", 2), Element::from("O", 1)],
                )),
            )
        );
    }
}
