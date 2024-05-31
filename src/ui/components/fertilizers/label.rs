use super::{FertilizersComponent, NitrogenFormValue};
use crate::model::chemistry::NitrogenForm;
use crate::model::labels::{Component, Label, Units};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersLabelProps {
    label: Signal<Label>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
    on_label_nitrogen_form_update: EventHandler<NitrogenForm>,
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
                            component: label[Component::Nitrogen(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            NitrogenFormValue {
                                nitrogen_form: label[NitrogenForm::Nitrate(0.0)],
                                on_update: props.on_label_nitrogen_form_update,
                            }
                        }

                        div {
                            class: "nutrient-value__form",

                            NitrogenFormValue {
                                nitrogen_form: label[NitrogenForm::Ammonium(0.0)],
                                on_update: props.on_label_nitrogen_form_update,
                            }
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Phosphor(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::PhosphorPentoxide(0.0)],
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
                            component: label[Component::Potassium(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::PotassiumOxide(0.0)],
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
                            component: label[Component::Calcium(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::CalciumOxide(0.0)],
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
                            component: label[Component::Magnesium(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::MagnesiumOxide(0.0)],
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
                            component: label[Component::Sulfur(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    div {
                        class: "nutrient-value__forms",

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::SulfurTrioxide(0.0)],
                                on_update: props.on_label_component_update,
                            }
                        }

                        div {
                            class: "nutrient-value__form",

                            FertilizersComponent {
                                component: label[Component::SulfurTetroxide(0.0)],
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
                            component: label[Component::Iron(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Manganese(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Copper(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Zinc(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Boron(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    div {
                        class: "nutrient-value__elemental",

                        FertilizersComponent {
                            component: label[Component::Molybdenum(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }
            }
        }
    }
}
