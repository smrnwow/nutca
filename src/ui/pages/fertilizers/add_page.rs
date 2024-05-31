use crate::model::fertilizers::Fertilizer;
use crate::model::formulas::Formula;
use crate::model::labels::{Label, Units};
use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::{
    FertilizersComposition, FertilizersDetails, FertilizersPreview,
};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersAddPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut fertilizer = use_signal(|| Fertilizer::build());

    let mut label = use_signal(|| Label::new(Units::Percent));

    let mut formula = use_signal(|| Formula::from(""));

    rsx! {
        div {
            class: "fertilizers-add-page",

            section {
                class: "editor",

                h3 {
                    class: "calculation-index__title",
                    "Редактор"
                }

                FertilizersDetails {
                    fertilizer,
                    on_name_update: move |name| {
                        fertilizer.write().with_name(name);
                    },
                    on_vendor_update: move |vendor| {
                        fertilizer.write().with_vendor(vendor);
                    },
                }

                FertilizersComposition {
                    fertilizer,
                    on_label_select: move |_| {
                        fertilizer.write().with_label(*label.read());
                    },
                    on_label_units_update: move |units| {
                        label.write().update_units(units);

                        fertilizer.write().with_label(*label.read());
                    },
                    on_label_component_update: move |component| {
                        label.write().update_component(component);

                        fertilizer.write().with_label(*label.read());
                    },
                    on_label_nitrogen_form_update: move |nitrogen_form| {
                        label.write().update_nitrogen_form(nitrogen_form);

                        fertilizer.write().with_label(*label.read());
                    },
                    on_formula_select: move |_| {
                        fertilizer.write().with_formula(formula.read().clone());
                    },
                    on_formula_update: move |formula_update: String| {
                        *formula.write() = Formula::from(formula_update);

                        fertilizer.write().with_formula(formula.read().clone());
                    }
                }
            }

            section {
                class: "preview",

                h3 {
                    class: "calculation-index__title",
                    "Результат"
                }

                FertilizersPreview {
                    fertilizer,
                }

                button {
                    class: "fertilizers-add-page__button",
                    onclick: move |_| {
                        let storage = fertilizers_storage.read();

                        let fertilizer_id = storage.add(fertilizer.read().clone());

                        println!("{:#?}", fertilizer_id);

                        navigator().push(Route::FertilizersIndexPage {});
                    },
                    "Добавить",
                }
            }
        }
    }
}
