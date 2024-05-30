use crate::model::fertilizers::{Composition, Contents, Fertilizer, Formula, Label, Units};
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

    let mut fertilizer_composition = use_signal(|| Composition::Label(Label::new(Units::Percent)));

    let mut fertilizer_label = use_signal(|| Label::new(Units::Percent));

    let mut fertilizer_name = use_signal(|| String::from(""));

    let mut fertilizer_vendor = use_signal(|| String::from(""));

    let mut nutrient_contents = use_signal(|| Contents::new());

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
                    name: fertilizer_name,
                    on_name_update: move |name| *fertilizer_name.write() = name,
                    vendor: fertilizer_vendor,
                    on_vendor_update: move |vendor| *fertilizer_vendor.write() = vendor,
                }

                FertilizersComposition {
                    composition: fertilizer_composition,
                    on_label_select: move |_| {
                        let label = fertilizer_label.read().clone();

                        *fertilizer_composition.write() = Composition::Label(label);

                        let composition = fertilizer_composition.read().clone();

                        *nutrient_contents.write() = composition.into();
                    },
                    on_label_units_update: move |units| {
                        fertilizer_label.write().update_units(units);

                        *fertilizer_composition.write() = Composition::Label(*fertilizer_label.read());

                        let composition = fertilizer_composition.read().clone();

                        *nutrient_contents.write() = composition.into();
                    },
                    on_label_component_update: move |component| {
                        fertilizer_label.write().update_component(component);

                        *fertilizer_composition.write() = Composition::Label(*fertilizer_label.read());

                        let composition = fertilizer_composition.read().clone();

                        *nutrient_contents.write() = composition.into();
                    },
                    on_formula_select: move |_| {
                        *fertilizer_composition.write() = Composition::Formula(Formula::new("".to_string()));

                        let composition = fertilizer_composition.read().clone();

                        *nutrient_contents.write() = composition.into();
                    },
                    on_formula_update: move |formula| {
                        *fertilizer_composition.write() = Composition::Formula(Formula::new(formula));

                        let composition = fertilizer_composition.read().clone();

                        *nutrient_contents.write() = composition.into();
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
                    name: fertilizer_name,
                    vendor: fertilizer_vendor,
                    nutrient_contents: nutrient_contents
                }

                button {
                    class: "fertilizers-add-page__button",
                    onclick: move |_| {
                        let storage = fertilizers_storage.read();


                        let fertilizer_id = storage.add(
                            Fertilizer::build()
                                .set_name(fertilizer_name.read().clone())
                                .set_vendor(fertilizer_vendor.read().clone())
                                .set_composition(fertilizer_composition.read().clone())
                        );

                        println!("{:#?}", fertilizer_id);

                        navigator().push(Route::FertilizersIndexPage {});
                    },
                    "Добавить",
                }
            }
        }
    }
}
