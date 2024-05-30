use crate::model::chemistry::{Symbol, Table};

#[derive(Debug, Clone)]
pub struct Element {
    symbol: Symbol,
    subscript: i32,
}

impl Element {
    pub fn new(symbol: Symbol, subscript: i32) -> Self {
        Self { symbol, subscript }
    }

    pub fn from(symbol: &str, subscript: i32) -> Self {
        let table = Table::new();

        Self {
            symbol: table.by_symbol(symbol).unwrap(),
            subscript,
        }
    }

    pub fn multiply(element: &Self, coefficient: i32) -> Self {
        Self {
            symbol: element.symbol,
            subscript: element.subscript * coefficient,
        }
    }

    pub fn symbol(&self) -> Symbol {
        self.symbol
    }

    pub fn subscript(&self) -> i32 {
        self.subscript
    }

    pub fn molar_mass(&self) -> f64 {
        self.symbol.atomic_weight() * self.subscript as f64
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.symbol != other.symbol {
            return false;
        }

        if self.subscript != other.subscript {
            return false;
        }

        true
    }
}
