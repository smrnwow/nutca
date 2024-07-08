use super::{FertilizerFormula, FertilizerLabel};
use crate::model::fertilizers::{Fertilizer, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{ButtonsGroup, ButtonsGroupButton, Title};
use dioxus::prelude::*;

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
                    ReferenceBadge {
                        article_id: "fertilizer-editor-composition-source",
                    },
                }

                ButtonsGroup {
                    value: source_type.value(),
                    buttons: vec![
                        ButtonsGroupButton {
                            label: SourceType::Label.label(),
                            value: SourceType::Label.value(),
                            badge: None,
                        },
                        ButtonsGroupButton {
                            label: SourceType::Formula.label(),
                            value: SourceType::Formula.value(),
                            badge: None,
                        },
                    ],
                    on_change: move |value| props.on_source_type_update.call(SourceType::from(value)),
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
                            formula: props.formula,
                            on_formula_update: props.on_formula_update,
                        }
                    }
                }
            }
        }
    }
}
