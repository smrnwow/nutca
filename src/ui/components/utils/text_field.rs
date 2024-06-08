use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextFieldProps {
    value: String,
    label: Option<String>,
    placeholder: Option<String>,
    on_input: EventHandler<String>,
}

#[component]
pub fn TextField(props: TextFieldProps) -> Element {
    rsx! {
        label {
            class: "text-field",

            if props.label.is_some() {
                "{props.label.unwrap()}"
            }

            input {
                class: "text-field__input",
                r#type: "text",
                placeholder: props.placeholder.unwrap_or(String::from("")),
                value: props.value,
                oninput: move |event| props.on_input.call(event.value()),
            }
        }
    }
}
