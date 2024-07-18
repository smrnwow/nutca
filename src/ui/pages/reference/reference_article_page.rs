use crate::model::reference::Topic;
use crate::repository::Storage;
use crate::ui::components::layout::{Column, Page, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceArticlePage(article_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let topic = use_signal(|| match storage.read().reference().get(article_id) {
        Ok(topic) => topic,
        Err(_) => Topic::new(),
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
                                {topic.read().title()},
                            }

                            Text {
                                size: "x-small",
                                {topic.read().text()},
                            }
                        }
                    }
                }
            }
        }
    }
}
