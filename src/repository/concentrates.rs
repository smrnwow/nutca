use crate::repository::schemas::ConcentrateSchema;
use crate::repository::{Error, RepositoryError};
use rusqlite::{named_params, params, Connection};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Concentrates {
    connection: Rc<Connection>,
}

impl Concentrates {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        Ok(storage)
    }

    pub fn add(&self, concentrate: ConcentrateSchema) -> Result<(), Error> {
        let data = serde_json::to_string(&concentrate)?;

        self.connection.execute(
            "INSERT INTO concentrates (id, name, data) VALUES (?1, ?2, ?3)",
            params![concentrate.id, concentrate.name.to_lowercase(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, concentrate_id: &str) -> Result<ConcentrateSchema, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM concentrates WHERE id = ?1")?;

        let response = statement.query_map(params![concentrate_id], |row| {
            let data: String = row.get(2)?;
            Ok(data)
        })?;

        match response.last() {
            Some(concentrate) => Ok(serde_json::from_str(&concentrate?)?),
            None => Err(Box::new(RepositoryError::new("not found"))),
        }
    }

    pub fn update(&self, concentrate: ConcentrateSchema) -> Result<(), Error> {
        let data = serde_json::to_string(&concentrate)?;

        self.connection
            .prepare("UPDATE concentrates SET name = ?2, data = ?3 WHERE id = ?1")?
            .execute(params![
                concentrate.id,
                concentrate.name.to_lowercase(),
                data
            ])?;

        Ok(())
    }

    pub fn delete(&self, concentrate_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM concentrates WHERE id = ?1")?
            .execute(params![concentrate_id])?;

        Ok(())
    }

    pub fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ConcentrateSchema>, Error> {
        let query_str = "SELECT * FROM concentrates WHERE name LIKE '%' || :search || '%' LIMIT :limit OFFSET :offset";

        let mut statement = self.connection.prepare(query_str)?;

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

        let mut concentrates = vec![];

        for item in response {
            let concentrate = serde_json::from_str::<ConcentrateSchema>(&item?)?;
            concentrates.push(concentrate);
        }

        Ok(concentrates)
    }

    fn setup(&self) -> Result<(), Error> {
        let mut statement = self.connection.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='concentrates'",
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
            "CREATE TABLE concentrates (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
        Ok(())
    }
}
