use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableRowProps {
    children: Element,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    rsx! {
        tr {
            class: "table__row",
            {props.children},
        }
    }
}
