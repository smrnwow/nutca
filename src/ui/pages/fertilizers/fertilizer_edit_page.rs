use crate::controller::fertilizers::FertilizerEditor;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::fertilizers::FertilizerEditor;
use crate::ui::components::layout::{Page, Section};
use dioxus::prelude::*;

#[component]
pub fn FertilizerEditPage(fertilizer_id: String) -> Element {
    let mut toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut fertilizer_editor = use_signal(|| FertilizerEditor::edit(storage, fertilizer_id));

    let mut fertilizer_builder = fertilizer_editor.read().fertilizer_builder();

    let fertilizer = fertilizer_editor.read().fertilizer();

    let validation = fertilizer_editor.read().validation();

    let label = use_memo(move || fertilizer_builder.read().label());

    let formula = use_memo(move || fertilizer_builder.read().formula());

    let source_type = use_memo(move || fertilizer_builder.read().source_type());

    use_effect(move || toaster.write().render(validation.read().list()));

    rsx! {
        Page {
            Section {
                FertilizerEditor {
                    fertilizer,
                    validation,
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
                    on_liquid_update: move |liquid| {
                        fertilizer_builder.write().update_liquid(liquid);
                    },
                    on_label_units_update: move |units| {
                        fertilizer_builder.write().update_label_units(units);
                    },
                    on_label_component_update: move |component| {
                        fertilizer_builder.write().update_label_component(component);
                    },
                    on_formula_update: move |formula| {
                        fertilizer_builder.write().update_formula(formula);
                    },
                    on_save: move |_| {
                        fertilizer_editor.write().update();
                    },
                    on_cancel: move |_| {
                        fertilizer_editor.write().back();
                    },
                }
            }
        }
    }
}
