use dioxus::prelude::*;

#[component]
pub fn ArrowUp() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M480-528 296-344l-56-56 240-240 240 240-56 56-184-184Z",
            }
        }
    }
}
