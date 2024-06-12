use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::{Component, Profile};
use crate::model::solutions::Solution;
use crate::ui::components::calculation::FertilizersBrowser;
use crate::ui::components::layout::Column;
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::utils::{
    Accordion, Block, Card, Divider, NumberField, Select, Step, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    profiles: Memo<Vec<Profile>>,
    fertilizers: Memo<Vec<Fertilizer>>,
    selected_fertilizers: Memo<Vec<String>>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_component_update: EventHandler<Component>,
    on_fertilizer_select: EventHandler<Fertilizer>,
    on_fertilizer_remove: EventHandler<String>,
    on_fertilizer_search: EventHandler<String>,
    on_water_amount_update: EventHandler<usize>,
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
                    Column {
                        Select {
                            placeholder: "выбрать готовый профиль",
                            value: (props.profile.read().id(), props.profile.read().name()),
                            options: props.profiles.read()
                                .iter()
                                .map(|profile| (profile.id(), profile.name()))
                                .collect(),
                            on_search: move |search_query| {
                                props.on_profile_search.call(search_query);
                            },
                            on_change: move |profile_id| {
                                props.on_profile_change.call(profile_id);
                            },
                        }

                        ProfileForm {
                            profile: props.profile,
                            on_component_update: props.on_profile_component_update,
                        }
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
                    NumberField {
                        value: props.solution.read().water_amount(),
                        units: "литр",
                        on_change: props.on_water_amount_update,
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
                        selected_fertilizers: props.selected_fertilizers,
                        on_select: props.on_fertilizer_select,
                        on_remove: props.on_fertilizer_remove,
                        on_search: props.on_fertilizer_search,
                    }
                }
            }
        }
    }
}
