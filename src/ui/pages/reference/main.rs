use crate::repository::Storage;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let topics = use_signal(|| match storage.read().reference().list() {
        Ok(topics) => topics,
        Err(_) => Vec::new(),
    });

    rsx! {
        Card {
            Block {
                Title {
                    "Справка",
                }
            }

            Divider {}

            Block {
                Column {
                    for topic in topics.read().clone() {
                        Title {
                            size: "small",
                            {topic.title()},
                        }

                        Text {
                            {topic.text()},
                        }
                    }
                }
            }
        }
    }
}
