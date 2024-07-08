use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ListProps {
    children: Element,
    size: Option<String>,
    limit: Option<usize>,
    empty: Option<bool>,
    stub_text: Option<String>,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let limit = props.limit.unwrap_or(0);

    let size = props.size.unwrap_or(String::from("small"));

    let empty = props.empty.unwrap_or(false);

    let stub_text = props.stub_text.unwrap_or(String::from("Список пуст"));

    rsx! {
        div {
            class: "list list_size-{size} list_limit-{limit}",

            if empty {
                div {
                    class: "list__stub",
                    {stub_text},
                }
            } else {
                {props.children}
            }
        }
    }
}
