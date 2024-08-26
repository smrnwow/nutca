#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BuildMode {
    Manual,
    Automatic,
}

impl BuildMode {
    pub fn is_automatic(&self) -> bool {
        match self {
            Self::Manual => false,
            Self::Automatic => true,
        }
    }
}
