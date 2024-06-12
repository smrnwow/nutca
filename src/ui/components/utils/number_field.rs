use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NumberFieldProps {
    value: usize,
    units: String,
    on_change: EventHandler<usize>,
}

#[component]
pub fn NumberField(props: NumberFieldProps) -> Element {
    rsx! {
        label {
            class: "number-field",

            input {
                class: "number-field__input",
                r#type: "number",
                value: "{props.value}",
                oninput: move |event| {
                    props.on_change.call(event.value().parse().unwrap());
                },
            }

            span {
                class: "number-field__units",
                "{props.units}",
            }
        }
    }
}
