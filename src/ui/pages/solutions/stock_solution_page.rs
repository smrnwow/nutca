use crate::model::solutions::StockSolutionBuilder;
use crate::storage::SolutionsStorage;
use crate::ui::components::layout::{Column, Page, Row, Section};
use crate::ui::components::utils::{
    Block, Card, Divider, NumberField, QuickAction, Select, Text, Title,
};
use crate::ui::components::UnitsSelect;
use dioxus::prelude::*;

fn render_fertilizer_weight(weight: f64, liquid: bool) -> String {
    let units = match liquid {
        true => String::from("мл"),
        false => String::from("г"),
    };

    format!("{:.3} {}", weight, units)
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

    let solution_select_value = use_memo(move || (solution.read().id(), solution.read().name()));

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            "Редактор рабочего раствора",
                        }
                    }

                    Divider {}

                    Block {
                        Column {
                            Select {
                                label: "Питательный раствор",
                                placeholder: "выбрать раствор",
                                value: solution_select_value,
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

                                Row {
                                    gap: "small",
                                    vertical: "end",

                                    NumberField {
                                        label: "Объем",
                                        value: *volume.read(),
                                        units: "литр",
                                        on_change: move |value| {
                                            stock_solution_builder.write().update_volume(value);
                                        },
                                    }

                                    UnitsSelect {
                                        on_change: move |units| {
                                            println!("units change {}", units);
                                        },
                                    }
                                }
                            }
                        }
                    }

                    Divider {}

                    Block {
                        Row {
                            Column {
                                gap: "medium",

                                Title {
                                    size: "small",
                                    "Часть A",
                                }

                                Column {
                                    gap: "x-small",

                                    for fertilizer in part_a.read().clone() {
                                        QuickAction {
                                            key: "{fertilizer.fertilizer.id()}",

                                            Text {
                                                size: "x-small",
                                                {fertilizer.fertilizer.name()},
                                            }

                                            Text {
                                                size: "x-small",
                                                {render_fertilizer_weight(fertilizer.weight, fertilizer.fertilizer.liquid())},
                                            }
                                        }
                                    }
                                }
                            }

                            Column {
                                gap: "medium",

                                Title {
                                    size: "small",
                                    "Часть B",
                                }

                                Column {
                                    gap: "x-small",

                                    for fertilizer in part_b.read().clone() {
                                        QuickAction {
                                            key: "{fertilizer.fertilizer.id()}",

                                            Text {
                                                size: "x-small",
                                                {fertilizer.fertilizer.name()},
                                            }

                                            Text {
                                                size: "x-small",
                                                {render_fertilizer_weight(fertilizer.weight, fertilizer.fertilizer.liquid())},
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
