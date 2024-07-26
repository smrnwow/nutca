use crate::ui::components::fertilizers::FertilizerComponentInput;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Label, Radio};
use dioxus::prelude::*;
use nutca::fertilizers::{Label, LabelComponent, LabelUnits};

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerLabelProps {
    label: Signal<Label>,
    on_label_units_update: EventHandler<LabelUnits>,
    on_label_component_update: EventHandler<LabelComponent>,
}

#[component]
pub fn FertilizerLabel(props: FertilizerLabelProps) -> Element {
    let label = props.label.read();

    rsx! {
        Column {
            gap: "medium",

            Label {
                text: "Единицы измерения",

                Row {
                    gap: "medium",

                    Radio {
                        text: "Проценты",
                        name: "units",
                        value: "percent",
                        checked: label.units() == LabelUnits::Percent,
                        on_change: move |_| props.on_label_units_update.call(LabelUnits::Percent),
                    }

                    Radio {
                        text: "мг/л",
                        name: "units",
                        value: "weight_volume",
                        checked: label.units() == LabelUnits::WeightVolume,
                        on_change: move |_| props.on_label_units_update.call(LabelUnits::WeightVolume),
                    }
                }
            }

            Label {
                text: "Компоненты",

                Row {
                    gap: "small",

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Nitrogen(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::NitrogenNitrate(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::NitrogenAmmonium(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Phosphorus(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::PhosphorusPentoxide(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Potassium(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::PotassiumOxide(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Calcium(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::CalciumOxide(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Magnesium(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::MagnesiumOxide(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        gap: "x-small",

                        FertilizerComponentInput {
                            component: label[LabelComponent::Sulfur(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::SulfurTrioxide(0.0)],
                            on_update: props.on_label_component_update,
                        }

                        FertilizerComponentInput {
                            component: label[LabelComponent::SulfurTetroxide(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }

                Row {
                    gap: "small",

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Iron(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Manganese(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Copper(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Zinc(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Boron(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }

                    Column {
                        FertilizerComponentInput {
                            component: label[LabelComponent::Molybdenum(0.0)],
                            on_update: props.on_label_component_update,
                        }
                    }
                }
            }
        }
    }
}
