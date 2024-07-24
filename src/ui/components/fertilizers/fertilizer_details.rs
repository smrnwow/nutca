use crate::controller::Validation;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Checkbox, TextField};
use dioxus::prelude::*;
use nutca::fertilizers::Fertilizer;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerDetailsProps {
    fertilizer: Memo<Fertilizer>,
    validation: Memo<Validation>,
    on_name_update: EventHandler<String>,
    on_vendor_update: EventHandler<String>,
    on_liquid_update: EventHandler<bool>,
}

#[component]
pub fn FertilizerDetails(props: FertilizerDetailsProps) -> Element {
    rsx! {
        Column {
            Row {
                TextField {
                    label: "Название",
                    value: props.fertilizer.read().name(),
                    error: props.validation.read().get("fertilizer-name"),
                    on_input: props.on_name_update,
                }

                TextField {
                    value: props.fertilizer.read().vendor(),
                    label: "Производитель",
                    on_input: props.on_vendor_update,
                }
            }

            Row {
                Checkbox {
                    text: "жидкое удобрение",
                    value: props.fertilizer.read().liquid(),
                    on_change: props.on_liquid_update,
                }
            }
        }
    }
}
