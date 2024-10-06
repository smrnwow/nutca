use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    value: String,
    size: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    rsx! {
        input {
            class: "input input_size-{size}",
            r#type: "text",
            value: props.value,
            oninput: move |event| {
                props.on_change.call(event.value());
            },
        }
    }
}
