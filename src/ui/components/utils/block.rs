use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BlockProps {
    children: Element,
}

#[component]
pub fn Block(props: BlockProps) -> Element {
    rsx! {
        div {
            class: "block",
            {props.children},
        }
    }
}
