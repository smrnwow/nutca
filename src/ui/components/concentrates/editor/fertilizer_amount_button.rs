use crate::controller::concentrates::FertilizersBrowser;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::{Check, Close, Plus};
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, FloatField, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerAmountButtonProps {
    fertilizers_browser: Memo<FertilizersBrowser>,
    on_fertilizer_add: EventHandler<(String, f64)>,
}

#[component]
pub fn FertilizerAmountButton(props: FertilizerAmountButtonProps) -> Element {
    let mut selected_fertilizer_id = use_signal(|| String::new());
    let mut selected_fertilizer_name = use_signal(|| String::new());
    let mut selected_weight = use_signal(|| 0.0);

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
                        for fertilizer in props.fertilizers_browser.read().fetch().into_iter().map(|fertilizer| Signal::new(fertilizer)) {
                            DropdownOption {
                                key: "{fertilizer.read().id()}",
                                on_click: move |_| {
                                    selected_fertilizer_id.set(fertilizer.read().id());
                                    selected_fertilizer_name.set(fertilizer.read().name());
                                },
                                {fertilizer.read().name()},
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

                    FloatField {
                        maximum: 1000000.0,
                        value: *selected_weight.read(),
                        on_change: move |value| {
                            selected_weight.set(value);
                        }
                    }

                    Button {
                        style: "compact",
                        on_click: move |_| {
                            let fertilizer_id = selected_fertilizer_id.read().clone();
                            let weight = *selected_weight.read();

                            props.on_fertilizer_add.call((fertilizer_id, weight));

                            selected_fertilizer_id.set(String::new());
                            selected_fertilizer_name.set(String::new());
                            selected_weight.set(0.0);
                        },
                        Check {},
                    }

                    Button {
                        style: "compact",
                        on_click: move |_| {
                            selected_fertilizer_id.set(String::new());
                            selected_fertilizer_name.set(String::new());
                            selected_weight.set(0.0);
                        },
                        Close {},
                    }
                }
            }
        }
    }
}
