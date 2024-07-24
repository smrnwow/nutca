use crate::ui::components::layout::Column;
use crate::ui::components::utils::{QuickAction, Text, Title};
use dioxus::prelude::*;
use nutca::solutions::FertilizerWeight;

#[derive(Props, PartialEq, Clone)]
pub struct StockSolutionPartProps {
    part_name: String,
    fertilizers_weights: Memo<Vec<FertilizerWeight>>,
}

#[component]
pub fn StockSolutionPart(props: StockSolutionPartProps) -> Element {
    rsx! {
        Column {
            gap: "medium",

            Title {
                size: "small",
                "Часть {props.part_name}",
            }

            Column {
                gap: "x-small",

                for fertilizer_weight in props.fertilizers_weights.read().clone() {
                    QuickAction {
                        key: "{fertilizer_weight.id()}",

                        Text {
                            size: "x-small",
                            {fertilizer_weight.name()},
                        }

                        Text {
                            size: "x-small",
                            {fertilizer_weight.amount()},
                        }
                    }
                }
            }
        }
    }
}
