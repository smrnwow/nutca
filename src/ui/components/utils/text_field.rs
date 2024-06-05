use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextFieldProps {
    value: String,
    label: String,
    on_input: EventHandler<String>,
}

#[component]
pub fn TextField(props: TextFieldProps) -> Element {
    rsx! {
        label {
            class: "text-field",

            "{props.label}",

            input {
                class: "text-field__input",
                r#type: "text",
                value: props.value,
                oninput: move |event| props.on_input.call(event.value()),
            }
        }
    }
}
