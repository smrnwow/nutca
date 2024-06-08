use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StepProps {
    number: i32,
    text: String,
}

#[component]
pub fn Step(props: StepProps) -> Element {
    rsx! {
        div {
            class: "step",

            span {
                class: "step__number",
                "{props.number}"
            }

            h6 {
                class: "step__text",
                "{props.text}"
            }
        }
    }
}
