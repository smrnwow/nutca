use crate::ui::components::utils::icons::Check;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    text: Option<String>,
    value: bool,
    on_change: EventHandler<bool>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        label {
            class: "checkbox checkbox_checked-{props.value}",

            div {
                class: "checkbox__control",

                input {
                    class: "checkbox__input",
                    r#type: "checkbox",
                    value: "{props.value}",
                    checked: "{props.value}",
                    onchange: move |event| {
                        props.on_change.call(event.value().parse().unwrap());
                    },
                }

                button {
                    class: "checkbox__check",
                }

                div {
                    class: "checkbox__icon",
                    Check {},
                }
            }

            if let Some(text) = props.text {
                span {
                    class: "checkbox__text",
                    {text},
                }
            }
        }
    }
}
