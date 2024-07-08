use crate::model::chemistry::Volume;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::solutions::FertilizersSetItem;
use crate::ui::components::utils::{List, Pagination, Title};
use crate::ui::components::VolumeField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetProps {
    solution: Memo<Solution>,
    on_volume_update: EventHandler<Volume>,
    on_exclude: EventHandler<String>,
}

#[component]
pub fn FertilizersSet(props: FertilizersSetProps) -> Element {
    let mut page_index = use_signal(|| 1);

    let fertilizers_set = use_memo(move || {
        let mut fertilizers_set = props.solution.read().fertilizers_set();

        fertilizers_set.paginate(*page_index.read());

        fertilizers_set
    });

    let volume = use_memo(move || props.solution.read().volume());

    rsx! {
        Column {
            gap: "medium",

            Row {
                gap: "small",

                Title {
                    size: "small",
                    "Используемые удобрения",
                    ReferenceBadge {
                        article_id: "solution-editor-fertilizers-set",
                    },
                }
            }

            VolumeField {
                label: "Объем раствора",
                volume,
                on_change: props.on_volume_update,
            }

            List {
                limit: fertilizers_set.read().limit(),
                empty: fertilizers_set.read().is_empty(),
                stub_text: "Выберите удобрения из списка",

                for fertilizer_weight in fertilizers_set.read().items() {
                    FertilizersSetItem {
                        key: "{fertilizer_weight.fertilizer.id()}",
                        fertilizer_weight: Signal::new(fertilizer_weight),
                        on_exclude: props.on_exclude,
                    }
                }
            }

            Pagination {
                page_index: fertilizers_set.read().page_index(),
                limit: fertilizers_set.read().limit(),
                total: fertilizers_set.read().total(),
                on_change: move |next_page| {
                    *page_index.write() = next_page;
                },
            }
        }
    }
}
