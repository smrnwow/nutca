use crate::controller::concentrates::FertilizersStack;
use crate::model::concentrates::parts::AutoPart;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::{Check, Close, Plus};
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, NumberField, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerPercentButtonProps {
    auto_part: Signal<AutoPart>,
    fertilizers_stack: Memo<FertilizersStack>,
    on_fertilizer_add: EventHandler<(String, String, f64)>,
}

#[component]
pub fn FertilizerPercentButton(props: FertilizerPercentButtonProps) -> Element {
    let list = props
        .fertilizers_stack
        .read()
        .stack()
        .into_iter()
        .map(|item| Signal::new(item));

    let mut selected_fertilizer_id = use_signal(|| String::new());
    let mut selected_fertilizer_name = use_signal(|| String::new());
    let mut selected_percent = use_signal(|| 0);
    let mut available_percent = use_signal(|| 0);

    rsx! {
        div {
            class: "concentrate-part__add-fertilizer",

            if selected_fertilizer_id.read().is_empty() {
                Dropdown {
                    header: rsx! {
                        button {
                            class: "concentrate-part__add-fertilizer-button",
                            Plus {},
                            Text {
                                size: "x-small",
                                "добавить удобрение",
                            },
                        }
                    },

                    options: rsx! {
                        for item in list {
                            DropdownOption {
                                key: "{item.read().0.id()}",
                                on_click: move |_| {
                                    selected_fertilizer_id.set(item.read().0.id());
                                    selected_fertilizer_name.set(item.read().0.name());
                                    selected_percent.set(item.read().1);
                                    available_percent.set(item.read().1);
                                },
                                "{item.read().0.name()} {item.read().1}",
                            }
                        }
                    }
                }
            } else {
                Row {
                    gap: "x-small",

                    Text {
                        size: "x-small",
                        {selected_fertilizer_name},
                    }

                    NumberField {
                        maximum: *available_percent.read(),
                        value: *selected_percent.read(),
                        on_change: move |value| {
                            selected_percent.set(value);
                        }
                    }

                    Button {
                        style: "compact",
                        on_click: move |_| {
                            let part_id = props.auto_part.read().id().clone();
                            let fertilizer_id = selected_fertilizer_id.read().clone();
                            let percent = *selected_percent.read();

                            props.on_fertilizer_add.call((part_id, fertilizer_id, percent as f64));

                            selected_fertilizer_id.set(String::new());
                            selected_fertilizer_name.set(String::new());
                            selected_percent.set(0);
                            available_percent.set(0);
                        },
                        Check {},
                    }

                    Button {
                        style: "compact",
                        on_click: move |_| {
                            selected_fertilizer_id.set(String::new());
                            selected_fertilizer_name.set(String::new());
                            selected_percent.set(0);
                            available_percent.set(0);
                        },
                        Close {},
                    }
                }
            }
        }
    }
}
