use super::ManualPart;
use crate::controller::concentrates::FertilizersBrowser;
use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::ManualFiller;
use crate::model::solutions::Solution;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ManualFillerProps {
    manual_filler: Memo<ManualFiller>,
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    fertilizers_browser: Memo<FertilizersBrowser>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_part_fertilizer_add: EventHandler<(String, String, f64)>,
    on_part_fertilizer_delete: EventHandler<(String, String)>,
}

#[component]
pub fn ManualFiller(props: ManualFillerProps) -> Element {
    rsx! {
        div {
            class: "concentrate__parts",

            for manual_part in props.manual_filler.read().parts().iter().map(|part| Signal::new(part.clone())) {
                ManualPart {
                    manual_part,
                    fertilizers_browser: props.fertilizers_browser,
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
