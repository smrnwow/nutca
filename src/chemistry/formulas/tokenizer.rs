use super::{Compound, Element, Group, Hydrate};
use crate::chemistry::Table;
use crate::error::Error;
use core::iter::Peekable;
use core::str::Chars;

pub struct Tokenizer<'a> {
    table: &'a Table,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(table: &'a Table, formula: &'a str) -> Self {
        Self {
            table,
            chars: formula.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Compound, Error> {
        self.compound()
    }

    fn compound(&mut self) -> Result<Compound, Error> {
        let mut compound = Compound::new();

        loop {
            if let Some(char) = self.chars.peek() {
                match char {
                    '0'..='9' if compound.empty() => compound.add_coefficient(self.coefficient()),
                    'A'..='Z' => compound.add_element(self.element()?),
                    '(' => compound.add_group(self.group()?),
                    '*' => compound.add_hydrate(self.hydrate()?),
                    _ => {
                        let message = format!("unexpected character \"{}\"", char);
                        break Err(Error::new(message));
                    }
                }
            } else {
                break Ok(compound);
            }
        }
    }

    fn group(&mut self) -> Result<Group, Error> {
        self.chars.next();

        let mut group = Group::new();

        loop {
            if let Some(char) = self.chars.peek() {
                match char {
                    'A'..='Z' => group.add_element(self.element()?),
                    '(' => group.add_group(self.group()?),
                    ')' => {
                        self.chars.next();

                        group.add_subscript(self.subscript());

                        break Ok(group);
                    }
                    _ => {
                        let message = format!("unexpected symbol \"{}\"", char);
                        break Err(Error::new(message));
                    }
                }
            } else {
                let message = format!("unexpected end of formula");
                break Err(Error::new(message));
            }
        }
    }

    fn hydrate(&mut self) -> Result<Hydrate, Error> {
        self.chars.next();

        let mut hydrate = Hydrate::new();

        loop {
            if let Some(char) = self.chars.peek() {
                match char {
                    '0'..='9' if hydrate.empty() => hydrate.add_coefficient(self.coefficient()),
                    'A'..='Z' => hydrate.add_element(self.element()?),
                    _ => break Ok(hydrate),
                }
            } else {
                break Ok(hydrate);
            }
        }
    }

    fn element(&mut self) -> Result<Element, Error> {
        let symbol = self.symbol();

        match self.table.by_symbol(symbol.as_str()) {
            Some(symbol) => Ok(Element::new(symbol, self.subscript())),
            None => {
                let message = format!("unknown element: {}", symbol);
                Err(Error::new(message))
            }
        }
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

        while let Some(&char) = self.chars.peek() {
            if char.is_alphabetic() {
                if char.is_uppercase() && symbol.len() == 0 {
                    symbol.push(char);
                    self.chars.next();
                    continue;
                }

                if char.is_lowercase() && symbol.len() > 0 {
                    symbol.push(char);
                    self.chars.next();
                    continue;
                }
            }

            break;
        }

        symbol
    }

    fn subscript(&mut self) -> i32 {
        let mut subscript = String::new();

        while let Some(&char) = self.chars.peek() {
            if char.is_digit(10) {
                subscript.push(char);
                self.chars.next();
                continue;
            }

            break;
        }

        subscript.parse().unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::chemistry::formulas::{Component, Compound, Element, Group, Hydrate};
    use crate::chemistry::Table;

    #[test]
    fn single_element() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "N").tokenize().unwrap(),
            Compound::from(1, vec![Component::Element(Element::from("N", 1))], None)
        );

        assert_eq!(
            Tokenizer::new(&table, "Mg").tokenize().unwrap(),
            Compound::from(1, vec![Component::Element(Element::from("Mg", 1))], None)
        );

        assert_eq!(
            Tokenizer::new(&table, "Mg3").tokenize().unwrap(),
            Compound::from(1, vec![Component::Element(Element::from("Mg", 3))], None)
        );
    }

    #[test]
    fn multiple_elements() {
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "KNO3").tokenize().unwrap(),
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
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "Ca(NO3)2").tokenize().unwrap(),
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
            Tokenizer::new(&table, "C14H18N3O10Fe(NH4)2")
                .tokenize()
                .unwrap(),
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
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "2C14H18N3O10Fe(NH4)2")
                .tokenize()
                .unwrap(),
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
        let table = Table::new();

        assert_eq!(
            Tokenizer::new(&table, "MgSO4*7H2O").tokenize().unwrap(),
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
