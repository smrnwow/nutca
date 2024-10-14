use super::PartsList;
use crate::controller::concentrates::FertilizersBrowser;
use crate::model::chemistry::Volume;
use crate::model::concentrates::{CompositionFromFertilizers, Concentrate};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FromFertilizersProps {
    fertilizers_browser: Memo<FertilizersBrowser>,
    composition: Signal<CompositionFromFertilizers>,
    concentrate: Memo<Concentrate>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_fertilizer_amount_add: EventHandler<(String, String, f64)>,
}

#[component]
pub fn FromFertilizers(props: FromFertilizersProps) -> Element {
    rsx! {
        PartsList {
            fertilizers_browser: props.fertilizers_browser,
            composition_from_fertilizers: props.composition,
            concentrate: props.concentrate,
            on_part_name_update: props.on_part_name_update,
            on_part_concentration_update: props.on_part_concentration_update,
            on_part_volume_update: props.on_part_volume_update,
            on_part_delete: props.on_part_delete,
            on_fertilizer_delete: props.on_fertilizer_delete,
            on_fertilizer_amount_add: props.on_fertilizer_amount_add,
        }
    }
}
