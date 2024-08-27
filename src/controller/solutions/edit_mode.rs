#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EditMode {
    Manual,
    Automatic,
}

impl EditMode {
    pub fn is_automatic(&self) -> bool {
        match self {
            Self::Manual => false,
            Self::Automatic => true,
        }
    }
}

impl Default for EditMode {
    fn default() -> Self {
        Self::Automatic
    }
}
