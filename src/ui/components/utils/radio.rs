use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    name: String,
    value: String,
    checked: bool,
    text: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let value = use_signal(|| props.value);

    rsx! {
        label {
            class: "radio radio_checked-{props.checked}",

            input {
                class: "radio__input",
                r#type: "radio",
                name: props.name,
                value,
                checked: props.checked,
                onchange: move |_| props.on_change.call(value.read().clone()),
            }

            span {
                class: "radio__control",

                span {
                    class: "radio__control_inner",
                }
            }

            if let Some(text) = props.text {
                span {
                    class: "radio__text",
                    {text},
                }
            }
        }
    }
}
