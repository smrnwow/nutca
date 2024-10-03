use super::icons::{ArrowDown, ArrowUp, Check};
use dioxus::prelude::*;

fn select_list_class(opened: bool) -> String {
    /*
    if opened {
        String::from("select__list select__list_opened")
    } else {
        String::from("select__list")
    }
    */

    String::from("select__list")
}

#[derive(Props, PartialEq, Clone)]
pub struct SelProps {
    value: Signal<(String, String)>,
    options: Vec<(String, String)>,
    size: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
pub fn Sel(props: SelProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let mut is_opened = use_signal(|| false);

    rsx! {
        div {
            class: "sel sel_size-{size}",

            label {
                class: "sel__header",

                button {
                    class: "sel__value",
                    onclick: move |_| async move {
                        is_opened.set(true);
                    },
                    onfocusin: move |_| {
                        is_opened.set(true);
                    },
                    {props.value.read().1.clone()},
                }

                div {
                    class: "sel__arrow",

                    if *is_opened.read() {
                        ArrowUp {}
                    } else {
                        ArrowDown {}
                    }
                }
            }

            ul {
                class: "sel__list",
                // class: select_list_class(*is_opened.read()),

                for option in props.options.iter().cloned().map(|o| Signal::new(o)) {
                    li {
                        class: "sel__item",

                        button {
                            class: "sel__button",
                            onclick: move |_| {
                                if option.read().0 != *props.value.read().0 {
                                    props.on_change.call(option.read().0.clone());
                                }

                                is_opened.set(false);
                            },
                            {option.read().1.clone()},
                        }

                        if option.read().0 == props.value.read().0 {
                            div {
                                class: "sel__check",
                                Check {},
                            }
                        }
                    }
                }
            }
        }
    }
}
