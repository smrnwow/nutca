use crate::storage::SolutionsStorage;
use crate::ui::components::solutions::SolutionListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionsListingPage() -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let solutions = use_memo(move || solutions_storage.read().list());

    rsx! {
        div {
            class: "solutions-listing-page",

            section {
                class: "solutions-listing-page__listing",

                Card {
                    Block {
                        Title {
                            text: "Список растворов",
                        }
                    }

                    Divider {}

                    Block {
                        div {
                            class: "solutions-listing-page__controls",

                            Button {
                                style: "primary",
                                text: "Добавить раствор",
                                on_click: move |_| {
                                    navigator().push(Route::SolutionAddPage {});
                                },
                            }
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Table {
                            header: rsx! {
                                TableCell {
                                    width: "100%",
                                    "Название",
                                }
                            },
                            body: rsx! {
                                for solution in solutions.read().clone() {
                                    SolutionListingItem {
                                        solution: solution.clone(),
                                        on_select: move |solution_id| {
                                            navigator().push(Route::SolutionEditPage {
                                                solution_id: solution_id,
                                            });
                                        },
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
