use super::icons::{ArrowDown, ArrowUp, Check};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    value: Memo<(String, String)>,
    options: Vec<(String, String)>,
    size: Option<String>,
    on_change: EventHandler<String>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let mut is_opened = use_signal(|| false);

    rsx! {
        div {
            class: "select select_size-{size}",

            label {
                class: "select__header",

                button {
                    class: "select__value",
                    onclick: move |_| async move {
                        is_opened.set(true);
                    },
                    onfocusin: move |_| {
                        is_opened.set(true);
                    },
                    {props.value.read().1.clone()},
                }

                div {
                    class: "select__controls",

                    div {
                        class: "select__arrow",

                        if *is_opened.read() {
                            ArrowUp {}
                        } else {
                            ArrowDown {}
                        }
                    }
                }
            }

            ul {
                class: "select__list",

                for option in props.options.iter().cloned().map(|o| Signal::new(o)) {
                    li {
                        class: "select__item",

                        button {
                            class: "select__button",
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
                                class: "select__check",
                                Check {},
                            }
                        }
                    }
                }
            }
        }
    }
}
