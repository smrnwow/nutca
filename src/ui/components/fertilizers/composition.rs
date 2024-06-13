use super::{FertilizersFormula, FertilizersLabel};
use crate::model::fertilizers::SourceType;
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use dioxus::prelude::*;

fn tab_active_class(source_type: SourceType, tab_value: SourceType) -> String {
    if source_type == tab_value {
        String::from("fertilizers-source__tab fertilizers-source__tab_active")
    } else {
        String::from("fertilizers-source__tab")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersCompositionProps {
    source_type: Memo<SourceType>,
    label: Memo<Label>,
    formula: Memo<Formula>,
    on_source_type_update: EventHandler<SourceType>,
    on_label_component_update: EventHandler<Component>,
    on_label_units_update: EventHandler<Units>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizersComposition(props: FertilizersCompositionProps) -> Element {
    let source_type = *props.source_type.read();

    rsx! {
        div {
            class: "fertilizers-source",

            div {
                class: "fertilizers-source__title",
                "Состав"
            }

            div {
                class: "fertilizers-source__tabs",

                button {
                    class: "{tab_active_class(source_type, SourceType::Label)}",
                    onclick: move |_| props.on_source_type_update.call(SourceType::Label),
                    "С этикетки"
                }

                button {
                    class: "{tab_active_class(source_type, SourceType::Formula)}",
                    onclick: move |_| props.on_source_type_update.call(SourceType::Formula),
                    "По формуле"
                }
            }
        }

        div {
            class: "composition__source",

            match source_type {
                SourceType::Label => {
                    rsx! {
                        FertilizersLabel {
                            label: props.label,
                            on_label_units_update: props.on_label_units_update,
                            on_label_component_update: props.on_label_component_update,
                        }
                    }
                }

                SourceType::Formula => {
                    rsx! {
                        FertilizersFormula {
                            formula: props.formula,
                            on_formula_update: props.on_formula_update,
                        }
                    }
                }
            }
        }
    }
}
