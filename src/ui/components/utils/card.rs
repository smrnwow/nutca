use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    children: Element,
    on_hover: Option<EventHandler<bool>>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        match props.on_hover {
            Some(on_hover) => rsx! {
                div {
                    class: "card",
                    onmouseover: move |_| on_hover.call(true),
                    onmouseout: move |_| on_hover.call(false),
                    {props.children},
                },
            },

            None => rsx! {
                div {
                    class: "card",
                    {props.children},
                },
            },
        }
    }
}
