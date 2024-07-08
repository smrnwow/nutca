use crate::storage::{Articles, Fertilizers, Profiles, Solutions};
use rusqlite::Connection;
use std::rc::Rc;

pub struct Storage {
    connection: Rc<Connection>,
    articles: Articles,
    fertilizers: Fertilizers,
    profiles: Profiles,
    solutions: Solutions,
}

impl Storage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Rc::new(Connection::open_in_memory()?);

        let articles = Articles::new(Rc::clone(&connection))?;

        let fertilizers = Fertilizers::new(Rc::clone(&connection))?;

        let profiles = Profiles::new(Rc::clone(&connection))?;

        let solutions = Solutions::new(Rc::clone(&connection))?;

        Ok(Self {
            connection,
            articles,
            fertilizers,
            profiles,
            solutions,
        })
    }

    pub fn articles(&self) -> &Articles {
        &self.articles
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
