use dioxus::prelude::*;

#[component]
pub fn Profiles() -> Element {
    rsx! {
        div {
            h1 {
                class: "title",
                "Профили",
            }
        }
    }
}
