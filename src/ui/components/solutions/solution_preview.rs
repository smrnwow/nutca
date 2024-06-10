use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::calculation::{CalculatedProfile, FertilizersAmount};
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionPreviewProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    on_save: EventHandler<String>,
}

#[component]
pub fn SolutionPreview(props: SolutionPreviewProps) -> Element {
    let mut solution_name = use_signal(|| props.solution.read().name());

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
                    profile: props.profile,
                    solution: props.solution,
                }
            }

            Divider {}

            Block {
                FertilizersAmount {
                    solution: props.solution,
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
