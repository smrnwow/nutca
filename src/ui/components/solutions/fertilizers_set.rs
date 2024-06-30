use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::solutions::{FertilizersSetItem, SolutionVolume};
use crate::ui::components::utils::{List, Pagination, Reference, Text, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetProps {
    solution: Memo<Solution>,
    on_volume_update: EventHandler<usize>,
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

    let volume = use_memo(move || props.solution.read().water_amount());

    let mut show_reference = use_signal(|| false);

    rsx! {
        Column {
            gap: "medium",
            on_hover: move |hovered| {
                show_reference.set(hovered);
            },

            Row {
                gap: "small",

                Title {
                    size: "small",

                    "Используемые удобрения",

                    Reference {
                        display: show_reference,
                        style: "badge",
                        tooltip: rsx! {
                            Title {
                                size: "x-small",
                                "Пока не придуманый заголовок",
                            }

                            Text {
                                size: "x-small",
                                "Еще не придуманный текст. Еще не придуманный текст. Еще не придуманный текст.",
                            }
                        },
                        tooltip_position: "top-center",
                    }
                }
            }

            SolutionVolume {
                volume,
                on_volume_update: props.on_volume_update,
                on_units_change: move |units| {
                    println!("units change {}", units);
                },
            }

            List {
                limit: 8,
                empty: fertilizers_set.read().is_empty(),
                stub_text: "Выберите удобрения из списка",

                for fertilizer in fertilizers_set.read().items() {
                    FertilizersSetItem {
                        key: "{fertilizer.fertilizer.id()}",
                        fertilizer,
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
