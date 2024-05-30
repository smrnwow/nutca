use dioxus::prelude::*;

#[component]
pub fn Reference() -> Element {
    rsx! {
        div {
            h1 {
                class: "title",
                "Справка"
            }
        }
    }
}
