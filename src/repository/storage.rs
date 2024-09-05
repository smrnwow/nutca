use crate::repository::{Concentrates, Fertilizers, Profiles, Solutions};
use rusqlite::Connection;
use std::env;
use std::fs;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Storage {
    #[allow(unused)]
    connection: Rc<Connection>,
    fertilizers: Fertilizers,
    profiles: Profiles,
    solutions: Solutions,
    concentrates: Concentrates,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_path = format!(
            "{}/.local/share/nutca/storage.db3",
            env::var("HOME").unwrap_or(String::from("/home"))
        );

        let path = Path::new(data_path.as_str());

        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::File::create(path)?;
        }

        let connection = Rc::new(Connection::open(data_path)?);

        let fertilizers = Fertilizers::new(Rc::clone(&connection))?;

        let profiles = Profiles::new(Rc::clone(&connection))?;

        let solutions = Solutions::new(Rc::clone(&connection))?;

        let concentrates = Concentrates::new(Rc::clone(&connection))?;

        Ok(Self {
            connection,
            fertilizers,
            profiles,
            solutions,
            concentrates,
        })
    }

    pub fn fertilizers(&self) -> &Fertilizers {
        &self.fertilizers
    }

    pub fn profiles(&self) -> &Profiles {
        &self.profiles
    }

    pub fn solutions(&self) -> &Solutions {
        &self.solutions
    }

    pub fn concentrates(&self) -> &Concentrates {
        &self.concentrates
    }
}
