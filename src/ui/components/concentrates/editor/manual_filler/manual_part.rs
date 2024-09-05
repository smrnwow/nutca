use super::FertilizerWeightButton;
use crate::controller::concentrates::FertilizersBrowser;
use crate::model::chemistry::Volume;
use crate::model::concentrates::parts::ManualPart;
use crate::ui::components::concentrates::editor::{FertilizerItem, PartSettings};
use crate::ui::components::layout::Column;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ManualPartProps {
    manual_part: Signal<ManualPart>,
    fertilizers_browser: Memo<FertilizersBrowser>,
    on_name_update: EventHandler<(String, String)>,
    on_concentration_update: EventHandler<(String, usize)>,
    on_volume_update: EventHandler<(String, Volume)>,
    on_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_part_fertilizer_add: EventHandler<(String, String, f64)>,
}

#[component]
pub fn ManualPart(props: ManualPartProps) -> Element {
    rsx! {
        div {
            class: "part-editor",

            PartSettings {
                name: props.manual_part.read().name(),
                concentration: props.manual_part.read().concentration(),
                volume: props.manual_part.read().volume(),
                on_name_update: move |name| {
                    props.on_name_update.call((props.manual_part.read().id().clone(), name));
                },
                on_concentration_update: move |concentration| {
                    props.on_concentration_update.call((props.manual_part.read().id().clone(), concentration));
                },
                on_volume_update: move |volume| {
                    props.on_volume_update.call((props.manual_part.read().id().clone(), volume));
                },
                on_delete: move |_| props.on_delete.call(props.manual_part.read().id().clone()),
            }

            Column {
                gap: "x-small",

                for fertilizer_item in props.fertilizers_browser.read().list(&props.manual_part.read()) {
                    FertilizerItem {
                        fertilizer_item,
                        on_delete: move |fertilizer_id| {
                            let part_id = props.manual_part.read().id().clone();

                            props.on_fertilizer_delete.call((part_id, fertilizer_id));
                        },
                    }
                }
            }

            FertilizerWeightButton {
                fertilizers_browser: props.fertilizers_browser,
                on_fertilizer_add: move |(fertilizer_id, amount)| {
                    let part_id = props.manual_part.read().id().clone();

                    props.on_part_fertilizer_add.call((part_id, fertilizer_id, amount));
                },
            }
        }
    }
}
