use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    align: Option<String>,
    gap: Option<String>,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let align = props.align.unwrap_or("start".to_string());

    let gap = props.gap.unwrap_or("large".to_string());

    rsx! {
        div {
            class: "row row_gap-{gap} row_align-{align}",
            {props.children},
        }
    }
}
