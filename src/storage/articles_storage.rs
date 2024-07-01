use super::provider::Provider;
use crate::model::reference::{Article, Browser};
use rusqlite::params;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ArticlesStorage {
    storage: Provider,
}

impl ArticlesStorage {
    pub fn new() -> Self {
        let storage = Provider::new();

        storage
            .connection()
            .execute(
                "CREATE TABLE reference_articles (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
                (),
            )
            .unwrap();

        let articles_storage = Self { storage };

        articles_storage.seed();

        articles_storage
    }

    pub fn add(&self, article: Article) -> i64 {
        let data = serde_json::to_string(&article).expect("Failed to serialize");

        self.storage
            .connection()
            .execute(
                "INSERT INTO reference_articles (id, data) VALUES (?1, ?2)",
                params![article.id(), data],
            )
            .unwrap();

        self.storage.connection().last_insert_rowid()
    }

    pub fn get(&self, article_id: String) -> Option<Article> {
        let query = format!("SELECT * FROM reference_articles WHERE id = \"{article_id}\"");

        let response = self.storage.connection().prepare(query.as_str());

        match response {
            Ok(mut result) => {
                let articles: Vec<Article> = result
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Article>(&data).expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .map(|article| article.unwrap())
                    .collect();

                if articles.len() > 0 {
                    return Some(articles.get(0).unwrap().clone());
                }

                None
            }
            Err(error) => {
                println!("article get error {:#?}", error);

                None
            }
        }
    }

    pub fn update(&self, article: Article) {
        let data = serde_json::to_string(&article).expect("Failed to serialize");

        let query = "UPDATE reference_articles SET data = ?2 WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        let response = statement.execute(params![article.id(), data]).unwrap();

        println!("response {:#?}", response);
    }

    pub fn delete(&self, article_id: String) {
        let query = "DELETE FROM reference_articles WHERE id = ?1";

        let mut statement = self.storage.connection().prepare(query).unwrap();

        statement.execute(params![article_id]).unwrap();
    }

    pub fn browse(&self) -> Browser {
        let statement = self
            .storage
            .connection()
            .prepare("SELECT * FROM reference_articles");

        match statement {
            Ok(mut query) => {
                let mut articles: HashMap<String, Article> = HashMap::new();

                query
                    .query_map([], |row| {
                        let data: String = row.get(1).unwrap();

                        Ok(serde_json::from_str::<Article>(&data)
                            .expect("Failed to deserialize"))
                    })
                    .unwrap()
                    .for_each(|article| if let Ok(article) = article {
                        articles.insert(article.id(), article);
                    });

                Browser::new(articles)
            }

            Err(_) => Browser::new(HashMap::new()),
        }
    }

    fn seed(&self) {
        let articles = vec![
            Article::new()
                .with_id("solution-editor-nutrient-profile")
                .with_title("Редактор раствора: Выбор профиля питания")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),
            Article::new()
                .with_id("solution-editor-fertilizers-browser")
                .with_title("Редактор раствора: Выбор удобрений")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),
            Article::new()
                .with_id("solution-editor-fertilizers-set")
                .with_title("Редактор раствора: Используемые удобрения")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),
            Article::new()
                .with_id("fertilizer-editor-composition-source")
                .with_title("Редактор удобрения: Состав")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),
            Article::new()
                .with_id("fertilizer-editor-nutrients")
                .with_title("Редактор удобрения: Питательные вещества")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),
            Article::new()
                .with_id("profile-editor-nutrients")
                .with_title("Редактор профиля питания: Питательные вещества")
                .with_text("Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст. Пока непридуманный текст.")
                .build(),    
            ];

        for article in articles {
            self.add(article);
        }
    }
}
