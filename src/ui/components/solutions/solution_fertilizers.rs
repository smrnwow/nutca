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
    let solution = props.solution.read();

    rsx! {
        div {
            class: "fertilizers-amount",

            if solution.fertilizers().len() == 0 {
                Text {
                    "Выберите удобрения из списка",
                }
            } else {
                for fertilizer in solution.fertilizers().clone() {
                    Row {
                        align: "space-between",

                        Text {
                            "{fertilizer.fertilizer.name()}",
                        }

                        Text {
                            "{round(fertilizer.weight)} {units(fertilizer.fertilizer.liquid())}",
                        }
                    }
                }
            }
        }
    }
}
