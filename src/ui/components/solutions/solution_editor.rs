use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::{Component, Profile};
use crate::model::solutions::Solution;
use crate::ui::components::calculation::{DesiredProfile, FertilizersBrowser};
use crate::ui::components::utils::{Accordion, Block, Card, Divider, Step, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Signal<Solution>,
    profile: Signal<Profile>,
    fertilizers: Signal<Vec<(bool, Fertilizer)>>,
    on_profile_change: EventHandler<Option<Profile>>,
    on_component_update: EventHandler<Component>,
    on_fertilizer_select: EventHandler<(bool, String)>,
    on_fertilizer_search: EventHandler<String>,
    on_water_amount_change: EventHandler<usize>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор раствора",
                }
            }

            Divider {}

            Accordion {
                opened: true,

                header: rsx! {
                    Step {
                        number: 1,
                        text: "Заполните профиль питания",
                    }
                },

                body: rsx! {
                    DesiredProfile {
                        profile: props.profile,
                        on_component_update: props.on_component_update,
                        on_profile_change: props.on_profile_change,
                    }
                }
            }

            Divider {}

            Accordion {
                header: rsx! {
                    Step {
                        number: 2,
                        text: "Укажите объем воды",
                    }
                },

                body: rsx! {
                    div {
                        class: "calculation-index__water-amount",

                        label {
                            class: "calculation-index__water-amount-label",

                            input {
                                class: "calculation-index__water-amount-input",
                                r#type: "number",
                                value: 1,
                                oninput: move |event| {
                                    props.on_water_amount_change.call(event.value().parse().unwrap());
                                },
                            }

                            span {
                                class: "calculation-index__water-amount-tip",
                                "литр"
                            }
                        }
                    }
                }
            }

            Divider {}

            Accordion {
                header: rsx! {
                    Step {
                        number: 3,
                        text: "Выберите удобрения",
                    }
                },

                body: rsx! {
                    FertilizersBrowser {
                        fertilizers: props.fertilizers,
                        on_select: props.on_fertilizer_select,
                        on_search: props.on_fertilizer_search,
                    }
                }
            }
        }
    }
}
