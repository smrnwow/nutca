use super::FertilizersSetItem;
use crate::controller::solutions::SolutionFertilizers;
use crate::model::chemistry::Volume;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{List, Pagination, Title};
use crate::ui::components::VolumeField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetProps {
    solution: Memo<Solution>,
    on_volume_update: EventHandler<Volume>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_amount_update: EventHandler<(String, f64)>,
}

#[component]
pub fn FertilizersSet(props: FertilizersSetProps) -> Element {
    let mut page_index = use_signal(|| 1);

    let solution_fertilizers = use_memo(move || {
        let fertilizers = props.solution.read().fertilizers();

        let mut solution_fertilizers = SolutionFertilizers::new(fertilizers);

        solution_fertilizers.paginate(*page_index.read());

        solution_fertilizers
    });

    let volume = use_memo(move || props.solution.read().volume());

    let fertilizers_weights = solution_fertilizers.read().items();

    let items_count = fertilizers_weights.len();

    rsx! {
        Column {
            gap: "medium",

            Row {
                gap: "small",

                Title {
                    size: "small",
                    "Используемые удобрения",
                }
            }

            VolumeField {
                label: "Объем раствора",
                volume,
                on_change: props.on_volume_update,
            }

            List {
                limit: solution_fertilizers.read().limit(),
                empty: solution_fertilizers.read().is_empty(),
                stub_text: "Выберите удобрения из списка",

                for fertilizer_weight in fertilizers_weights {
                    FertilizersSetItem {
                        key: "{fertilizer_weight.id()}",
                        fertilizer_weight: Signal::new(fertilizer_weight),
                        on_exclude: props.on_fertilizer_exclude,
                        on_amount_update: props.on_fertilizer_amount_update,
                    }
                }
            }

            Pagination {
                page_index: solution_fertilizers.read().page_index(),
                limit: solution_fertilizers.read().limit(),
                items_count,
                on_change: move |next_page| {
                    *page_index.write() = next_page;
                },
            }
        }
    }
}
