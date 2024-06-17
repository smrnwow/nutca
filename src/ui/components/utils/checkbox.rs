use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    value: bool,
    on_change: EventHandler<bool>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        label {
            class: "checkbox",

            input {
                class: "checkbox__input",
                r#type: "checkbox",
                value: "{props.value}",
                checked: "{props.value}",
                onchange: move |event| {
                    props.on_change.call(event.value().parse().unwrap());
                },
            }
        }
    }
}
