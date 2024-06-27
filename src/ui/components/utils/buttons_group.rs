use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonsGroupProps {
    children: Element,
}

#[component]
pub fn ButtonsGroup(props: ButtonsGroupProps) -> Element {
    rsx! {
        div {
            class: "buttons-group",
            {props.children},
        }
    }
}
