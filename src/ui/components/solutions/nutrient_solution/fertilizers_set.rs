use super::FertilizersSetItem;
use crate::controller::solutions::FertilizersUsed;
use crate::model::chemistry::Volume;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Label, List, Pagination, Title, VolumeInput};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetProps {
    solution: Memo<Solution>,
    fertilizers_used: Memo<FertilizersUsed>,
    on_volume_update: EventHandler<Volume>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_amount_update: EventHandler<(String, f64)>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn FertilizersSet(props: FertilizersSetProps) -> Element {
    let volume = use_memo(move || props.solution.read().volume());

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

            Label {
                text: "Объем раствора",

                VolumeInput {
                    volume,
                    on_change: props.on_volume_update,
                }
            }

            /*
            VolumeField {
                label: "Объем раствора",
                volume,
                on_change: props.on_volume_update,
            }
            */

            List {
                limit: props.fertilizers_used.read().limit(),
                empty: props.fertilizers_used.read().is_empty(),
                stub_text: "Выберите удобрения из списка",

                for fertilizer_amount in props.fertilizers_used.read().items().into_iter().map(|f| Signal::new(f)) {
                    FertilizersSetItem {
                        key: "{fertilizer_amount.read().fertilizer().id()}",
                        fertilizer_amount,
                        on_exclude: props.on_fertilizer_exclude,
                        on_amount_update: props.on_fertilizer_amount_update,
                    }
                }
            }

            Pagination {
                page_index: props.fertilizers_used.read().page_index(),
                limit: props.fertilizers_used.read().limit(),
                items_count: props.fertilizers_used.read().count(),
                on_change: move |next_page| props.on_paginate.call(next_page),
            }
        }
    }
}
