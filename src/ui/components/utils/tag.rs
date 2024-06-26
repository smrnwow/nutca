use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TagProps {
    text: Option<String>,
    multiple_text: Option<Vec<String>>,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    let text = props.text.unwrap_or(String::new());

    let multiple_text = props.multiple_text.unwrap_or(Vec::new());

    rsx! {
        div {
            class: "tag",

            if text.len() > 0 {
                span {
                    class: "tag__text",
                    {text},
                }
            }

            if multiple_text.len() > 0 {
                for text in multiple_text {
                    span {
                        class: "tag__text",
                        {text},
                    }
                }
            }
        }
    }
}
