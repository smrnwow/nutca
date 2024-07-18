use crate::repository::storage::{Fertilizers, Profiles, Reference, Solutions};
use rusqlite::Connection;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Storage {
    connection: Rc<Connection>,
    reference: Reference,
    fertilizers: Fertilizers,
    profiles: Profiles,
    solutions: Solutions,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Rc::new(Connection::open_in_memory()?);

        let reference = Reference::new(Rc::clone(&connection))?;

        let fertilizers = Fertilizers::new(Rc::clone(&connection))?;

        let profiles = Profiles::new(Rc::clone(&connection))?;

        let solutions = Solutions::new(Rc::clone(&connection))?;

        Ok(Self {
            connection,
            reference,
            fertilizers,
            profiles,
            solutions,
        })
    }

    pub fn reference(&self) -> &Reference {
        &self.reference
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
