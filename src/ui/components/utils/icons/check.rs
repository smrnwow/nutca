use dioxus::prelude::*;

#[component]
pub fn Check() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M382-240 154-468l57-57 171 171 367-367 57 57-424 424Z",
            }
        }
    }
}
