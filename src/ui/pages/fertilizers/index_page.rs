use crate::storage::FertilizersStorage;
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::NutrientValue;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersIndexPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let fertilizers_list = fertilizers_storage.read().list();

    rsx! {
        div {
            class: "fertilizers-index",

            div {
                class: "fertilizers-index__list",

                h3 {
                    class: "fertilizers-index__title",
                    "Список удобрений ({ fertilizers_list.len() })",
                }

                div {
                    class: "fertilizers__summary",

                    label {
                        class: "fertilizers-index__search-label",

                        SearchIcon {}

                        input {
                            class: "fertilizers-index__search-input",
                            r#type: "text",
                            placeholder: "поиск удобрения",
                        }
                    }

                    button {
                         class: "fertilizers-index__button-add",
                         onclick: |_| {
                             navigator().push(Route::FertilizersAddPage {});
                         },
                         "Добавить удобрение",
                    }
                }

                div {
                    class: "fertilizers__list",

                    for fertilizer in fertilizers_list {
                        div {
                            class: "fertilizers__item",

                            p {
                                class: "fertilizers__name",
                                "{fertilizer.name()}"
                            }

                            div {
                                class: "fertilizers-index__nutrients",

                                for nutrient in fertilizer.nutrients() {
                                    div {
                                        class: "fertilizers-index__nutrient",

                                        span {
                                            class: "fertilizers-index__nutrient-symbol",
                                            "{nutrient.symbol()}",
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
