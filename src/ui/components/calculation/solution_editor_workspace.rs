use super::{DesiredProfile, FertilizersBrowser};
use crate::model::calculation::Profile;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Accordion, Block, Card, Divider, Step, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorWorkspaceProps {
    profile: Signal<Profile>,
    fertilizers: Signal<Vec<(bool, Fertilizer)>>,
    on_requirement_update: EventHandler<NutrientAmount>,
    on_nitrogen_form_update: EventHandler<NitrogenForm>,
    on_fertilizer_select: EventHandler<(bool, String)>,
    on_fertilizer_search: EventHandler<String>,
    on_water_amount_change: EventHandler<usize>,
}

#[component]
pub fn SolutionEditorWorkspace(props: SolutionEditorWorkspaceProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    text: "Расчет раствора",
                }
            }

            Divider {}

            Accordion {
                opened: true,

                header: rsx! {
                    Step {
                        number: 1,
                        text: "Заполните желаемый профиль питания",
                    }
                },

                body: rsx! {
                    DesiredProfile {
                        profile: props.profile,
                        on_requirement_update: props.on_requirement_update,
                        on_nitrogen_form_update: props.on_nitrogen_form_update,
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
