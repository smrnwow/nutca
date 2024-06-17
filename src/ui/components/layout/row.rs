use dioxus::prelude::*;

fn row_class(align: Option<String>, gap: Option<String>) -> String {
    let mut class = String::from("row");

    if let Some(align) = align {
        class.push_str(format!(" row_align-{}", align).as_str());
    }

    if let Some(gap) = gap {
        class.push_str(format!(" row_gap-{}", gap).as_str())
    }

    class
}

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    align: Option<String>,
    gap: Option<String>,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div {
            class: row_class(props.align, props.gap),
            {props.children},
        }
    }
}
