use crate::model::solutions::{FertilizerWeight, Solution};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn total_weight(fertilizers: &Vec<FertilizerWeight>) -> String {
    let mut total_weight = 0.0;

    fertilizers.iter().for_each(|fertilizer_weight| {
        total_weight += fertilizer_weight.weight;
    });

    round(total_weight)
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

                div {
                    class: "fertilizers-amount__total",

                    "Общий объем удобрений: {total_weight(&solution.fertilizers())}",
                }
            }
        }
    }
}
