use crate::model::calculation::ResultProfile;
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersAmountProps {
    result_profile: Signal<ResultProfile>,
}

#[component]
pub fn FertilizersAmount(props: FertilizersAmountProps) -> Element {
    let result_profile = props.result_profile.read();

    rsx! {
        div {
            class: "fertilizers-amount",

            if result_profile.fertilizers_weights.len() == 0 {
                p {
                    class: "fertilizers-amount__empty",
                    "Выберите удобрения из списка",
                }
            } else {
                for fertilizer_weight in result_profile.fertilizers_weights.clone() {
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
