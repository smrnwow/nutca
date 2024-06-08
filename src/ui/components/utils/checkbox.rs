use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    checked: bool,
    on_change: EventHandler<Event<FormData>>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    rsx! {
        label {
            class: "checkbox",

            input {
                class: "checkbox__input",
                r#type: "checkbox",
                value: "{props.checked}",
                onchange: move |event| props.on_change.call(event),
            }
        }
    }
}
