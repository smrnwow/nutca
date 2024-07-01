use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    horizontal: Option<String>,
    vertical: Option<String>,
    gap: Option<String>,
    wrap: Option<bool>,
    on_hover: Option<EventHandler<bool>>,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let horizontal = props.horizontal.unwrap_or("start".to_string());

    let vertical = props.vertical.unwrap_or("start".to_string());

    let gap = props.gap.unwrap_or("large".to_string());

    let wrap = props.wrap.unwrap_or(false);

    match props.on_hover {
        Some(on_hover) => rsx! {
            div {
                class: "row row_gap-{gap} row_horizontal-{horizontal} row_vertical-{vertical} row_wrap-{wrap}",
                onmouseover: move |_| on_hover.call(true),
                onmouseout: move |_| on_hover.call(false),
                {props.children},
            }
        },

        None => rsx! {
            div {
                class: "row row_gap-{gap} row_horizontal-{horizontal} row_vertical-{vertical} row_wrap-{wrap}",
                {props.children},
            }
        },
    }
}
