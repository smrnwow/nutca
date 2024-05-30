use super::NutrientContent;
use crate::model::fertilizers::Contents;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersPreviewProps {
    name: Signal<String>,
    vendor: Signal<String>,
    nutrient_contents: Signal<Contents>,
}

#[component]
pub fn FertilizersPreview(props: FertilizersPreviewProps) -> Element {
    let nutrient_contents = &*props.nutrient_contents.read();

    let name = if *props.name.read() == String::from("") {
        "Без названия".to_string()
    } else {
        props.name.read().clone()
    };

    let vendor = if *props.vendor.read() == String::from("") {
        "Не известно".to_string()
    } else {
        props.vendor.read().clone()
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
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.nitrogen(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.phosphor(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.potassium(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.calcium(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.magnesium(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.sulfur(),
                        }
                    }
                }
            }

            div {
                class: "fertilizers-preview__group",

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.iron(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.manganese(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.copper(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.zinc(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.boron(),
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient nutrient-value__nutrient_small",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientContent {
                            nutrient_content: nutrient_contents.molybdenum(),
                        }
                    }
                }
            }
        }
    }
}
