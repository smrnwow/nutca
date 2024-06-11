use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    header: Element,
    options: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    rsx! {
        div {
            class: "dropdown",

            div {
                class: "dropdown__header",

                {props.header},
            }

            ul {
                class: "dropdown__list",

                {props.options},
            }
        }
    }
}
