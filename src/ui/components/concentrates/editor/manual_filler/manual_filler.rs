use super::ManualPart;
use crate::controller::concentrates::TanksSet;
use crate::model::chemistry::Volume;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ManualFillerProps {
    tanks_set: Memo<TanksSet>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_part_fertilizer_add: EventHandler<(String, String, f64)>,
    on_part_fertilizer_delete: EventHandler<(String, String)>,
}

#[component]
pub fn ManualFiller(props: ManualFillerProps) -> Element {
    let fertilizers_browser =
        use_memo(move || props.tanks_set.read().fertilizers_browser().clone());

    rsx! {
        div {
            class: "concentrate__parts",

            for manual_part in props.tanks_set.read().manual_parts() {
                ManualPart {
                    manual_part,
                    fertilizers_browser,
                    on_name_update: props.on_part_name_update,
                    on_concentration_update: props.on_part_concentration_update,
                    on_volume_update: props.on_part_volume_update,
                    on_delete: props.on_part_delete,
                    on_part_fertilizer_add: props.on_part_fertilizer_add,
                    on_fertilizer_delete: props.on_part_fertilizer_delete,
                }
            }
        }
    }
}
