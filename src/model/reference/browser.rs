use crate::model::reference::Article;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Browser {
    articles: HashMap<String, Article>,
}

impl Browser {
    pub fn new(articles: HashMap<String, Article>) -> Self {
        Self { articles }
    }

    pub fn empty() -> Self {
        Self {
            articles: HashMap::new(),
        }
    }

    pub fn summary(&self, article_id: &String) -> Article {
        match self.articles.get(article_id) {
            Some(article) => article.clone(),
            None => Article::new()
                .with_id(article_id)
                .with_title("Заголовок отсутствует")
                .with_text("Текст отсутствует")
                .build(),
        }
    }
}
