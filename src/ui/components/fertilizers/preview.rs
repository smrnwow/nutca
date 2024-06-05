use super::{NitrogenFormValue, NutrientContentValue};
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersPreviewProps {
    fertilizer: Signal<Fertilizer>,
}

#[component]
pub fn FertilizersPreview(props: FertilizersPreviewProps) -> Element {
    let fertilizer = &*props.fertilizer.read();

    let name = if fertilizer.name() == String::from("") {
        "Без названия".to_string()
    } else {
        fertilizer.name()
    };

    let vendor = if fertilizer.vendor() == String::from("") {
        "Не известно".to_string()
    } else {
        fertilizer.vendor()
    };

    rsx! {
        div {
            class: "fertilizers-preview",

            p {
                class: "fertilizers-preview__title",
                "{name}"
            }

            p {
                class: "fertilizers-preview__text",
                "Производство: {vendor}"
            }

            p {
                class: "fertilizers-preview__text",
                "Состав:"
            }

            div {
                class: "fertilizers-preview__group",

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Nitrogen(0.0)],
                    }

                    NitrogenFormValue {
                        nitrogen_form: fertilizer[NitrogenForm::Nitrate(0.0)],
                        on_update: move |_| {},
                    }

                    NitrogenFormValue {
                        nitrogen_form: fertilizer[NitrogenForm::Ammonium(0.0)],
                        on_update: move |_| {},
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Phosphorus(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Potassium(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Calcium(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Magnesium(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Sulfur(0.0)],
                    }
                }
            }

            div {
                class: "fertilizers-preview__group",

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Iron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Manganese(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Copper(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Zinc(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Boron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientContentValue {
                        nutrient_amount: fertilizer[NutrientAmount::Molybdenum(0.0)],
                    }
                }
            }
        }
    }
}
