use super::{FertilizerFormula, FertilizerLabel};
use crate::model::fertilizers::{Fertilizer, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{ButtonsGroup, Title};
use dioxus::prelude::*;

fn tab_class(source_type: SourceType, tab_value: SourceType) -> String {
    if source_type == tab_value {
        String::from(
            "buttons-group__button buttons-group__button_size-small buttons-group__button_active",
        )
    } else {
        String::from("buttons-group__button buttons-group__button_size-small")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerSourceProps {
    source_type: Memo<SourceType>,
    fertilizer: Memo<Fertilizer>,
    label: Memo<Label>,
    formula: Memo<Formula>,
    on_source_type_update: EventHandler<SourceType>,
    on_label_component_update: EventHandler<Component>,
    on_label_units_update: EventHandler<Units>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizerSource(props: FertilizerSourceProps) -> Element {
    let source_type = *props.source_type.read();

    rsx! {
        Column {
            gap: "medium",

            Row {
                horizontal: "space-between",
                vertical: "center",

                Title {
                    size: "small",
                    "Состав",
                }

                ButtonsGroup {
                    button {
                        class: tab_class(source_type, SourceType::Label),
                        onclick: move |_| props.on_source_type_update.call(SourceType::Label),
                        "С этикетки",
                    }

                    button {
                        class: tab_class(source_type, SourceType::Formula),
                        onclick: move |_| props.on_source_type_update.call(SourceType::Formula),
                        "По формуле",
                    }
                }
            }

            match source_type {
                SourceType::Label => {
                    rsx! {
                        FertilizerLabel {
                            label: props.label,
                            on_label_units_update: props.on_label_units_update,
                            on_label_component_update: props.on_label_component_update,
                        }
                    }
                }

                SourceType::Formula => {
                    rsx! {
                        FertilizerFormula {
                            fertilizer: props.fertilizer,
                            formula: props.formula,
                            on_formula_update: props.on_formula_update,
                        }
                    }
                }
            }
        }
    }
}
