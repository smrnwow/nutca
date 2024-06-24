use crate::model::solutions::Solution;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn units(liquid: bool) -> String {
    match liquid {
        true => String::from("мл"),
        false => String::from("г"),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionFertilizersProps {
    solution: Memo<Solution>,
}

#[component]
pub fn SolutionFertilizers(props: SolutionFertilizersProps) -> Element {
    let fertilizers = props.solution.read().fertilizers();

    rsx! {
        div {
            class: "fertilizers-amount",

            if fertilizers.len() == 0 {
                Text {
                    size: "x-small",
                    "Выберите удобрения из списка",
                }
            } else {
                for fertilizer in fertilizers {
                    Row {
                        horizontal: "space-between",

                        Text {
                            size: "x-small",
                            "{fertilizer.fertilizer.name()}",
                        }

                        Text {
                            size: "x-small",
                            "{round(fertilizer.weight)} {units(fertilizer.fertilizer.liquid())}",
                        }
                    }
                }
            }
        }
    }
}
