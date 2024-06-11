use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DropdownOptionProps {
    children: Element,
    on_click: EventHandler<()>,
}

#[component]
pub fn DropdownOption(props: DropdownOptionProps) -> Element {
    rsx! {
        li {
            class: "dropdown__item",

            button {
                class: "dropdown__button",
                onclick: move |_| props.on_click.call(()),

                {props.children},
            }
        }
    }
}
