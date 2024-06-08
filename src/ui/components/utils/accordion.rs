use super::icons::ArrowDown;
use super::icons::ArrowUp;
use dioxus::prelude::*;

fn body_class(opened: bool) -> String {
    if opened {
        String::from("accordion__body accordion__body_opened")
    } else {
        String::from("accordion__body")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    opened: Option<bool>,
    header: Element,
    body: Element,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut opened = use_signal(|| {
        if props.opened.is_some() {
            props.opened.unwrap()
        } else {
            false
        }
    });

    rsx! {
        div {
            class: "accordion",

            div {
                class: "accordion__header",

                onclick: move |_| {
                    let is_opened = *opened.read();

                    *opened.write() = !is_opened;
                },

                {props.header},

                if *opened.read() {
                    ArrowUp {}
                } else {
                    ArrowDown {}
                }
            }

            div {
                class: body_class(*opened.read()),

                {props.body},
            }
        }
    }
}
