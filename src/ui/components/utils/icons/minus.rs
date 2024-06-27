use dioxus::prelude::*;

#[component]
pub fn Minus() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M200-440v-80h560v80H200Z",
            }
        }
    }
}
