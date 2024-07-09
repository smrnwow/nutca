use dioxus::prelude::*;

#[component]
pub fn Exclamation() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 -960 960 960",

            path {
                d: "M440-400v-360h80v360h-80Zm0 200v-80h80v80h-80Z",
            }
        }
    }
}
