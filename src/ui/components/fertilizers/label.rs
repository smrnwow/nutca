use super::FertilizersComponent;
use crate::model::fertilizers::{Component, Label, Units};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersLabelProps {
    label: Signal<Label>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
}

#[component]
pub fn FertilizersLabel(props: FertilizersLabelProps) -> Element {
    let label = props.label.read();

    rsx! {
        div {
            class: "fertilizers-label",

            div {
                class: "fertilizers-label__units",

                p {
                    class: "fertilizers-label__text",
                    "Единицы измерения: ",
                }

                label {
                    class: "fertilizers-label__unit",

                    input {
                        class: "fertilizers-label__input",
                        r#type: "radio",
                        name: "units",
                        value: "percent",
                        checked: "{label.units() == Units::Percent}",
                        onchange: move |_| props.on_label_units_update.call(Units::Percent),
                    },

                    "Проценты",
                }

                label {
                    class: "fertilizers-label__unit",

                    input {
                        class: "fertilizers-label__input",
                        r#type: "radio",
                        name: "units",
                        value: "weight_volume",
                        checked: "{label.units() == Units::WeightVolume}",
                        onchange: move |_| props.on_label_units_update.call(Units::WeightVolume),
                    },

                    "мг/л",
                }
            }

            p {
                class: "fertilizers-label__title",
                "Макроэлементы"
            }

            div {
                class: "fertilizers-label__group",

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.nitrogen(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.nitrogen_nitrate(),
                                on_update: props.on_label_component_update,
                            }
                        }

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.nitrogen_ammonium(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.phosphor(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.phosphor_pentoxide(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.potassium(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.potassium_oxide(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.calcium(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.calcium_oxide(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.magnesium(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.magnesium_oxide(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.sulfur(),
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.sulfur_trioxide(),
                                on_update: props.on_label_component_update,
                            }
                        }

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label.sulfur_tetroxide(),
                                on_update: props.on_label_component_update,
                            }
                        }
                    }
                }
            }

            p {
                class: "fertilizers-label__title",
                "Микроэлементы"
            }

            div {
                class: "fertilizers-label__group",

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.iron(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.manganese(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.copper(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.zinc(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.boron(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label.molybdenum(),
                            on_update: props.on_label_component_update,
                        }
                    }
                }
            }
        }
    }
}
