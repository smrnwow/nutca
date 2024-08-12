use crate::repository::{Fertilizers, Profiles, Solutions};
use rusqlite::Connection;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Storage {
    connection: Rc<Connection>,
    fertilizers: Fertilizers,
    profiles: Profiles,
    solutions: Solutions,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Rc::new(Connection::open("./data/storage.db3")?);

        let fertilizers = Fertilizers::new(Rc::clone(&connection))?;

        let profiles = Profiles::new(Rc::clone(&connection))?;

        let solutions = Solutions::new(Rc::clone(&connection))?;

        Ok(Self {
            connection,
            fertilizers,
            profiles,
            solutions,
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
}
