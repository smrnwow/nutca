use crate::controller::solutions::StockSolutionEditor as Editor;
use crate::repository::Storage;
use crate::ui::components::layout::{Page, Row, Section};
use crate::ui::components::solutions::{StockSolutionControls, StockSolutionPart};
use crate::ui::components::utils::{Block, Card, Divider, Title};
use dioxus::prelude::*;

#[component]
pub fn StockSolutionPage(solution_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut editor = use_signal(|| Editor::new(storage, solution_id));

    let solutions_listing = use_memo(move || editor.read().list_solutions());

    let solution = use_memo(move || editor.read().solution().clone());

    let concentration = use_memo(move || editor.read().concentration());

    let volume = use_memo(move || editor.read().volume());

    let part_a = use_memo(move || editor.read().part_a());

    let part_b = use_memo(move || editor.read().part_b());

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
                        StockSolutionControls {
                            solution,
                            solutions_listing,
                            concentration,
                            volume,
                            on_solution_search: move |search_query| {
                                editor.write().search_solution(search_query);
                            },
                            on_solution_change: move |solution_id| {
                                editor.write().change_solution(solution_id);
                            },
                            on_concentration_change: move |value| {
                                editor.write().change_concentration(value);
                            },
                            on_volume_change: move |volume| {
                                editor.write().change_volume(volume);
                            },
                        }
                    }

                    Divider {}

                    Block {
                        Row {
                            StockSolutionPart {
                                part_name: "A",
                                fertilizers_weights: part_a,
                            }

                            StockSolutionPart {
                                part_name: "B",
                                fertilizers_weights: part_b,
                            }
                        }
                    }
                }
            }
        }
    }
}
