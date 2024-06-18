use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PageProps {
    children: Element,
}

#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        main {
            class: "page",
            {props.children},
        }
    }
}
