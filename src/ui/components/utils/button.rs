use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    children: Element,
    size: Option<String>,
    style: Option<String>,
    hidden: Option<Signal<bool>>,
    on_click: Option<EventHandler<()>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let style = props.style.unwrap_or(String::from("default"));

    let size = props.size.unwrap_or(String::from("small"));

    let hidden = props.hidden.unwrap_or(Signal::new(false));

    match props.on_click {
        Some(on_click) => rsx! {
            button {
                class: "button button_{style} button_size-{size} button_hidden-{*hidden.read()}",
                onclick: move |_| on_click.call(()),
                {props.children},
            }
        },

        None => rsx! {
            button {
                class: "button button_{style} button_size-{size} button_hidden-{*hidden.read()}",
                {props.children},
            }
        },
    }
}
