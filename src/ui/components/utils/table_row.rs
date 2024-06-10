use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableRowProps {
    children: Element,
    on_click: Option<EventHandler<()>>,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    rsx! {
        if props.on_click.is_some() {
            tr {
                class: "table__row table__row_clickable",
                onclick: move |_| props.on_click.unwrap().call(()),
                {props.children},
            }
        } else {
            tr {
                class: "table__row",
                {props.children},
            }
        }
    }
}
