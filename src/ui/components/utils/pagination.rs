use crate::ui::components::utils::icons::{ArrowLeft, ArrowRight};
use dioxus::prelude::*;

fn disabled_cell_class(is_disabled: bool) -> String {
    if is_disabled {
        String::from("pagination__cell pagination__cell_disabled")
    } else {
        String::from("pagination__cell pagination__cell_default")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    size: Option<String>,
    page_index: usize,
    items_count: usize,
    limit: usize,
    on_change: EventHandler<usize>,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let is_first_page = props.page_index == 1;

    let is_last_page = props.items_count < props.limit;

    rsx! {
        div {
            class: "pagination pagination_size-{size}",

            button {
                class: disabled_cell_class(is_first_page),
                onclick: move |_| {
                    if !is_first_page {
                        props.on_change.call(props.page_index - 1);
                    }
                },
                ArrowLeft {},
            }

            button {
                class: disabled_cell_class(is_last_page),
                onclick: move |_| {
                    if !is_last_page {
                        props.on_change.call(props.page_index + 1);
                    }
                },
                ArrowRight {},
            }
        }
    }
}
