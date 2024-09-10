use super::Component;

pub struct Composition {
    components: Vec<Component>,
}

impl Composition {
    pub fn new() -> Self {
        Self { components: vec![] }
    }
}
