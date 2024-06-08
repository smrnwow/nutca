use super::{CalculatedProfile, FertilizersAmount};
use crate::model::calculation::{Profile, ResultProfile};
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ResultSolutionPreviewProps {
    profile: Signal<Profile>,
    result: Signal<ResultProfile>,
    on_save: EventHandler<String>,
}

#[component]
pub fn ResultSolutionPreview(props: ResultSolutionPreviewProps) -> Element {
    let mut solution_name = use_signal(|| String::new());

    rsx! {
        Card {
            Block {
                Title {
                    text: "Результат",
                }
            }

            Divider {}

            Block {
                CalculatedProfile {
                    desired_profile: props.profile,
                    result_profile: props.result,
                }
            }

            Divider {}

            Block {
                FertilizersAmount {
                    result_profile: props.result,
                }
            }

            Divider {}

            Block {
                div {
                    class: "calculation-index__result-controls",

                    TextField {
                        placeholder: "название раствора",
                        value: solution_name.read(),
                        on_input: move |event| {
                            *solution_name.write() = event;
                        },
                    }

                    Button {
                        style: "primary",
                        text: "Сохранить",
                        on_click: move |_| {
                            props.on_save.call(solution_name.read().clone());

                            *solution_name.write() = String::new();
                        },
                    }
                }
            }
        }
    }
}
