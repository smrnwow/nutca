use crate::model::reference::Article;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::rc::Rc;
use crate::repository::{Error, RepositoryError, ArticlesBrowser};

#[derive(Clone, Debug)]
pub struct Articles {
    connection: Rc<Connection>,
}

impl Articles {
    pub fn new(connection: Rc<Connection>) -> Result<Self, Error> {
        let storage = Self { connection };

        storage.setup()?;

        storage.seed()?;

        Ok(storage)
    }

    pub fn add(&self, article: Article) -> Result<(), Error> {
        let data = serde_json::to_string(&article)?;

        self.connection.execute(
            "INSERT INTO articles (id, data) VALUES (?1, ?2)",
            params![article.id(), data],
        )?;

        Ok(())
    }

    pub fn get(&self, article_id: String) -> Result<Article, Error> {
        let mut statement = self.connection
            .prepare("SELECT * FROM articles WHERE id = ?1")?;

        let rows = statement.query_map(params![article_id], |row| {
            let data: String = row.get(1)?;
            Ok(data)
        })?;

        match rows.last() {
            Some(article) => Ok(serde_json::from_str(&article?)?),
            None => Err(Box::new(RepositoryError::new("not found")))
        }
    }

    pub fn _update(&self, article: Article) -> Result<(), Error> {
        let data = serde_json::to_string(&article)?;

        self.connection
            .prepare("UPDATE articles SET data = ?2 WHERE id = ?1")?
            .execute(params![article.id(), data])?;

        Ok(())
    }

    pub fn _delete(&self, article_id: String) -> Result<(), Error> {
        self.connection
            .prepare("DELETE FROM articles WHERE id = ?1")?
            .execute(params![article_id])?;

        Ok(())
    }

    pub fn browse(&self) -> Result<ArticlesBrowser, Error> {
        let mut statement = self
            .connection
            .prepare("SELECT * FROM articles")?;


        let response = statement.query_map([], |row| {
            let data: String = row.get(1)?;
            
            Ok(data)
        })?;

        let mut articles: HashMap<String, Article> = HashMap::new();

        for item in response {
            let article = serde_json::from_str::<Article>(&item?)?;

            articles.insert(article.id(), article);
        }

        Ok(ArticlesBrowser::new(articles))
    }

    fn setup(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE articles (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            (),
        )?;

        Ok(())
    }

    fn seed(&self) -> Result<(), Error> {
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
            self.add(article)?;
        }

        Ok(())
    }
}
