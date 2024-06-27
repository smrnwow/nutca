use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::solutions::FertilizersUsedItem;
use crate::ui::components::utils::{NumberField, Pagination, Text, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersUsedProps {
    solution: Memo<Solution>,
    on_volume_update: EventHandler<usize>,
    on_exclude: EventHandler<String>,
}

#[component]
pub fn FertilizersUsed(props: FertilizersUsedProps) -> Element {
    let mut page_index = use_signal(|| 1);

    let fertilizers_set = use_memo(move || {
        let mut fertilizers_set = props.solution.read().fertilizers_set();

        fertilizers_set.paginate(*page_index.read());

        fertilizers_set
    });

    rsx! {
        Column {
            gap: "medium",

            Title {
                size: "small",
                text: "Используемые удобрения",
            }

            Row {
                gap: "small",
                vertical: "center",

                Text {
                    nowrap: true,
                    "Объем раствора: ",
                }

                NumberField {
                    value: props.solution.read().water_amount(),
                    units: "литр",
                    on_change: props.on_volume_update,
                }
            }

            if fertilizers_set.read().is_empty() {
                div {
                    class: "fertilizers-used__stub",

                    Text {
                        size: "x-small",
                        "Выберите удобрения из списка",
                    }
                }
            } else {
                div {
                    class: "fertilizers-used__table",

                    for fertilizer in fertilizers_set.read().items() {
                        FertilizersUsedItem {
                            key: "{fertilizer.fertilizer.id()}",
                            fertilizer,
                            on_exclude: props.on_exclude,
                        }
                    }
                }

                div {
                    class: "fertilizers-browser__pagination",

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
    }
}
