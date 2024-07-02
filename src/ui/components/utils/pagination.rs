use crate::ui::components::utils::icons::{ArrowLeft, ArrowRight};
use dioxus::prelude::*;

fn disabled_cell_class(is_disabled: bool) -> String {
    if is_disabled {
        String::from("pagination__cell pagination__cell_disabled")
    } else {
        String::from("pagination__cell pagination__cell_default")
    }
}

fn active_cell_class(page_index: usize, cell_index: String) -> String {
    let cell_index: usize = cell_index.parse().unwrap();

    if cell_index == page_index {
        String::from("pagination__cell pagination__cell_active")
    } else {
        String::from("pagination__cell pagination__cell_default")
    }
}

fn cells(page_index: usize, total: usize, limit: usize) -> Vec<String> {
    let pages_count = (total as f32 / limit as f32).ceil() as usize;

    if pages_count < 3 {
        let mut cells: Vec<String> = Vec::new();

        for page in 1..=pages_count {
            cells.push(page.to_string());
        }

        return cells;
    } else {
        let mut cells: Vec<String> = Vec::new();

        let count_left = page_index - 1;

        let count_right = pages_count - page_index;

        if count_left > 0 {
            if count_right == 0 {
                let prev_page = page_index - 2;

                if prev_page > 0 {
                    cells.push(prev_page.to_string());
                }
            }

            let prev_page = page_index - 1;

            cells.push(prev_page.to_string());
        }

        cells.push(page_index.to_string());

        if count_right > 0 {
            if count_left == 0 {
                let next_page = page_index + 1;

                if next_page < pages_count {
                    cells.push(next_page.to_string());
                }

                let next_page = page_index + 2;

                cells.push(next_page.to_string());
            } else {
                let next_page = page_index + 1;

                cells.push(next_page.to_string());
            }
        }

        return cells;
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    size: Option<String>,
    page_index: usize,
    limit: usize,
    total: usize,
    on_change: EventHandler<usize>,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let page_index = props.page_index;

    let pages_count = (props.total as f32 / props.limit as f32).ceil() as usize;

    let is_first_page = page_index == 1;

    let is_last_page = pages_count == 0 || page_index == pages_count;

    rsx! {
        div {
            class: "pagination pagination_size-{size}",

            button {
                class: disabled_cell_class(is_first_page),
                onclick: move |_| {
                    if !is_first_page {
                        props.on_change.call(page_index - 1);
                    }
                },
                ArrowLeft {},
            }

            for cell in cells(page_index, props.total, props.limit) {
                button {
                    class: active_cell_class(page_index, cell.clone()),
                    onclick: move |_| {
                        props.on_change.call(cell.parse().unwrap());
                    },
                    "{cell}",
                }
            }


            button {
                class: disabled_cell_class(is_last_page),
                onclick: move |_| {
                    if !is_last_page {
                        props.on_change.call(page_index + 1);
                    }
                },
                ArrowRight {},
            }
        }
    }
}
