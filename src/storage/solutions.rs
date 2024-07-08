use crate::model::solutions::{Solution, SolutionsListing};
use crate::storage::Error;
use rusqlite::{params, Connection};
use std::rc::Rc;

#[derive(Debug)]
pub struct Solutions {
    connection: Rc<Connection>,
}

impl Solutions {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Box<dyn std::error::Error>> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, solution: Solution) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&solution)?;

        self.connection.execute(
            "INSERT INTO solutions (id, data) VALUES (?1, ?2)",
            params![solution.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, solution_id: String) -> Result<Solution, Box<dyn std::error::Error>> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM solutions WHERE id = ?1")?;

        let response = statement.query_map(params![solution_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match response.last() {
            Some(solution) => Ok(serde_json::from_str(&solution?)?),
            None => Err(Box::new(Error::new("not found"))),
        }
    }

    pub fn update(&self, solution: Solution) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&solution)?;

        self.connection
            .prepare("UPDATE solutions SET data = ?2 WHERE id = ?1")?
            .execute(params![solution.id(), data])?;

        Ok(())
    }

    pub fn delete(&self, solution_id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.connection
            .prepare("DELETE FROM solutions WHERE id = ?1")?
            .execute(params![solution_id])?;

        Ok(())
    }

    pub fn list(&self) -> Result<SolutionsListing, Box<dyn std::error::Error>> {
        let mut statement = self.connection.prepare("SELECT * FROM solutions")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut solutions = vec![];

        for item in response {
            let solution: Solution = serde_json::from_str(&item?)?;
            solutions.push(solution);
        }

        Ok(SolutionsListing::new(solutions))
    }

    fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute(
            "CREATE TABLE solutions (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Box<dyn std::error::Error>> {
        let solutions = vec![];

        for solution in solutions {
            self.add(solution)?;
        }

        Ok(())
    }
}
