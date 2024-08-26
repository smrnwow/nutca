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
    gap: Option<String>,
    gap_vertical: Option<String>,
    gap_horizontal: Option<String>,
    on_hover_in: Option<EventHandler<()>>,
    on_hover_out: Option<EventHandler<()>>,
    children: Element,
}

#[component]
pub fn Block(props: BlockProps) -> Element {
    let gap = props.gap.unwrap_or(String::from("large"));

    let gap_vertical = props.gap_vertical.unwrap_or(gap.clone());

    let gap_horizontal = props.gap_horizontal.unwrap_or(gap.clone());

    rsx! {
        div {
            class: "block block-gap_{gap} block-gap-vertical_{gap_vertical} block-gap-horizontal_{gap_horizontal}",
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
