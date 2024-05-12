#[derive(Debug, Clone)]
pub struct Element {
    pub symbol: String,
    pub subscript: i32,
}

impl Element {
    pub fn from(symbol: &str, subscript: i32) -> Self {
        Self {
            symbol: symbol.to_string(),
            subscript,
        }
    }

    pub fn new() -> Self {
        Self {
            symbol: String::from(""),
            subscript: 1,
        }
    }

    pub fn multiply(element: &Self, coefficient: i32) -> Self {
        Self {
            symbol: element.symbol.clone(),
            subscript: element.subscript * coefficient,
        }
    }

    pub fn add_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn add_subscript(&mut self, subscript: i32) {
        self.subscript = subscript;
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
