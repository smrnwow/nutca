use crate::controller::reference::TopicId;
use crate::controller::Validation;
use crate::ui::components::fertilizers::{
    FertilizerComposition, FertilizerDetails, FertilizerSource,
};
use crate::ui::components::layout::Row;
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{Block, Button, Card, Divider, Title};
use dioxus::prelude::*;
use nutca::fertilizers::{Fertilizer, LabelComponent, LabelUnits, SourceType};

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerEditorProps {
    fertilizer: Memo<Fertilizer>,
    validation: Memo<Validation>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
    on_source_type_update: EventHandler<SourceType>,
    on_liquid_update: EventHandler<bool>,
    on_label_units_update: EventHandler<LabelUnits>,
    on_label_component_update: EventHandler<LabelComponent>,
    on_formula_update: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn FertilizerEditor(props: FertilizerEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        {TopicId::FertilizerEditor.title()},
                        ReferenceBadge {
                            topic_id: TopicId::FertilizerEditor,
                        },
                    }
                }
            }

            Divider {}

            Block {
                FertilizerDetails {
                    fertilizer: props.fertilizer,
                    validation: props.validation,
                    on_name_update: props.on_name_update,
                    on_vendor_update: props.on_vendor_update,
                    on_liquid_update: props.on_liquid_update,
                }
            }

            Divider {}

            Block {
                FertilizerSource {
                    fertilizer: props.fertilizer,
                    on_source_type_update: props.on_source_type_update,
                    on_label_units_update: props.on_label_units_update,
                    on_label_component_update: props.on_label_component_update,
                    on_formula_update: props.on_formula_update,
                }
            }

            Divider {}

            Block {
                FertilizerComposition {
                    fertilizer: props.fertilizer
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
