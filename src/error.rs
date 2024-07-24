#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    SolutionNameEmpty,
    SolutionNameTooLong,
    ProfileNameEmpty,
    ProfileNameTooLong,
    FertilizerNameEmpty,
    FertilizerNameTooLong,
}

impl Error {
    pub fn entity(&self) -> String {
        match self {
            Self::SolutionNameEmpty => String::from("solution-name"),
            Self::SolutionNameTooLong => String::from("solution-name"),
            Self::ProfileNameEmpty => String::from("profile-name"),
            Self::ProfileNameTooLong => String::from("profile-name"),
            Self::FertilizerNameEmpty => String::from("fertilizer-name"),
            Self::FertilizerNameTooLong => String::from("fertilizer-name"),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::SolutionNameEmpty => String::from("название раствора не заполнено"),
            Self::SolutionNameTooLong => {
                String::from("название раствора не должно превышать 100 символов")
            }
            Self::ProfileNameEmpty => String::from("название профиля не заполнено"),
            Self::ProfileNameTooLong => {
                String::from("название профиля не должно превышать 100 символов")
            }
            Self::FertilizerNameEmpty => String::from("название удобрения не заполнено"),
            Self::FertilizerNameTooLong => {
                String::from("название удобрения не должно превышать 100 символов")
            }
        }
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.message()
    }
}
