use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{
    Badge, Block, Button, Card, Divider, Search, Table, TableCell, TableRow, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ListingProps {
    fertilizers: Memo<Vec<Fertilizer>>,
    on_search: EventHandler<String>,
    on_editor_open: EventHandler<()>,
}

#[component]
pub fn Listing(props: ListingProps) -> Element {
    let fertilizers = props.fertilizers.read();

    rsx! {
        Card {
            Block {
                Title {
                    text: "Список удобрений ({ fertilizers.len() })",
                }
            }

            Divider {}

            Block {
                div {
                    class: "fertilizer-listing__header",

                    Search {
                        placeholder: "поиск удобрения",
                        on_change: props.on_search,
                    }

                    Button {
                        style: "primary",
                        text: "Добавить удобрение",
                        on_click: props.on_editor_open,
                    }
                }
            }

            div {
                class: "fertilizers-listing__table",

                Table {
                    header: rsx! {
                        TableCell {
                            width: "50%",
                            "Название",
                        }

                        TableCell {
                            width: "50%",
                            "Состав",
                        }
                    },
                    body: rsx! {
                        for fertilizer in fertilizers.clone() {
                            TableRow {
                                TableCell {
                                    p {
                                        class: "fertilizer-listing__name",
                                        "{fertilizer.name()}",
                                    }

                                    p {
                                        class: "fertilizer-listing__vendor",
                                        "Производитель: {fertilizer.vendor()}",
                                    }
                                }

                                TableCell {
                                    div {
                                        class: "fertilizer-listing__nutrients",

                                        for nutrient in fertilizer.nutrients() {
                                            Badge {
                                                text: nutrient.symbol(),
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
