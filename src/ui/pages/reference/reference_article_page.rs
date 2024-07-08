use crate::model::reference::Article;
use crate::storage::Storage;
use crate::ui::components::layout::{Column, Page, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceArticlePage(article_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let article = use_signal(|| match storage.read().articles().get(article_id) {
        Ok(article) => article,
        Err(_) => Article::new(),
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
