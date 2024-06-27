use dioxus::prelude::*;

#[component]
pub fn Plus() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z",
            }
        }
    }
}
