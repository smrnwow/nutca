use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    header: Element,
    body: Element,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    rsx! {
        table {
            class: "table",

            thead {
                class: "table__header",

                tr {
                    {props.header},
                }
            }

            tbody {
                class: "table__body",
                {props.body},
            }
        }
    }
}
