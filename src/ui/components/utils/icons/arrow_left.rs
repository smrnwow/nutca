use dioxus::prelude::*;

#[component]
pub fn ArrowLeft() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M560-240 320-480l240-240 56 56-184 184 184 184-56 56Z",
            }
        }
    }
}
