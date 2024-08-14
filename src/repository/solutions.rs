use crate::model::solutions::Solution;
use crate::repository::{Error, RepositoryError};
use rusqlite::{named_params, params, Connection};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Solutions {
    connection: Rc<Connection>,
}

impl Solutions {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        Ok(storage)
    }

    pub fn add(&self, solution: Solution) -> Result<(), Error> {
        let data = serde_json::to_string(&solution)?;

        self.connection.execute(
            "INSERT INTO solutions (id, name, data) VALUES (?1, ?2, ?3)",
            params![solution.id(), solution.name().to_lowercase(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, solution_id: &str) -> Result<Solution, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM solutions WHERE id = ?1")?;

        let response = statement.query_map(params![solution_id], |row| {
            let data: String = row.get(2)?;
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
            .prepare("UPDATE solutions SET name = ?2, data = ?3 WHERE id = ?1")?
            .execute(params![solution.id(), solution.name().to_lowercase(), data])?;

        Ok(())
    }

    pub fn delete(&self, solution_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM solutions WHERE id = ?1")?
            .execute(params![solution_id])?;

        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize, offset: usize) -> Result<Vec<Solution>, Error> {
        let mut statement = self.connection.prepare("SELECT * FROM solutions WHERE name LIKE '%' || :search || '%' LIMIT :limit OFFSET :offset")?;

        let response = statement.query_map(
            named_params! {
                ":search": query,
                ":limit": limit,
                ":offset": offset,
            },
            |row| {
                let data: String = row.get(2)?;
                Ok(data)
            },
        )?;

        let mut solutions = vec![];

        for item in response {
            let solution: Solution = serde_json::from_str(&item?)?;

            solutions.push(solution);
        }

        Ok(solutions)
    }

    fn setup(&self) -> Result<(), Error> {
        let mut statement = self.connection.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='solutions'",
        )?;

        let response = statement.query_map(params![], |row| {
            let data: usize = row.get(0)?;
            Ok(data)
        })?;

        match response.last() {
            Some(res) => match res {
                Ok(table_count) => {
                    if table_count == 0 {
                        self.create_table()?;
                        self.seed()?;
                    }
                }
                Err(error) => println!("error = {:#?}", error),
            },
            None => {
                self.create_table()?;
                self.seed()?;
            }
        }

        Ok(())
    }

    fn create_table(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE solutions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
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
