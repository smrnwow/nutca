use crate::model::solutions::Solution;
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersAmountProps {
    solution: Memo<Solution>,
}

#[component]
pub fn FertilizersAmount(props: FertilizersAmountProps) -> Element {
    let solution = props.solution.read();

    rsx! {
        div {
            class: "fertilizers-amount",

            if solution.fertilizers().len() == 0 {
                p {
                    class: "fertilizers-amount__empty",
                    "Выберите удобрения из списка",
                }
            } else {
                for fertilizer_weight in solution.fertilizers().clone() {
                    div {
                        class: "fertilizers-amount__fertilizer",

                        p {
                            class: "fertilizers-amount__fertilizer-name",
                            "{fertilizer_weight.fertilizer.name()}",
                        }

                        p {
                            class: "fertilizers-amount__fertilizer-weight",
                            "{round(fertilizer_weight.weight)} г",
                        }
                    }
                }
            }
        }
    }
}
