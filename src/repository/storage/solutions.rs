use crate::repository::{Error, RepositoryError, SolutionsListing};
use nutca::solutions::Solution;
use rusqlite::{params, Connection};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Solutions {
    connection: Rc<Connection>,
}

impl Solutions {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, solution: Solution) -> Result<(), Error> {
        let data = serde_json::to_string(&solution)?;

        self.connection.execute(
            "INSERT INTO solutions (id, data) VALUES (?1, ?2)",
            params![solution.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, solution_id: String) -> Result<Solution, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM solutions WHERE id = ?1")?;

        let response = statement.query_map(params![solution_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match response.last() {
            Some(solution) => Ok(serde_json::from_str(&solution?)?),
            None => Err(Box::new(RepositoryError::new("not found"))),
        }
    }

    pub fn update(&self, solution: Solution) -> Result<(), Error> {
        let data = serde_json::to_string(&solution)?;

        self.connection
            .prepare("UPDATE solutions SET data = ?2 WHERE id = ?1")?
            .execute(params![solution.id(), data])?;

        Ok(())
    }

    pub fn delete(&self, solution_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM solutions WHERE id = ?1")?
            .execute(params![solution_id])?;

        Ok(())
    }

    pub fn list(&self) -> Result<SolutionsListing, Error> {
        let mut statement = self.connection.prepare("SELECT * FROM solutions")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut solutions = vec![];

        for item in response {
            solutions.push(serde_json::from_str(&item?)?);
        }

        Ok(SolutionsListing::new(solutions))
    }

    fn setup(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE solutions (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
        let solutions = vec![];

        for solution in solutions {
            self.add(solution)?;
        }

        Ok(())
    }
}
