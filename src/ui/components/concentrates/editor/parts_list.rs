use super::PartEditor;
use crate::controller::concentrates::FertilizersBrowser;
use crate::model::chemistry::Volume;
use crate::model::concentrates::{
    CompositionFromFertilizers, CompositionFromSolution, Concentrate, Part,
};
use crate::ui::components::layout::Row;
use crate::ui::components::utils::Pagination;
use dioxus::prelude::*;

fn build_parts_list(parts: Vec<&Part>, limit: usize, page_index: usize) -> Vec<Signal<Part>> {
    let start = (page_index - 1) * limit;

    let end = (page_index * limit) - 1;

    if end < parts.len() {
        return parts[start..=end]
            .into_iter()
            .map(|p| Signal::new((*p).clone()))
            .collect();
    } else {
        return parts[start..]
            .into_iter()
            .map(|p| Signal::new((*p).clone()))
            .collect();
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct PartsListProps {
    fertilizers_browser: Memo<FertilizersBrowser>,
    composition_from_solution: Option<Signal<CompositionFromSolution>>,
    composition_from_fertilizers: Option<Signal<CompositionFromFertilizers>>,
    concentrate: Memo<Concentrate>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_fertilizer_amount_add: Option<EventHandler<(String, String, f64)>>,
    on_fertilizer_percent_distribute: Option<EventHandler<(String, String, usize)>>,
}

#[component]
pub fn PartsList(props: PartsListProps) -> Element {
    let mut page_index = use_signal(|| 1);

    let limit = 2;

    let concentrate = props.concentrate.read();

    let parts: Vec<Signal<Part>> = build_parts_list(concentrate.parts(), limit, *page_index.read());

    let items_count = concentrate.parts().len();

    rsx! {
        div {
            class: "concentrate__parts",

            Row {
                for part in parts {
                    PartEditor {
                        fertilizers_browser: props.fertilizers_browser,
                        composition_from_solution: props.composition_from_solution,
                        composition_from_fertilizers: props.composition_from_fertilizers,
                        part,
                        on_part_name_update:  props.on_part_name_update,
                        on_part_concentration_update: props.on_part_concentration_update,
                        on_part_volume_update: props.on_part_volume_update,
                        on_part_delete: props.on_part_delete,
                        on_fertilizer_delete: props.on_fertilizer_delete,
                        on_fertilizer_amount_add: props.on_fertilizer_amount_add,
                        on_fertilizer_percent_distribute: props.on_fertilizer_percent_distribute,
                    }
                }
            }

            Pagination {
                page_index: *page_index.read(),
                limit,
                items_count,
                on_change: move |next_page| page_index.set(next_page),
            }
        }
    }
}
