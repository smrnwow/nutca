use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReferenceSubjectProps {
    children: Element,
}

#[component]
pub fn ReferenceSubject(props: ReferenceSubjectProps) -> Element {
    rsx! {
        div {
            class: "reference-subject",
            {props.children},
        }
    }
}
