use crate::repository::Storage;
use crate::ui::components::layout::{Column, Page, Row, Section};
use crate::ui::components::utils::{Block, Card, Divider, Text, Title};
use dioxus::prelude::*;

#[component]
pub fn ReferenceMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let topics = use_signal(|| match storage.read().reference().list() {
        Ok(topics) => topics,
        Err(_) => Vec::new(),
    });

    /*
    let left_side = use_memo(move || {
        let topics = topics.read().clone();

        let s = topics[0..4];
    });
    */

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
                        Row {
                            Column {
                                gap: "none",

                                for topic in topics.read().clone()[0..4] {
                                    Block {
                                        exclude_padding: "top",

                                        Title {
                                            size: "small",
                                            {topic.title()},
                                        }

                                        Block {
                                            exclude_padding: "top",

                                            for block in topic.blocks() {
                                                Title {
                                                    size: "x-small",
                                                    {block.title().clone()},
                                                }

                                                Text {
                                                    {block.text().clone()},
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            Column {
                                gap: "none",

                                for topic in topics.read().clone()[4..] {
                                    Block {
                                        exclude_padding: "top",

                                        Title {
                                            size: "small",
                                            {topic.title()},
                                        }

                                        Block {
                                            exclude_padding: "top",

                                            for block in topic.blocks() {
                                                Title {
                                                    size: "x-small",
                                                    {block.title().clone()},
                                                }

                                                Text {
                                                    {block.text().clone()},
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
