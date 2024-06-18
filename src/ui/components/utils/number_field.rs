use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NumberFieldProps {
    label: Option<String>,
    value: usize,
    units: Option<String>,
    on_change: EventHandler<usize>,
}

#[component]
pub fn NumberField(props: NumberFieldProps) -> Element {
    rsx! {
        div {
            class: "number-field",

            if let Some(label) = props.label {
                span {
                    class: "number-field__label",
                    "{label}",
                }
            }

            label {
                class: "number-field__value",

                input {
                    class: "number-field__input",
                    r#type: "number",
                    value: "{props.value}",
                    oninput: move |event| {
                        props.on_change.call(event.value().parse().unwrap_or(1));
                    },
                }

                if let Some(units) = props.units {
                    span {
                        class: "number-field__units",
                        "{units}",
                    }
                }
            }
        }
    }
}
