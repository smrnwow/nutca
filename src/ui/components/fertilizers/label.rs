use crate::model::labels::{Component, Label, Units};
use crate::ui::components::fertilizers::FertilizerComponentInput;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersLabelProps {
    label: Memo<Label>,
    on_label_units_update: EventHandler<Units>,
    on_label_component_update: EventHandler<Component>,
}

#[component]
pub fn FertilizersLabel(props: FertilizersLabelProps) -> Element {
    let label = props.label.read();

    rsx! {
        Column {
            gap: "medium",

            Row {
                gap: "small",

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Nitrogen(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::NitrogenNitrate(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::NitrogenAmmonium(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Phosphor(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::PhosphorPentoxide(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Potassium(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::PotassiumOxide(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Calcium(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::CalciumOxide(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Magnesium(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::MagnesiumOxide(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    gap: "x-small",

                    FertilizerComponentInput {
                        component: label[Component::Sulfur(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::SulfurTrioxide(0.0)],
                        on_update: props.on_label_component_update,
                    }

                    FertilizerComponentInput {
                        component: label[Component::SulfurTetroxide(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }
            }

            Row {
                gap: "small",

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Iron(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Manganese(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Copper(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Zinc(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Boron(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }

                Column {
                    FertilizerComponentInput {
                        component: label[Component::Molybdenum(0.0)],
                        on_update: props.on_label_component_update,
                    }
                }
            }

            Row {
                gap: "medium",

                Text {
                    size: "x-small",
                    nowrap: true,
                    "Единицы измерения",
                }

                Row {
                    gap: "medium",

                    Row {
                        gap: "x-small",

                        input {
                            class: "fertilizers-label__input",
                            r#type: "radio",
                            name: "units",
                            value: "percent",
                            checked: "{label.units() == Units::Percent}",
                            onchange: move |_| props.on_label_units_update.call(Units::Percent),
                        },

                        Text {
                            size: "x-small",
                            "Проценты",
                        }
                    }

                    Row {
                        gap: "x-small",

                        input {
                            class: "fertilizers-label__input",
                            r#type: "radio",
                            name: "units",
                            value: "weight_volume",
                            checked: "{label.units() == Units::WeightVolume}",
                            onchange: move |_| props.on_label_units_update.call(Units::WeightVolume),
                        },

                        Text {
                            size: "x-small",
                            "мг/л",
                        }
                    }
                }
            }
        }
    }
}
