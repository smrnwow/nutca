use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LabelProps {
    text: String,
    children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label {
            class: "label",

            span {
                class: "label__text",
                {props.text},
            }

            {props.children},
        }
    }
}
