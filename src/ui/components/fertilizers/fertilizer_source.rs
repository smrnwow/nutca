use crate::ui::components::fertilizers::{FertilizerFormula, FertilizerLabel};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{ButtonsGroup, ButtonsGroupButton, Title};
use dioxus::prelude::*;
use nutca::fertilizers::{Fertilizer, LabelComponent, LabelUnits, SourceComposition, SourceType};

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerSourceProps {
    fertilizer: Memo<Fertilizer>,
    on_source_type_update: EventHandler<SourceType>,
    on_label_component_update: EventHandler<LabelComponent>,
    on_label_units_update: EventHandler<LabelUnits>,
    on_formula_update: EventHandler<String>,
}

#[component]
pub fn FertilizerSource(props: FertilizerSourceProps) -> Element {
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
                    value: props.fertilizer.read().source_composition().source_type().value(),
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

            match props.fertilizer.read().source_composition().clone() {
                SourceComposition::Label(label) => {
                    rsx! {
                        FertilizerLabel {
                            label: Signal::new(label),
                            on_label_units_update: props.on_label_units_update,
                            on_label_component_update: props.on_label_component_update,
                        }
                    }
                }

                SourceComposition::Formula(formula) => {
                    rsx! {
                        FertilizerFormula {
                            formula: Signal::new(formula),
                            on_formula_update: props.on_formula_update,
                        }
                    }
                }
            }
        }
    }
}
