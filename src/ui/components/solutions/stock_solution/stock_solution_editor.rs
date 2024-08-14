use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::solutions::{FertilizerWeight, Solution};
use crate::ui::components::layout::Row;
use crate::ui::components::solutions::stock_solution::{StockSolutionControls, StockSolutionPart};
use crate::ui::components::utils::{Block, Card, Divider, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StockSolutionEditorProps {
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
    concentration: Memo<usize>,
    volume: Memo<Volume>,
    part_a: Memo<Vec<FertilizerWeight>>,
    part_b: Memo<Vec<FertilizerWeight>>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_concentration_change: EventHandler<usize>,
    on_volume_change: EventHandler<Volume>,
}

#[component]
pub fn StockSolutionEditor(props: StockSolutionEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        "Редактор рабочего раствора",
                    }
                }
            }

            Divider {}

            Block {
                StockSolutionControls {
                    solution: props.solution,
                    solutions_listing: props.solutions_listing,
                    concentration: props.concentration,
                    volume: props.volume,
                    on_solution_search: props.on_solution_search,
                    on_solution_change: props.on_solution_search,
                    on_concentration_change: props.on_concentration_change,
                    on_volume_change: props.on_volume_change,
                }
            }

            Divider {}

            Block {
                Row {
                    StockSolutionPart {
                        part_name: "A",
                        fertilizers_weights: props.part_a,
                    }

                    StockSolutionPart {
                        part_name: "B",
                        fertilizers_weights: props.part_b,
                    }
                }
            }
        }
    }
}
