use dioxus::prelude::*;

fn button_class(style: String) -> String {
    format!("button button_{style}")
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    text: String,
    style: String,
    on_click: EventHandler<()>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: button_class(props.style),
            onclick: move |_| props.on_click.call(()),
            {props.text},
        }
    }
}
