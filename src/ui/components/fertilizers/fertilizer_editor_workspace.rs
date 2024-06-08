use crate::model::chemistry::NitrogenForm;
use crate::model::fertilizers::Fertilizer;
use crate::model::labels::{Component, Units};
use crate::ui::components::fertilizers::FertilizersComposition;
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerEditorWorkspaceProps {
    fertilizer: Signal<Fertilizer>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
    on_label_select: EventHandler<()>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
    on_label_nitrogen_form_update: EventHandler<NitrogenForm>,
    on_formula_select: EventHandler<()>,
    on_formula_update: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn FertilizerEditorWorkspace(props: FertilizerEditorWorkspaceProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор удобрения",
                }
            }

            Divider {}

            Block {
                div {
                    class: "fertilizer-editor__details",

                    TextField {
                        value: props.fertilizer.read().name(),
                        label: "Название",
                        on_input: props.on_name_update,
                    }

                    TextField {
                        value: props.fertilizer.read().vendor(),
                        label: "Производитель",
                        on_input: props.on_vendor_update,
                    }
                }
            }

            Divider {}

            Block {
                FertilizersComposition {
                    fertilizer: props.fertilizer,
                    on_label_select: props.on_label_select,
                    on_label_units_update: props.on_label_units_update,
                    on_label_component_update: props.on_label_component_update,
                    on_label_nitrogen_form_update: props.on_label_nitrogen_form_update,
                    on_formula_select: props.on_formula_select,
                    on_formula_update: props.on_formula_update,
                }
            }

            Divider {}

            Block {
                div {
                    class: "fertilizer-editor__controls",

                    Button {
                        style: "stroke",
                        text: "Сбросить",
                        on_click: props.on_cancel,
                    }

                    Button {
                        style: "primary",
                        text: "Сохранить",
                        on_click: props.on_save,
                    }
                }
            }
        }
    }
}
