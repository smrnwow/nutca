use crate::model::fertilizers::FertilizerBuilder;
use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::FertilizerEditor;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizerEditPage(fertilizer_id: String) -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut fertilizer_builder = use_signal(|| {
        let fertilizer = fertilizers_storage.read().get(fertilizer_id);

        match fertilizer {
            Some(fertilizer) => FertilizerBuilder::from(fertilizer),
            None => FertilizerBuilder::new(),
        }
    });

    let fertilizer = use_memo(move || fertilizer_builder.read().build());

    let source_type = use_memo(move || fertilizer_builder.read().source_type());

    let label = use_memo(move || fertilizer_builder.read().label());

    let formula = use_memo(move || fertilizer_builder.read().formula());

    rsx! {
        div {
            class: "fertilizers-add-page",

            section {
                class: "fertilizer-editor",

                FertilizerEditor {
                    fertilizer,
                    source_type,
                    label,
                    formula,
                    on_name_update: move |name| {
                        fertilizer_builder.write().update_name(name);
                    },
                    on_vendor_update: move |vendor| {
                        fertilizer_builder.write().update_vendor(vendor);
                    },
                    on_source_type_update: move |source_type| {
                        fertilizer_builder.write().update_source_type(source_type);
                    },
                    on_label_units_update: move |units| {
                        fertilizer_builder.write().update_label_units(units);
                    },
                    on_label_component_update: move |component| {
                        fertilizer_builder.write().update_label_component(component);
                    },
                    on_formula_update: move |formula: String| {
                        fertilizer_builder.write().update_formula(formula);
                    },
                    on_save: move |_| {
                        let storage = fertilizers_storage.read();

                        storage.update(fertilizer.read().clone());

                        navigator().push(Route::FertilizersListingPage {});
                    },
                    on_cancel: move |_| {
                        navigator().push(Route::FertilizersListingPage {});
                    },
                }
            }
        }
    }
}
