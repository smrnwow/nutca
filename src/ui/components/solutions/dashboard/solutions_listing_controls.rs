use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{Button, TextField};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionsListingControlsProps {
    search_query: String,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
}

#[component]
pub fn SolutionsListingControls(props: SolutionsListingControlsProps) -> Element {
    rsx! {
        Row {
            TextField {
                value: props.search_query,
                placeholder: "найти раствор",
                on_input: props.on_search,
                icon_left: rsx! {
                    SearchIcon {}
                },
            }

            Button {
                style: "primary",
                on_click: props.on_add,
                "Добавить раствор",
            }
        }
    }
}
