use crate::model::solutions::StockSolutionBuilder;
use crate::storage::SolutionsStorage;
use crate::ui::components::layout::{Column, Page, Row, Section};
use crate::ui::components::utils::{Block, Card, Divider, NumberField, Select, Text, Title};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[component]
pub fn StockSolutionPage(solution_id: String) -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let mut stock_solution_builder = use_signal(|| {
        let solution = solutions_storage.read().get(solution_id);

        match solution {
            Some(solution) => StockSolutionBuilder::from(solution),
            None => StockSolutionBuilder::new(),
        }
    });

    let mut solutions_listing = use_signal(|| solutions_storage.read().list());

    let solutions = use_memo(move || solutions_listing.read().list());

    let solution = use_memo(move || stock_solution_builder.read().solution().clone());

    let concentration_factor =
        use_memo(move || stock_solution_builder.read().concentration_factor());

    let volume = use_memo(move || stock_solution_builder.read().volume());

    let part_a = use_memo(move || stock_solution_builder.read().part_a().clone());

    let part_b = use_memo(move || stock_solution_builder.read().part_b().clone());

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            text: "Рабочий раствор (A + B)",
                        }
                    }

                    Divider {}

                    Block {
                        Column {
                            Select {
                                label: "Питательный раствор",
                                placeholder: "выбрать раствор",
                                value: (solution.read().id(), solution.read().name()),
                                options: solutions.read()
                                    .iter()
                                    .map(|solution| (solution.id(), solution.name()))
                                    .collect(),
                                on_search: move |search_query| {
                                    solutions_listing.write().search(search_query);
                                },
                                on_change: move |solution_id| {
                                    let solution = solutions_listing.read().find(solution_id);

                                    stock_solution_builder.write().update_solution(solution);
                                },
                            }

                            Row {
                                NumberField {
                                    label: "Концентрация",
                                    value: *concentration_factor.read(),
                                    on_change: move |value| {
                                        stock_solution_builder.write().update_concentration_factor(value);
                                    },
                                }

                                NumberField {
                                    label: "Объем",
                                    value: *volume.read(),
                                    units: "литр",
                                    on_change: move |value| {
                                        stock_solution_builder.write().update_volume(value);
                                    },
                                }
                            }
                        }
                    }

                    Divider {}

                    Block {
                        Column {
                            Column {
                                gap: "medium",

                                Title {
                                    text: "Часть A",
                                }

                                Column {
                                    gap: "x-small",

                                    for fertilizer in part_a.read().clone() {
                                        Row {
                                            align: "space-between",

                                            Text {
                                                size: "x-small",
                                                "{fertilizer.fertilizer.name()}",
                                            }

                                            Text {
                                                size: "x-small",
                                                "{round(fertilizer.weight)}",
                                            }
                                        }
                                    }
                                }
                            }

                            Divider {}

                            Column {
                                gap: "medium",

                                Title {
                                    text: "Часть B",
                                }

                                Column {
                                    gap: "x-small",

                                    for fertilizer in part_b.read().clone() {
                                        Row {
                                            align: "space-between",

                                            Text {
                                                size: "x-small",
                                                "{fertilizer.fertilizer.name()}",
                                            }

                                            Text {
                                                size: "x-small",
                                                "{round(fertilizer.weight)}",
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