use super::FertilizerPercentButton;
use crate::controller::concentrates::FertilizersStack;
use crate::model::chemistry::Volume;
use crate::model::concentrates::parts::AutoPart;
use crate::ui::components::concentrates::editor::{FertilizerItem, PartSettings};
use crate::ui::components::layout::Column;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AutoPartProps {
    auto_part: Signal<AutoPart>,
    fertilizers_stack: Memo<FertilizersStack>,
    on_name_update: EventHandler<(String, String)>,
    on_concentration_update: EventHandler<(String, usize)>,
    on_volume_update: EventHandler<(String, Volume)>,
    on_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_fertilizer_add: EventHandler<(String, String, f64)>,
}

#[component]
pub fn AutoPart(props: AutoPartProps) -> Element {
    rsx! {
        div {
            class: "part-editor",

            Column {
                gap: "x-small",

                PartSettings {
                    name: props.auto_part.read().name(),
                    concentration: props.auto_part.read().concentration(),
                    volume: props.auto_part.read().volume(),
                    on_name_update: move |name| {
                        props.on_name_update.call((props.auto_part.read().id().clone(), name));
                    },
                    on_concentration_update: move |concentration| {
                        props.on_concentration_update.call((props.auto_part.read().id().clone(), concentration));
                    },
                    on_volume_update: move |volume| {
                        props.on_volume_update.call((props.auto_part.read().id().clone(), volume));
                    },
                    on_delete: move |_| props.on_delete.call(props.auto_part.read().id().clone()),
                }

                Column {
                    gap: "xx-small",

                    for fertilizer_item in props.fertilizers_stack.read().list(&props.auto_part.read()) {
                        FertilizerItem {
                            key: "{fertilizer_item.read().0.id()}",
                            fertilizer_item,
                            on_delete: move |fertilizer_id| {
                                let part_id = props.auto_part.read().id().clone();

                                props.on_fertilizer_delete.call((part_id, fertilizer_id));
                            },
                        }
                    }
                }

                FertilizerPercentButton {
                    auto_part: props.auto_part,
                    fertilizers_stack: props.fertilizers_stack,
                    on_fertilizer_add: props.on_fertilizer_add,
                }
            }
        }
    }
}
