use crate::controller::reference::{Topic, TopicId};
use crate::repository::{Error, ReferenceBrowser, RepositoryError};
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Reference {
    connection: Rc<Connection>,
}

impl Reference {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, topic: Topic) -> Result<(), Error> {
        let data = serde_json::to_string(&topic)?;

        self.connection.execute(
            "INSERT INTO topics (id, data) VALUES (?1, ?2)",
            params![topic.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, topic_id: String) -> Result<Topic, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM topics WHERE id = ?1")?;

        let rows = statement.query_map(params![topic_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match rows.last() {
            Some(topic) => Ok(serde_json::from_str(&topic?)?),
            None => Err(Box::new(RepositoryError::new("not found"))),
        }
    }

    pub fn browse(&self) -> Result<ReferenceBrowser, Error> {
        let mut statement = self.connection.prepare("SELECT * FROM topics")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut topics: HashMap<String, Topic> = HashMap::new();

        for item in response {
            let topic = serde_json::from_str::<Topic>(&item?)?;
            topics.insert(topic.id(), topic);
        }

        Ok(ReferenceBrowser::new(topics))
    }

    pub fn list(&self) -> Result<Vec<Topic>, Error> {
        let mut statement = self.connection.prepare("SELECT * FROM topics")?;

        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        let mut topics: Vec<Topic> = Vec::new();

        for item in response {
            let topic = serde_json::from_str::<Topic>(&item?)?;
            topics.push(topic);
        }

        Ok(topics)
    }

    fn setup(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE topics (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
        let topics = vec![
            Topic::new()
                .with_id(TopicId::ProfilesDashboard)
                .with_title("Профили питания")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::ProfileEditor)
                .with_title("Редактор профиля питания")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::FertilizersDashboard)
                .with_title("Удобрения")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::FertilizerEditor)
                .with_title("Редактор удобрения")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::SolutionsDashboard)
                .with_title("Растворы")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::NutrientSolutionEditor)
                .with_title("Редактор питательного раствора")
                .with_text(""),
            Topic::new()
                .with_id(TopicId::StockSolutionEditor)
                .with_title("Редактор рабочего раствора")
                .with_text(""),
        ];

        for topic in topics {
            self.add(topic)?;
        }

        Ok(())
    }
}
