use dioxus::prelude::*;

fn button_class(style: String) -> String {
    format!("button button_{style}")
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    children: Element,
    style: String,
    on_click: Option<EventHandler<()>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        if props.on_click.is_some() {
            button {
                class: button_class(props.style),
                onclick: move |_| props.on_click.unwrap().call(()),
                {props.children},
            }
        } else {
            button {
                class: button_class(props.style),
                {props.children},
            }
        }
    }
}
