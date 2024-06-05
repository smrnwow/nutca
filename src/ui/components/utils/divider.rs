use dioxus::prelude::*;

#[component]
pub fn Divider() -> Element {
    rsx! {
        div {
            class: "divider",
        }
    }
}
