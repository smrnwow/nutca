use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ColumnProps {
    gap: Option<String>,
    horizontal: Option<String>,
    vertical: Option<String>,
    on_hover: Option<EventHandler<bool>>,
    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let gap = props.gap.unwrap_or("large".to_string());

    let horizontal = props.horizontal.unwrap_or("stretch".to_string());

    let vertical = props.vertical.unwrap_or("start".to_string());

    match props.on_hover {
        Some(on_hover) => rsx! {
            div {
                class: "column column_gap-{gap} column_horizontal-{horizontal} column_vertical-{vertical}",
                onmouseover: move |_| on_hover.call(true),
                onmouseout: move |_| on_hover.call(false),
                {props.children},
            }
        },

        None => rsx! {
            div {
                class: "column column_gap-{gap} column_horizontal-{horizontal} column_vertical-{vertical}",
                {props.children},
            }
        },
    }
}
