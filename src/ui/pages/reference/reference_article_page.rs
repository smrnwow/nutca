use crate::model::reference::Article;
use crate::storage::ArticlesStorage;
use crate::ui::components::layout::{Column, Page, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceArticlePage(article_id: String) -> Element {
    let articles_storage = consume_context::<Signal<ArticlesStorage>>();

    let article = use_signal(|| match articles_storage.read().get(article_id) {
        Some(article) => article,
        None => Article::new(),
    });

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            "Справка",
                        }
                    }

                    Divider {}

                    Block {
                        Column {
                            gap: "small",

                            Title {
                                size: "small",
                                {article.read().title()},
                            }

                            Text {
                                size: "x-small",
                                {article.read().text()},
                            }
                        }
                    }
                }
            }
        }
    }
}
