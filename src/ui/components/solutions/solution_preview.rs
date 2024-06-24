use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::solutions::{SolutionComposition, SolutionFertilizers};
use crate::ui::components::utils::{
    Block, Button, Card, Divider, NumberField, Text, TextField, Title,
};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionPreviewProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    on_volume_update: EventHandler<usize>,
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
                SolutionComposition {
                    solution: props.solution,
                }
            }

            Divider {}

            Block {
                Column {
                    SolutionFertilizers {
                        solution: props.solution,
                    }

                    Divider {}

                    Row {
                        NumberField {
                            value: props.solution.read().water_amount(),
                            units: "литр",
                            on_change: props.on_volume_update,
                        }

                        Row {
                            horizontal: "space-between",

                            Text {
                                size: "x-small",
                                "EC",
                            }

                            Text {
                                size: "x-small",
                                "{round(props.solution.read().ec())}",
                            }
                        }
                    }
                }
            }

            Divider {}

            Block {
                Row {
                    TextField {
                        placeholder: "название раствора",
                        value: solution_name.read(),
                        on_input: move |event| {
                            *solution_name.write() = event;
                        },
                    }

                    Button {
                        style: "primary",
                        on_click: move |_| {
                            props.on_save.call(solution_name.read().clone());

                            *solution_name.write() = String::new();
                        },

                        "Сохранить",
                    }
                }
            }
        }
    }
}
