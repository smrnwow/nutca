use dioxus::prelude::*;

fn cell_style(width: Option<String>) -> String {
    if width.is_some() {
        return format!("width: {}", width.unwrap());
    }

    String::new()
}

#[derive(Props, PartialEq, Clone)]
pub struct TableCellProps {
    width: Option<String>,
    children: Element,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    rsx! {
        td {
            class: "table__cell",
            style: cell_style(props.width),

            {props.children},
        }
    }
}
