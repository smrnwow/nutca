use super::icons::SearchIcon;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SearchProps {
    placeholder: String,
    on_change: EventHandler<String>,
}

#[component]
pub fn Search(props: SearchProps) -> Element {
    rsx! {
        label {
            class: "search",

            SearchIcon {}

            input {
                class: "search__input",
                r#type: "text",
                placeholder: "{props.placeholder}",
                oninput: move |event| props.on_change.call(event.value()),
            }
        }
    }
}
