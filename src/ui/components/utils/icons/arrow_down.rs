use dioxus::prelude::*;

#[component]
pub fn ArrowDown() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M480-344 240-584l56-56 184 184 184-184 56 56-240 240Z",
            }
        }
    }
}
