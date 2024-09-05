use crate::ui::components::utils::icons::{Minus, Plus};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NumberFieldProps {
    maximum: Option<usize>,
    size: Option<String>,
    label: Option<String>,
    value: usize,
    units: Option<String>,
    on_change: EventHandler<usize>,
}

#[component]
pub fn NumberField(props: NumberFieldProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let maximum = props.maximum.unwrap_or(100000);

    rsx! {
        div {
            class: "number-field number-field_size-{size} number-field_controls-true",

            if let Some(label) = props.label {
                span {
                    class: "number-field__label",
                    "{label}",
                }
            }

            label {
                class: "number-field__value",

                button {
                    class: "number-field__button number-field__button_left",
                    onclick: move |_| {
                        if props.value >= 1 {
                            props.on_change.call(props.value - 1);
                        }
                    },
                    Minus {},
                }

                input {
                    class: "number-field__input",
                    r#type: "text",
                    size: 1,
                    value: "{props.value}",
                    oninput: move |event| {
                        let value = event.value().parse().unwrap_or(1);

                        if value < maximum {
                            props.on_change.call(value);
                        } else {
                            props.on_change.call(maximum);
                        }
                    },
                }

                button {
                    class: "number-field__button number-field__button_right",
                    onclick: move |_| {
                        if props.value < maximum {
                            props.on_change.call(props.value + 1);
                        }
                    },
                    Plus {},
                }

                /*
                if let Some(units) = props.units {
                    span {
                        class: "number-field__units",
                        "{units}",
                    }
                }
                */
            }
        }
    }
}
