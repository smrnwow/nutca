use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SectionProps {
    children: Element,
}

#[component]
pub fn Section(props: SectionProps) -> Element {
    rsx! {
        section {
            class: "section",
            {props.children},
        }
    }
}
