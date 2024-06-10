use super::provider::Provider;
use crate::model::solutions::Solution;
use rusqlite::params;

#[derive(Debug)]
pub struct SolutionsStorage {
    storage: Provider,
}

impl SolutionsStorage {
    pub fn new() -> Self {
        let storage = Provider::new();

        storage
            .connection()
            .execute(
                "CREATE TABLE solutions (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
                (),
            )
            .unwrap();

        let solutions_storage = Self { storage };

        solutions_storage.seed();

        solutions_storage
    }

    pub fn add(&self, solution: Solution) -> i64 {
        let data = serde_json::to_string(&solution).expect("Failed to serialize");

        self.storage
            .connection()
            .execute(
                "INSERT INTO solutions (id, data) VALUES (?1, ?2)",
                params![solution.id(), data],
            )
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn get(&self, solution_id: String) -> Option<Solution> {
        let query = format!("SELECT * FROM solutions WHERE id = \"{solution_id}\"");

        let response = self.storage.connection().prepare(query.as_str());

        match response {
            Ok(mut result) => {
                let solutions: Vec<Solution> = result
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Solution>(&data).expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|solution| solution.unwrap())
                    .collect();

                if solutions.len() > 0 {
                    return Some(solutions.get(0).unwrap().clone());
                }

                None
            }
            Err(error) => {
                println!("solution get error {:#?}", error);

                None
            }
        }
    }

    pub fn list(&self) -> Vec<Solution> {
        let query = self.storage.connection().prepare("SELECT * FROM solutions");

        if query.is_ok() {
            query
                .unwrap()
                .query_map([], |row| {
                    // println!("{:#?}", row);

                    let data: String = row.get(1).unwrap();

                    Ok(serde_json::from_str::<Solution>(&data).expect("Failed to deserialize"))
                })
                .unwrap()
                .map(|profile| profile.unwrap())
                .collect()
        } else {
            vec![]
        }
    }

    fn seed(&self) {
        let solutions = vec![];

        for solution in solutions {
            self.add(solution);
        }
    }
}
