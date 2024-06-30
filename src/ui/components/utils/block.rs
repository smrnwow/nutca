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
    on_hover_in: Option<EventHandler<()>>,
    on_hover_out: Option<EventHandler<()>>,
    children: Element,
}

#[component]
pub fn Block(props: BlockProps) -> Element {
    rsx! {
        div {
            class: block_class(props.exclude_padding),
            onmouseover: move |_| {
                if let Some(on_hover_in) = props.on_hover_in {
                    on_hover_in.call(());
                }
            },
            onmouseout: move |_| {
                if let Some(on_hover_out) = props.on_hover_out {
                    on_hover_out.call(());
                }
            },
            {props.children},
        }
    }
}
