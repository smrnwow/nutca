use dioxus::prelude::*;

#[component]
pub fn ArrowRight() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z",
            }
        }
    }
}
