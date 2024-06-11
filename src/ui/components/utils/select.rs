use super::icons::{ArrowDown, ArrowUp, Check, Close, SearchIcon};
use dioxus::prelude::*;

fn select_list_class(opened: bool) -> String {
    if opened {
        String::from("select__list select__list_opened")
    } else {
        String::from("select__list")
    }
}

fn select_item_class(selected: bool) -> String {
    if selected {
        String::from("select__item select__item_selected")
    } else {
        String::from("select__item")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    placeholder: Option<String>,
    value: (String, String),
    options: Vec<(String, String)>,
    on_search: Option<EventHandler<String>>,
    on_select: EventHandler<String>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut search_query = use_signal(|| String::new());

    let mut opened = use_signal(|| false);

    let placeholder = props.placeholder.unwrap_or(String::new());

    let (value, text) = props.value.clone();

    let render_value = value != String::from("") && !*opened.read();

    let render_search = !render_value && props.on_search.is_some();

    let render_placeholder = !render_value && !render_search;

    rsx! {
        div {
            class: "select",

            div {
                class: "select__header",

                if render_value {
                    button {
                        class: "select__value",
                        onclick: move |_| {
                            *opened.write() = true;
                        },

                        "{text}",
                    }
                }

                if render_search {
                    label {
                        class: "select__search",
                        onclick: move |_| {
                            *opened.write() = true;
                        },
                        /*
                        onfocusout: move |_| {
                            *opened.write() = false;
                        },
                        */

                        div {
                            class: "select__icon",

                            SearchIcon {}
                        }

                        input {
                            class: "select__input",
                            r#type: "text",
                            placeholder: "{placeholder}",
                            value: "{search_query}",
                            oninput: move |event| {
                                *search_query.write() = event.value();

                                props.on_search.unwrap().call(search_query.read().clone())
                            },
                        }
                    }
                }

                if render_placeholder {
                        button {
                            class: "select__placeholder",
                            onclick: move |_| {
                                *opened.write() = true;
                            },

                            "{placeholder}"
                        }
                    }
                }

                div {
                    class: "select__controls",

                    if value != String::new() {
                        button {
                            class: "select__cancel",
                            onclick: move |_| props.on_cancel.call(()),

                            Close {}
                        }
                    }

                    div {
                        class: "select__arrow",

                        if *opened.read() {
                            ArrowUp {}
                        } else {
                            ArrowDown {}
                        }
                    }
                }
            }

            ul {
                class: select_list_class(*opened.read()),

                for (option_value, option_text) in props.options.clone() {
                    li {
                        class: select_item_class(option_value == value),

                        button {
                            class: "select__button",
                            onclick: move |_| {
                                *opened.write() = false;

                                props.on_select.call(option_value.clone());
                            },

                            "{option_text}",
                        }

                        if value == option_value {
                            div {
                                class: "select__check",

                                Check {}
                            }
                        }
                    }
                }
            }
    }
}
