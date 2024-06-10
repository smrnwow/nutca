use dioxus::prelude::*;

fn block_class(exclude_padding: Option<String>) -> String {
    match exclude_padding {
        Some(padding_side) => format!("block block_exclude-{padding_side}"),
        None => String::from("block"),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct BlockProps {
    exclude_padding: Option<String>,
    children: Element,
}

#[component]
pub fn Block(props: BlockProps) -> Element {
    rsx! {
        div {
            class: block_class(props.exclude_padding),
            {props.children},
        }
    }
}
