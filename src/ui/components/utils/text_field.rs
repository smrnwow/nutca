use dioxus::prelude::*;

fn input_class(icon_left: &Option<Element>, icon_right: &Option<Element>) -> String {
    let mut class = String::from("text-field__input");

    if icon_left.is_some() {
        class.push_str(" text-field__input_left-icon");
    }

    if icon_right.is_some() {
        class.push_str(" text-field__input_right-icon");
    }

    class
}

fn value_class(value: &String) -> String {
    if value.len() > 0 {
        String::from("text-field__value text-field__value_filled")
    } else {
        String::from("text-field__value")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TextFieldProps {
    value: String,
    size: Option<String>,
    placeholder: Option<String>,
    label: Option<String>,
    icon_left: Option<Element>,
    icon_right: Option<Element>,
    error: Option<String>,
    on_input: EventHandler<String>,
}

#[component]
pub fn TextField(props: TextFieldProps) -> Element {
    let class = input_class(&props.icon_left, &props.icon_right);

    let size = props.size.unwrap_or(String::from("small"));

    let errored = props.error.is_some();

    rsx! {
        div {
            class: "text-field text-field_size-{size} text-field_error-{errored}",

            if let Some(label) = props.label {
                {label}
            }

            label {
                class: value_class(&props.value),

                div {
                    class: "text-field__icon text-field__icon_left",

                    if let Some(icon_left) = props.icon_left {
                        {icon_left}
                    }
                }

                input {
                    class,
                    r#type: "text",
                    placeholder: props.placeholder.unwrap_or(String::from("")),
                    value: props.value,
                    oninput: move |event| props.on_input.call(event.value()),
                }

                div {
                    class: "text-field__icon text-field__icon_right",

                    if let Some(icon_right) = props.icon_right {
                        {icon_right}
                    }
                }
            }

            if let Some(error) = props.error {
                div {
                    class: "text-field__error",
                    {error},
                }
            }
        }
    }
}
