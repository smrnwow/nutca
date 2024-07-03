use crate::model::fertilizers::{Fertilizer, FertilizerError, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use crate::ui::components::fertilizers::{
    FertilizerComposition, FertilizerDetails, FertilizerSource,
};
use crate::ui::components::layout::Row;
use crate::ui::components::utils::{Block, Button, Card, Divider, Title};
use crate::ui::components::ReferenceSubject;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerEditorProps {
    fertilizer: Memo<Fertilizer>,
    fertilizer_error: Memo<FertilizerError>,
    source_type: Memo<SourceType>,
    label: Memo<Label>,
    formula: Memo<Formula>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
    on_source_type_update: EventHandler<SourceType>,
    on_liquid_update: EventHandler<bool>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
    on_formula_update: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn FertilizerEditor(props: FertilizerEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    "Редактор удобрения",
                }
            }

            Divider {}

            Block {
                FertilizerDetails {
                    fertilizer: props.fertilizer,
                    fertilizer_error: props.fertilizer_error,
                    on_name_update: props.on_name_update,
                    on_vendor_update: props.on_vendor_update,
                    on_liquid_update: props.on_liquid_update,
                }
            }

            Divider {}

            ReferenceSubject {
                Block {
                    FertilizerSource {
                        fertilizer: props.fertilizer,
                        source_type: props.source_type,
                        label: props.label,
                        formula: props.formula,
                        on_source_type_update: props.on_source_type_update,
                        on_label_units_update: props.on_label_units_update,
                        on_label_component_update: props.on_label_component_update,
                        on_formula_update: props.on_formula_update,
                    }
                }
            }

            Divider {}

            ReferenceSubject {
                Block {
                    FertilizerComposition {
                        fertilizer: props.fertilizer
                    }
                }
            }

            Divider {}

            Block {
                Row {
                    horizontal: "end",

                    Button {
                        style: "stroke",
                        on_click: props.on_cancel,
                        "Сбросить",
                    }

                    Button {
                        style: "primary",
                        on_click: props.on_save,
                        "Сохранить",
                    }
                }
            }
        }
    }
}
